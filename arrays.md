# Arrays and Vectors

Rust arrays are pretty different from C arrays. For starters they come in
statically and dynamically sized flavours. These are more commonly known as
fixed length arrays and slices. As we'll see, the former is kind of a bad name
since both kinds of array have fixed (as opposed to growable) length. For a
growable 'array', Rust provides the `Vec` collection.


## Fixed length arrays

The length of a fixed length array is known statically and features in its
type. E.g., `[i32; 4]` is the type of an array of `i32`s with length four.

Array literal and array access syntax is the same as C:

```rust
let a: [i32; 4] = [1, 2, 3, 4];     // As usual, the type annotation is optional.
println!("The second element is {}", a[1]);
```

You'll notice that array indexing is zero-based, just like C.

However, unlike C/C++<sup>[1](#1)</sup>, array indexing is bounds checked. In
fact all access to arrays is bounds checked, which is another way Rust is a
safer language.

If you try to do `a[4]`, then you will get a runtime panic. Unfortunately, the
Rust compiler is not clever enough to give you a compile time error, even when
it is obvious (as in this example).

If you like to live dangerously, or just need to get every last ounce of
performance out of your program, you can still get unchecked access to arrays.
To do this, use the `get_unchecked` method on an array. Unchecked array accesses
must be inside an unsafe block. You should only need to do this in the rarest
circumstances.

Just like other data structures in Rust, arrays are immutable by default and
mutability is inherited. Mutation is also done via the indexing syntax:

```rust
let mut a = [1, 2, 3, 4];
a[3] = 5;
println!("{:?}", a);
```

And just like other data, you can borrow an array by taking a reference to it:

```rust
fn foo(a: &[i32; 4]) {
    println!("First: {}; last: {}", a[0], a[3]);
}

fn main() {
    foo(&[1, 2, 3, 4]);
}
```

Notice that indexing still works on a borrowed array.

This is a good time to talk about the most interesting aspect of Rust arrays for
C++ programmers - their representation. Rust arrays are value types: they are
allocated on the stack like other values and an array object is a sequence of
values, not a pointer to those values (as in C). So from our examples above, `let
a = [1_i32, 2, 3, 4];` will allocate 16 bytes on the stack and executing `let b
= a;` will copy 16 bytes. If you want a C-like array, you have to explicitly
make a pointer to the array, this will give you a pointer to the first element.

A final point of difference between arrays in Rust and C++ is that Rust arrays
can implement traits, and thus have methods. To find the length of an array, for
example, you use `a.len()`.


## Slices

A slice in Rust is just an array whose length is not known at compile time. The
syntax of the type is just like a fixed length array, except there is no length:
e.g., `[i32]` is a slice of 32 bit integers (with no statically known length).

There is a catch with slices: since the compiler must know the size of all
objects in Rust, and it can't know the size of a slice, then we can never have a
value with slice type. If you try and write `fn foo(x: [i32])`, for example, the
compiler will give you an error.

So, you must always have pointers to slices (there are some very technical
exceptions to this rule so that you can implement your own smart pointers, but
you can safely ignore them for now). You must write `fn foo(x: &[i32])` (a
borrowed reference to a slice) or `fn foo(x: *mut [i32])` (a mutable raw pointer
to a slice), etc.

The simplest way to create a slice is by coercion. There are far fewer implicit
coercions in Rust than there are in C++. One of them is the coercion from fixed
length arrays to slices. Since slices must be pointer values, this is
effectively a coercion between pointers. For example, we can coerce `&[i32; 4]`
to `&[i32]`, e.g.,

```rust
let a: &[i32] = &[1, 2, 3, 4];
```

Here the right hand side is a fixed length array of length four, allocated on
the stack. We then take a reference to it (type `&[i32; 4]`). That reference is
coerced to type `&[i32]` and given the name `a` by the let statement.

Again, access is just like C (using `[...]`), and access is bounds checked. You
can also check the length yourself by using `len()`. So clearly the length of
the array is known somewhere. In fact all arrays of any kind in Rust have known
length, since this is essential for bounds checking, which is an integral part
of memory safety. The size is known dynamically (as opposed to statically in the
case of fixed length arrays), and we say that slice types are dynamically sized
types (DSTs, there are other kinds of dynamically sized types too, they'll be
covered elsewhere).

Since a slice is just a sequence of values, the size cannot be stored as part of
the slice. Instead it is stored as part of the pointer (remember that slices
must always exist as pointer types). A pointer to a slice (like all pointers to
DSTs) is a fat pointer - it is two words wide, rather than one, and contains the
pointer to the data plus a payload. In the case of slices, the payload is the
length of the slice.

So in the example above, the pointer `a` will be 128 bits wide (on a 64 bit
system). The first 64 bits will store the address of the `1` in the sequence
`[1, 2, 3, 4]`, and the second 64 bits will contain `4`. Usually, as a Rust
programmer, these fat pointers can just be treated as regular pointers. But it
is good to know about (it can affect the things you can do with casts, for
example).


### Slicing notation and ranges

A slice can be thought of as a (borrowed) view of an array. So far we have only
seen a slice of the whole array, but we can also take a slice of part of an
array. There is a special notation for this which is like the indexing
syntax, but takes a range instead of a single integer. E.g., `a[0..4]`, which
takes a slice of the first four elements of `a`. Note that the range is
exclusive at the top and inclusive at the bottom. Examples:

```rust
let a: [i32; 4] = [1, 2, 3, 4];
let b: &[i32] = &a;   // Slice of the whole array.
let c = &a[0..4];     // Another slice of the whole array, also has type &[i32].
let c = &a[1..3];     // The middle two elements, &[i32].
let c = &a[1..];      // The last three elements.
let c = &a[..3];      // The first three elements.
let c = &a[..];       // The whole array, again.
let c = &b[1..3];     // We can also slice a slice.
```

Note that in the last example, we still need to borrow the result of slicing.
The slicing syntax produces an unborrowed slice (type: `[i32]`) which we must
then borrow (to give a `&[i32]`), even if we are slicing a borrowed slice.

Range syntax can also be used outside of slicing syntax. `a..b` produces an
iterator which runs from `a` to `b-1`. This can be combined with other iterators
in the usual way, or can be used in `for` loops:

```rust
// Print all numbers from 1 to 10.
for i in 1..11 {
    println!("{}", i);
}
```

## Vecs

A vector is heap allocated and is an owning reference. Therefore (and like
`Box<_>`), it has move semantics. We can think of a fixed length array
analogously to a value, a slice to a borrowed reference. Similarly, a vector in
Rust is analogous to a `Box<_>` pointer.

It helps to think of `Vec<_>` as a kind of smart pointer, just like `Box<_>`,
rather than as a value itself. Similarly to a slice, the length is stored in the
'pointer', in this case the 'pointer' is the Vec value.

A vector of `i32`s has type `Vec<i32>`. There are no vector literals, but we can
get the same effect by using the `vec!` macro. We can also create an empty
vector using `Vec::new()`:

```rust
let v = vec![1, 2, 3, 4];      // A Vec<i32> with length 4.
let v: Vec<i32> = Vec::new();  // An empty vector of i32s.
```

In the second case above, the type annotation is necessary so the compiler can
know what the vector is a vector of. If we were to use the vector, the type
annotation would probably not be necessary.

Just like arrays and slices, we can use indexing notation to get a value from
the vector (e.g., `v[2]`). Again, these are bounds checked. We can also use
slicing notation to take a slice of a vector (e.g., `&v[1..3]`).

The extra feature of vectors is that their size can change - they can get longer
or shorter as needed. For example, `v.push(5)` would add the element `5` to the
end of the vector (this would require that `v` is mutable). Note that growing a
vector can cause reallocation, which for large vectors can mean a lot of
copying. To guard against this you can pre-allocate space in a vector using
`with_capacity`, see the [Vec docs](https://doc.rust-lang.org/std/vec/struct.Vec.html)
for more details.


## The `Index` traits

Note for readers: there is a lot of material in this section that I haven't
covered properly yet. If you're following the tutorial, you can skip this
section, it is a somewhat advanced topic in any case.

The same indexing syntax used for arrays and vectors is also used for other
collections, such as `HashMap`s. And you can use it yourself for your own
collections. You opt-in to using the indexing (and slicing) syntax by
implementing the `Index` trait. This is a good example of how Rust makes
available nice syntax to user types, as well as built-ins (`Deref` for
dereferencing smart pointers, as well as `Add` and various other traits, work in
a similar way).

The `Index` trait looks like

```rust
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}
```

`Idx` is the type used for indexing. For most uses of indexing this is `usize`.
For slicing this is one of the `std::ops::Range` types. `Output` is the type
returned by indexing, this will be different for each collection. For slicing it
will be a slice, rather than the type of a single element. `index` is a method
which does the work of getting the element(s) out of the collection. Note that
the collection is taken by reference and the method returns a reference to the
element with the same lifetime.

Let's look at the implementation for `Vec` to see how what an implementation
looks like:

```rust
impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &(**self)[index]
    }
}
```

As we said above, indexing is done using `usize`. For a `Vec<T>`, indexing will
return a single element of type `T`, thus the value of `Output`. The
implementation of `index` is a bit weird - `(**self)` gets a view of the whole
vec as a slice, then we use indexing on slices to get the element, and finally
take a reference to it.

If you have your own collections, you can implement `Index` in a similar way to
get indexing and slicing syntax for your collection.


## Initialiser syntax

As with all data in Rust, arrays and vectors must be properly initialised. Often
you just want an array full of zeros to start with and using the array literal
syntax is a pain. So Rust gives you a little syntactic sugar to initialise an
array full of a given value: `[value; len]`. So for example to create an array
with length 100 full of zeros, we'd use `[0; 100]`.

Similarly for vectors, `vec![42; 100]` would give you a vector with 100
elements, each with the value 42.

The initial value is not limited to integers, it can be any expression. For
array initialisers, the length must be an integer constant expression. For
`vec!`, it can be any expression with type `usize`.


##### 1

In C++11 there is `std::array<T, N>` that provides boundary checking when
`at()` method is used.
