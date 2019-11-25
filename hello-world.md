# Introduction - hello world!

If you are using C or C++, it is probably because you have to - either you need
low-level access to the system, or need every last drop of performance, or both.
Rust aims to offer the same level of abstraction around memory, the same
performance, but be safer and make you more productive.

Concretely, there are many languages out there that you might prefer to use to
C++: Java, Scala, Haskell, Python, and so forth, but you can't because either
the level of abstraction is too high (you don't get direct access to memory,
you are forced to use garbage collection, etc.), or there are performance issues
(either performance is unpredictable or it's simply not fast enough). Rust does
not force you to use garbage collection, and as in C++, you get raw pointers to
memory to play with. Rust subscribes to the 'pay for what you use' philosophy of
C++. If you don't use a feature, then you don't pay any performance overhead for
its existence. Furthermore, all language features in Rust have a predictable (and
usually small) cost.

Whilst these constraints make Rust a (rare) viable alternative to C++, Rust also
has benefits: it is memory safe - Rust's type system ensures that you don't get
the kind of memory errors which are common in C++ - memory leaks, accessing un-
initialised memory, dangling pointers - all are impossible in Rust. Furthermore,
whenever other constraints allow, Rust strives to prevent other safety issues
too - for example, all array indexing is bounds checked (of course, if you want
to avoid the cost, you can (at the expense of safety) - Rust allows you to do
this in unsafe blocks, along with many other unsafe things. Crucially, Rust
ensures that unsafety in unsafe blocks stays in unsafe blocks and can't affect
the rest of your program). Finally, Rust takes many concepts from modern
programming languages and introduces them to the systems language space.
Hopefully, that makes programming in Rust more productive, efficient, and
enjoyable.

In the rest of this section we'll download and install Rust, create a minimal
Cargo project, and implement Hello World.


## Getting Rust

You can get Rust from [http://www.rust-lang.org/install.html](http://www.rust-lang.org/install.html).
The downloads from there include the Rust compiler, standard libraries, and
Cargo, which is a package manager and build tool for Rust.

Rust is available on three channels: stable, beta, and nightly. Rust works on a
rapid-release, schedule with new releases every six weeks. On the release date,
nightly becomes beta and beta becomes stable.

Nightly is updated every night and is ideal for users who want to experiment with
cutting edge features and ensure that their libraries will work with future Rust.

Stable is the right choice for most users. Rust's stability guarantees only
apply to the stable channel.

Beta is designed to mostly be used in users' CI to check that their code will
continue to work as expected.

So, you probably want the stable channel. If you're on Linux or OS X, the
easiest way to get it is to run

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

On Windows, a similarly easy way would be to run

```
choco install rust
```

For other ways to install, see [http://www.rust-lang.org/install.html](http://www.rust-lang.org/install.html).

You can find the source at [github.com/rust-lang/rust](https://github.com/rust-lang/rust).
To build the compiler, run `./configure && make rustc`. See
[building-from-source](https://github.com/rust-lang/rust#building-from-source)
for more detailed instructions.


## Hello World!

The easiest and most common way to build Rust programs is to use Cargo. To start
a project called `hello` using Cargo, run `cargo new --bin hello`. This will
create a new directory called `hello` inside which is a `Cargo.toml` file and
a `src` directory with a file called `main.rs`.

`Cargo.toml` defines dependencies and other metadata about our project. We'll
come back to it in detail later.

All our source code will go in the `src` directory. `main.rs` already contains
a Hello World program. It looks like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

To build the program, run `cargo build`. To build and run it, `cargo run`. If
you do the latter, you should be greeted in the console. Success!

Cargo will have made a `target` directory and put the executable in there.

If you want to use the compiler directly you can run `rustc src/hello.rs` which
will create an executable called `hello`. See `rustc --help` for lots of
options.

OK, back to the code. A few interesting points - we use `fn` to define a
function or method. `main()` is the default entry point for our programs (we'll
leave program args for later). There are no separate declarations or header
files as with C++. `println!` is Rust's equivalent of printf. The `!` means that
it is a macro. A subset of the standard library is available without needing to
be explicitly imported/included (the prelude). The `println!` macro is included
as part of that subset.

Lets change our example a little bit:

```rust
fn main() {
    let world = "world";
    println!("Hello {}!", world);
}
```

`let` is used to introduce a variable, world is the variable name and it is a
string (technically the type is `&'static str`, but more on that later). We
don't need to specify the type, it will be inferred for us.

Using `{}` in the `println!` statement is like using `%s` in printf. In fact, it
is a bit more general than that because Rust will try to convert the variable to
a string if it is not one already<sup>[1](#1)</sup> (like `operator<<()` in C++).
You can easily play around with this sort of thing - try multiple strings and
using numbers (integer and float literals will work).

If you like, you can explicitly give the type of `world`:

```rust
let world: &'static str = "world";
```

In C++ we write `T x` to declare a variable `x` with type `T`. In Rust we write
`x: T`, whether in `let` statements or function signatures, etc. Mostly we omit
explicit types in `let` statements, but they are required for function
arguments. Lets add another function to see it work:

```rust
fn foo(_x: &'static str) -> &'static str {
    "world"
}

fn main() {
    println!("Hello {}!", foo("bar"));
}
```

The function `foo` has a single argument `_x` which is a string literal (we pass
it "bar" from `main`)<sup>[2](#2)</sup>.

The return type for a function is given after `->`. If the function doesn't
return anything (a void function in C++), we don't need to give a return type at
all (as in `main`). If you want to be super-explicit, you can write `-> ()`,
`()` is the void type in Rust.

You don't need the `return` keyword in Rust, if the last expression in a
function body (or any other block, we'll see more of this later) is not finished
with a semicolon, then it is the return value. So `foo` will return
"world". The `return` keyword still exists so we can do early returns. You can
replace `"world"` with `return "world";` and it will have the same effect.


## Why?

I would like to motivate some of the language features above. Local type
inference is convenient and useful without sacrificing safety or performance
(it's even in modern versions of C++ now). A minor convenience is that language
items are consistently denoted by keyword (`fn`, `let`, etc.), this makes
scanning by eye or by tools easier, in general the syntax of Rust is simpler and
more consistent than C++. The `println!` macro is safer than printf - the number
of arguments is statically checked against the number of 'holes' in the string
and the arguments are type checked. This means you can't make the printf
mistakes of printing memory as if it had a different type or addressing memory
further down the stack by mistake. These are fairly minor things, but I hope
they illustrate the philosophy behind the design of Rust.


##### 1

This is a programmer specified conversion which uses the `Display` trait, which
works a bit like `toString` in Java. You can also use `{:?}` which gives a
compiler generated representation which is sometimes useful for debugging. As
with printf, there are many other options.

##### 2

We don't actually use that argument in `foo`. Usually,
Rust will warn us about this. By prefixing the argument name with `_` we avoid
these warnings. In fact, we don't need to name the argument at all, we could
just use `_`.
