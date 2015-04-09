# Graphs and arena allocation

(Note you can run the examples in this chapter by downloading this directory and
running `cargo run`).

Graphs are a bit awkward to construct in Rust because of Rust's stringent
lifetime and mutability requirements. Graphs of objects are very common in OO
programming. In this tutorial I'm going to go over a few different approaches to
implementation. My preferred approach uses arena allocation and makes slightly
advanced use of explicit lifetimes. I'll finish up by discussing a few potential
Rust features which would make using such an approach easier.

A [graph](http://en.wikipedia.org/wiki/Graph_%28abstract_data_type%29) is a
collection of nodes with edges between some of those nodes. Graphs are a
generalisation of lists and trees. Each node can have multiple children and
multiple parents (we usually talk about edges into and out of a node, rather
than parents/children). Graphs can be represented by adjacency lists or
adjacency matrices. The former is basically a node object for each node in the
graph, where each node object keeps a list of its adjacent nodes. An adjacency
matrix is a matrix of booleans indicating whether there is an edge from the row
node to the column node. We'll only cover the adjacency list representation,
adjacency matrices have very different issues which are less Rust-specific.

There are essentially two orthogonal problems: how to handle the lifetime of the
graph and how to handle it's mutability.

The first problem essentially boils down to what kind of pointer to use to point
to other nodes in the graph. Since graph-like data structures are recursive (the
types are recursive, even if the data is not) we are forced to use pointers of
some kind rather than have a totally value-based structure. Since graphs can be
cyclic, and ownership in Rust cannot be cyclic, we cannot use `Box<Node>` as our
pointer type (as we might do for tree-like data structures or linked lists).

No graph is truly immutable. Because there may be cycles, the graph cannot be
created in a single statement. Thus, at the very least, the graph must be mutable
during its initialisation phase. The usual invariant in Rust is that all
pointers must either be unique or immutable. Graph edges must be mutable (at
least during initialisation) and there can be more than one edge into any node,
thus no edges are guaranteed to be unique. So we're going to have to do
something a little bit advanced to handle mutability.

One solution is to use mutable raw pointers (`*mut Node`). This is the most
flexible approach, but also the most dangerous. You must handle all the lifetime
management yourself without any help from the type system. You can make very
flexible and efficient data structures this way, but you must be very careful.
This approach handles both the lifetime and mutability issues in one fell swoop.
But it handles them by essentially ignoring all the benefits of Rust - you will
get no help from the compiler here (it's also not particularly ergonomic since
raw pointers don't automatically (de-)reference). Since a graph using raw
pointers is not much different from a graph in C++, I'm not going to cover that
option here.

The options you have for lifetime management are reference counting (shared
ownership, using `Rc<...>`) or arena allocation (all nodes have the same lifetime,
managed by an arena; using borrowed references `&...`). The former is
more flexible (you can have references from outside the graph to individual
nodes with any lifetime), the latter is better in every other way.

For managing mutability, you can either use `RefCell`, i.e., make use of Rust's
facility for dynamic, interior mutability, or you can manage the mutability
yourself (in this case you have to use `UnsafeCell` to communicate the interior
mutability to the compiler). The former is safer, the latter is more efficient.
Neither is particularly ergonomic.

Note that if your graph might have cycles, then if you use `Rc`, further action
is required to break the cycles and not leak memory. Since Rust has no cycle
collection of `Rc` pointers, if there is a cycle in your graph, the ref counts
will never fall to zero, and the graph will never be deallocated. You can solve
this by using `Weak` pointers in your graph or by manually breaking cycles when
you know the graph should be destroyed. The former is more reliable. We don't
cover either here, in our examples we just leak memory. The approach using
borrowed references and arena allocation does not have this issue and is thus
superior in that respect.

To compare the different approaches I'll use a pretty simple example. We'll just
have a `Node` object to represent a node in the graph, this will hold some
string data (representative of some more complex data payload) and a `Vec` of
adjacent nodes (`edges`). We'll have an `init` function to create a simple graph
of nodes, and a `traverse` function which does a pre-order, depth-first
traversal of the graph. We'll use this to print the payload of each node in the
graph. Finally, we'll have a `Node::first` method which returns a reference to
the first adjacent node to the `self` node and a function `foo` which prints the
payload of an individual node. These functions stand in for more complex
operations involving manipulation of a node interior to the graph.

To try and be as informative as possible without boring you, I'll cover two
combinations of possibilities: ref counting and `RefCell`, and arena allocation
and `UnsafeCell`. I'll leave the other two combinations as an exercise.


## `Rc<RefCell<Node>>`

See [full example](https://github.com/nrc/r4cppp/blob/master/graphs/src/rc_graph.rs).

This is the safer option because there is no unsafe code. It is also the least
efficient and least ergonomic option. It is pretty flexible though, nodes of the
graph can be easily reused outside the graph since they are ref-counted. I would
recommend this approach if you need a fully mutable graph, or need your nodes to
exist independently of the graph.

The node structure looks like

```rust
struct Node {
    datum: &'static str,
    edges: Vec<Rc<RefCell<Node>>>,
}
```

Creating a new node is not too bad: `Rc::new(RefCell::new(Node { ... }))`. To
add an edge during initialisation, you have to borrow the start node as mutable,
and clone the end node into the Vec of edges (this clones the pointer,
incrementing the reference count, not the actual node). E.g.,

```rust
let mut mut_root = root.borrow_mut();
mut_root.edges.push(b.clone());
```

The `RefCell` dynamically ensures that we are not already reading or writing the
node when we write it.

Whenever you access a node, you have to use `.borrow()` to borrow the `RefCell`.
Our `first` method has to return a ref-counted pointer, rather than a borrowed
reference, so callers of `first` also have to borrow:

```rust
fn first(&self) -> Rc<RefCell<Node>> {
    self.edges[0].clone()
}

pub fn main() {
    let g = ...;
    let f = g.first();
    foo(&*f.borrow());
}
```


## `&Node` and `UnsafeCell`

See [full example](https://github.com/nrc/r4cppp/blob/master/graphs/src/ref_graph.rs).

In this approach we use borrowed references as edges. This is nice and ergonomic
and lets us use our nodes with 'regular' Rust libraries which primarily operate
with borrowed references (note that one nice thing about ref counted objects in
Rust is that they play nicely with the lifetime system. We can create a borrowed
reference into the `Rc` to directly (and safely) reference the data. In the
previous example, the `RefCell` prevents us doing this, but an `Rc`/`UnsafeCell`
approach should allow it).

Destruction is correctly handled too - the only constraint is that all the nodes
must be destroyed at the same time. Destruction and allocation of nodes is
handled using an arena.

On the other hand, we do need to use quite a few explicit lifetimes.
Unfortunately we don't benefit from lifetime elision here. At the end of the
section I'll discuss some future directions for the language which could make
things better.

During construction we will mutate our nodes which might be multiply referenced.
This is not possible in safe Rust code, so we must initialise inside an `unsafe`
block. Since our nodes are mutable and multiply referenced, we must use an
`UnsafeCell` to communicate to the Rust compiler that it cannot rely on its
usual invariants.

When is this approach feasible? The graph must only be mutated during
initialisation. In addition, we require that all nodes in the graph have the
same lifetime (we could relax these constraints somewhat to allow adding nodes
later as long as they can all be destroyed at the same time). Similarly, we
could rely on more complicated invariants for when the nodes can be mutated, but
it pays to keep things simple, since the programmer is responsible for safety
in those respects.

Arena allocation is a memory management technique where a set of objects have
the same lifetime and can be deallocated at the same time. An arena is an object
responsible for allocating and deallocating the memory. Since large chunks of
memory are allocated and deallocated at once (rather than allocating individual
objects), arena allocation is very efficient. Usually, all the objects are
allocated from a contiguous chunk of memory, that improves cache coherency when
you are traversing the graph.

In Rust, arena allocation is supported by the [libarena](https://doc.rust-lang.org/arena/index.html)
crate and is used throughout the compiler. There are two kinds of arenas - typed
and untyped. The former is more efficient and easier to use, but can only
allocate objects of a single type. The latter is more flexible and can allocate
any object. Arena allocated objects all have the same lifetime, which is a
parameter of the arena object. The type system ensures references to arena
allocated objects cannot live longer than the arena itself.

Our node struct must now include the lifetime of the graph, `'a`. We wrap our
`Vec` of adjacent nodes in an `UnsafeCell` to indicate that we will mutate it
even when it should be immutable:

```rust
struct Node<'a> {
    datum: &'static str,
    edges: UnsafeCell<Vec<&'a Node<'a>>>,
}
```

Our new function must also use this lifetime and must take as an argument the
arena which will do the allocation:

```rust
fn new<'a>(datum: &'static str, arena: &'a TypedArena<Node<'a>>) -> &'a Node<'a> {
    arena.alloc(Node {
        datum: datum,
        edges: UnsafeCell::new(Vec::new()),
    })
}
```

We use the arena to allocate the node. The lifetime of the graph is derived from
the lifetime of the reference to the arena, so the arena must be passed in from
the scope which covers the graph's lifetime. For our examples, that means we
pass it into the `init` method. (One could imagine an extension to the type
system which allows creating values at scopes outside their lexical scope, but
there are no plans to add such a thing any time soon). When the arena goes out
of scope, the whole graph is destroyed (Rust's type system ensures that we can't
keep references to the graph beyond that point).

Adding an edge is a bit different looking:

```rust
(*root.edges.get()).push(b);
```

We're essentially doing the obvious `root.edges.push(b)` to push a node (`b`) on
to the list of edges. However, since `edges` is wrapped in an `UnsafeCell`, we
have to call `get()` on it. That gives us a mutable raw pointer to edges (`*mut
Vec<&Node>`), which allows us to mutate `edges`. However, it also requires us to
manually dereference the pointer (raw pointers do not auto-deref), thus the
`(*...)` construction. Finally, dereferencing a raw pointer is unsafe, so the
whole lot has to be wrapped up in an unsafe block.

The interesting part of `traverse` is:

```rust
for n in &(*self.edges.get()) {
    n.traverse(f, seen);
}
```

We follow the previous pattern for getting at the edges list, which requires an
unsafe block. In this case we know it is in fact safe because we must be post-
initialisation and thus there will be no mutation.

Again, the `first` method follows the same pattern for getting at the `edges`
list. And again must be in an unsafe block. However, in contrast to the graph
using `Rc<RefCell<_>>`, we can return a straightforward borrowed reference to
the node. That is very convenient. We can reason that the unsafe block is safe
because we do no mutation and we are post-initialisation.

```rust
fn first(&'a self) -> &'a Node<'a> {
    unsafe {
        (*self.edges.get())[0]
    }
}
```

### Future language improvements for this approach

I believe that arena allocation and using borrowed references are an important
pattern in Rust. We should do more in the language to make these patterns safer
and easier to use. I hope use of arenas becomes more ergonomic with the ongoing
work on [allocators](https://github.com/rust-lang/rfcs/pull/244). There are
three other improvements I see:

#### Safe initialisation

There has been lots of research in the OO world on mechanisms for ensuring
mutability only during initialisation. How exactly this would work in Rust is an
open research question, but it seems that we need to represent a pointer which
is mutable and not unique, but restricted in scope. Outside that scope any
existing pointers would become normal borrowed references, i.e., immutable *or*
unique.

The advantage of such a scheme is that we have a way to represent the common
pattern of mutable during initialisation, then immutable. It also relies on the
invariant that, while individual objects are multiply owned, the aggregate (in
this case a graph) is uniquely owned. We should then be able to adopt the
reference and `UnsafeCell` approach, without the `UnsafeCell`s and the unsafe
blocks, making that approach more ergonomic and more safer.

Alex Summers and Julian Viereck at ETH Zurich are investigating this
further.


#### Generic modules

The 'lifetime of the graph' is constant for any particular graph. Repeating the
lifetime is just boilerplate. One way to make this more ergonomic would be to
allow the graph module to be parameterised by the lifetime, so it would not need
to be added to every struct, impl, and function. The lifetime of the graph would
still need to be specified from outside the module, but hopefully inference
would take care of most uses (as it does today for function calls).

See [ref_graph_generic_mod.rs](https://github.com/nrc/r4cppp/blob/master/graphs/src/ref_graph_generic_mod.rs) for how that might look.
(We should also be able to use safe initialisation (proposed above) to remove
the unsafe code).

See also this [RFC issue](https://github.com/rust-lang/rfcs/issues/424).

This feature would vastly reduce the syntactic overhead of the reference and
`UnsafeCell` approach.


#### Lifetime elision

We currently allow the programmer to elide some lifetimes in function signatures
to improve ergonomics. One reason the `&Node` approach to graphs is a bit ugly
is because it doesn't benefit from any of the lifetime elision rules.

A common pattern in Rust is data structures with a common lifetime. References
into such data structures give rise to types like `&'a Foo<'a>`, for example
`&'a Node<'a>` in the graph example. It would be nice to have an elision
rule that helps in this case. I'm not really sure how it should work though.

Looking at the example with generic modules, it doesn't look like we need to
extend the lifetime elision rules very much (I'm not actually sure if
`Node::new` would work without the given lifetimes, but it seems like a fairly
trivial extension to make it work if it doesn't). We might want to add some new
rule to allow elision of module-generic lifetimes if they are the only ones in
scope (other than `'static`), but I'm not sure how that would work with multiple
in- scope lifetimes (see the `foo` and `init` functions, for example).

If we don't add generic modules, we might still be able to add an elision rule
specifically to target `&'a Node<'a>`, not sure how though.
