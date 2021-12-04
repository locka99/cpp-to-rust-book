# Strings

## Summary

In C/C++

* A string is typically a pointer to an array of `char`, `wchar_t`, `char16_t` or `char32_t` values. Historically most strings are `char` but efforts have been made to support wide character strings as well as Unicode encodings such as UTF-8.
* A string's length is calculated by looking for a special nul (`'\0'`) value that signifies the end of the string. So a 2000 character string requires iterating through the whole string, potentially 2000 times looking for a nul. The `std::basic_string<>` type holds a length to negate the need to calculate the length and also to handle byte arrays that may contain nuls.
* In C++ types derived from `std::basic_string<>` template are the recommended way to manage strings safely. But other 3rd party libraries also have their own string wrappers, e.g. `QString` in QT.
* Only `char16_t` and `char32_t` types are considered to be Unicode (encoded as UTF-16, UTF-32 respectively). There is no encoding knowledge about the meaning of other kinds of string.
* In C++17 you may create a `std::basic_string_view<>` which is a view onto a read-only string.

In Rust

* A `char` is a primitive in Rust that is 32-bits and can represent any Unicode character. So it equivalent to `char32_t` in C++.
* A `str` is a primitive that represents a read-only string. You don't declare these, rather any string in your code is implicitly a `str` and your variables bind to it via a string slice.
* A `&str` is a string slice - a reference to a `str`, consisting of a pointer and a length. 
* A `String` is a heap allocated writable buffer for strings. i.e. it can be truncated, extended etc. It implements all the functions of `str` and can also be used as a `&str`.
* All strings are internally encoded as UTF-8 but there are functions to iterate by character.

## What is a character exactly?

Historically, a string has been a sequence of bytes, each representing a character. Each character is mapped to a byte (or multiple bytes) to be displayed on a screen. This is known as an encoding, i.e. the computer knows that a byte with value 65 corresponds as the letter 'A' and has a table lookup for every printable character.

The two predominent encodings early on were EBDIC and ASCII. Each of these assigned characters to byte values. We won't mention EBDIC since ASCII essentially won the encoding war but basically ASCII uses the first 7-bits \(0-127\) of the byte to assign values to upper and lower case letters in the English alphabet, numbers, punctuation marks and certain control characters. The eighth wasn't used because at the time it was used as a control bit for serial communications.

![ASCII Table](USASCII_code_chart.png "ASCII Table (wikimedia)")

But what about the rest of the world? It is no use to someone writing in French if there are no letters with acute, grave or circumflex chars to use ASCII since there is no character for 'À'. It would be no use at all for Japanese unless the entire language were written phonetically. For that reason the values in the 128-255 range started to be co-opted to represent other characters. 

Languages like Chinese, Japanese, Korean, Thai, Arabic etc. have thousands of symbols so there is no way to encode them in this range. So they had to use multi-byte encodings, one character being more than one byte in length.

The problem here is that unless you know what language the string of bytes represents, you cannot render the character. The same sequence of bytes may mean different things depending on the encoding. So operating systems like DOS & Windows gave software a clue how to handle strings - the codepage. The codepage was an environmental setting that said that on this computer, the bytes should be interpreted as.. Rusian, Latin, Japanese etc. So for example Microsoft's code page 932 use an encoding called Shift JIS \(Japanese\) where some symbols are two bytes.

The code page works for one computer, but not documents sent between multiple computers with differing code pages. The bytes in one computer mean one thing and another in the other computer. 

Obviously this was rapidly becoming a mess. Each code page interpretted the same byte array differently according to some external setting. So you could not send a file written in Chinese to someone with a different code page and expect it to render properly.

### Unicode

The Unicode standard was created to solve this problem. It assigns every printable character or glyph in existence with a unique 32-bit value, called a code point. Code points are arranged into planes - blocks of 65536 code points.

Most characters fall in plane 0 called the Basic Multilingual Plane \(BMP\). These can be encoded with 2 bytes, but China has compelled software makers to support all code points since some Chinese symbols are not contained in the BMP.

#### Unicode encoding

So Unicode represents every character in 32-bits but it would be less than inefficient to store every character in the stream as 32-bits.

Thus a number of encoding formats exist that attempt to losslessly represent 32-bit values in an efficient manner:

* UTF-8 is backwards compatible with ASCII. ASCII characters require one byte to encode, other characters may require up to 4 bytes depending on their code point.
* UTF-16 encodes the characters in Basic Multilingual Plane in two bytes but uses four bytes for code points outside this range. 
* UTF-32 encodes every character in 4 bytes.

Each encoding can be losslessly transformed to the others with helper code such as the popular `libicu` library.

Generally speaking UTF-8 is the most popular encoding because it is also the most efficient way to store characters most of the time. Even web pages in Japanese contain a lot of ASCII characters for the markup. So the vast majority of content is stored and served up in UTF-8. UTF-16 is not a popular encoding however the Windows operating system and also Java use it for their string encodings due to decisions made to support the BMP before China mandated full code point support.

## C / C++

### There is no string primitive

