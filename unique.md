# Unique pointers

Rust is a systems language and therefore must give you raw access to memory. It
does this (as in C++) via pointers. Pointers are one area where Rust and C++ are
very different, both in syntax and semantics. Rust enforces memory safety by
type checking pointers. That is one of its major advantages over other
languages. Although the type system is a bit complex, you get memory safety and
bare-metal performance in return.

I had intended to cover all of Rust's pointers in one post, but I think the
subject is too large. So this post will cover just one kind - unique pointers -
and other kinds will be covered in follow up posts.

First, an example without pointers:

```rust
fn foo() {
    let x = 75;

    // ... do something with `x` ...
}
```

When we reach the end of `foo`, `x` goes out of scope (in Rust as in C++). That
means the variable can no longer be accessed and the memory for the variable can
be reused.

In Rust, for every type `T` we can write `Box<T>` for an owning (aka unique)
pointer to `T`. We use `Box::new(...)` to allocate space on the heap and
initialise that space with the supplied value. This is similar to `new` in C++.
For example,

```rust
fn foo() {
    let x = Box::new(75);
}
```

Here `x` is a pointer to a location on the heap which contains the value `75`.
`x` has type `Box<isize>`; we could have written `let x: Box<isize> =
Box::new(75);`. This is similar to writing `int* x = new int(75);` in C++.
Unlike in C++, Rust will tidy up the memory for us, so there is no need to call
`free` or `delete`<sup>[1](#1)</sup>. Unique pointers behave similarly to
values - they are deleted when the variable goes out of scope. In our example,
at the end of the function `foo`, `x` can no longer be accessed and the memory
pointed at by `x` can be reused.

Owning pointers are dereferenced using the `*` as in C++. E.g.,

```rust
fn foo() {
    let x = Box::new(75);
    println!("`x` points to {}", *x);
}
```

As with primitive types in Rust, owning pointers and the data they point to are
immutable by default. Unlike in C++, you can't have a mutable (unique) pointer to
immutable data or vice versa. Mutability of the data follows from the pointer.
E.g.,

```rust
fn foo() {
    let x = Box::new(75);
    let y = Box::new(42);
    // x = y;         // Not allowed, x is immutable.
    // *x = 43;       // Not allowed, *x is immutable.
    let mut x = Box::new(75);
    x = y;            // OK, x is mutable.
    *x = 43;          // OK, *x is mutable.
}
```

Owning pointers can be returned from a function and continue to live on. If they
are returned, then their memory will not be freed, i.e., there are no dangling
pointers in Rust. The memory will not leak. However, it will eventually go out of
scope and then it will be freed. E.g.,

```rust
fn foo() -> Box<i32> {
    let x = Box::new(75);
    x
}

fn bar() {
    let y = foo();
    // ... use y ...
}
```

Here, memory is initialised in `foo`, and returned to `bar`. `x` is returned
from `foo` and stored in `y`, so it is not deleted. At the end of `bar`, `y`
goes out of scope and so the memory is reclaimed.

Owning pointers are unique (also called linear) because there can be only one
(owning) pointer to any piece of memory at any time. This is accomplished by
move semantics. When one pointer points at a value, any previous pointer can no
longer be accessed. E.g.,

```rust
fn foo() {
    let x = Box::new(75);
    let y = x;
    // x can no longer be accessed
    // let z = *x;   // Error.
}
```

Likewise, if an owning pointer is passed to another function or stored in a
field, it can no longer be accessed:

```rust
fn bar(y: Box<isize>) {
}

fn foo() {
    let x = Box::new(75);
    bar(x);
    // x can no longer be accessed
    // let z = *x;   // Error.
}
```

Rust's unique pointers are similar to C++ `std::unique_ptr`s. In Rust, as in
C++, there can be only one unique pointer to a value and that value is deleted
when the pointer goes out of scope. Rust does most of its checking statically
rather than at runtime. So, in C++ accessing a unique pointer whose value has
moved will result in a runtime error (since it will be null). In Rust this
produces a compile time error and you cannot go wrong at runtime.

We'll see later that it is possible to create other pointer types which point at
a unique pointer's value in Rust. This is similar to C++. However, in C++ this
allows you to cause errors at runtime by holding a pointer to freed memory. That
is not possible in Rust (we'll see how when we cover Rust's other pointer
types).

As shown above, owning pointers must be dereferenced to use their values.
However, method calls automatically dereference, so there is no need for a `->`
operator or to use `*` for method calls. In this way, Rust pointers are a bit
similar to both pointers and references in C++. E.g.,

```rust
fn bar(x: Box<Foo>, y: Box<Box<Box<Box<Foo>>>>) {
    x.foo();
    y.foo();
}
```

Assuming that the type `Foo` has a method `foo()`, both these expressions are OK.

Calling `Box::new()` with an existing value does not take a reference to that
value, it copies that value. So,

```rust
fn foo() {
    let x = 3;
    let mut y = Box::new(x);
    *y = 45;
    println!("x is still {}", x);
}
```

In general, Rust has move rather than copy semantics (as seen above with unique
pointers). Primitive types have copy semantics, so in the above example the
value `3` is copied, but for more complex values it would be moved. We'll cover
this in more detail later.

Sometimes when programming, however, we need more than one reference to a value.
For that, Rust has borrowed pointers. I'll cover those in the next post.


##### 1

In C++11 the `std::unique_ptr<T>` was introduced that may be in some aspects
associated to Rust `Box<T>` but there are also significant differences.

`std::unique_ptr<T>` like `Box<T>` automatically releases the memory being
pointed once it goes out of the scope and has only move semantics.

In some way the `let x = Box::new(75)` may be interpreted as `const auto x =
std::unique_ptr<const int>{new int{75}};` in C++11 and `const auto x =
std::make_unique<const int>{75};` since C++14.

But there are still important differences between `Box<T>` and
`std::unique_ptr<T>` that should be taken into account:

1. If `std::unique_ptr<T>` is created by passing the pointer to its constructor
   there is a possibility to have several unique pointers to the same memory
   that is not possible with `Box<T>`
2. Once `std::unique_ptr<T>` is moved to another variable or to function
   dereference of this pointer causes undefined behavior that is also
   impossible in Rust
3. Mutability or immutability does not go "through" `std::unique_ptr<T>` 
   -- dereferencing a `const std::unique_ptr<T>` still yields a mutable 
   (non-`const`) reference to the underlying data. In Rust, an immutable
   `Box<T>` does not allow mutation of the data it points to
