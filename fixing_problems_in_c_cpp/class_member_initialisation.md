# Class Member Initialisation

C++ does not require that you initialise all variables in every constructor.

* A member that is a C++ class with a default constructor doesn't need to be initialised
* A member that is a C++ class without a default constructor must be explicitly initialised.
* A member that is a references must be explicitly initialised
* Primitive types, including pointers do not have to be initialised
* Members do not have to be initialised in the order they are declared

Some compilers may issue warnings if you forget to initialise members or botch their ordering, but they will still compile the code.

TODO C++11 allows initialisation of variables in-place but this depends on every variable being initialised the same way for each overloaded constructor.

## How Rust helps

You MUST initialise all members of a struct. If your code does not initialise a struct you will get a compiler error.

This will not compile:

```rust
struct Alphabet {
  a: i32,
  b: u32,
  c: bool,
}

let a = Alphabet { a: -10, c: true };
```

If you try you will get an error like this:

```
rustc 1.13.0-beta.1 (cbbeba430 2016-09-28)
error[E0063]: missing field `b` in initializer of `main::Alphabet`
  |
9 |     let a = Alphabet { a: -10, c: true };
  |             ^^^^^^^^ missing `b`
```
