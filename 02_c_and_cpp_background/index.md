
|
# C and C++ Background

This section talks about C and C++. It describes its history, standards and provides a background as to how it ended up where it is today.

## History of C

### Early Days

The C [^1] programming language was developed as part of Unix [^2] by Dennis Ritchie and Ken Thompson.

Unix started life on the PDP-7 microcomputer and was originally written in assembly language. The OS was then ported to the PDP-11 architecture, also in assembly. 

Ritchie developed C in 1972 as a higher level language and compiler for writing Unix software in a more abstract way than pure assembly. The language provided constructs such as static types, loops, conditionals, expressions etc and the compiler produced machine code which was almost as efficient as hand written assembly. Over time much of Unix itself, including much of the kernel was rewritten in C which in turn aided portability.

The compiler was also a bundled component of Unix, so Unix users could develop their own software using the language. Thus C became a popular way of developing Unix software.

[^1] C was called C because it was influenced by a language called B developed by Thompson as a simplified version of BCPL.

[^2] Unix is a multi-tasking operating system that evolved into many commerical and free variants such as Solaris, HPUX, BSD etc. Many of these are in use today as well as Linux. You may be thinking Linux is Unix, but Linux *isn't* derived from Unix code. However, it did independently implement most of the same interfaces and concepts as a typical Unix OS. These days Linux is the dominant Unix-like OS and can be found in anything from lightbulbs all the way up to mainframe computers.

### Defacto standard and emerging popularity

In 1978 C was formalised into a defacto standard called K&R C, named after Brian Kernighan & Dennis Ritche who published the standard as a book. K&R formalised the language and defined the standard I/O library and some other features.

Over time the use of C became more widespread and compilers such as Turbo C, Lattice C, Microsoft C popularized C on other operating systems including personal computers.

### International Standards

C was later standardised by the American National Standards Institute (ANSI) and thus became known as ANSI C, or C89. Further standards followed such as C99, C11, C18 and so on.

Later standards have attempted to align C with developments in C++ to keep the two in sync as much as possible. Some functionality that was introduced in C++ has also found its way back into C standards. For example, the // style single-line comment and variable declaration rules in blocks.

## History of C++

C++ first appeared in 1983 as C with classes. It was invented by Bjarne Stroustrop as a way to imbue C with Simula-like [^3] object-oriented features. 

C++ added these concepts as extensions to the C language and used a precompiler called `cfront` to transform the C++ extensions into C code that could then be compiled into machine code. So a C++ program could have the high level object oriented concepts but without the overhead that came with Simula.

C++ became popular in its own right and outgrew the limitations of cfront preprocessor to become supported by compilers in its own right. Thus toolchains such as Microsoft Visual C++, GCC, Clang etc. support both languages. Some toolchains have also been given to favouring C++ over C, for example Microsoft's compiler has been very slow to implement C99.

Object oriented programming has mostly been used in higher level software - applications, games,  simulations and mathematical work.

C++ has also become formalised standards with C++98, C++03, C++11 and so on.

[^3] Simula is a language that allowed concepts such as objects, classes and inheritance to be expressed in code and as its name suggests was created for running simulations. However it was considered too slow for systems programming and so something that combined speed of C with object oriented concepts was highly desirable.

### Modern C++

C++11 onwards is a distinctly different beast from earlier iterations and strives to add functionality that if used correctly can eliminate a lot of issues that will be discussed later on:

* Unique and shared pointers
* `auto` keyword
* move semantics \(i.e. moving data ownership of data from one variable to another\)
* rvalue references
* perfect forwarding
* `nullptr_t` and `nullptr` explicit type

However it is worth noting that since many of these things are late additions to C++. Things like move semantics must be explicitly used and have complexity as we shall see.

### The relationship between C and C++

While C++ grew out of C and has developed alongside it, C++ is *not* a superset of C. It is _mostly_ a superset but C can use C++ keywords as variable / function names that would cause a compiler error with C++.

C++ also has function overloading and classes and uses name mangling to disambiguate overloaded functions. But in practice it is possible to write C as a subset of C++ and compile the two into the same executable. Most real-world C code could be called C++ _without_ classes.

C and C++ are even usually handled by the same toolchain. Most compilers would consist of a front half that parses the language into an intermediate form and a back half which turns the intermediate form into optimized machine code. Finally the linker would join all the binary objects together to form an executable. C and C++ would share most of this code path.

