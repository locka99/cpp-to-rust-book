# Fixing Problems in C/C++

This section is not so much concerned with how a programmer should write code as what the C/C++ languages allow. If the language allows a programmer to write patent nonsense, or simply become too trusting of the capabilities of the language then bad things will happen.

All of these things should be considered bad and any construct in the language that enables them is also bad:

* Calling a pure virtual function in a constructor / destructor of a base class
* Calling a dangling pointer
* Freeing memory more than once
* Overflowing a buffer, e.g. being off by one with some string operation or not testing a boundary condition
* Memory leaks due to memory allocation / ownership issues
* Heap corruption

The C++ programming language is a very large specification, one that only grows and gets more nuanced and qualified with each release.

The problem from a programmer's perspective is understanding what things C++ allows them to do as oppose to what things they should do.

In each case we'll see how Rust might have stopped us getting into this situation in the first place.

## What about C?

C++ will come in for most of the criticism in this section. Someone might be inclined to think that therefore C does not suffer from problems.

Yes that is true to some extent, but it is akin to arguing we don't need shoes because we have no legs. C++ exists and is popular because it is perceived as a step up from C++. The ability to model classes and bind methods to them is a major advance. The ability to write RAII style code does improve the software's chances of keeping its memory and resource use under control.

## Compilers Will Catch Some Errors

Modern C/C++ compilers can spot some of the errors mentioned in this section. But usually they'll just throw a warning out. The simplest way to protect C / C++ from dumb errors is to bump up warnings to be errors. While it is not going to protect against every error it is still better than nothing.

* In Microsoft VC++ enable a high warning level, e.g. /W4 and possibly /WX to warnings into errors.

* In GCC enable -Wall, -pedantic-errors and possibly -Werror to turn warnings into errors. The pedantic flag rejects code that doesn't follow ISO C and C++ standards. There are a lot of errors that can be [configured](https://gcc.gnu.org/onlinedocs/gcc/Warning-Options.html#Warning-Options).

However this will probably throw up a lot of noise in your compilation process and some of these errors may be beyond your means to control.

In addition it is a good to run a source code analysis tool or linter. However these tend to be expensive and in many cases can be extremely unwieldy.
