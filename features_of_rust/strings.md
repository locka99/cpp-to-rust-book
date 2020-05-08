# Strings

The way that Rust handles strings is quite a bit different from C or C++.

## Summary

In C/C++

* A string is a pointer to an array of `char`, `wchar_t`, `char16_t` or `char32_t` values. Historically most strings are `char` but efforts have been made to support wide character strings.
* A string's length is calculated by looking for a special nul (`'\0'`) value that signifies the end of the string. So a 2000 character string requires iterating through the whole string, potentially 2000 times looking for a nul.
* Only `char16_t` and `char32_t` types are considered to be Unicode (encoded as UTF-16, UTF-32 respectively). There is no encoding knowledge about the meaning of other kinds of string.
* In C++ types derived from `std::basic_string<>` template are the recommended way to manage strings safely. Other 3rd party libraries also have their own string wrappers, e.g. `QString` in QT.

In Rust

* A `char` is a primitive in Rust that is 32-bits and can represent any Unicode character. So it equivalent to `char32_t` in C++.
* A `str` is a primitive that represents a read-only string. Typically you will use a through a special borrow reference `&str`.
* A `&str`, also called a slice, consists of a pointer to a string and a length. So obtaining the length of a string is a cheap operation.
* A `String` is a heap allocated growable string. i.e. it can be truncated, extended etc. It implements all the functions of `str` and can also be used as a `&str`.
* All strings are internally coded with UTF-8 to reduce memory overhead but they can be iterated by `char`.

## What is a character exactly?

Historically in C and C++, a char type is 8-bits. Strictly speaking a char is signed type \(usually -128 to 127\), but the values essentially represent the values 0-255.

The US-ASCII standard uses the first 7-bits \(0-127\) to assign values to upper and lower case letters in the English alphabet, numbers, punctuation marks and certain control characters.

It didn't help the rest of the world who use different character sets. And even ASCII was competing with another standard called EBDIC which was found on mainframe computers.

What about the upper values from 128-255? Some operating systems came up with a concept called a "code page". According to what "code page" was in effect, the symbol that the user would see for a character in the 128-255 range would change.

But even this is not enough. Some languages like Chinese, Japanese, Korean, Thai, Arabic etc. have thousands of symbols that must be encoded with more than one byte. So the first byte might be a modifier that combines with further bytes to render as a symbol. For example Microsoft's code page 932 use an encoding called Shift JIS \(Japanese\) where some symbols are two bytes.

Obviously this was rapidly becoming a mess. Each code page interpretted the same byte array differently according to some external setting. So you could not send a file written in Chinese to someone with a different code page and expect it to render properly.

### Unicode to the rescue

The Unicode standard assigns every printable symbol in existence with a unique 32-bit value, called a code point. Most symbols fall in the first 16-bits called the Basic Multilingual Plane \(BMP\).

China has mandated all software must support all 32-bits. We'll see how this has become a major headache for C and C++

## C / C++

### There is no string primitive

C and C++ does not have a string primitive type, instead it has `char` type, that is one byte. A "string" is a pointer to an array of chars that are terminated with a zero byte, `'\0'`.

```c++
// The array that my_string points at ends with a hidden \0
const char *my_string = "This is as close to a string primitive as you can get";
printf("String is %d chars long.\n", strlen(my_string));
```

#### String functions

In C, functions such as `strlen(s)`, `strcpy(dst, src)`, `strdup(s)` etc. allow strings to be inspected, copied or duplicated. But they work all work by essentially walking the array of bytes until they reach the `\0`. So `strcpy(dst, src)` copies from `src` to `dst` a byte at a time up to and including the `\0`. 

But what if `dst` was not a big enough array to hold the input? Well now we have a buffer overrun which may crash the software, or cause it to be compromisd.

C11 introduces "safe" versions of functions like `strcpy` that say how large the destination is, i.e. `strcpy_s(dst, dstlen, src)`. It will not overstep the output buffer's size and if the output is not big enough to hold the input or is truncated, the function will return an error. Even so, we are required to set this size correctly, include space for the terminator byte and test for errors.

#### std::string template

C++ provides `std::string` which can manage the lifetime of a string and provide methods for modifying the string in a safe manner. It is a vast improvement over C.

```c++
#include <string>
//...
std::string my_string = "Mary had a little lamb";
std::cout << "String is " << my_string.size() << " chars long." << std::endl;
```

However this is not a primitive type. Instead `std::string` is a fairly opaque template defined in `<string>` that is included, compiled and linked to the executable just like every other class. 

Now note that `std::string` is actually a specialisation of a more general template `std::basic_string<char_type>` that works with various width of character. We'll look at that in a moment.

In addition, a `std::string` will normally uses the heap to store the string's data which can have repercussions for memory usage and fragmentation. There is also a hidden cost to assigning one string to another strings are duplicated in the process. So even if you had 5 strings holding the same value, you would be looking at 5 copies.


#### std::string_view template

C++17 supports a `string_view` which is equivalent to a read-only slice of a string. This allows string operations to be more efficient, however it offers no guarantees that the view points to anything by the time it is accessed.

### Unicode support

