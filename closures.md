# Closures and first-class functions

Closures and first-class and higher order functions are a core part of Rust. In
C and C++ there are function pointers (and those weird member/method pointer
things in C++ that I never got the hang of). However, they are used relatively
rarely and are not very ergonomic. C++11 introduced lambdas, and these are
pretty close to Rust closures, in particular they have a very similar
implementation strategy.

To start with, I want to establish some intuition for these things. Then, we'll
dive in to the details.

Lets say we have a function `foo`: `pub fn foo() -> u32 { 42 }`. Now let's
imagine another function `bar` which takes a function as an argument (I'll leave
`bar`'s signature for later): `fn bar(f: ...) { ... }`. We can pass `foo` to
`bar` kind of like we would pass a function pointer in C: `bar(foo)`. In the
body of `bar` we can call `f` as if it were a function: `let x = f();`.

We say that Rust has first-class functions because we can pass them around and
use them like we can with other values. We say `bar` is a higher-order function
because it takes a function as an argument, i.e., it is a function that operates
on functions.

Closures in Rust are anonymous functions with a nice syntax. A closure `|x| x +
2` takes an argument and returns it with `2` added. Note that we don't have to
give types for the arguments to a closure (they can usually be inferred). We
also don't need to specify a return type. If we want the closure body to be more
than just one expression, we can use braces: `|x: i32| { let y = x + 2; y }`. We
can pass closures just like functions: `bar(|| 42)`.

The big difference between closures and other functions is that closures capture
their environment. This means that we can refer to variables outside the closure
from the closure. E.g.,

```rust
let x = 42;
bar(|| x);
```

Note how `x` is in scope in the closure.

We've seen closures before, used with iterators, and this is a common use case
for them. E.g., to add a value to each element of a vector:

```rust
fn baz(v: Vec<i32>) -> Vec<i32> {
    let z = 3;
    v.iter().map(|x| x + z).collect()
}
```

Here `x` is an argument to the closure, each member of `v` will be passed as an
`x`. `z` is declared outside of the closure, but because it's a closure, `z` can
be referred to. We could also pass a function to map:

```rust
fn add_two(x: i32) -> i32 {
    x + 2
}

fn baz(v: Vec<i32>) -> Vec<i32> {
    v.iter().map(add_two).collect()
}
```

Note that Rust also allows declaring functions inside of functions. These are
*not* closures - they can't access their environment. They are merely a
convenience for scoping.

```rust
fn qux(x: i32) {
    fn quxx() -> i32 {
        x // ERROR x is not in scope.
    }

    let a = quxx();
}
```

## Function types

Lets introduce a new example function:

```rust
fn add_42(x: i32) -> i64 {
    x as i64 + 42
}
```

As we saw before, we can store a function in a variable: `let a = add_42;`. The
most precise type of `a` cannot be written in Rust. You'll sometimes see the
compiler render it as `fn(i32) -> i64 {add_42}` in error messages. Each function
has its own unique and anonymous type. `fn add_41(x: i32) -> i64` has have a different
type, even though they have the same signature.

We can write less precise types, for example, `let a: fn(i32) -> i64 = add_42;`.
All function types with the same signature can be coerced to a `fn` type
(which can be written by the programmer).

`a` is represented by the compiler as a function pointer, however, if the
compiler knows the precise type, it doesn't actually use that function pointer.
A call like a() is statically dispatched based on the type of a. If the
compiler doesn't know the precise type (e.g., it only knows the fn type), then
the call is dispatched using the function pointer in the value.

There are also `Fn` types (note the capital 'F'). These `Fn` types are bounds,
just like traits (in fact they *are* traits, as we'll see later). `Fn(i32) -> i64`
is a bound on the types of all function-like objects with that signature. When
we take a reference to a function pointer, we're actually creating a trait
object which is represented by a fat pointer (see DSTs).

To pass a function to another function, or to store the function in a field, we
must write a type. We have several choices, we can either use either a `fn` type
or a `Fn` type. The latter is better because it includes closures (and
potentially other function-like things), whereas `fn` types don't. The `Fn`
types are dynamically sized which means we cannot use them as value types. We
must either pass function objects or use generics. Let's look at the generic
approach first. For example,

```rust
fn bar<F>(f: F) -> i64
    where F: Fn(i32) -> i64
{
    f(0)
}
```

`bar` takes any function with the signature `Fn(i32) -> i64`, i.e., we can
instantiate the `F` type parameter with any function-like type. We could call
`bar(add_42)` to pass `add_42` to `bar` which would instantiate `F` with
`add_42`'s anonymous type. We could also call `bar(add_41)` and that would work
too.

You can also pass closures to `bar`, e.g., `bar(|x| x as i64)`. This works
because closure types are also bounded by the `Fn` bound matching their
signature (like functions, each closure has it's own anonymous type).

Finally, you can pass references to functions or closures too: `bar(&add_42)` or
`bar(&|x| x as i64)`.

One could also write `bar` as `fn bar(f: &Fn(i32) -> i64) ...`. These two
approaches (generics vs a function/trait object) have quite different semantics.
In the generics case, `bar` will be monomorphised so when code is generated, the
compiler know the exact type of `f`, that means it can be statically dispatched.
If using a function object, the function is not monomorphised. The exact type of
`f` is not known, and so the compiler must generate a virtual dispatch. The
latter is slower, but the former will produce more code (one monomorphised
function per type parameter instance).

There are actually more function traits than just `Fn`; there are `FnMut` and
`FnOnce` too. These are used in the same way as `Fn`, e.g., `FnOnce(i32) ->
i64`. A `FnMut` represents an object which can be called and can be mutated
during that call. This doesn't apply to normal functions, but for closures it
means the closure can mutate its environment. `FnOnce` is a function which can
only be called (at most) once. Again, this is only relevant for closures.

`Fn`, `FnMut`, and `FnOnce` are in a sub-trait hierarchy. `Fn`s are `FnMut`s
(because one can call a `Fn` function with permission to mutate and no harm is
done, but the opposite is not true). `Fn`s and `FnMut`s are `FnOnce`s (because
there is no harm done if a regular function is only called once, but not the
opposite).

So, to make a higher-order function as flexible as possible, you should use the
`FnOnce` bound, rather than the `Fn` bound (or use the `FnMut` bound if you must
call the function more than once).


### Methods

You can use methods in the same way as functions - take pointers to them store
them in variables, etc. You can't use the dot syntax, you must explicitly name
the method using the fully explicit form of naming (sometimes called UFCS for
universal function call syntax). The `self` parameter is the first argument to
the method. E.g.,

```rust
struct Foo;

impl Foo {
    fn bar(&self) {}
}

trait T {
    fn baz(&self);
}

impl T for Foo {
    fn baz(&self) {}
}

fn main() {
    // Inherent method.
    let x = Foo::bar;
    x(&Foo);
    
    // Trait method, note the fully explicit naming form.
    let y = <Foo as T>::baz;
    y(&Foo);
}
```


### Generic functions

You can't take a pointer to a generic function and there is no way to express a
generic function type. However, you can take a reference to a function if all
its type parameters are instantiated. E.g.,

```rust
fn foo<T>(x: &T) {}

fn main() {
    let x = &foo::<i32>;
    x(&42);
}
```

There is no way to define a generic closure. If you need a closure to work over
many types you can use trait objects, macros (for generating closures), or pass
a closure which returns closures (each returned closure can operate on a
different type).


### Lifetime-generic functions and higher-ranked types

It *is* possible to have function types and closures which are generic over
lifetimes. 

Imagine we have a closure which takes a borrowed reference. The closure can work
the same way no matter what lifetime the reference has (and indeed in the
compiled code, the lifetime will have been erased). But, what does the type look
like?

For example,

```rust
fn foo<F>(x: &Bar, f: F) -> &Baz
    where F: Fn(&Bar) -> &Baz
{
    f(x)
}
```

what are the lifetimes of the references here? In this simple example, we can
use a single lifetime (no need for a generic closure):

```rust
fn foo<'b, F>(x: &'b Bar, f: F) -> &'b Baz
    where F: Fn(&'b Bar) -> &'b Baz
{
    f(x)
}
```

But what if we want `f` to work on inputs with different lifetimes? Then we need
a generic function type:

```rust
fn foo<'b, 'c, F>(x: &'b Bar, y: &'c Bar, f: F) -> (&'b Baz, &'c Baz)
    where F: for<'a> Fn(&'a Bar) -> &'a Baz
{
    (f(x), f(y))
}
```

The novelty here is the `for<'a>` syntax, this is used to denote a function type
which is generic over a lifetime. It is read "for all 'a, ...". In theoretical
terms, the function type is universally quantified.

Note that we cannot hoist up `'a` to `foo` in the above example. Counter-example:

```rust
fn foo<'a, 'b, 'c, F>(x: &'b Bar, y: &'c Bar, f: F) -> (&'b Baz, &'c Baz)
    where F: Fn(&'a Bar) -> &'a Baz
{
    (f(x), f(y))
}
```

will not compile because when the compiler infers lifetimes for a call to `foo`,
it must pick a single lifetime for `'a`, which it can't do if `'b` and `'c` are
different.

A function type which is generic in this way is called a higher-ranked type.
Lifetime variables at the outer level have rank one. Because `'a` in the above
example cannot be moved to the outer level, it's rank is higher than one.

Calling functions with higher-ranked function type arguments is easy - the
compiler will infer the lifetime parameters. E.g., `foo(&Bar { ... }, &Bar
{...}, |b| &b.field)`.

In fact, most of the time you don't even need to worry about such things. The
compiler will allow you to elide the quantified lifetimes in the same way that
you are allowed to elide many lifetimes on function arguments. For example, the
example above can just be written as

```rust
fn foo<'b, 'c, F>(x: &'b Bar, y: &'c Bar, f: F) -> (&'b Baz, &'c Baz)
    where F: Fn(&Bar) -> &Baz
{
    (f(x), f(y))
}
```

(and you only need `'b` and `'c` because it is a contrived example).

Where Rust sees a function type with a borrowed references, it will apply the
usual elision rules, and quantify the elided variables at the scope of the
function type (i.e., with higher rank).

You might be wondering why bother with all this complexity for what looks like a
fairly niche use case. The real motivation is functions which take a function
to operate on some data provided by the outer function. For example,

```rust
fn foo<F>(f: F)
    where F: Fn(&i32) // Fully explicit type: for<'a> Fn(&'a i32)
{
    let data = 42;
    f(&data)
}
```

In these cases, we *need* higher-ranked types. If we added a lifetime parameter
to `foo` instead, we could never infer a correct lifetime. To see why, let's
look at how it might work, consider `fn foo<'a, F: Fn(&'a i32')> ...`. Rust
requires that any lifetime parameter must outlive the item it is declared on (if
this were not the case, an argument with that lifetime could be used inside that
function, where it is not guaranteed to be live). In the body of `foo` we use
`f(&data)`, the lifetime Rust will infer for that reference will last (at most)
from where `data` is declared to where it goes out of scope. Since `'a` must
outlive `foo`, but that inferred lifetime does not, we cannot call `f` in this
way.

However, with higher-ranked lifetimes `f` can accept any lifetime and so the
anonymous one from `&data` is fine and the function type checks.


### Enum constructors

This is something of a digression, but it is sometimes a useful trick. All
variants of an enum define a function from the fields of the variant to the enum
type. For example,

```rust
enum Foo {
    Bar,
    Baz(i32),
}
```

defines two functions, `Foo::Bar: Fn() -> Foo` and `Foo::Baz: Fn(i32) -> Foo`.
We don't normally use the variants in this way, we treat them as data types
rather than functions. But sometimes it is useful, for example if we have a list
of `i32`s we can create a list of `Foo`s with

```rust
list_of_i32.iter().map(Foo::Baz).collect()
```


## Closure flavours

A closure has two forms of input: the arguments which are passed to it explicitly
and the variables it *captures* from its environment. Usually, everything about
both kinds of input is inferred, but you can have more control if you want it.

For the arguments, you can declare types instead of letting Rust infer them. You
can also declare a return type. Rather than writing `|x| { ... }` you can write
`|x: i32| -> String { ... }`. Whether an argument is owned or borrowed is 
determined by the types (either declared or inferred).

For the captured variables, the type is mostly known from the environment, but
Rust does a little extra magic. Should a variable be captured by reference or
value? Rust infers this from the body of the closure. If possible, Rust captures
by reference. E.g.,

```rust
fn foo(x: Bar) {
    let f = || { ... x ... };
}
```

All being well, in the body of `f`, `x` has the type `&Bar` with a lifetime
bounded by the scope of `foo`. However, if `x` is mutated, then Rust will infer
that the capture is by mutable reference, i.e., `x` has type `&mut Bar`. If `x`
is moved in `f` (e.g., is stored into a variable or field with value type), then
Rust infers that the variable must be captured by value, i.e., it has the type
`Bar`.

This can be overridden by the programmer (sometimes necessary if the closure
will be stored in a field or returned from a function). By using the `move`
keyword in front of a closure. Then, all of the captured variables are captured
by value. E.g., in `let f = move || { ... x ... };`, `x` would always have type
`Bar`.

We talked earlier about the different function kinds: `Fn`, `FnMut`, and `FnOnce`.
We can now explain why we need them. For closures, the mutable-ness and once-ness
refer to the captured variables. If a capture mutates any of the variables it
captures then it will have a `FnMut` type (note that this is completely inferred
by the compiler, no annotation is necessary). If a variable is moved into a
closure, i.e., it is captured by value (either because of an explicit `move` or
due to inference), then the closure will have a `FnOnce` type. It would be unsafe
to call such a closure multiple times because the captured variable would be
moved more than once.

Rust will do its best to infer the most flexible type for the closure if it can.


## Implementation

A closure is implemented as an anonymous struct. That struct has a field for
each variable captured by the closure. It is lifetime-parametric with a single
lifetime parameter which is a bound on the lifetime of captured variables. The
anonymous struct implements a `call` method which is called to execute the
closure.

For example, consider

```rust
fn main() {
    let x = Foo { ... };
    let f = |y| x.get_number() + y;
    let z = f(42);
}
```

the compiler treats this as

```rust
struct Closure14<'env> {
    x: &'env Foo,
}

// Not actually implemented like this, see below.
impl<'env> Closure14<'env> {
    fn call(&self, y: i32) -> i32 {
        self.x.get_number() + y
    }
}

fn main() {
    let x = Foo { ... };
    let f = Closure14 { x: x }
    let z = f.call(42);
}
```

As we mentioned above, there are three different function traits - `Fn`,
`FnMut`, and `FnOnce`. In reality the `call` method is required by these traits
rather than being in an inherent impl. `Fn` has a method `call` which takes
`self` by reference, `FnMut` has `call_mut` taking `self` by mutable reference,
and `FnOnce` has `call_once` which takes `self` by values.

When we've seen function types above, they look like `Fn(i32) -> i32` which
doesn't look much like a trait type. There is a little bit of magic here. Rust allows
this round bracket sugar only for function types. To desugar to a regular type
(an 'angle bracket type'), the argument types are treated as a tuple type and
passed as a type parameter and the return type as an associated type called
`Output`. So, `Fn(i32) -> i32` is desugared to `Fn<(i32,), Output=i32>` and the
`Fn` trait definition looks like

```rust
pub trait Fn<Args> : FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}
```

The implementation for `Closure14` above would therefore look more like

```rust
impl<'env> FnOnce<(i32,)> for Closure14<'env> {
    type Output = i32;
    fn call_once(self, args: (i32,)) -> i32 {
        ...
    }
}
impl<'env> FnMut<(i32,)> for Closure14<'env> {
    fn call_mut(&mut self, args: (i32,)) -> i32 {
        ...
    }
}
impl<'env> Fn<(i32,)> for Closure14<'env> {
    fn call(&self, args: (i32,)) -> i32 {
        ...
    }
}
```

You can find the function traits in
[core::ops](https://dxr.mozilla.org/rust/source/src/libcore/ops.rs)

We talked above about how using generics gives static dispatch and using trait
objects gives virtual dispatch. We can now see in a bit more detail why.

When we call `call`, it is a statically dispatched method call, there is no
virtual dispatch. If we pass it to a monomorphised function, we still know the
type statically, and we still get a static dispatch.

We can make the closure into a trait object, e.g., `&f` or `Box::new(f)` with
types `&Fn(i32)->i32` or `Box<Fn(i32)->i32>`. These are pointer types, and
because they are pointer-to-trait types, the pointers are fat pointers. That
means they consist of the pointer to the data itself and a pointer to a vtable.
The vtable is used to lookup the address of `call` (or `call_mut` or whatever).

You'll sometimes hear these two representations of closures called boxed and
unboxed closures. An unboxed closure is the by-value version with static
dispatch. A boxed version is the trait object version with dynamic dispatch. In
the olden days, Rust only had boxed closures (and the system was quite a bit
different).

## References

* [RFC 114 - Closures](https://github.com/rust-lang/rfcs/blob/master/text/0114-closures.md)
* [Finding Closure in Rust blog post](http://huonw.github.io/blog/2015/05/finding-closure-in-rust/)
* [RFC 387 - Higher ranked trait bounds](https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md)
* [Purging proc blog post](http://smallcultfollowing.com/babysteps/blog/2014/11/26/purging-proc/)

FIXME: relate to closures in C++ 11
