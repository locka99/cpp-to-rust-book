# Attributes

C++ has various ways to give *directives* during compilation:

* Compile flags that control numerous behavious
* #pragma statements in code
* #define with ubquitous #ifdef / #else / #endif blocks
* Keywords inline, const, volatile etc.. These hint the code and allow the compiler to make decisions that might change its output or optimization.

Rust uses a notation called *attributes* that serves a similar role. Attributes are enclosed in a #[ ] block and provide compiler directives that allow:

* Mark unit tests
* Conditional compilation for target OS
* Enable / disable lint rules
* Enable / disable compiler features
* Change the entry point function from main to something else
* Conditional compilation for target architecture, OS, family, endianess, pointer width,
* Inline hinting
* Derivation of certain traits
* Enabling compiler features

An attribute applies to the next item it is declared before:

```rust
#[test]
fn this_is_a_test() {
  //...
}
```

It can also be expressed as #![foo] which applies to the thing its contained by. Attributes can also have name=value pairs as part of the directive.

## Linking to native libraries

TODO

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
