# Graphs and arena allocation

(Note you can run the examples in this chapter by downloading this directory and
running `cargo run`).

Graphs are slightly awkward to construct in Rust because of Rust's stringent
lifetime requirements. Graphs of objects are very common in OO programming. In
this tutorial I'm going to go over a few different approaches to implementation.
My preferred approach uses arena allocation and makes slightly advanced use of
explicit lifetimes. I'll finish up by discussing a few potential Rust features
which would make using these kinds of data structures easier.

A [graph](http://en.wikipedia.org/wiki/Graph_%28abstract_data_type%29) is a
collection of nodes with edges between some of those nodes. Graphs are a
generalisation of lists and trees. Each node can have multiple children and
multiple parents (we usually talk about edges into and out of a node, rather
than parents/children). Graphs can be represented by adjacency lists or
adjacency matrices. The former is basically a node object for each node in the
graph, where each node object keeps a list of its adjacent nodes. An adjacency
matrix is a matrix of booleans indicating whether there is an edge from the row
node to the column node. We'll only cover the adjacency list representation -
adjacency matrices have very different issues which are less Rust-specific.

Since graph-like data structures are recursive (the types are recursive, even if
the data is not) we are forced to use pointers of some kind rather than have a
totally value-based structure. Since graphs can be cyclic, and ownership in Rust
cannot be cyclic, we cannot use `Box<Node>` as our pointer type (as we might do
for tree-like data structures or linked lists).

That leaves us with three options: reference counting, borrowed references, or
raw (unsafe) pointers. The reference counted graph can be approached in two ways
- either using `RefCell` or using unsafe code. The former is safer, the latter
is more ergonomic.

Note that if your graph might have cycles, then the `Rc` graphs require further
action to break the cycles and not leak memory. Since Rust has no cycle
collection of `Rc` pointers, if there is a cycle in your graph, the ref counts
will never fall to zero, and the graph will never be deallocated. You can solve
this by using `Weak` pointers in your graph or by manually breaking cycles when
you know the graph should be destroyed. The former is more reliable. We don't
cover either here, in our examples we just leak memory. The approach using
borrowed references and arena allocation does not have this issue and is thus
superior in that respect.

None of these options are totally 'idiomatic Rust' - you either use reference
counting or have some unsafe code. But, that is Ok, this is pragmatic Rust and
the fact that you can do this is a reason I like the language. Although the
borrowed reference approach is my preferred approach, there is no clearly best
solution, it really depends on your requirements.

One big factor in your choice of approach will be how mutable the graph must be.
No graph is truly immutable. Because there may be cycles, the graph cannot be
created in a single statement. At the very least, the graph must be mutable
during its initialisation phase. If your graph has no distinct initialisation
phase and must always be mutable, then you must use either the unsafe pointer
approach or the `Rc<RefCell<Node>>` approach. These are the least safe, and the
least ergonomic, respectively. If your graph is immutable after initialisation,
then the other two (better) approaches are feasible. You might require some
restricted kind of mutability; for example, if you will only add nodes to the
graph and never remove or change existing nodes, then you might be able to
specialise the data structure for your needs.

Using raw pointers is the most flexible, but also the most dangerous. You must
handle all the lifetime management yourself without any help from the type
system. You can make very flexible and efficient data structures this way, but
you must be very careful. Since an graph using raw pointers is not much
different from a graph in C++, I'm not going to cover that option here.

To compare the different approaches I'll use a pretty simple example. We'll just
have a `Node` object to represent a node in the graph, this will hold some
string data (representative of some more complex data payload) and a `Vec` of
adjacent nodes. We'll have an `init` function to create a simple graph of nodes,
and a `traverse` function which does a pre-order, depth-first traversal of the
graph. We'll use this to print the payload of each node in the graph. Finally,
we'll have a `Node::first` method which returns a reference to the first
adjacent node to the `self` node and a function `foo` which prints the payload
of an individual node. These functions stand in for more complex operations
involving manipulation of a node interior to the graph.

## `Rc<RefCell<Node>>`

See [full example](https://github.com/nrc/r4cppp/blob/master/graphs/src/rc_refcell_graph.rs).

This is the safest option because there is no unsafe code. It is also the least
efficient and least ergonomic option. I wouldn't really recommend this approach
unless you need a mutable graph and don't have any invariants which allow you to
reason about the mutability (or you're paranoid about safety).

The node structure looks like

```
struct Node {
    datum: &'static str,
    next: Vec<Rc<RefCell<Node>>>,
}
```

Creating a new node is not too bad: `Rc::new(RefCell::new(Node { ... }))`. To
add an edge during initialisation, you have to borrow the start node as mutable,
and clone the end node into the Vec of edges (this clones the pointer,
incrementing the reference count, not the actual node). E.g.,

```
let mut mut_root = root.borrow_mut();
mut_root.next.push(b.clone());
```

Whenever you access a node, you have to use `.borrow()` to borrow the `RefCell`.
Worse, our `first` method has to return a ref-counted pointer, rather than a
borrowed reference, so callers of `first` also have to borrow:

```
fn first(&self) -> Rc<RefCell<Node>> {
    self.next[0].clone()
}

pub fn main() {
    let g = ...;
    let f = g.first();
    foo(&*f.borrow());
}
```


## `Rc<Node>`

See [full example](https://github.com/nrc/r4cppp/blob/master/graphs/src/rc_graph.rs).

We can make things a lot nicer to work with at the expense of a little unsafe
code by dumping the `RefCell`. Our invariant for maintaining safety is that the
graph is never mutated after initialisation, and initialisation will always
succeed (this last requirement can be relaxed slightly - what we want to avoid
is that we start destroying nodes whilst creating others).

Our Node looks fairly similar to the previous one:

```
struct Node {
    datum: &'static str,
    next: Vec<Rc<Node>>,
}
```

Creating a new node is also similar, but we create an `Rc<Node>` rather than an
`Rc<RefCell<Node>>`. Creating an edge, on the other hand, is pretty different:

```
let mut_root: &mut Node = mem::transmute(&*root);
mut_root.next.push(b.clone());
```

We still clone the end of the new edge, but rather than using `borrow_mut`, we
just transmute the immutable reference to a mutable one. Using a RefCell causes
a runtime check to ensure that nobody else is currently mutating this node.
Here, we reason about this ourselves rather than depending on the compiler or
runtime checks. By the end of the unsafe block (which is most of the `init`
method), our mutable references have expired and we have re- established the
compiler's invariant (trivially, in this case, because there are no references
to any values).

Since we can take a borrowed reference to the contents of an `Rc`, our `first`
method is much simpler to write and to use:

```
fn first(&self) -> &Node {
    &self.next[0]
}

pub fn main() {
    let g = ...;
    foo(g.first());
}
```


## `&Node`

See [full example](https://github.com/nrc/r4cppp/blob/master/graphs/src/ref_graph.rs).

Borrowed references are Rust's primary kind of pointer, so it would be nice if
we could use them in a graph. However, we must think of allocation. Luckily
there exists the `arena` crate which has two kinds of arenas for handling
exactly this scenario. In many ways this is the 'best' solution - it uses
borrowed references which are highly ergonomic, there is no reference counting
overhead, and destruction of the graph is handled correctly. It does still
require unsafe code for initialisation, but it is pretty much the same as the
`Rc<Node>` approach. We don't benefit so much from lifetime elision though, so
there is a bit more line noise than in the previous approach. Hopefully that
will be fixed in the future (and I'll discuss how specifically at the end of the
section).

First, when is this approach feasible? Similarly to the `Rc` graph we have the
same constraint about only mutating the graph during initialisation. In addition, we
require that all nodes in the graph have the same lifetime (we could relax these
constraints somewhat to allow adding nodes later as long as they can all be
destroyed at the same time).

Arena allocation is a memory management technique where a set of objects have
the same lifetime and can be deallocated at the same time. An arena is an object
responsible for allocating and deallocating the memory. Since large chunks of
memory are allocated and deallocated at once (rather than individual objects),
arena allocation is very efficient. Usually, all the objects are allocated from
a contiguous chunk of memory which improves cache coherency if you are
traversing a graph.

In Rust, arena allocation is supported by the [libarena](https://doc.rust-lang.org/arena/index.html)
crate and is used throughout the compiler. There are two kinds of arenas - typed
and untyped. The former is more efficient and easier to use, but can only
allocate objects of a single type. The latter is more flexible and can allocate
any object. Arena allocated objects all have the same lifetime, which is a
parameter of the arena object. The type system ensures references to arena
allocated objects cannot live longer than the arena and its memory.

Our node struct must now include the lifetime of the graph, `'a`:

```
struct Node<'a> {
    datum: &'static str,
    next: Vec<&'a Node<'a>>,
}
```

Our new function must also use this lifetime and must take the arena which will
do the allocation as an argument:

```
fn new<'a>(datum: &'static str, arena: &'a TypedArena<Node<'a>>) -> &'a Node<'a> {
    arena.alloc(Node {
        datum: datum,
        next: Vec::new(),
    })
}
```

We use the arena to allocate the node. (As an aside, I thought
`TypedArean::alloc` used to take a closure as an argument to avoid the copy of
the thing it allocates. I'm not sure why (or even if) that changed, but
hopefully it will be fixed at some point). The lifetime of the graph is derived
from the lifetime of the reference to the arena, so the arena must be passed in
from the scope which denotes the graph's lifetime. For our examples, that means
we pass it into the `init` method. (One could imagine an extension to the type
system which allows creating values at scopes outside their lexical scope, but
there are no plans to add such a thing any time soon). When the arena goes out
of scope, the whole graph is destroyed (Rust's type system ensures that we can't
keep references to the graph beyond that point).

Adding an edge is similar to `Rc`, we use basically the same transmute, but we
don't need to `clone` the 'other' node:

```
let mut_root: &mut Node = mem::transmute(root);
mut_root.next.push(b);
```

The `first` method is almost identical to the `Rc` version, except that we don't
reap the benefits of lifetime elision and so have to be a bit more explicit:

```
fn first<'a>(&'a self) -> &'a Node<'a> {
    &self.next[0]
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
unique. Alex Summers and Julian Viereck at ETH Zurich are investigating this
further.

#### Generic modules

The 'lifetime of the graph' is constant for any particular graph. Repeating the
lifetime is just boilerplate. One way to make this more ergonomic would be to
allow the graph module to be parameterised by the lifetime, so it would not need
to be added to every struct, impl, and function. The lifetime of the graph would
still need to be specified from outside the module, but hopefully inference
would take care of most uses (as it does today for function calls).

See [ref_graph_generic_mod.rs](https://github.com/nrc/r4cppp/blob/master/graphs/src/ref_graph_generic_mod.rs) for how that might look.

See also this [RFC issue](https://github.com/rust-lang/rfcs/issues/424).

#### Lifetime elision

We currently allow the programmer to elide some lifetimes in function signatures
to improve ergonomics. One reason the `&Node` approach to graphs is a bit ugly
is because it doesn't benefit from any of the lifetime elision rules.

A common pattern in Rust is data structures with a common lifetime. References
into such data structures give rise to types like `&'a Foo<'a>`, for example
`&'a Node<'a>` in the graph example. It would be nice to have an elision
rule that helps in this case. I'm not really sure how it should work though.

Looking at the example with generic modules, it doesn't look like we need to
extend the lifetime elision rules at all (I'm not actually sure if `Node::new`
would work without the given lifetimes, but it seems like a fairly trivial
extension to make it work if it doesn't). We might want to add some new rule to
allow elision of module-generic lifetimes if they are the only ones in scope
(other than `'static`), but I'm not sure how that would work with multiple in-
scope lifetimes.

If we don't add generic modules, we might still be able to add an elision rule
specifically to target `&'a Node<'a>`, not sure how though.
