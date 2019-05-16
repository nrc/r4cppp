# Data types

In this post I'll discuss Rust's data types. These are roughly equivalent to
classes, structs, and enums in C++. One difference with Rust is that data and
behaviour are much more strictly separated in Rust than C++ (or Java, or other
OO languages). Behaviour is defined by functions and those can be defined in
traits and `impl`s (implementations), but traits cannot contain data, they are
similar to Java's interfaces in that respect. I'll cover traits and impls in a
later post, this one is all about data.

## Structs

A rust struct is similar to a C struct or a C++ struct without methods. Simply a
list of named fields. The syntax is best seen with an example:

```rust
struct S {
    field1: i32,
    field2: SomeOtherStruct
}
```

Here we define a struct called `S` with two fields. The fields are comma
separated; if you like, you can comma-terminate the last field too.

Structs introduce a type. In the example, we could use `S` as a type.
`SomeOtherStruct` is assumed to be another struct (used as a type in the
example), and (like C++) it is included by value, that is, there is no pointer
to another struct object in memory.

Fields in structs are accessed using the `.` operator and their name. An example
of struct use:

```rust
fn foo(s1: S, s2: &S) {
    let f = s1.field1;
    if f == s2.field1 {
        println!("field1 matches!");
    }
}
```

Here `s1` is struct object passed by value and `s2` is a struct object passed by
reference. As with method call, we use the same `.` to access fields in both, no
need for `->`.

Structs are initialised using struct literals. These are the name of the struct
and values for each field. For example,

```rust
fn foo(sos: SomeOtherStruct) {
    let x = S { field1: 45, field2: sos };  // initialise x with a struct literal
    println!("x.field1 = {}", x.field1);
}
```

Structs cannot be recursive; that is, you can't have cycles of struct names
involving definitions and field types. This is because of the value semantics of
structs. So for example, `struct R { r: Option<R> }` is illegal and will cause a
compiler error (see below for more about Option). If you need such a structure
then you should use some kind of pointer; cycles with pointers are allowed:

```rust
struct R {
    r: Option<Box<R>>
}
```

If we didn't have the `Option` in the above struct, there would be no way to
instantiate the struct and Rust would signal an error.

Structs with no fields do not use braces in either their definition or literal
use. Definitions do need a terminating semi-colon though, presumably just to
facilitate parsing.

```rust
struct Empty;

fn foo() {
    let e = Empty;
}
```

## Tuples

Tuples are anonymous, heterogeneous sequences of data. As a type, they are
declared as a sequence of types in parentheses. Since there is no name, they are
identified by structure. For example, the type `(i32, i32)` is a pair of
integers and `(i32, f32, S)` is a triple. Tuple values are initialised in the
same way as tuple types are declared, but with values instead of types for the
components, e.g., `(4, 5)`. An example:

```rust
// foo takes a struct and returns a tuple
fn foo(x: SomeOtherStruct) -> (i32, f32, S) {
    (23, 45.82, S { field1: 54, field2: x })
}
```

Tuples can be used by destructuring using a `let` expression, e.g.,

```rust
fn bar(x: (i32, i32)) {
    let (a, b) = x;
    println!("x was ({}, {})", a, b);
}
```

We'll talk more about destructuring next time.


## Tuple structs

Tuple structs are named tuples, or alternatively, structs with unnamed fields.
They are declared using the `struct` keyword, a list of types in parentheses,
and a semicolon. Such a declaration introduces their name as a type. Their
fields must be accessed by destructuring (like a tuple), rather than by name.
Tuple structs are not very common.

```rust
struct IntPoint (i32, i32);

fn foo(x: IntPoint) {
    let IntPoint(a, b) = x;  // Note that we need the name of the tuple
                             // struct to destructure.
    println!("x was ({}, {})", a, b);
}
```

## Enums

Enums are types like C++ enums or unions, in that they are types which can take
multiple values. The simplest kind of enum is just like a C++ enum:

```rust
enum E1 {
    Var1,
    Var2,
    Var3
}

fn foo() {
    let x: E1 = Var2;
    match x {
        Var2 => println!("var2"),
        _ => {}
    }
}
```

However, Rust enums are much more powerful than that. Each variant can contain
data. Like tuples, these are defined by a list of types. In this case they are
more like unions than enums in C++. Rust enums are tagged unions rather than untagged unions (as in C++). 
That means you can't mistake one variant of an enum for another at runtime<sup>[1](#1)</sup>. An example:

```rust
enum Expr {
    Add(i32, i32),
    Or(bool, bool),
    Lit(i32)
}

fn foo() {
    let x = Or(true, false);   // x has type Expr
}
```

Many simple cases of object-oriented polymorphism are better handled in Rust
using enums.

To use enums we usually use a match expression. Remember that these are similar
to C++ switch statements. I'll go into more depth on these and other ways to
destructure data next time. Here's an example:

```rust
fn bar(e: Expr) {
    match e {
        Add(x, y) => println!("An `Add` variant: {} + {}", x, y),
        Or(..) => println!("An `Or` variant"),
        _ => println!("Something else (in this case, a `Lit`)"),
    }
}
```

Each arm of the match expression matches a variant of `Expr`. All variants must
be covered. The last case (`_`) covers all remaining variants, although in the
example there is only `Lit`. Any data in a variant can be bound to a variable.
In the `Add` arm we are binding the two i32s in an `Add` to `x` and `y`. If we
don't care about the data, we can use `..` to match any data, as we do for `Or`.


## Option

