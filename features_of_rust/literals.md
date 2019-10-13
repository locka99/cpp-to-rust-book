# Literals

## C++

### Integers

Integer numbers are a decimal value followed by an optional type suffix.

In C++ an [integer literal](http://en.cppreference.com/w/cpp/language/integer_literal#The_type_of_the_literal) can be expressed as just the number or also with a suffix. Values in hexadecimal, octal and binary are denoted with a prefix:

```c++
// Integers
42
999U
43424234UL
-3456676L
329478923874927ULL
-80968098606968LL
// C++14
329'478'923'874'927ULL
// Hex, octal, binary
0xfffe8899bcde3728 // or 0X
07583752256
0b111111110010000 // or 0B
```

The `u`, `l`, and `ll` suffixes on integers denotes if it is `unsigned`, `long` or a `long long` type. The `u` and `l`/`ll` can be upper or lowercase. Ordinarily the `u` must precede the size but C++14 allows the reverse order, i.e. `ull` or `llu`.

C++14 also allows single quotes to be inserted into the number as separators - these quotes can appear anywhere and are ignored.

### Floating point numbers

Floating point numbers may represent whole or fractional numbers.

C++ offers `float` and `double` types that typically map onto the corresponding 32-bit or 64-bit IEEE-754 floating point number. All the C and C++ specifications say is that `float` must be less than or the same number of bytes as a `double`

By default floating point numbers are `double` unless they have a `f` suffix.

```c++
100.0
0.134
2.3f
12e+99
```

Compilers can also support a `long double` type which may be 64-bit (same as `double`) but may be higher such as a 80-bits (padded to 12-bytes). This is architecture dependent - x86 processors support an extended precision format with 80-bits whereas other architectures may not.

### Boolean values

C/C++ `bool` literals are `true` or `false`. The language supports
integral promotion where `true` can evaluate 1 and `false` to 0.

### Characters and Strings

A character literal is enclosed by single quotes and an optional width prefix. The prefix `L` indicates a wide character, `u` for UTF-16 and `U` for UTF-32.

```c++
'a'
L'a' // wchar_t
u'\u20AC' // char16_t
U'\U0001D11E' // char32_t
```

One oddity of a `char` literal is that `sizeof('a')` yields `sizeof(int)` in C but `sizeof(char)` in C++. It isn't a good idea to test the size of a character literal.

A `char16_t` and `char32_t` are sufficient to hold any UTF-16 and UTF-32 code unit respectively.

A string is a sequence of characters enclosed by double quotes. A zero value terminator is always appended to the end. Prefixes work the same as for character literals with an additional `u8` type to indicate a UTF-8 encoded string.

```c++
"Hello"
u8"Hello" // char with UTF-8
L"Hello"   // wchar_t
u"Hello"   // char16_t with UTF-16
U"Hello"   // char32_t with UTF-32
```

### User-defined literals

C++11 introduced [user-defined literals](http://en.cppreference.com/w/cpp/language/user_literal). These allow integer, floats, chars and strings to have a user defined type suffix consisting of an underscore and a lowercase string. The prefix may act as a form of decorator or even a constant expression operator which modifies the value at compile time.

C++14 goes further and defines user-defined literals for complex numbers and units of time.

See the link for more information.

## Rust

## Integers

In Rust [number literals](https://doc.rust-lang.org/reference.html#integer-literals) can also be expressed as just the number or also with a suffix. The suffix denotes the signage and bit size of the value, e.g. `u8` is an unsigned 8-bit value, whereas a `i64` is a signed 64-bit value.

There are two special sizes called `usize` (unsigned) and `isize` signed which are not a fixed size. These values are designed for indexing and memory addressing and therefore their size is architecture dependent. On a 32-bit architecture, the `usize`/`isize` types are 32-bits, on a 64-bit architecture they are 64-bits. In Rust you will find that every collection class uses the `usize` and `isize` for every index addressable operation.

Values in hexadecimal, octal and binary are also denoted with a prefix:

```rust
// Integers
123i32
123u32
123_444_474u32
0usize
// Hex, octal, binary
0xff_u8
0o70_i16
0b111_111_11001_0000_i32
```

The underscore in Rust is a separator and functions the same way as the single quote in C++14.

### Floating point numbers

Floating point numbers may represent whole or fractional numbers. As with integers they may be suffixed to indicate their type.

```rust
100.0f64
0.134f64
2.3f32 // But 2.f32 is not valid (note 1)
12E+99_E64
```

One quirk with floating point numbers is the decimal point is used for float assignments but it's also used for member and function invocation. So you can't say `2.f32` since it thinks you are referencing f32 on 2. Instead syntax requires you to say `2.0f32` or alter how you declare the type, e.g. `let v: f32 = 2.;`.

Rust does not presently support 80-bit format floats.

## Booleans

Boolean literals are simply `true` or `false`.

```rust
true
false
```

There is no integral promotion in Rust so you cannot just pass an expression such as `1` to a function that takes a boolean (or `false` to a function that takes an integer).

## Characters and Strings

A character in Rust is any UTF-32 code point enclosed by single quotes. This value may be escaped or not since .rs files are UTF-8 encoded. 

A special prefix `b` may be used to denote a byte string, i.e. a string where each character is a single byte.

```rust
'x'
'\'' # Escaped single quote
b'&' # byte character is a u8
```

Strings are the string text enclosed by double quotes:

```rust
"This is a string"
b"This is a byte string"
```

The prefix `b` denotes a byte string, i.e. single byte characters. Rust allows newlines, space, double quotes and backslashes to be escaped using backslash notation similar to C++.

```rust
"This is a \
  multiline string"
"This string has a newline\nand an escaped \\ backslash"
"This string has a hex char \x52"
```

Strings can also be 'raw' to avoid escaping. In this case, the string is prefixed r followed by zero or more hash marks, the string in double quotes and the same number of hash marks to close. Byte strings are uninterpretted byte values in a string.

```rust
r##"This is a raw string that can contain \n, \ and other stuff without escaping"##
br##"A raw byte string with "stuff" like \n \, \u and other things"##
```