C and C++ does not have a string primitive type, instead it has a `char` type, that is one byte. A "string" is a pointer to an array of chars that are terminated with a zero byte, `'\0'`.

```c++
// The array that my_string points at ends with a hidden \0
const char *my_string = "This is as close to a string primitive as you can get";
printf("String is %d chars long.\n", strlen(my_string));
```

#### String functions

In C, functions such as `strlen(s)`, `strcpy(dst, src)`, `strdup(s)` etc. allow strings to be inspected, copied or duplicated. But they work all work by essentially walking the array of bytes until they reach the `\0`. So `strcpy(dst, src)` copies from `src` to `dst` a byte at a time up to and including the `\0`. 

But what if `dst` was not a big enough array to hold the input? Well now we have a buffer overrun which may crash the software, or cause it to be compromisd.

C11 introduces "safe" versions of functions like `strcpy` that say how large the destination is, i.e. `strcpy_s(dst, dstlen, src)`. It will not overstep the output buffer's size and if the output is not big enough to hold the input or is truncated, the function will return an error. Even so, we are required to set this size correctly, include space for the terminator byte and test for errors.

#### std::basic_string template

C++ provides `std::basic_string<char_type>`. Normally `char_type` would be a `char` and there is a typedef to `std::string` which is precisely that. The `std::string` manages the lifetime of a string and provide methods for modifying the string in a safe manner. It is a vast improvement over C.

```c++
#include <string>
//...
std::string my_string = "Mary had a little lamb";
std::cout << "String is " << my_string.size() << " chars long." << std::endl;
```

Note that this is not a primitive type. Instead `std::string` is a fairly opaque template defined in `<string>` that is included, compiled and linked to the executable just like every other template class. 

In addition, a `std::string` will normally uses the heap to store the string's data which can have repercussions for memory usage and fragmentation. There is also a hidden cost to assigning one string to another strings are duplicated in the process. Operations like `substr()` create copies of the section of the string.

```c++
#include <string>
//...
std::string str = "The Evolution of Man";
std::string str2 = str.substr(4, 9);
// Str2 contains a copy of "Evolution"
```

In this example we use `substr()` to get a portion of the containing string, but to do so, a new string must be created. In a trivial example it doesn't matter but we might have large loops where a lot of string manipulation is occuring and it becomes inefficient. For example, if the code was consuming records delimited with a `|` where there might be 100 fields in a record, then we have a lot of allocation going on.

Recognizing this C++17 did this...

#### std::basic_string_view template

C++17 supports a `std::basic_string_view<char_type>` template defined in `<string_view>`. Normally `char_type` would be a `char` and there is a typedef `std::string_view`. A string view is a read-only slice of a string.

```c++
#include <string_view>
//...
std::string str = "The Evolution of Man";
std::string_view sv { str }
std::string_view sv2 { sv.substr(4, 9) };
```

In this example, the `sv` points to `str` but it does not copy it. Basically it is just a pointer and a length onto the string. And `sv2` is a pointer and a length onto the substring portion. There is no heap or string copy happening here so it is much more efficient.

This allows string operations to be more efficient, however it requires the underlying string to be read only and for the view to be lifetime dependent on the string. i.e. it offers no guarantees that the view points to anything by the time it is accessed and it is the developers job to make sure it doesn't. For example, if I changed `str` in the previous example, then `sv` and `sv2` could be pointing at junk.

### String encoding is whatever you want it to be

C++ more or less expects you to know the encoding of your own strings. It has no inherent knowledge that an array of bytes is ASCII, EBDIC, UTF-8 or anything else. What you put in your strings is your own business and it doesn't care if the bytes are valid Unicode encodings or not. 

However it does support Unicode if that is your intent. It has a long string notation that indicates a wide character string:

```c++
wchar_t *msg = L"Hello World!";
```

The `L` prefix indicates the string is in a wide format. Unfortunately `wchar_t` type can be either 2 or 4 bytes wide and is a compiler / platform specific decision. In Microsoft Visual C++ the wide char is 2 bytes and in gcc it can be 2 or 4 bytes according to the compiler flags.

Oops...

So C++11 rectifies this by introducing unambiguous `char16_t` and `char32_t` types and corresponding versions of string called `std::u16string` and `std::u32string`. It also has string prefixes for declaring encodings of strings when you combine them with `auto`:

```c++
// Wide string with L prefix
wchar_t hello_chinese = L"\u4f60\u597d";
// C11 and C++11 add UTF string literal prefixes
auto hello_8  = u8"\u4f60\u597d"; // UTF-8 encoded
auto hello_16 =  u"\u4f60\u597d"; // UTF-16
auto hello_32 =  U"\u4f60\u597d"; // UTF-32
```

Even with this you can see above that encoding Unicode in your source code isn't very nice. The Unicode characters are escaped. Some C++ compilers may allow the character set of the source file to be encoded (e.g. with UTF-8) to overcome this but it is not a certainty.

### Character types

So C++ has 4 character types. Great huh?