One particularly common enum in Rust is `Option`. This has two variants - `Some`
and `None`. `None` has no data and `Some` has a single field with type `T`
(`Option` is a generic enum, which we will cover later, but hopefully the
general idea is clear from C++). Options are used to indicate a value might be
there or might not. Any place you use a null pointer in C++<sup>[2](#2)</sup>.
to indicate a value which is in some way undefined, uninitialised, or false,
you should probably use an Option in Rust. Using Option is safer because you
must always check it before use; there is no way to do the equivalent of
dereferencing a null pointer. They are also more general, you can use them with
values as well as pointers. An example:

```rust
use std::rc::Rc;

struct Node {
    parent: Option<Rc<Node>>,
    value: i32
}

fn is_root(node: Node) -> bool {
    match node.parent {
        Some(_) => false,
        None => true
    }
}
```

Here, the parent field could be either a `None` or a `Some` containing an
`Rc<Node>`. In the example, we never actually use that payload, but in real life
you usually would.


There are also convenience methods on Option, so you could write the body of
`is_root` as `node.parent.is_none()` or `!node.parent.is_some()`.

## Inherited mutability and Cell/RefCell

Local variables in Rust are immutable by default and can be marked mutable using
`mut`. We don't mark fields in structs or enums as mutable, their mutability is
inherited. This means that a field in a struct object is mutable or immutable
depending on whether the object itself is mutable or immutable. Example:

```rust
struct S1 {
    field1: i32,
    field2: S2
}
struct S2 {
    field: i32
}

fn main() {
    let s = S1 { field1: 45, field2: S2 { field: 23 } };
    // s is deeply immutable, the following mutations are forbidden
    // s.field1 = 46;
    // s.field2.field = 24;

    let mut s = S1 { field1: 45, field2: S2 { field: 23 } };
    // s is mutable, these are OK
    s.field1 = 46;
    s.field2.field = 24;
}
```

Inherited mutability in Rust stops at references. This is similar to C++ where
you can modify a non-const object via a pointer from a const object. If you want
a reference field to be mutable, you have to use `&mut` on the field type:

```rust
struct S1 {
    f: i32
}
struct S2<'a> {
    f: &'a mut S1   // mutable reference field
}
struct S3<'a> {
    f: &'a S1       // immutable reference field
}

fn main() {
    let mut s1 = S1{f:56};
    let s2 = S2 { f: &mut s1};
    s2.f.f = 45;   // legal even though s2 is immutable
    // s2.f = &mut s1; // illegal - s2 is not mutable
    let s1 = S1{f:56};
    let mut s3 = S3 { f: &s1};
    s3.f = &s1;     // legal - s3 is mutable
    // s3.f.f = 45; // illegal - s3.f is immutable
}
```

(The `'a` parameter on `S2` and `S3` is a lifetime parameter, we'll cover those soon).

Sometimes whilst an object is logically immutable, it has parts which need to be
internally mutable. Think of various kinds of caching or a reference count
(which would not give true logical immutability since the effect of changing the
ref count can be observed via destructors). In C++, you would use the `mutable`
keyword to allow such mutation even when the object is const. In Rust we have
the Cell and RefCell structs. These allow parts of immutable objects to be
mutated. Whilst that is useful, it means you need to be aware that when you see
an immutable object in Rust, it is possible that some parts may actually be
mutable.

RefCell and Cell let you get around Rust's strict rules on mutation and
aliasability. They are safe to use because they ensure that Rust's invariants
are respected dynamically, even though the compiler cannot ensure that those
invariants hold statically. Cell and RefCell are both single threaded objects.

Use Cell for types which have copy semantics (pretty much just primitive types).
Cell has `get` and `set` methods for changing the stored value, and a `new`
method to initialise the cell with a value. Cell is a very simple object - it
doesn't need to do anything smart since objects with copy semantics can't keep
references elsewhere (in Rust) and they can't be shared across threads, so there
is not much to go wrong.

Use RefCell for types which have move semantics, that means nearly everything in
Rust, struct objects are a common example. RefCell is also created using `new`
and has a `set` method. To get the value in a RefCell, you must borrow it using
the borrow methods (`borrow`, `borrow_mut`, `try_borrow`, `try_borrow_mut`)
these will give you a borrowed reference to the object in the RefCell. These
methods follow the same rules as static borrowing - you can only have one
mutable borrow, and can't borrow mutably and immutably at the same time.
However, rather than a compile error you get a runtime failure. The `try_`
variants return an Option - you get `Some(val)` if the value can be borrowed and
`None` if it can't. If a value is borrowed, calling `set` will fail too.

Here's an example using a ref-counted pointer to a RefCell (a common use-case):

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct S {
    field: i32
}

fn foo(x: Rc<RefCell<S>>) {
    {
        let s = x.borrow();
        println!("the field, twice {} {}", s.field, x.borrow().field);
        // let s = x.borrow_mut(); // Error - we've already borrowed the contents of x
    }

    let mut s = x.borrow_mut(); // OK, the earlier borrows are out of scope
    s.field = 45;
    // println!("The field {}", x.borrow().field); // Error - can't mut and immut borrow
    println!("The field {}", s.field);
}

fn main() {
    let s = S{field:12};
    let x: Rc<RefCell<S>> = Rc::new(RefCell::new(s));
    foo(x.clone());

    println!("The field {}", x.borrow().field);
}
```

If you're using Cell/RefCell, you should try to put them on the smallest object
you can. That is, prefer to put them on a few fields of a struct, rather than
the whole struct. Think of them like single threaded locks, finer grained
locking is better since you are more likely to avoid colliding on a lock.


##### 1

In C++17 there is `std::variant<T>` type that is closer to Rust enums than unions.

##### 2

Since C++17 `std::optional<T>` is the best alternative of Option in Rust.