C/C++ added Unicode support by creating a wide character called `wchar_t`. And C++ has an equivalent `std::wstring` which is `std::basic_string<wchar_t>`.

We're sorted now right?

Oops no, because `wchar_t` type can be either 2 or 4 bytes wide and is a compiler / platform specific decision.

In Microsoft Visual C++ the wide char is an `unsigned short` \(corresponding to Win32's Unicode API\), in gcc it can be 32-bits or 16-bits according to the compile flags.

A 16-bit value will hold symbols from the Basic Multilingual Plane but not the full 32-bit range. This means that 16-bit wide strings should be assumed to be UTF-16 encoded because they cannot support Unicode properly otherwise.

C++11 rectifies this by introducing explicit `char16_t` and `char32_t` types and corresponding versions of string called `std::u16string` and `std::u32string`.

### Character types

So now C++ has 4 character types. Great huh?

| Character type | Encoding |
| --- | --- |
| `char` | C, ASCII, EBDIC, UTF-8, ad hoc, ??? |
| `wchar_t` | UTF-16 or UTF-32 |
| `char16_t` | UTF-16 |
| `char32_t` | UTF-32 |

## Rust

Rust has been rather fortunate. Unicode preceded it so it makes a very simple design choice.

* A `char` type is a 32-bit Unicode character, always enough to hold a single character. 
* A `str` type is a UTF-8 encoded string held in memory. Code tends to use &str which is a string slice, basically a reference to the str, or a portion of it. A str does not need to be terminated with a zero byte and can contain zero bytes if it wants.
* A `std::String` is a heap allocated string type use for manipulating strings, building them, reading them from file, cloning them etc.

Note that internally UTF-8 is used for encoding yet a char is 32-bits. The length of a strings is considered to be its byte length. There are special iterators for walking the string and decoding UTF-8 into 32-bit characters.

Finally there is a platform specific type `OSString` that handles any differences in how the operating system sees strings compared to Rust.

## Types Comparison

| C++ | Rust |
| --- | --- |
|  | `char *` or `wchar_t *` |
| C++11 - `char16_t *`, `char32_t *` | `str`, `&str` |
|  | `std::string`, `std::wstring` |
| `std::u16string` `std::u32string` | `std::String` |

## char \* vs str

C/C++ do not have a string primitive. A string is a pointer to some bytes in memory that are nul terminated. The same applies for wider chars, except of course they require 2 or 4 bytes.

```c++
// The traditional way
char *my_str = "Hello"; // Terminates with \0
// or
char my_str[] = "Hello"; // Terminates with \0
// or wide string with L prefix
wchar_t hello_chinese = L"\u4f60\u597d";
// C11 and C++11 add UTF string literal prefixes
auto hello_8  = u8"\u4f60\u597d"; // UTF-8 encoded
auto hello_16 =  u"\u4f60\u597d"; // UTF-16
auto hello_32 =  U"\u4f60\u597d"; // UTF-32
```

Rust would use a `str` for this purpose. A `str` is an _immutable_ array of bytes  somewhere in memory. The `str` could be on the heap when it points to a `String` object, or it could be in global memory if the string is static. A str _slice_ is `&str`, is reference to a str which also contains a length value.

```rust
let my_str = "Hello";
let hello_chinese = "你好";
```

Type inferences for these assignments will create a string slice pointing to the statically allocated string data. The data itself doesn't move and the `&str` is read-only.

We can also observe that Rust removes the mess of character width and literal prefixes that C and C++ have to suffer under because Unicode characters are implicitly supported.

The `str` has functions for iterating over the string in bytes / characters, splitting, find a pattern etc.

```rust
let my_str = "Hello"; // v is a &’static str
println!("My string is {} and it is {} bytes long", v, v.len());
```

Note `len()` is the length in bytes because strings are UTF-8 encoded. A single character may be encoded as 1, 2, 3, or 4 bytes. It may not be the number of characters a human would actually see.

```rust
let my_str = "你好";
println!("Number of bytes = {}", my_str.len());
println!("Number of chars = {}", my_str.chars().count());
```

```
Number of bytes = 6
Number of chars = 2
```

You can split a `&str` to produce a left and a right `&str` slice like this:

```rust
let (part1, part2) = "Hello".split_at(3);
println!("Part 1 = {}", part1);
println!("Part 2 = {}", part2);
```

```
Part 1 = Hel
Part 2 = lo
```

## std::basic\_string \(C++\) vs std::String \(Rust\)

The standard C++ library also has template class `std::basic_string` that acts as a wrapper around the various character types and can be used for manipulating a string of any width. This template is specialised as  
`std::string`, `std:wstring`, `std::u16string` and `std::u32string`.

```c++
std::string my_str = "Hello";
my_str += " world";

// C++11 also allows some type inference with autos
auto s1 =   "Hello"s; // std::string
auto s2 = u8"Hello"s; // std::string, forces UTF-8 encoding
auto s3 = L"Hello"s;  // std::wstring
auto s4 = u"Hello"s;  // std::u16string
auto s5 = U"Hello"s;  // std::u32string
```

In Rust, the `std::String` type serves the same purpose:

```rust
let v = String::from("Hello");
v.push_str(" world");
```

Using it is fairly simple

```rust
let mut v = String::from("This is a String");
v.push_str(" that we can modify");
```

A `String` has functions to do actions such as appending, e.g.

```rust
let b = String::from(" Bananas");
let mut result = String::new();
result.push_str("Apples ");
result.push('&'); // Push a char
result.push_str(b.as_str());
println!("result = {}", result);
```

Strings are always valid UTF-8.

Internally a String has a pointer to the data, its length and a capacity \(max size\). If you intend to expand a string, then you should ensure the `String` has sufficient capacity to accommodate its longest value otherwise you may cause it to reallocate itself excessively.

Strings will never shrink their capacity unless you explicitly call `shrink_to_fit()`. This means if you use a temporary string in a loop, it's probably best to place it outside the loop and reserve space to make it efficient.

```rust
let mut v = String::with_capacity(100);
// or
let mut v = String::new();
v.reserve_exact(100);
```

Strings also have all the methods of str thanks to implementing `Deref` trait.

### Formatting strings

Strings can be formatted in C with `printf` or `sprintf` or in C++ composed with stream operators, e.g. to a `std::stringstream`.

Rust uses `format!` and `println!` macros that more resemble the `sprintf` model. 

| C++ | Rust formatting trait | Purpose |
| :--- | :--- | :--- |
| `%s`, `%u`, `%d`, `%i`, `%f`, `%c` | `{}` | C/C++ require the type of the parameter to be specified. In Rust the type is inferred and `{}` will invoked the type's Display trait regardless of what it is. So a String outputs its text, numeric types return their value, boolean as returns true or false, and so on. |
| `%lld`, `%llu` | `{}` | C/C++ has extensions to deal with different size ints and floats, e.g. ll for long long due to the way arguments are passed to the function. In Rust, there is no need for that. |
|  | `{:?}`, `{:#?}` | In Rust `{:?}` returns whatever is implemented by a type's Debug trait. The `{:#?}` variant can be used to pretty-print the output for types that derive the Debug trait. |
| `%-10s` | `{:<10}` | Format left aligned string padded to minimum of 10 spaces |
| `%04` | `{:04}` | Pad a number with zero's to a width of 4 |
| `%.3` | `{:.3}` | Pad a number's precision to 3 decimal places. May also be zero-padded, e.g. {:.03} |
| `%e`, `%E` | `{:e}`, `{:E}` | Exponent in lower or uppercase |
| `%x`, `%X` | `{:x}`, `{:X}` | Hexadecimal in lower or uppercase. Note `{:#x}`, `{:#X}` prefixes the output with 0x |
| `%o` | `{:o}` | Octal. Note `{:#o}` prefixes the output with 0o |
|  | `{:b}` | Binary. Note `{:#b}` prefixes the output with 0b |
| `%p` | `{:p}` | Presents a struct's memory location, i.e. pointer |

Rust has many [more formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits) than this.

For example it allows named parameters like this example:

```rust
let message = format!("The temperature {temp}C is within {percent} of maximum", temp = 104, percent = 99);
```

Named parameters would be particularly useful for localization where the order of values may be different in one language compared to another.

## Display and Debug traits

Rust allows types to be formatted as strings based upon the formatting traits they implement.

The two main implementation traits are:

* `Display` - this is for standard textual representation of a type.
* `Debug` - this is for the debugging textual representation of a type. It might present additional information or be formatted separately to the Display trait. It is possible to `#[derive(Debug)]` this trait which is usually enough for the purpose of debugging.

If we look at the traits we can see they're identical

```rust
// std::fmt::Display
pub trait Display {
    fn fmt(&self, &mut Formatter) -> Result<(), Error>;
}
// std::fmt::Debug
pub trait Debug {
    fn fmt(&self, &mut Formatter) -> Result<(), Error>;
}
```

All of the intrinsic types implement `Display` and `Debug`. We can explicitly implement Display on our own structs too:

```rust
use std::fmt::{self, Formatter, Display};

struct Person {
  first_name: String,
  last_name: String,
}

impl Display for Person {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{} {}", self.first_name, self.last_name)
  }
}
//...
let person = Person { first_name: "Susan".to_string(), last_name: "Smith".to_string() };
println!("Person - {}", person);
```

```
Person - Susan Smith
```

Implementing `Debug` is usually done by `#[derive(Debug)]` but it could also be implemented. The derived `Debug` will print out the struct name, and then the members in curly braces:

```rust
#[derive(Debug)]
struct Person {
  //...
}
//...
println!("Person - {:?}", person);
```

```
Person - Person { first_name: "Susan", last_name: "Smith" }
```

Many types process formatting traits which are values held between the `{}` braces in the string. These are fairly similar to the patterns used in C functions for printf, sprintf etc.

## OsString / OsStr

Rust recognises there are times when you need to pass or receive a string from a system API.

In this case you may use `OsString` which allows interchange between Rust and a system dependent representations of strings. On Windows it will return UTF-16 strings, on Linux / Unix systems it will return UTF-8.

An `OsStr` is a slice onto `OsString`, analogous to `str` and `String`.

