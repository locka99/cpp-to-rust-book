# Strings

Strings in C++ are a bit messy. They have to deal with how they were historically defined and how strings in the modern world must behave to deal with multiple languages that don't fit neatly into a byte.

To explain the issue requires some backstory...

## What is a character exactly?

Historically in C and C++, a char type is 8-bits. Strictly speaking a char is signed type (-127 to 128), but the values essentially represent the values 0-255. Standards like US-ASCII used the first 7-bits (0-127) to assign values to upper and lower case letters in the English alphabet, numbers, punctuation marks and certain control characters. Other standards like EBDIC also assign values to upper and lower case letters, number, punctuation marks, but different values! By default, the encoding the compiler choose is what it sees fit - most compilers would use ASCII but some mainframes may still use EBDIC.

The problem is that the world uses MANY symbols, and they can't all fit in 256 values.

So in time, some operating systems came up with a hack - the OS set itself to a "code page" which defined how values mapped onto symbols. Normally the upper 128-255 values were repurposed according to the code page in effect. Thus most languages only had a handful of additional symbols that could be specified in this range. But CJK (Chinese-Japanese-Korean) languages as well as Thai, Arabic etc. have thousands of symbols so some languages require 2-byte symbols. For example Microsoft's code page 932 implemented Shift JIS (Japanese) where some symbols are two bytes.

Obviously this was rapidly becoming a mess. Software that rendered properly in one language didn't on another. In some cases, the fact that characters could be multiple bytes broke code which attempted to count bytes or split strings.

## Unicode to the rescue

The Unicode standard defines every printable symbol with a unique 32-bit value, a code point. Most symbols fall in the first 16-bits, the Basic Multilingual Plane (BMP) some Chinese characters still did not fit and China has mandated all software must to support all 32-bits.

C++ has a wide character called wchar_t that should correspond to a code point but on Windows it's only 16 bits wide, i.e. UTF-16. On gcc, it's a compiler switch. Normally this is fine, but some characters don't reside in 0-65535 and must be escaped. So C++11 adds explicit char16_t and char32_t types and corresponding versions of string called std::u16string and std::u32string.

So now C++ has 4 character types. Great huh?

Character type | Encoding
-------------- | --------
char | ad hoc, ASCII, EBDIC, UTF-8, ???
wchar_t | UTF-16 or UTF-32
char16_t | UTF-16
char32_t | UTF-32

### C++

C and C++ never had a string primitive type, instead it uses a pointer to an array of chars which are zero-byte terminated.

The char type is a byte wide. The std::string template wraps a char pointer and provides methods for modifying the string in a safe manner.