C++ tends to be more popular with applications level programming. Part of the reason C++ hasn't found itself in the lower layers is the perception that exception handling, name mangling, linking and issues of that nature add unwanted complexity or that somehow the generated code is less efficient. Arguments have been made that this is not the case, but the perception still remains.

C still tends to be more popular in low level systems programming. Components such as the Linux kernel are pure C with some assembly. Many popular open source libraries such as sqlite3 are also written in C.

## Objective-C

Objective-C is another C derived language that added objects and classes. Unlike C++, Objective-C behaves as a strict superset of C.

The language was developed in the 1980s and was popularized in the NeXTSTEP operating system and later in Apple's OS X and iOS. It hasn't gained much popularity outside of those platforms but the success of the iPhone has ensured it has a sizeable developer base of its own. It is also well supported by the GCC and Clang toolchains. Apple has begun to deprecate Objective-C in favour of Swift which is a modern high level language similar in some respects to Rust but more application focussed.

Objective-C is strongly influenced by Smalltalk \(as opposed to Simula in C++\) and so code works somewhat differently than C++.

Notionally code calls objects by sending them a message. An object defines an interface specifying what messages it accepts and an implementation that binds those messages to code. The caller code sends a message to call a method. Objects can also receive dynamic messages, i.e. ones not defined by their interfaces, so they can do certain tasks such as intercepting and forwarding messages. In addition an object can ignore a message or not implement it without it being considered an error. In a broad sense, an ObjC message and a C++ method are or more or less analogous in functionality.

## C/C++ Timeline

These are the major revisions of C and C++

| Year | Event | Description |
| --- | --- | --- |
| 1972 | C | C for PDP-11, other Unix systems |
| 1978 | K&R C | C as defined in "The C Programming Language" book by Kernighan & Ritchie |
| 1979 | C with classes -&gt; C++ | Bjarne Stroustrops |
| 1989 | C89 \(ANSI X3.159-1989\) | C is standardized as ANSI C, or C89. C90 \(ISO/IEC 9899:1990\) is the ISO ratified version of this same standard. |
| 1987 | GNU C Compiler \(GCC\) | An open source C compiler that was quickly adopted by developers and became the defacto compiler for most Unix / Linux systems. Acronym `GCC` was rebranded as GNU Compiler Chain since the project encapsulated many other languages including C++, Objective-C, Ada, Fortran etc. that had individual compiler front-ends but shared the code generating backend. |
| 1995 | C95 \(ISO/IEC 9899/AMD1:1995\) | Wide character support, digraphs, new macros, and some other minor changes. |
| 1998 | C++98 \(ISO/IEC 14882:1998\) | C++ is standardized for the first time. |
| 1999 | C99 \(ISO/IEC 9899:1999\) | Single line \(//\) comments, mixing declarations with code, new intrinsic types, inlining, new headers, variable length arrays |
| 2003 | C++03 \(ISO/IEC 14882:2003\) | Primarily a defect revision, addressing various defects in the specification. |
| 2003 | LLVM | Similar to GCC, a compiler and toolchain that started predominantly to support compiling C and C++ via the Clang compiler with many backends for generating machine code across across many platforms. The toolchain is also used by the Rust compiler. |
| 2011 | C++11 \(ISO/IEC 14882:2011\) | A major revision that introduces type inference \(auto\), range based loops, lambdas, strongly typed enums, a nullptr constant, struct initialization. Improved Unicode char16\_t, char32\_t, u, U and u8 string literals. |
| 2011 | C11 \(ISO/IEC 9899:2011\) | Multi-threading support. Improved unicode char16\_t, char32\_t, u, U and u8 string literals. Other minor changes |
| 2014 | C++14 \(ISO/IEC 14882:2014\) | A major revision that introduces auto return types, variable templates, digit separators \(1'000'000\), generic lambdas, lambda capture expressions, deprecated attribute. 
| 2017 | C++17 \(ISO/IEC 14882:2017\) | A major revision that introduces a number of enhancements but notably a filesystem library, `string_view` and `option` types and `UTF-8` strings |

In a sense C++ is converging with Rust since many of the enhancements which have gone into the language have the potential to make code safer and more efficient. 
