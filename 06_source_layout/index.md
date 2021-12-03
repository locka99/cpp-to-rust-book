# Source Layout and Other General Points

## Header files

### C/ C++

C and C++ code tends to be split over two general kinds of file:

* The Header file contains class definitions, external function signatures, macros, templates, inline functions. Sometimes inline functions get stored in their own file. The standard template library C++ headers may omit file extension. Some 3rd party libraries like Qt may also omit the file extension.
* The Source file contains the implementation of the source code. Source files will usually `#include` one or more header files to obtain the definitions of things the implementation needs to call or implement.

Prior to compilation, a _preprocessor_ will read and follow the `#include` directives in a source file and produce a concatenated file for the compiler to parse. The preprocessor will also handle `#ifdef` and `#define` commands in this step. 

Splitting source into definitions and implementation can increase code maintenance. So too that the compiler does not support forward referencing - you must describe a type / function before the thing that uses it. 

### Rust

Rust does use header files. Instead every struct, implementation, function, const, and macro resides in a file ending in .rs. Code is made public or not by structuring .rs files into modules and exposing functions via the `pub` keyword.

For functions, the definition and the implementation are the same thing - the function implementation's signature is its definition.

For structs, the structure is declared and there are zero or more implementation blocks for functions and traits that are associated with the struct. Usually the implementation and definition will reside right next to each other.

Other modules can `use` the other module's public types and functions and the compiler will obtain the definition.

Rust can forward reference structs or functions, or even `use` the very same module that a piece of code is a part of. 

The only time that ordering matters is for macro definitions. A macro must be defined before a module that uses it. 

Rust files reference non-dependent modules with the `use` keyword and pull-in dependent modules with the `mod` keyword.

## Namespaces

## C / C++

C does not use namespaces. Libraries tend to prefix their functions and structs with a qualifying name of some sort. 

e.g. the SQLite3 library prefixes every function,  struct and macro definition

```c++
SQLITE_API SQLITE_EXTERN const char sqlite3_version[];
SQLITE_API const char *SQLITE_STDCALL sqlite3_libversion(void);
SQLITE_API const char *SQLITE_STDCALL sqlite3_sourceid(void);
SQLITE_API int SQLITE_STDCALL sqlite3_libversion_number(void);
```

C++ _does_ have namespaces but their use is optional and varies from one piece of code to the next. Some code may hold all their definitions in a single flat namespace while others may nest namespaces. 

A `math.h`

```c++
namespace math {
  double sqrt(double v);
}
```

```c++
#include "math.h"
//...
auto v = math::sqrt(4);
}
```

### Rust

Rust has modules which are like `#include` and namespaces rolled into one

One major convenience definition and implementation are one and the same. Implementing a function brings it into existence. Any other module that chooses to "use" it simply says so and the compiler will ensure it compiles properly.

See [Namespacing with modules](../namespacing_with_modules/index.md)

## File name conventions

In C++ filenames typically end in:

* `.h`, `.hpp`, `.inl` for headers or inline code. Some libraries may omit an extension from their header files.
* `.c`, `.cpp`, `.cc` for source code. A compiler will usually infer the source is C or C++ by its extension.

Aside from the extension \(which may kick off the compiler expecting C or C++\) there is next to no expected arrangement or naming convention for files. They can be in any folder structure and any mix of upper and lower case.

Rust files have a `.rs` extension and are are snake\_case.  The filename DOES matter because the name is also the module name that scopes whatever is in it. 

So if I have a file called `math.rs`, then the module is called `math`. Alternatively I can create a file `math/mod.rs` and the module is also inferred to be `math`. So if there were a `sqrt(v: f64)` function, then it would be called everything in it is scoped `math::sqrt`.

In `math.rs`

```rust
pub fn sqrt(v: f64) : f64 {
  //....
}
```

Caller:

```rust
use math;
//...
let v = math::sqrt(4);
}

## Unicode support

Using Unicode in C++ has always been a pain. 

Here are just some of the problems

1. Source code is normally only safe to use characters 0-127 although some compilers may have parameters that allow makefiles to specify other character encodings. 
2. Other characters outside of 0-127 are normally escaped
3. C++98 has `char` and `wchar_t` types for 8-bit and 32-bit characters and corresponding `std::string` and `std::wstring` template types. Providing we assume UTF-8 and UTF-32 are the encodings our problem is solved?
4. No because `wchar_t` was immediately subverted be compilers such as MSVC where it is treated as only 16-bits wide.
5. 16-bits is only sufficient to hold Unicode's basic multilingual plane. Characters outside of that plane must use control points.
6. So  "wide" `wchar_t` can be UTF-32 on some compilers and must assumed to be UTF-16 on others such as Windows.
7. This messed up definition makes operations such as slicing strings dangerous due to the risk of cutting through a control point.
8. C++11 tried to rectify this with new and explicit `char16_t` and `char32_t` types and corresponding `std::u16string` and `std::u32string` template types.
9. So now we have four(!) character types and their corresponding string types to hold different character widths.
10. But that doesn't even cover anything to do with UTF. The `u` in `u16string` suggests Unicode but nothing in the string types can convert between UTF-8, UTF-16, UTF-32 or even to walk the string by displayable characters.
11. Linux tends to favour UTF-8 encoding of strings while Windows favours UTF-16 encoding. This means portable code has to be able to losslessly convert between types.
12. 3rd party conversion libraries like ICU4C are commonly used. Libraries like boost, Qt use libicu for converting between encodings

So it's messy.

Rust simplifies things a lot by benefit of hindsight.

* Source code is UTF-8 encoded.
* Comments, characters and string literals can contain Unicode characters without escaping.
* The native `char `type is 4 bytes wide – as wide as a Unicode characters.
* The native `str &` and `String` types internally use UTF-8 to save space but may be iterated by `char` or by `u8` byte according to what the function is doing.

Since source code is UTF-8 encoded you may embed strings straight into the source.

```rust
let hello = "你好";
for c in hello.chars() { /* iterate chars */
  //...
}
```



