# Source Layout and Other General Points

## Header files

### C/ C++

C and C++ code tends to be split over two general kinds of file:

* The Header file \(.h, .hpp\) contains class definitions, external function signatures, macros, templates, inline functions. Sometimes inline functions get stored in their own file. The standard template library C++ headers do not have a file extension. Some 3rd party libraries like QT may sometimes omit the extension.
* The Source file \(.c, .cc, .cpp\) contains the implementation of classes and anything private.  Sometimes C++ will use tricks such as forward class references and Pimpl patterns to keep complex or dependent code out of the header file.  

Occasionally you may also see files with a .inl, or .ipp extension which are headers with a lot of inline templates or functions. 

Compilers are only interested in source files and what they `#include` so what's really happening in most C/C++ code is that a preprocessor concatenates various header files to the front of the source file according to the `#` directives within it and the resulting file is fed to a compiler.

Splitting definition and implementation across multiple files can be a nuisance since it means that changes to a single class can require modifications to multiple files.

### Rust

Rust does not have header files. Every struct, implementation and macro resides in a file ending in .rs. Code is made public or not by structuring .rs files into modules and exposing functions via the `pub` keyword.

Ordering is less important too. It is possible to forward reference structs or functions, or even `use` the very same module that a piece of code is a part of. The only time that ordering matters is for macro definitions. A macro must be defined before a module that uses it. 

Rust files reference non-dependent modules with the `use` keyword and pull-in dependent modules with the `mod` keyword.

## Namespaces

## C / C++

C does not use namespaces. Libraries tend to prefix their functions and structs with a qualifying name of some sort. 

C++ _does_ have namespaces but their use is optional and varies from one piece of code to the next.

### Rust

Rust has modules which are like `#include` and namespaces rolled into one

One major convenience definition and implementation are one and the same. Implementing a function brings it into existence. Any other module that chooses to "use" it simply says so and the compiler will ensure it compiles properly.

See Namespacing with modules TODO ref

## File name conventions

In C++ filenames typically end in:

* .h, .hpp, .inl for headers or inline code
* .c, .cpp, .cc for source code

Aside from the extension \(which may kick off the compiler expecting C or C++\) there is next to no expected arrangement or naming convention for files.

You can compile a file called deeply/nested/Timbuktu.cpp which defines 20 classes and 30 interfaces if you like and the name does not matter.

Rust files are snake\_case and end in .rs.  The filename DOES matter because the name is the module name that scopes whatever is in it. There are also some special files called main.rs, lib.rs and mod.rs.

So if you name your file foo.rs, then everything inside is scoped foo::\* when externally referenced.

## Unicode support

Using Unicode in C++ has always been a pain. Neither C nor C++ had support for it at all, and various solutions have appeared over time. Recent implementations of the standards of C and C++ provide string literal types for UTF encodings, but prior to that it was strictly ascii or wide characters.

Here are some general guidelines for Unicode in C / C++:

* Source code is normally only safe to use characters 0-127 although some compilers may have parameters that allow makefiles to specify other encodings.
* C++ has char and wchar\_t types for 8-bit and 32-bit or possibly 16-bit wide strings. Part of the problem with wchar\_t was the width was immediately subverted.
* Char type implies no encoding. It normally means ASCII but could also mean UTF-8, Latin1, or in fact any form of encoding that predates Unicode. Basically it is up to the program to understand the meaning.
* A "wide" wchar\_t is NOT UTF-32. It might be, or it might be UTF-16 on some platforms \(e.g Windows\). This messed up definition makes operations such as slicing strings dangerous due to the risk of cutting through a control point.
* What if I want to read Unicode arguments from the command-line such as file paths - what encoding are they in? The main\(\) method passes them as char_. Windows has a wmain\(\) that takes wchar\_t_. What am I supposed to do?
* Windows favours wide \(UTF-16\) character strings for its APIs although it has ASCII versions too. The ASCII versions are not UTF-8. Compiled code has \#define UNICODE to support multiple languages.
* Linux tends to favour UTF-8 encoded char strings. Most languages, toolkits and tools assume UTF-8. The exception is QT which has chosen to use UTF-16 internally.
* C-lib has acquired various wide versions of its strdup, strcmp, strcpy etc. It also acquired wide versions of functions for opening files on disk and so forth.
* C++ lib has acquired std::string / std::wstring classes. C++ has acquired explicit UTF-16 and UTF-32 versions of these classes.
* C11 and C++11 introduce explicit string literals for various UTF widths.
* Limited conversion capabilities between wide / narrow in C++. Some operating systems have incomplete conversion capabilities.
* 3rd party conversion libraries like ICU4C are commonly used. Libraries like boost, QT use libicu for converting between encodings
* Embedding Unicode into C source involves using escape codes or hex values

Rust simplifies things a lot by benefit of hindsight.

* Source code is UTF-8 encoded.
* Comments, characters and string literals can contain Unicode characters without escaping.
* The native char type is 4 bytes wide – as wide as a Unicode characters.
* The native str & String types are internally UTF-8 to save space but may be iterated by char or by byte according to what the function is doing.

Since source code is UTF-8 encoded you may embed strings straight into the source.

```rust
let hello = "你好";
for c in hello.chars() { /* iterate chars */
  //...
}
```



