# Attributes

C++ has various ways to give *directives* during compilation:

* Compile flags that control numerous behavious
* #pragma statements - once, optimize, comment, pack etc. Some pragmas such as comment have been wildly abused in some compilers to insert "comments" into object files that control the import / export of symbols, static linking and other functionality.
* #define with ubquitous #ifdef / #else / #endif blocks
* Keywords inline, const, volatile etc.. These hint the code and allow the compiler to make decisions that might change its output or optimization. Compilers often have their own proprietary extensions.

Rust uses a notation called *attributes* that serves a similar role to all of these things but in a more consistent form.

An attribute applies to the next item it is declared before:

```rust
#[test]
fn this_is_a_test() {
  //...
}
```

Attributes are enclosed in a #[ ] block and provide compiler directives that allow:

* Functions to be marked as unit or benchmark tests
* Functions to be marked for conditional compilation for a target OS. A function can be defined that only compiles for one target. e.g. perhaps the code that communicates with another process on Windows and Linux is encapsulated in the same function but implemented differently.
* Enable / disable lint rules
* Enable / disable compiler features. Certain features of rust may be experimental or deprecated and may have to be enabled to be accessed.
* Change the entry point function from `main` to something else
* Conditional compilation according to target architecture, OS, family, endianess, pointer width
* Inline hinting
* Deriving certain traits
* Enabling compiler features such as plugins that implement procedural macros.
* Importing macros from other crates

Attributes can be expressed as #![foo] which affects the thing they're contained by rather the thing that follows them. Attributes can also have name=value pairs as part of the directive.

## Conditional compilation

TODO

## Linking to native libraries

In C/C++ code is first compiled and then it is linked. The linking phase takes a list of object files and a list of static libs to be used to resolve functions.

C++ | Rust
--- | ----
\#pragma (comment, "somelib") | #[link(name = "somelib")]
- | #[link(name = "somelib", kind = "static")]

Default "kind" is "dynamic" library but "static" can be specified.

## Inlining code

Inlining happens where your function logic is inserted in-place to the code that invokes it. It tends to happen when the function does something trivial such as return a value or execute a simple conditional. The overhead of duplicating the code is outweighed by the performance benefit.

Inlining is achieved in C++ by declaring and implementing a function, class method or template method in a header or marking it with the inline keyword.

In Rust, inlining is only a hint. Rust recommends not forcing inlning, rather leaving it as a hint for the LLVM compiler to do with as it sees fit.

C++ | Rust
--- | ----
Explicitly with "inline" or implicitly through methods implemented in class / struct | #[inline], #[inline(always)], #[inline(never)]

Another alternative to explicitly inlining code is to use the link-time optimisation in LLVM.

```
rustc -C lto
```