The wchar_t type is for wide strings and is either 2 or 4 bytes wide and is compiler / platform specific. In Microsoft Visual C++ it is an unsigned short (corresponding to Win32's Unicode API), in gcc it can be 32-bits or 16-bits according to the compile flags. There is a corresponding  std::wstring.

### Rust
Rust has been rather fortunate in that Unicode existed before it did and therefore it doesn't have any legacy baggage. It can choose to be UTF-8 encoding internally and expose 32-bit chars.
Rust only has str and std::String but they handle all cases.

## Types Comparison

C++ | Rust
--- | ----
char * or wchar_t *
C++11 - char16_t \*, char32_t \* | &str
std::string, std::wstring
std::u16string std::u32string | std::String

## char * vs str

The closest C or C++ has to a string primitive is a char pointer, i.e. a string points to an arbitrary sequence of non zero bytes.  A zero byte terminates the string. The same applies for wider chars, except of course they require 2 or 4 bytes.

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

Rust would use a str for all of these and does not need to specify a width or escape its Unicode characters.

```rust
let my_str = "Hello";
let hello_chinese = "你好";
```

We can instantly see it removes the mess of character width and literal prefixes that C and C++ have to suffer under.

Type inference creates a reference to a string slice, a &str. A string slice is a pointer to the data and a length (in bytes) of the portion to view. The string itself doesn't move and &str is read-only.

The str has functions for iterating over the string in bytes / characters, splitting, find a pattern etc.

```rust
let my_str = "Hello"; // v is a &’static str
println!("My string is {} and it is {} bytes long", v, v.len());
```

Note len() is the length in bytes because strings are UTF-8 encoded. A single character may be encoded as 1, 2, 3, or 4 bytes. It may not be the number of characters a human would actually see.

```rust
let my_str = "你好";
println!("Number of bytes = {}", my_str.len());
println!("Number of chars = {}", my_str.chars().count());
```

```
Number of bytes = 6
Number of chars = 2
```

You can do a split on a &str to produce a left and a right slice like this:

```rust
let (part1, part2) = "Hello".split_at(3);
println!("Part 1 = {}", part1);
println!("Part 2 = {}", part2);
```

```
Part 1 = Hel
Part 2 = lo
```

## std::basic_string (C++) vs std::String (Rust)

The standard C++ library also has template class std::basic_string that acts as a wrapper around the various character types and can be used for manipulating a string of any width. This template is specialised as
std::string, std:wstring, std::u16string, std::u32string.

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

In Rust, the std::String type serves the same purpose:

```rust
let v = String::from("Hello");
v.push_str(" world");
```

Using it is fairly simple

```rust
let mut v = String::from("This is a String");
v.push_str(" that we can modify");
```

To add two Strings together

```rust
let b = String::from(" Bananas");
let mut result = String::new();
result.push_str("Apples ");
result.push('&'); // Push a char
result.push_str(b.as_str());
println!("result = {}", result);
```

Strings are always valid UTF-8.

Internally a String has a pointer to the data, its length and a capacity (max size). If you intend to expand a string, then you should ensure the String has sufficient capacity to accommodate its longest value otherwise you may cause it to reallocate itself excessively.

Strings will never shrink their capacity unless you explicitly shrink_to_fit(). This means if you use a temporary string in a loop, it's probably best to place it outside the loop and reserve space to make it efficient.

```rust
let mut v = String::with_capacity(100);
// or
let mut v = String::new();
v.reserve_exact(100);
```

Strings also have all the methods of str thans to implementing Deref trait.

### Formatting strings

In C or C++ it's common to see code invoke sprintf or one of its related functions. These days,

| C++ | Rust formatting trait | Purpose
| --- | ---------
| %s, %u, %d, %i, %f, %c | {} | C/C++ require the type of the parameter to be specified. In Rust the type is inferred and {} will invoked the type's Display trait regardless of what it is. So a String outputs its text, numeric types return their value, boolean as returns true or false, and so on.
| %lld, %llu | {} | C/C++ has extensions to deal with different size ints and floats, e.g. ll for long long due to the way arguments are passed to the function. In Rust, there is no need for that.
| | {:?} | In Rust {:?} returns whatever is implemented by a type's Debug trait. Supplying {#?} instead would pretty-print the output.
| %-10s | {:<10} | Format left aligned string padded to minimum of 10 spaces
| %04 | {:04} | Pad a number with zero's to a width of 4
| %.3 | {:.3} | Pad a number's precision to 3 decimal places. May also be zero-padded, e.g. {:.03}
| %e, %E | {:e}, {:E} | Exponent in lower or uppercase
| %x, %X | {:x}, {:X} | Hexadecimal in lower or uppercase. Note {:#x}, {:#X} precedes output with 0x
| %o | {:o} | Octal. Note {:#o} precedes output with 0o
|  | {:b} | Binary. Note {:#b} precedes output with 0b
| %p | {:p} | Presents a struct's memory location, i.e. pointer

Rust has many [more formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits) than this.

For example it allows named parameters like this example:

```rust
let message = format!("The temperature {temp}C is within {percent} of maximum", temp = 104, percent = 99);
```

Named parameters would be particularly useful for localization where the order of values may be different in one language compared to another.

## Display and Debug traits

Rust allows types to be formatted as strings based upon the formatting traits they implement.

The two main implementation traits are:

- Display - this is for standard textual representation of a type.
- Debug - this is for the debugging textual representation of a type. It might present additional information or be formatted separately to the Display trait. It is possible to #[derive(Debug)] this trait which is usually enough for the purpose of debugging.

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

All of the intrinsic types implement Display and Debug. We can explicitly implement Display on our own structs too:

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

Implementing Debug is usually done by #[derive(Debug)] but it could also be implemented. The derived Debug will print out the struct name, and then the members in curly braces:

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

Many types process formatting traits which are values held between the {} braces in the string. These are fairly similar to the patterns used in C functions for printf, sprintf etc.


## OsString / OsStr

Rust recognises there are times when you need to pass or receive a string from a system API.

In this case you may use OsString which cheaply allows interchange between Rust and a system dependent representations of strings. On Windows it will return UTF-16 strings, on Linux / Unix systems it will return UTF-8.

An OsStr is a slice onto OsString.
