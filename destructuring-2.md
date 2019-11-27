# Destructuring pt2 - match and borrowing

When destructuring there are some surprises in store where borrowing is
concerned. Hopefully, nothing surprising once you understand borrowed references
really well, but worth discussing (it took me a while to figure out, that's for
sure. Longer than I realised, in fact, since I screwed up the first version of
this blog post).

Imagine you have some `&Enum` variable `x` (where `Enum` is some enum type). You
have two choices: you can match `*x` and list all the variants (`Variant1 =>
...`, etc.) or you can match `x` and list reference to variant patterns
(`&Variant1 => ...`, etc.). (As a matter of style, prefer the first form where
possible since there is less syntactic noise). `x` is a borrowed reference and
there are strict rules for how a borrowed reference can be dereferenced, these
interact with match expressions in surprising ways (at least surprising to me),
especially when you are modifying an existing enum in a seemingly innocuous way
and then the compiler explodes on a match somewhere.

Before we get into the details of the match expression, lets recap Rust's rules
for value passing. In C++, when assigning a value into a variable or passing it
to a function there are two choices - pass-by-value and pass-by-reference. The
former is the default case and means a value is copied either using a copy
constructor or a bitwise copy. If you annotate the destination of the parameter
pass or assignment with `&`, then the value is passed by reference - only a
pointer to the value is copied and when you operate on the new variable, you are
also operating on the old value.

Rust has the pass-by-reference option, although in Rust the source as well as
the destination must be annotated with `&`. For pass-by-value in Rust, there are
two further choices - copy or move. A copy is the same as C++'s semantics
(except that there are no copy constructors in Rust). A move copies the value
but destroys the old value - Rust's type system ensures you can no longer access
the old value. As examples, `i32` has copy semantics and `Box<i32>` has move
semantics:

```rust
    fn foo() {
    let x = 7i;
    let y = x;                // x is copied
    println!("x is {}", x);   // OK

    let x = box 7i;
    let y = x;                // x is moved
    //println!("x is {}", x); // error: use of moved value: `x`
}
```

You can also choose to have copy semantics for user-defined types
by implementing the `Copy` trait. One straightforward way to do that is 
to add `#[derive(Copy)]` before the definition of the `struct`. Not all
user-defined types are allowed to implement the `Copy` trait. All fields of 
a type must implement `Copy` and the type must not have a destructor. 
Destructors probably need a post of their own, but for now, an object 
in Rust has a destructor if it implements the `Drop`trait. 
Just like C++, the destructor is executed just before an object is 
destroyed.

Now, it is important that a borrowed object is not moved, otherwise you would
have a reference to the old object which is no longer valid. This is equivalent
to holding a reference to an object which has been destroyed after going out of
scope - it is a kind of dangling pointer. If you have a pointer to an object,
there could be other references to it. So if an object has move semantics and
you have a pointer to it, it is unsafe to dereference that pointer. (If the
object has copy semantics, dereferencing creates a copy and the old object will
still exist, so other references will be fine).

OK, back to match expressions. As I said earlier, if you want to match some `x`
with type `&T` you can dereference once in the match clause or match the
reference in every arm of the match expression. Example:

```rust
enum Enum1 {
    Var1,
    Var2,
    Var3
}

fn foo(x: &Enum1) {
    match *x {  // Option 1: deref here.
        Var1 => {}
        Var2 => {}
        Var3 => {}
    }

    match x {
        // Option 2: 'deref' in every arm.
        &Var1 => {}
        &Var2 => {}
        &Var3 => {}
    }
}
```

In this case you can take either approach because `Enum1` has copy semantics.
Let's take a closer look at each approach: in the first approach we dereference
`x` to a temporary variable with type `Enum1` (which copies the value in `x`)
and then do a match against the three variants of `Enum1`. This is a 'one level'
match because we don't go deep into the value's type. In the second approach
there is no dereferencing. We match a value with type `&Enum1` against a
reference to each variant. This match goes two levels deep - it matches the type
(always a reference) and looks inside the type to match the referred type (which
is `Enum1`).

Either way, we must ensure that we (that is, the compiler) respect 
Rust's invariants around moves and references - we must not move any
part of an object if it is referenced. If the value being matched has copy
semantics, that is trivial. If it has move semantics then we must make sure that
moves don't happen in any match arm. This is accomplished either by ignoring
data which would move, or making references to it (so we get by-reference
passing rather than by-move).

