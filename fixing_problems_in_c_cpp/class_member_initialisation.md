# Class Member Initialisation

C++ does not require that you initialise all variables in every constructor.

* A member that is a C++ class with a default constructor doesn't need to be initialised
* A member that is a C++ class without a default constructor must be explicitly initialised.
* A member that is a references must be explicitly initialised
* Primitive types, including pointers do not have to be initialised
* Members do not have to be initialised in the order they are declared

Some compilers may issue warnings if you forget to initialise members or their ordering, but they will still compile the code.

C++11 allows classes to have default member initializers which are used in the absence of a constructor setting the value to something else:

```
class Coords {
public:
  double x = 0.0;
  double y = 0.0;
  double z = 0.0;

  // 2D initializer, x and y are set with the inputs, z is set to 0
  Coords(x, y) : x(x), y(y) {} {
  }
};
```

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
