# C and C++ Background

This section talks about C and C++. It describes its history, standards and provides a background as to how it ended up where it is today.

## History of C

The creation of C is closely associated with the early days of Unix. Bell Labs developed Unix out of an earlier project called Multics. The first version ran on PDP-7 microcomputer and funding was given to move it to PDP-11. Dennis Ritchie was a key member on this project and set about creating a language that could help him develop Unix while minimizing the amount of assembly language he had to write. Up until that point most development was expressed in assembly language which was error prone and obviously non portable.

Ritchie developed C so that he could write code in terms of variables, expressions, loops, functions etc. and use a _compiler_ to translate C code into machine code. The generated code ran almost as fast as hand written assembly and was more portable since only the compiler had to be changed in order to support a new architecture. C itself was influenced by B (hence why it was called C), which itself was influenced by BCPL.

Over time the use of C became more widespread and compilers such as Turbo C, Lattice C, Microsoft C popularized C on other operating systems including personal computers.

In 1978 C was formalised into a defacto standard called K&R C, named after Brian Kernighan & Dennis Ritche who published the standard as a book. C later became an ANSI & international standard, C89. A further standard followed with C99 and C is still under review and development.

So C started life as a systems programming language but spread into application development - games, word processors and other high level software.

Some functionality that was introduced in C++ has also found its way back into C standards. For example, single-line comments and variable declaration rules in blocks.

## History of C++
C++ first appeared in 1983 as C with classes. It was invented by Bjarne Stroustrop as a way to imbue C with Simula-like features. Simula was a language that allowed concepts such as objects, classes and inheritance to be expressed in code and as its name suggests was created for running simulations. It was considered too slow for systems use.

C++ added these concepts as extensions to the C language and used a precompiler called cfront to transform the C++ extensions into C code that could then be compiled into machine code. So a C++ program could have the high level object oriented concepts but without the overhead that came with Simula.

C++ became popular in its own right and outgrew the limitations of cfront preprocessor to become supported by compilers in its own right. Thus toolchains such as Microsoft Visual C++, GCC, Clang etc. support both languages. Some toolchains have also been given to favouring C++ over C, for example Microsoft's compiler has been very slow to implement C99.

Object oriented programming has mostly been used in higher level software - applications, games,  simulations and mathematical work.

C++ has also become formalised standards with C++98, C++03, C++11 and so on.

### The relationship between C and C++

While C++ grew out of C and has developed alongside it, it is not true to say C++ is a superset of C. Rather it is _mostly_ a superset. There are differences such as keywords and headers that C recognizes that C++ does not.

C++ has function overloading and classes and uses name mangling to disambiguate overloaded functions. But in practice it is possible to write C as a subset of C++ and compile the two into the same executable. Most real-world C code could be called C++ _without_ classes.

C and C++ are even usually handled by the same toolchain. Most compilers would consist of a front half that parses the language into an intermediate form and a back half which turns the intermediate form into optimized machine code. Finally the linker would join all the binary objects together to form an executable. C and C++ would share most of this code path.

C++ tends to be more popular with applications level programming. Part of the reason C++ hasn't found itself in the lower layers is the perception that exception handling, name mangling, linking and issues of that nature add unwanted complexity or that somehow the generated code is less efficient. Arguments have been made that this is not the case, but the perception still remains.

C still tends to be more popular in low level systems programming. Components such as the Linux kernel are pure C with some assembly. Many popular open source libraries such as sqlite3 are also written in C.

## Objective-C

Objective-C is another C derived language that added objects and classes. Unlike C++, Objective-C behaves as a strict superset of C.

The language was developed in the 1980s and was popularized in the NeXTSTEP operating system and later in Apple's OS X and iOS. It hasn't gained much popularity outside of those platforms but the success of the iPhone has ensured it has a sizeable developer base of its own. It is also well supported by the GCC and Clang toolchains. Apple has begun deprecating Objective-C in favour of Swift which is a modern high level language similar in some respects to Rust but more application focussed.

Objective-C is strongly influenced by Smalltalk (as opposed to Simula in C++) and so instead of code calling or implementing methods, code sends messages.

An object in Objective-C is something that defines an interface specifying the message signatures it can receive and an implementation that binds those messages to code. Other code sends a message to call a method. Objects can also receive dynamic messages, i.e. ones not defined by their interfaces, so they can do certain tasks such as intercepting and forwarding messages. In addition an object can ignore a message or not implement it and it is not a program error.

## C/C++ Timeline

These are the major revisions of C and C++

Year | Event | Description
-----| ----- | ----
1972 | C | C for PDP-11, other Unix systems
1978 | K&R C | C as defined in "The C Programming Language" book by Kernighan & Ritchie
1989 | C89 (ANSI X3.159-1989) | C is standardized as ANSI C, or C89. C90 (ISO/IEC 9899:1990) is the ISO ratified version of this same standard.
1979 | C with classes -> C++ | Bjarne Stroustrops
1995 | C95 (ISO/IEC 9899/AMD1:1995) | Wide character support, digraphs, new macros, and some other minor changes.
1998 | C++98 (ISO/IEC 14882:1998) | C++ is standardized for the first time.
1999 | C99 (ISO/IEC 9899:1999) | Single line (//) comments, mixing declarations with code, new intrinsic types, inlining, new headers, variable length arrays
2003 | C++03 (ISO/IEC 14882:2003) | Primarily a defect revision, addressing various defects in the specification.
2011 | C++11 (ISO/IEC 14882:2011) | A major revision that introduces type inference (auto), range based loops, lambdas, strongly typed enums, a nullptr constant, struct initialization. Improved unicode char16_t, char32_t, u, U and u8 string literals.
2011 | C11 (ISO/IEC 9899:2011) | Multi-threading support. Improved unicode char16_t, char32_t, u, U and u8 string literals. Other minor changes
2014 | C++14 (ISO/IEC 14882:2014) | Another major revision that introduces auto return types, variable templates, digit separators (1'000'000), generic lambdas, lambda capture expressions, deprecated attribute.
