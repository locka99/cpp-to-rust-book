# Polymorphism

## C++

C++ has 4 types of polymorphism:

1. Function name overloading - multiple definitions of the same function taking different arguments. 
2. Coercion - implicit type conversion, e.g. assigning a double to an int or a bool.
3. Parametric - compile type substitution of parameters in templates
4. Inclusion - subtyping a class with virtual methods overloads their functionality. Your code can use the pointer to a base class, yet when you call the method you are calling the function implemented by the subtype.

That is to say, the same named function can be overloaded with different parameters. 

### Function name overloading

```c++
class Variant {
public:
  void set(); // Null variant
  void set(bool value);
  void set(int value);
  void set(float value);
  void set(Array *value);
};
```

One of the biggest issues that you might begin to see from the above example is that is too easy to inadvertantly call the wrong function because C++ will also implicitly convert types. On top of that C++ also has default parameter values _and_ default constructors. So you might call a function using one signature and be calling something entirely different after the compiler resolves it.

```c++

// Sample code
Variant v;
//...
v.set(NULL);
```

This example will call the integer overload because `NULL` evaluates to 0. One of the changes to `C++11` was to introduce an explicit `nullptr` value and type to avoid this issues.

## Rust

Rust has limited support for polymorphism. 

1. Function name overloading - there is none. See section below for alternatives.
2. Coercion. Rust allows limited, explict coercion between numeric types using the `as` keyword. Otherwise see below for use on `Into` and `From` traits.
3. Parameteric - similar to C++ via generics
4. Inclusion - there is no inheritance in Rust. The nearest thing to a virtual method in rust is a trait with an implemented function that an implementation overrides with its own. However this override is at compile time.

### Alternatives to function name overloading

If you have a few functions you can just disambiguate them, e.g.

```rust
fn new(name: &str) -> Foo { /* ... */ }
fn new_age(name: &str, age: u16) -> Foo { /* ... */ }
```

#### Use traits

A common way to do polymorphism is with _traits_.
 
There are two standard traits for this purpose:

* The `From<>` trait converts from some type into the our type. 
* The `Into<>` trait converts some type (consuming it in the process) into our type 

You only need to implement `From` or `Into` because one implies the other.

The `From` trait is easier to implement:

```rust
use std::convert::From;

impl From<&'static str> for Foo {
  fn from(v: &'static str) -> Self {
    Foo { /* ... */ }
  }
}

impl From<(&'static str, u16)> for Foo {
  fn from(v: (&'static str, u16)) -> Self {
    Foo { /* ... */ }
  }
}
//...

let f = Foo::from("Bob");
let f = Foo::from(("Mary", 16));
```

But let's say we want an explicit `new` constructor function on type `Foo`. In that case, we could write it using the `Into` trait:

```rust
impl Foo {
  pub fn new<T>(v: T) -> Foo where T: Into<Foo> {
    let result = Foo::foo(v);
    // we could code here that we do here after making Foo by whatever means
    result
  }
}
```

Since `From` implies `Into` we can just call the constructor like so:

```rust
let f = Foo::new("Bob");
let f = Foo::new(("Mary", 16));
```

If you prefer you could implement `Into` but it's more tricky since it consumes the input, which might not be what you want.

```rust
// This Into works on a string slice
impl Into<Foo> for &'static str {
    fn into(self) -> Foo {    
        //... constructor
    }    
}

// This Into works on a tuple consisting of a string slice and a u16
impl Into<Foo> for (&'static str, u16) {    
    fn into(self) -> Foo {    
        //... constructor
    }    
}

//...
let f: Foo = "Bob".into();
let f: Foo = ("Mary", 16).into();
// OR
let f = Foo::new("Bob");
let f = Foo::new(("Mary", 16));
```

#### Use enums

Remember that an enumeration in Rust can contain actual data, so we could also implement a function that takes an enumeration as an argument that has values for each kind of value it accepts:

```rust
pub enum FooCtorArgs {
   String(String),
   StringU16(String, u16)
}

impl Foo {
  pub fn new(v: FooCtorArgs) {
    match v {
      FooCtorArgs::String(s) => { /* ... */ }
      FooCtorArgs::StringU16(s, i) => { /* ... */ }
    }
  }
}
//...
let f = Foo::new(FooCtorArgs::String("Bob".to_string()));
let f = Foo::new(FooCtorArgs::StringU16("Mary".to_string(), 16));
```