| Character type | Encoding |
| --- | --- |
| `char` | C, ASCII, EBDIC, UTF-8, ad hoc, ??? |
| `wchar_t` | UTF-16 or maybe UTF-32 |
| `char16_t` | UTF-16 |
| `char32_t` | UTF-32 |

You basically cannot assume what a char string is encoded with. To preserve your sanity you are best to choose an encoding, e.g. UTF-8 and enforce it, sanitizing your inputs where you need to. If you are calling external libraries you also need to see what rules they follow for string handling. If necessary you may need to use `libicu` to properly handle strings including iterating through them since you cannot slice strings without knowing where characters lie in the buffer.

## Rust

Rust has implicit string in the language itself.

* A `char` type is a 32-bit Unicode character.
* A `str` type is a UTF-8 encoded string held in memory. Code will never directly use this type. Instead it will use `&str` which is a string slice, a pointer and length to the str or a portion of it. 
* A `std::String` is a heap allocated `str` that you can manipulate. Code may also obtain a `&str` to a `String` but the compiler will enforce that the string is read-only while the slice exists upon it.

Finally there are foreign function interface (FFI) types `OSString` and `OSStr` akin to `String` and `str` that handles any differences in how the outside world sees strings compared to Rust, e.g. changing the character width or encoding and putting a null terminator on the end. Normally you would only use these types when you need to call an external API.

## Types Comparison

| C/C++ | Rust |
| --- | --- |
| `char *` or `wchar_t *` | `str`, `&str` |
| | `OSStr`, `&OSStr` (FFI) |
| C11 | |
| `char16_t *`, `char32_t *` | `str`, `&str` |
| C++11 | |
| `std::string`, `std::wstring`, `std::u16string`, `std::u32string` | `std::String` |
| | `OSString` (FFI) |

### Slices 

A _slice_ is a reference to some or all of a `str`. It is written `&str` and also contains a pointer and a byte length value. 

We saw that C++17 introduces a `std::string_view` and it's like that, but it is an intrinsic part of the language and the compiler makes sure you cannot use the slice unsafely.

```rust
// These variables are &str pointing to a str consisting of UTF-8 encoded bytes
let my_str = "Hello"; 
let hello_chinese = "你好";
```

Type inferences for these assignments will create a `&str` slice pointing to the statically allocated `str` and its bytes. The data itself doesn't move and the `&str` is implicitly read-only.

There is no need for the different prefixes in C++ for different character widths as it is implicitly Unicode. The developer may type any Unicode they like into their UTF-8 encoded source code or between string delimiters and the compiler will just take it. 

The `str` has functions for iterating over the string in bytes / characters, for creating slices, to find a pattern etc.

```rust
let my_str = "Hello"; // v is a &’static str
println!("My string is {} and it is {} bytes long", v, v.len());
```

Note `len()` is the length in bytes because strings are UTF-8 encoded. A single character may be encoded as 1, 2, 3, or 4 bytes. It may not be the number of characters a human would actually see. Characters may even be clustered together to form a graphene.

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

### Raw strings

A raw string is a convenience for strings that contain backslashes, quotes etc. that you don't want to have to escape out.

It is a Unicode string starting with `r"` prefix. If your string contains a double quotation mark you can also put one or more hashes around the outer quotes, `r#"A quote is this symbol ""#;`. You could even put more hashes if your string contained `#"`, e.g. `r##"`, `"##`.

```rust
let quote = r#"And the man said "the best is yet to come""#;
let multiline = r"All good things
to those who wait";
```

### Byte strings

A byte string is just an unchecked byte array that is not assumed to be any kind of encoding:

```rust
fn main() {
let b = b"SUCCESS\x0f";
}
```

In this case, `b` is a reference to a static byte array `&'static [u8]`. There is no `str`. This may be useful for reading data from a stream where there is no assumption about its encoding.

# Unicode

As state already strings are Unicode. It is worth remember that internally the string is UTF-8 encoded. What that means is you have to be a little careful conflating length with number of characters. 

```rust
fn main() {
    let s = "你好";
    println!("Length of string = {}", s.len());
    s.chars().for_each(|c| {
      println!("Char = {}", c);
    })
}
```

And the output would be:

```
Length of string = 6
Char = 你
Char = 好
```

Notice that the _length_ of the string is 6 bytes, but there are only two characters. It takes 6 bytes to UTF-8 encode the string and that is what is referring to. Functions that manipulate strings will be indexed by byte too and will error if you are incorrect. Therefore, you are better to use `chars()` for character wise operation.

So a `char` in Rust is a Unicode character, 32-bits. This may seem weird at first but remember that Rust has explicit `i8` and `u8` types if you really want to deal in bytes. But strings in Rust are not bytes, they're characters.

It gets more complex than "character" though because some languages have clusters of characters that are known as graphene clusters, which are akin to letters. So a character may be an adornment for another character, not necessarily printable. Therefore even with Unicode baked in you may find your code has to have special knowledge of what it is doing to make sense for what the user sess.

In addition, to manipulate strings, then there is a type for that purpose, `String`:

```rust
fn main() {
    let mut s = String::from("你好");
    s.push_str("世界");
    println!("{}", s);
}
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

