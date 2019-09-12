# Class Member Initialisation

C++ does not require that you initialise all variables in every constructor.

* A member that is a C++ class with its own default constructor doesn't need to be initialised.
* A member that is a C++ class without a default constructor must be explicitly initialised.
* A member that is a reference must be explicitly initialised.
* Primitive types, including pointers do not have to be initialised although the compiler may warn if they are not.
* Members do not have to be initialised in the order they are declared although the compiler may warn if they are not.

C++11 allows classes to have default member initializers which are used in the absence of a constructor setting the 
value to something else:

```c++
class Coords {
public:
    double x = 0.0;
    double y = 0.0;
    double z = 0.0;

    // 2D initializer, x and y are set with the inputs, z is set to 0
    Coords(double x, double y) : x(x), y(y) {}
};
```

This is obviously a lot easier to read and ensures that if we have multiple constructors that we don't have to initialize
members if the default value will do.

However what is not so nice is that initialisation is spread all over the place in the code. Some of it may be in a header file, some in the 
constructor. Nothing is very clear at all.

## How Rust helps

You MUST initialise all members of a struct. You CANNOT forget to initialise anything or it becomes a compiler error. This
always means fields in structs are in an initialised state.

So this will not compile:

```rust
struct Alphabet {
  a: i32,
  b: u32,
  c: bool,
}

// Forgot to init b
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

* Forcing you to initialise the members of the struct ensures the struct is always in a consistent predictable state.
* Ordering of initialisation does not matter providing all of the fields are set.

Structs often implement a `new()` function which encapsulates this initialisation and acts like a class constructor in C++, e.g.

```rust
struct Coord {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Coord {
  pub fn new(x: f64, y:f64) {
    Coord { x: x, y: y, z: 0f64 }
  }
}
///...
let coord1 = Coord::new(100f64, 200f64);
```

Alternatively the struct might implement one or more `From<>` traits:

```rust
impl From<(f64, f64)> for Coord {
  fn from(value: (f64, f64)) -> Coord {
    Coord { x: value.0, y: value.1, z: 0.0 }
  }
}

impl From<(f64, f64, f64)> for Coord {
  fn from(value: (f64, f64, f64)) -> Coord {
    Coord { x: value.0, y: value.1, z: value.2 }
  }
}


//...
let coord = Coord::from((10.0, 20.0));
let coord = Coord::from((10.0, 20.0, 30.0));
```

There can be multiple `From` trait implementations so we can implement a form of polymorphism.
