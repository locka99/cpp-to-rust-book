# Attributes

C++ has various ways to give compiler *directives* during compilation:

* Compile flags that control numerous behaviours
* `#pragma` statements - `once`, `optimize`, `comment`, `pack` etc. Some pragmas such as `comment` have been wildly abused in some compilers to insert "comments" into object files that control the import / export of symbols, static linking and other functionality.
* `#define` with ubquitous `#ifdef` / `#else` / `#endif` blocks
* Keywords `inline`, `const`, `volatile` etc.. These hint the code and allow the compiler to make decisions that might change its output or optimization. Compilers often have their own proprietary extensions.

Rust uses a notation called *attributes* that serves a similar role to all of these things but in a more consistent form.

An attribute `#[foo]` applies to the next item it is declared before. A common attribute is used to denote a unit test case with `#[test]`:

```rust
#[test]
fn this_is_a_test() {
  //...
}
```

Attributes can also be expressed as `#![foo]` which affects the thing they're contained *by* rather the thing that follows them. 

```rust
fn this_is_a_test() {
  #![test]
  //...
}
```

Attributes are enclosed in a `#[ ]` block and provide compiler directives that allow:

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
* Used by certain crates like serde and rocket to instrument code - NB Rocket uses unstable compiler hooks for this and in so doing limits itself to working in nightly builds only. 

## Conditional compilation

Conditional compilation allows you to test the target configurations and optionally compile functions or modules in or not. 

The main configurations you will test include:

* Target architecture - "x86", "x86_64", mips", "arm" etc.
* Target OS - "windows", "macos", "ios", "linux", "android", "freebsd" etc.
* Target family - "unix" or "windows"
* Target environment - "gnu", "msvc" etc
* Target endianess
* Target pointer width

So if you have a function which is implemented one way for Windows and another for Linux you might code it like so:

```rust
#[cfg(windows)]
fn get_app_data_dir() -> String { /* ... */ }

#[cfg(not(windows))]
fn get_app_data_dir() -> String { /* ... */ }
```

Many more possibilities are listed in the [documentation](https://doc.rust-lang.org/reference/attributes.html#crate-only-attributes).

## Linking to native libraries

In C/C++ code is first compiled and then it is linked, either by additional arguments to the compiler, or by invoking a linker.

In Rust most of your linking is taken care for you providing you use `cargo`. 

1. All your sources are compiled and linked together. 
2. External crates are automatically built as static libs and linked in. 
3. But if you have to link against something external through FFI you have to write a `#link` directive in your `lib.rs` or `main.rs`. This is somewhat analogous to the `#pragma(comment, "somelib")` in C++.

C++ | Rust
--- | ----
`#pragma (comment, "somelib")` | `#[link(name = "somelib")]`
- | `#[link(name = "somelib", kind = "static")]`

The default kind for `#link` is `dynamic` library but `static` can be explicitly stated specified.

## Inlining code

Inlining happens where your function logic is inserted in-place to the code that invokes it. It tends to happen when the function does something trivial such as return a value or execute a simple conditional. The overhead of duplicating the code is outweighed by the performance benefit.

Inlining is achieved in C++ by declaring and implementing a function, class method or template method in a header or marking it with the inline keyword.

In Rust, inlining is only a hint. Rust recommends not forcing inlning, rather leaving it as a hint for the LLVM compiler to do with as it sees fit.

C++ | Rust
--- | ----
Explicitly with `inline` or implicitly through methods implemented in class / struct | `#[inline]`, `#[inline(always)]`, `#[inline(never)]`

Another alternative to explicitly inlining code is to use the link-time optimisation in LLVM.

```
rustc -C lto
```
