# Reference counted and raw pointers

TODO remove all references to Gc pointers

TODO add discussion of custom pointers and Deref trait (maybe later, not here)

So far we've covered unique and borrowed pointers. Unique pointers are very
similar to the new std::unique_ptr in C++ and borrowed references are the
'default' pointer you usually reach for if you would use a pointer or reference
in C++. Rust has a few more, rarer pointers either in the libraries or built in
to the language. These are mostly similar to various kinds of smart pointers you
might be used to in C++.

This post took a while to write and I still don't like it. There are a lot of
loose ends here, both in my write up and in Rust itself. I hope some will get
better with later posts and some will get better as the language develops. If
you are learning Rust, you might even want to skip this stuff for now, hopefully
you won't need it. Its really here just for completeness after the posts on
other pointer types.

It might feel like Rust has a lot of pointer types, but it is pretty similar to
C++ once you think about the various kinds of smart pointers available in
libraries. In Rust, however, you are more likely to meet them when you first
start learning the language. Because Rust pointers have compiler support, you
are also much less likely to make errors when using them.

I'm not going to cover these in as much detail as unique and borrowed references
because, frankly, they are not as important. I might come back to them in more
detail later on.

## Rc<T>

Reference counted pointers come as part of the rust standard library. They are
in the `std::rc` module (we'll cover modules soon-ish. The modules are the
reason for the `use` incantations in the examples). A reference counted pointer
to an object of type `T` has type `Rc<T>`. You create reference counted pointers
using a static method (which for now you can think of like C++'s, but we'll see
later they are a bit different) - `Rc::new(...)` which takes a value to create
the pointer to. This constructor method follows Rust's usual move/copy semantics
(like we discussed for unique pointers) - in either case, after calling Rc::new,
you will only be able to access the value via the pointer.

As with the other pointer types, the `.` operator does all the dereferencing you
need it to. You can use `*` to manually dereference.

To pass a ref-counted pointer you need to use the `clone` method. This kinda
sucks, and hopefully we'll fix that, but that is not for sure (sadly). You can
take a (borrowed) reference to the pointed at value, so hopefully you don't need
to clone too often. Rust's type system ensures that the ref-counted variable
will not be deleted before any references expire. Taking a reference has the
added advantage that it doesn't need to increment or decrement the ref count,
and so will give better performance (although, that difference is probably
marginal since Rc objects are limited to a single thread and so the ref count
operations don't have to be atomic). As in C++, you can also take a reference to
the Gc pointer.

An Rc example:

```
use std::rc::Rc;

fn bar(x: Rc<int>) { }
fn baz(x: &int) { }

fn foo() {
    let x = Rc::new(45);
    bar(x.clone());   // Increments the ref-count
    baz(&*x);         // Does not increment
    println!("{}", 100 - *x);
}  // Once this scope closes, all Rc pointers are gone, so ref-count == 0
   // and the memory will be deleted.
```

Ref counted pointers are always immutable. If you want a mutable ref-counted
object you need to use a RefCell (or Cell) wrapped in an `Rc`.

## Cell and RefCell

Cell and RefCell are structs which allow you to 'cheat' the mutability rules.
This is kind of hard to explain without first covering Rust data structures and
how they work with mutability, so I'm going to come back to these slightly
tricky objects later. For now, you should know that if you want a mutable, ref
counted object you need a Cell or RefCell wrapped in an Rc. As a first
approximation, you probably want Cell for primitive data and RefCell for objects
with move semantics. So, for a mutable, ref-counted int you would use
`Rc<Cell<int>>`.

## *T - unsafe pointers

TODO raw pointers must be const or mut now

Finally Rust has unsafe pointers. These are denoted `*T` and are created using
`&` (you might need to specify a type to get a `*T` rather than a `&T` since the
`&` operator can create either a borrowed reference or an unsafe pointer). These
are like C pointers, just a pointer to memory with no restrictions on how they
are used (you can't do pointer arithmetic without casting, but you can do it
that way if you must). Unsafe pointers are the only pointer type in Rust which
can be null. There is no automatic dereferencing of unsafe pointers (so to call
a method you have to write `(*x).foo()`) and no automatic referencing. The most
important restriction is that they can't be dereferenced (and thus can't be
used) outside of an unsafe block. In regular Rust code you can only pass them
around.

So, what is unsafe code? Rust has strong safety guarantees, and (rarely) they
prevent you doing something you need to do. Since Rust aims to be a systems
language, it has to be able to do anything that is possible and sometimes that
means doing things the compiler can't verify is safe. To accomplish that, Rust
has the concept of unsafe blocks, marked by the `unsafe` keyword. In unsafe code
you can do unsafe things - dereference an unsafe pointer, index into an array
without bounds checking, call code written in another language via the FFI, or
cast variables. Obviously, you have to be much more careful writing unsafe code
than writing regular Rust code. In fact, you should only very rarely write
unsafe code. Mostly it is used in very small chunks in libraries, rather than in
client code. In unsafe code you must do all the things you normally do in C++ to
ensure safety. Furthermore, you must ensure that by the time the unsafe block
finishes, you have re-established all of the invariants that the Rust compiler
would usually enforce, otherwise you risk causing bugs in safe code too.

An example of using an unsafe pointer:

```
fn foo() {
    let x = 5;
    let xp: *int = &5;
    println!("x+5={}", add_5(xp));
}

fn add_5(p: *int) -> int {
    unsafe {
        if !p.is_null() { // Note that *-pointers do not auto-deref, so this is
                          // a method implemented on *int, not int.
            *p + 5
        } else {
            -1            // Not a recommended error handling strategy.
        }
    }
}
```

As with borrowed references, unsafe pointers are immutable by default and can be
made mutable using the `mut` keyword, for example `*mut int`.

And that concludes our tour of Rust's pointers. Next time we'll take a break
from pointers and look at Rust's data structures. We'll come back to borrowed
references again in a later post though.
