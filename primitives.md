# Primitive types and operators

Rust has pretty much the same arithmetic and logical operators as C++. `bool` is
the same in both languages (as are the `true` and `false` literals). Rust has
similar concepts of integers, unsigned integers, and floats. However the syntax
is a bit different. Rust uses `isize` to mean an integer and `usize` to mean an
unsigned integer. These types are pointer sized. E.g., on a 32 bit system,
`usize` means a 32 bit unsigned integer. Rust also has explicitly sized types
which are `u` or `i` followed by 8, 16, 32, or 64. So, for example, `u8` is an 8
bit unsigned integer and `i32` is a 32 bit signed integer. For floats, Rust has
`f32` and `f64`.

Numeric literals can take suffixes to indicate their type (using `i` and `u`
instead of `isize` and `usize`). If no suffix is given, Rust tries to infer the
type. If it can't infer, it uses `isize` or `f64` (if there is a decimal point).
Examples:

```rust
fn main() {
    let x: bool = true;
    let x = 34;   // type isize
    let x = 34u;  // type usize
    let x: u8 = 34u8;
    let x = 34i64;
    let x = 34f32;
}
```

As a side note, Rust lets you redefine variables so the above code is legal -
each `let` statement creates a new variable `x` and hides the previous one. This
is more useful than you might expect due to variables being immutable by
default.

Numeric literals can be given as binary, octal, and hexadecimal, as well as
decimal. Use the `0b`, `0o`, and `0x` prefixes, respectively. You can use an
underscore anywhere in a numeric literal and it will be ignored. E.g,

```rust
fn main() {
    let x = 12;
    let x = 0b1100;
    let x = 0o14;
    let x = 0xe;
    let y = 0b_1100_0011_1011_0001;
}
```

Rust has chars and strings, but since they are Unicode, they are a bit different
from C++. I'm going to postpone talking about them until after I've introduced
pointers, references, and vectors (arrays).

Rust does not implicitly coerce numeric types. In general, Rust has much less
implicit coercion and subtyping than C++. Rust uses the `as` keyword for
explicit coercions and casting. Any numeric value can be cast to another numeric
type. `as` cannot be used to convert between booleans and numeric types. E.g.,

```rust
fn main() {
    let x = 34u as isize;   // cast usize to isize
    let x = 10 as f32;      // isize to float
    let x = 10.45f64 as i8; // float to i8 (loses precision)
    let x = 4u8 as u64;     // gains precision
    let x = 400u16 as u8;   // 144, loses precision (and thus changes the value)
    println!("`400u16 as u8` gives {}", x);
    let x = -3i8 as u8;     // 253, signed to unsigned (changes sign)
    println!("`-3i8 as u8` gives {}", x);
    //let x = 45u as bool;  // FAILS!
}
```

Rust has the following numeric operators: `+`, `-`, `*`, `/`, `%`; bitwise
operators: `|`, `&`, `^`, `<<`, `>>`; comparison operators: `==`, `!=`, `>`,
`<`, `>=`, `<=`; short-circuit logical operators: `||`, `&&`. All of these
behave as in C++, however, Rust is a bit stricter about the types the operators
can be applied to - the bitwise operators can only be applied to integers and
the logical operators can only be applied to booleans. Rust has the `-` unary
operator which negates a number. The `!` operator negates a boolean and inverts
every bit on an integer type (equivalent to `~` in C++ in the latter case). Rust
has compound assignment operators as in C++, e.g., `+=`, but does not have
increment or decrement operators (e.g., `++`).