```rust
enum Enum2 {
    // Box has a destructor so Enum2 has move semantics.
    Var1(Box<i32>),
    Var2,
    Var3
}

fn foo(x: &Enum2) {
    match *x {
        // We're ignoring nested data, so this is OK
        Var1(..) => {}
        // No change to the other arms.
        Var2 => {}
        Var3 => {}
    }

    match x {
        // We're ignoring nested data, so this is OK
        &Var1(..) => {}
        // No change to the other arms.
        &Var2 => {}
        &Var3 => {}
    }
}
```

In either approach we don't refer to any of the nested data, so none of it is
moved. In the first approach, even though `x` is referenced, we don't touch its
innards in the scope of the dereference (i.e., the match expression) so nothing
can escape. We also don't bind the whole value (i.e., bind `*x` to a variable),
so we can't move the whole object either.

We can take a reference to any variant in the second match, but not in the
dereferenced version. So, in the second approach replacing the second arm with `a
@ &Var2 => {}` is OK (`a` is a reference), but under the first approach we
couldn't write `a @ Var2 => {}` since that would mean moving `*x` into `a`. We
could write `ref a @ Var2 => {}` (in which `a` is also a reference), although
it's not a construct you see very often.

But what about if we want to use the data nested inside `Var1`? We can't write:

```rust
match *x {
    Var1(y) => {}
    _ => {}
}
```

or

```rust
match x {
    &Var1(y) => {}
    _ => {}
}
```

because in both cases it means moving part of `x` into `y`. We can use the 'ref'
keyword to get a reference to the data in `Var1`: `&Var1(ref y) => {}`. That is
OK, because now we are not dereferencing anywhere and thus not moving any part
of `x`. Instead we are creating a pointer which points into the interior of `x`.

Alternatively, we could destructure the Box (this match is going three levels
deep): `&Var1(box y) => {}`. This is OK because `i32` has copy semantics and `y`
is a copy of the `i32` inside the `Box` inside `Var1` (which is 'inside' a
borrowed reference). Since `i32` has copy semantics, we don't need to move any
part of `x`. We could also create a reference to the int rather than copy it:
`&Var1(box ref y) => {}`. Again, this is OK, because we don't do any
dereferencing and thus don't need to move any part of `x`. If the contents of
the Box had move semantics, then we could not write `&Var1(box y) => {}`, we
would be forced to use the reference version. We could also use similar
techniques with the first approach to matching, which look the same but without
the first `&`. For example, `Var1(box ref y) => {}`.

Now lets get more complex. Lets say you want to match against a pair of
reference-to-enum values. Now we can't use the first approach at all:

```rust
fn bar(x: &Enum2, y: &Enum2) {
    // Error: x and y are being moved.
    // match (*x, *y) {
    //     (Var2, _) => {}
    //     _ => {}
    // }

    // OK.
    match (x, y) {
        (&Var2, _) => {}
        _ => {}
    }
}
```

The first approach is illegal because the value being matched is created by
dereferencing `x` and `y` and then moving them both into a new tuple object. So
in this circumstance, only the second approach works. And of course, you still
have to follow the rules above for avoiding moving parts of `x` and `y`.

If you do end up only being able to get a reference to some data and you need
the value itself, you have no option except to copy that data. Usually that
means using `clone()`. If the data doesn't implement clone, you're going to have
to further destructure to make a manual copy or implement clone yourself.

What if we don't have a reference to a value with move semantics, but the value
itself. Now moves are OK, because we know no one else has a reference to the
value (the compiler ensures that if they do, we can't use the value). For
example,

```rust
fn baz(x: Enum2) {
    match x {
        Var1(y) => {}
        _ => {}
    }
}
```

There are still a few things to be aware of. Firstly, you can only move to one
place. In the above example we are moving part of `x` into `y` and we'll forget
about the rest. If we wrote `a @ Var1(y) => {}` we would be attempting to move
all of `x` into `a` and part of `x` into `y`. That is not allowed, an arm like
that is illegal. Making one of `a` or `y` a reference (using `ref a`, etc.) is
not an option either, then we'd have the problem described above where we move
whilst holding a reference. We can make both `a` and `y` references and then
we're OK - neither is moving, so `x` remains intact and we have pointers to the
whole and a part of it.

Similarly (and more common), if we have a variant with multiple pieces of nested
data, we can't take a reference to one datum and move another. For example if we
had a `Var4` declared as `Var4(Box<int>, Box<int>)` we can have a match arm
which references both (`Var4(ref y, ref z) => {}`) or a match arm which moves
both (`Var4(y, z) => {}`) but you cannot have a match arm which moves one and
references the other (`Var4(ref y, z) => {}`). This is because a partial move
still destroys the whole object, so the reference would be invalid.
