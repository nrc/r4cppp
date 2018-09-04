# Control flow

## If

The `if` statement is pretty much the same in Rust as C++. One difference is
that the braces are mandatory, but parentheses around the expression being tested
are not. Another is that `if` is an expression, so you can use it the same way
as the ternary `?:` operator in C++ (remember from the previous section that if the last
expression in a block is not terminated by a semi-colon, then it becomes the
value of the block). There is no ternary `?:` in Rust. So, the following two
functions do the same thing:

```rust
fn foo(x: i32) -> &'static str {
    let mut result: &'static str;
    if x < 10 {
        result = "less than 10";
    } else {
        result = "10 or more";
    }
    return result;
}

fn bar(x: i32) -> &'static str {
    if x < 10 {
        "less than 10"
    } else {
        "10 or more"
    }
}
```

The first is a fairly literal translation of what you might write in C++. The
second is better Rust style.

You can also write `let x = if ...`, etc.


## Loops

Rust has while loops, again just like C++:

```rust
fn main() {
    let mut x = 10;
    while x > 0 {
        println!("Current value: {}", x);
        x -= 1;
    }
}
```

There is no `do...while` loop in Rust, but there is the `loop` statement which
just loops forever:

```rust
fn main() {
    loop {
        println!("Just looping");
    }
}
```

Rust has `break` and `continue` just like C++.


## For loops

Rust also has `for` loops, but these are a bit different. Lets say you have a
vector of integers and you want to print them all (we'll cover vectors/arrays,
iterators, and generics in more detail in the future. For now, know that a
`Vec<T>` is a sequence of `T`s and `iter()` returns an iterator from anything
you might reasonably want to iterate over). A simple `for` loop would look like:

```rust
fn print_all(all: Vec<i32>) {
    for a in all.iter() {
        println!("{}", a);
    }
}
```

TODO also &all/all instead of all.iter()

If we want to index over the indices of `all` (a bit more like a standard C++
for loop over an array), you could do

```rust
fn print_all(all: Vec<i32>) {
    for i in 0..all.len() {
        println!("{}: {}", i, all[i]);
    }
}
```

Hopefully, it is obvious what the `len` function does. TODO range notation

A more Rust-like equivalent of the preceding example would be to use an
enumerating iterator:

```rust
fn print_all(all: Vec<i32>) {
    for (i, a) in all.iter().enumerate() {
        println!("{}: {}", i, a);
    }
}
```

Where `enumerate()` chains from the iterator `iter()` and yields the current
count and the element during iteration.

*The following example incorporates more advanced topics covered in the section
on [Borrowed Pointers](borrowed.md).* Let's say you have a vector of integers
and want to call the function, passing the vector by reference and have the
vector modified in place. Here the `for` loop uses a mutable iterator which
gives mutable refererences - the `*` dereferencing should be familiar to C++
programmers:

```rust
fn double_all(all: &mut Vec<i32>) {
    for a in all.iter_mut() {
        *a += *a;
    }
}
```


## Switch/Match

Rust has a match expression which is similar to a C++ switch statement, but much
more powerful. This simple version should look pretty familiar:

```rust
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        y => println!("x is something else {}", y),
    }
}
```

There are some syntactic differences - we use `=>` to go from the matched value
to the expression to execute, and the match arms are separated by `,` (that last
`,` is optional). There are also some semantic differences which are not so
obvious: the matched patterns must be exhaustive, that is all possible values of
the matched expression (`x` in the above example) must be covered. Try removing
the `y => ...` line and see what happens; that is because we only have matches
for 0, 1, and 10, but there are obviously lots of other integers which don't get
matched. In that last arm, `y` is bound to the value being matched (`x` in this
case). We could also write:

```rust
fn print_some(x: i32) {
    match x {
        x => println!("x is something else {}", x)
    }
}
```

Here the `x` in the match arm introduces a new variable which hides the argument
`x`, just like declaring a variable in an inner scope.

If we don't want to name the variable, we can use `_` for an unnamed variable,
which is like having a wildcard match. If we don't want to do anything, we can
provide an empty branch:

```rust
fn print_some(x: i32) {
    match x {
        0 => println!("x is zero"),
        1 => println!("x is one"),
        10 => println!("x is ten"),
        _ => {}
    }
}
```

Another semantic difference is that there is no fall through from one arm to the
next so it works like `if...else if...else`.

We'll see in later posts that match is extremely powerful. For now I want to
introduce just a couple more features - the 'or' operator for values and `if`
clauses on arms. Hopefully an example is self-explanatory:

```rust
fn print_some_more(x: i32) {
    match x {
        0 | 1 | 10 => println!("x is one of zero, one, or ten"),
        y if y < 20 => println!("x is less than 20, but not zero, one, or ten"),
        y if y == 200 => println!("x is 200 (but this is not very stylish)"),
        _ => {}
    }
}
```

Just like `if` expressions, `match` statements are actually expressions so we
could re-write the last example as:

```rust
fn print_some_more(x: i32) {
    let msg = match x {
        0 | 1 | 10 => "one of zero, one, or ten",
        y if y < 20 => "less than 20, but not zero, one, or ten",
        y if y == 200 => "200 (but this is not very stylish)",
        _ => "something else"
    };

    println!("x is {}", msg);
}
```

Note the semi-colon after the closing brace, that is because the `let` statement
is a statement and must take the form `let msg = ...;`. We fill the rhs with a
match expression (which doesn't usually need a semi-colon), but the `let`
statement does. This catches me out all the time.

Motivation: Rust match statements avoid the common bugs with C++ switch
statements - you can't forget a `break` and unintentionally fall through; if you
add a case to an enum (more later on) the compiler will make sure it is covered
by your `match` statement.


## Method call

Finally, just a quick note that methods exist in Rust, similarly to C++. They
are always called via the `.` operator (no `->`, more on this in another post).
We saw a few examples above (`len`, `iter`). We'll go into more detail in the
future about how they are defined and called. Most assumptions you might make
from C++ or Java are probably correct.
