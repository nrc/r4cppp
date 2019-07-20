# Borrowed pointers

In the last post I introduced unique pointers. This time I will talk about
another kind of pointer which is much more common in most Rust programs:
borrowed pointers (aka borrowed references, or just references).

If we want to have a reference to an existing value (as opposed to creating a
new value on the heap and pointing to it, as with unique pointers), we must use
`&`, a borrowed reference. These are probably the most common kind of pointer in
Rust, and if you want something to fill in for a C++ pointer or reference (e.g.,
for passing a parameter to a function by reference), this is probably it.

We use the `&` operator to create a borrowed reference and to indicate reference
types, and `*` to dereference them. The same rules about automatic dereferencing
apply as for unique pointers. For example,

```rust
fn foo() {
    let x = &3;   // type: &i32
    let y = *x;   // 3, type: i32
    bar(x, *x);
    bar(&y, y);
}

fn bar(z: &i32, i: i32) {
    // ...
}
```

The `&` operator does not allocate memory (we can only create a borrowed
reference to an existing value) and if a borrowed reference goes out of scope,
no memory gets deleted.

Borrowed references are not unique - you can have multiple borrowed references
pointing to the same value. E.g.,

```rust
fn foo() {
    let x = 5;                // type: i32
    let y = &x;               // type: &i32
    let z = y;                // type: &i32
    let w = y;                // type: &i32
    println!("These should all be 5: {} {} {}", *w, *y, *z);
}
```

Like values, borrowed references are immutable by default. You can also use
`&mut` to take a mutable reference, or to denote mutable reference types.
Mutable borrowed references are unique (you can only take a single mutable
reference to a value, and you can only have a mutable reference if there are no
immutable references). You can use a mutable reference where an immutable one is
wanted, but not vice versa. Putting all that together in an example:

```rust
fn bar(x: &i32) { ... }
fn bar_mut(x: &mut i32) { ... }  // &mut i32 is a reference to an i32 which
                                 // can be mutated

fn foo() {
    let x = 5;
    //let xr = &mut x;     // Error - can't make a mutable reference to an
                           // immutable variable
    let xr = &x;           // Ok (creates an immutable ref)
    bar(xr);
    //bar_mut(xr);         // Error - expects a mutable ref

    let mut x = 5;
    let xr = &x;           // Ok (creates an immutable ref)
    //*xr = 4;             // Error - mutating immutable ref
    //let xr = &mut x;     // Error - there is already an immutable ref, so we
                           // can't make a mutable one

    let mut x = 5;
    let xr = &mut x;       // Ok (creates a mutable ref)
    *xr = 4;               // Ok
    //let xr = &x;         // Error - there is already a mutable ref, so we
                           // can't make an immutable one
    //let xr = &mut x;     // Error - can only have one mutable ref at a time
    bar(xr);               // Ok
    bar_mut(xr);           // Ok
}
```

Note that the reference may be mutable (or not) independently of the mutableness
of the variable holding the reference. This is similar to C++ where pointers can
be const (or not) independently of the data they point to. This is in contrast
to unique pointers, where the mutableness of the pointer is linked to the
mutableness of the data. For example,

```rust
fn foo() {
    let mut x = 5;
    let mut y = 6;
    let xr = &mut x;
    //xr = &mut y;        // Error xr is immutable

    let mut x = 5;
    let mut y = 6;
    let mut xr = &mut x;
    xr = &mut y;          // Ok

    let x = 5;
    let y = 6;
    let mut xr = &x;
    xr = &y;              // Ok - xr is mut, even though the referenced data is not
}
```

If a mutable value is borrowed, it becomes immutable for the duration of the
borrow. Once the borrowed pointer goes out of scope, the value can be mutated
again. This is in contrast to unique pointers, which once moved can never be
used again. For example,

```rust
fn foo() {
    let mut x = 5;            // type: i32
    {
        let y = &x;           // type: &i32
        //x = 4;              // Error - x has been borrowed
        println!("{}", x);    // Ok - x can be read
    }
    x = 4;                    // OK - y no longer exists
}
```

The same thing happens if we take a mutable reference to a value - the value
still cannot be modified. In general in Rust, data can only ever be modified via
one variable or pointer. Furthermore, since we have a mutable reference, we
can't take an immutable reference. That limits how we can use the underlying
value:

```rust
fn foo() {
    let mut x = 5;            // type: i32
    {
        let y = &mut x;       // type: &mut i32
        //x = 4;              // Error - x has been borrowed
        //println!("{}", x);  // Error - requires borrowing x
    }
    x = 4;                    // OK - y no longer exists
}
```

Unlike C++, Rust won't automatically reference a value for you. So if a function
takes a parameter by reference, the caller must reference the actual parameter.
However, pointer types will automatically be converted to a reference:

```rust
fn foo(x: &i32) { ... }

fn bar(x: i32, y: Box<i32>) {
    foo(&x);
    // foo(x);   // Error - expected &i32, found i32
    foo(y);      // Ok
    foo(&*y);    // Also ok, and more explicit, but not good style
}
```

## `mut` vs `const`

At this stage it is probably worth comparing `mut` in Rust to `const` in C++.
Superficially they are opposites. Values are immutable by default in Rust and
can be made mutable by using `mut`. Values are mutable by default in C++, but
can be made constant by using `const`. The subtler and more important difference
is that C++ const-ness applies only to the current use of a value, whereas
Rust's immutability applies to all uses of a value. So in C++ if I have a
`const` variable, someone else could have a non-const reference to it and it
could change without me knowing. In Rust if you have an immutable variable, you
are guaranteed it won't change.

As we mentioned above, all mutable variables are unique. So if you have a
mutable value, you know it is not going to change unless you change it.
Furthermore, you can change it freely since you know that no one else is relying
on it not changing.

## Borrowing and lifetimes

One of the primary safety goals of Rust is to avoid dangling pointers (where a
pointer outlives the memory it points to). In Rust, it is impossible to have a
dangling borrowed reference. It is only legal to create a borrowed reference to
memory which will be alive longer than the reference (well, at least as long as
the reference). In other words, the lifetime of the reference must be shorter
than the lifetime of the referenced value.

That has been accomplished in all the examples in this post. Scopes introduced
by `{}` or functions are bounds on lifetimes - when a variable goes out of scope
its lifetime ends. If we try to take a reference to a shorter lifetime, such as
in a narrower scope, the compiler will give us an error. For example,

```rust
fn foo() {
    let x = 5;
    let mut xr = &x;  // Ok - x and xr have the same lifetime
    {
        let y = 6;
        //xr = &y     // Error - xr will outlive y
    }                 // y is released here
}                     // x and xr are released here
```

In the above example, xr and y don't have the same lifetime because y starts
later than xr, but it's the end of lifetimes which is more interesting, since you
can't reference a variable before it exists in any case - something else which
Rust enforces and which makes it safer than C++.

## Explicit lifetimes

After playing with borrowed pointers for a while, you'll probably come across
borrowed pointers with an explicit lifetime. These have the syntax `&'a T` ([cf.](https://en.wikipedia.org/wiki/Cf.)
`&T`). They're kind of a big topic since I need to cover lifetime-polymorphism
at the same time so I'll leave it for another post (there are a few more less
common pointer types to cover first though). For now, I just want to say that
`&T` is a shorthand for `&'a T` where `a` is the current scope, that is the
scope in which the type is declared.
