# Destructuring

Last time we looked at Rust's data types. Once you have some data inside a structure, you
will want to get that data out. For structs, Rust has field access, just like
C++. For tuples, tuple structs, and enums you must use destructuring (there are
various convenience functions in the library, but they use destructuring
internally). Destructuring of data structures exists in C++ only since C++17, so
it most likely familiar from languages such as Python or various functional
languages.  The idea is that just as you can initialize a data structure by
filling out its fields with data from a bunch of local variables, you can fill
out a bunch of local variables with data from a data structure.  From this
simple beginning, destructuring has become one of Rust's most powerful
features. To put it another way, destructuring combines pattern matching with
assignment into local variables.

Destructuring is done primarily through the let and match statements. The match
statement is used when the structure being destructured can have different
variants (such as an enum). A let expression pulls the variables out into the
current scope, whereas match introduces a new scope. To compare:

```rust
fn foo(pair: (int, int)) {
    let (x, y) = pair;
    // we can now use x and y anywhere in foo

    match pair {
        (x, y) => {
            // x and y can only be used in this scope
        }
    }
}
```

The syntax for patterns (used after `let` and before `=>` in the above example)
in both cases is (pretty much) the same. You can also use these patterns in
argument position in function declarations:

```rust
fn foo((x, y): (int, int)) {
}
```

(Which is more useful for structs or tuple-structs than tuples).

Most initialisation expressions can appear in a destructuring pattern and they
can be arbitrarily complex. That can include references and primitive literals
as well as data structures. For example,

```rust
struct St {
    f1: int,
    f2: f32
}

enum En {
    Var1,
    Var2,
    Var3(int),
    Var4(int, St, int)
}

fn foo(x: &En) {
    match x {
        &Var1 => println!("first variant"),
        &Var3(5) => println!("third variant with number 5"),
        &Var3(x) => println!("third variant with number {} (not 5)", x),
        &Var4(3, St { f1: 3, f2: x }, 45) => {
            println!("destructuring an embedded struct, found {} in f2", x)
        }
        &Var4(_, ref x, _) => {
            println!("Some other Var4 with {} in f1 and {} in f2", x.f1, x.f2)
        }
        _ => println!("other (Var2)")
    }
}
```

Note how we destructure through a reference by using `&` in the patterns and how
we use a mix of literals (`5`, `3`, `St { ... }`), wildcards (`_`), and
variables (`x`).

You can use `_` wherever a variable is expected if you want to ignore a single
item in a pattern, so we could have used `&Var3(_)` if we didn't care about the
integer. In the first `Var4` arm we destructure the embedded struct (a nested
pattern) and in the second `Var4` arm we bind the whole struct to a variable.
You can also use `..` to stand in for all fields of a tuple or struct. So if you
wanted to do something for each enum variant but don't care about the content of
the variants, you could write:

```rust
fn foo(x: En) {
    match x {
        Var1 => println!("first variant"),
        Var2 => println!("second variant"),
        Var3(..) => println!("third variant"),
        Var4(..) => println!("fourth variant")
    }
}
```

When destructuring structs, the fields don't need to be in order and you can use
`..` to elide the remaining fields. E.g.,

```rust
struct Big {
    field1: int,
    field2: int,
    field3: int,
    field4: int,
    field5: int,
    field6: int,
    field7: int,
    field8: int,
    field9: int,
}

fn foo(b: Big) {
    let Big { field6: x, field3: y, ..} = b;
    println!("pulled out {} and {}", x, y);
}
```

As a shorthand with structs you can use just the field name which creates a
local variable with that name. The let statement in the above example created
two new local variables `x` and `y`. Alternatively, you could write

```rust
fn foo(b: Big) {
    let Big { field6, field3, .. } = b;
    println!("pulled out {} and {}", field3, field6);
}
```

Now we create local variables with the same names as the fields, in this case
`field3` and `field6`.

There are a few more tricks to Rust's destructuring. Lets say you want a
reference to a variable in a pattern. You can't use `&` because that matches a
reference, rather than creates one (and thus has the effect of dereferencing the
object). For example,

```rust
struct Foo {
    field: &'static int
}

fn foo(x: Foo) {
    let Foo { field: &y } = x;
}
```

Here, `y` has type `int` and is a copy of the field in `x`.

To create a reference to something in a pattern, you use the `ref` keyword. For
example,

```rust
fn foo(b: Big) {
    let Big { field3: ref x, ref field6, ..} = b;
    println!("pulled out {} and {}", *x, *field6);
}
```

Here, `x` and `field6` both have type `&int` and are references to the fields in `b`.

One last trick when destructuring is that if you are destructuring a complex
object, you might want to name intermediate objects as well as individual
fields. Going back to an earlier example, we had the pattern `&Var4(3, St{ f1:
3, f2: x }, 45)`. In that pattern we named one field of the struct, but you
might also want to name the whole struct object. You could write `&Var4(3, s,
45)` which would bind the struct object to `s`, but then you would have to use
field access for the fields, or if you wanted to only match with a specific
value in a field you would have to use a nested match. That is not fun. Rust
lets you name parts of a pattern using `@` syntax. For example `&Var4(3, s @ St{
f1: 3, f2: x }, 45)` lets us name both a field (`x`, for `f2`) and the whole
struct (`s`).

That just about covers your options with Rust pattern matching. There are a few
features I haven't covered, such as matching vectors, but hopefully you know how
to use `match` and `let` and have seen some of the powerful things you can do.
Next time I'll cover some of the subtle interactions between match and borrowing
which tripped me up a fair bit when learning Rust.
