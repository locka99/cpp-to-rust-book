# Polymorphism

## C++

C++ has 4 types of polymorphism:

1. Function name overloading - multiple definitions of the same function taking different arguments. 
2. Coercion - implicit type conversion, e.g. assigning a double to an int or a bool.
3. Parametric - compile type substitution of parameters in templates
4. Inclusion - subtyping a class with virtual methods overloads their functionality. Your code can use the pointer to a base class, yet when you call the method you are calling the function implemented by the subtype.

These conspire to allow the same named function to be called with different parameters that may or may not match their signatures with the compiler following a set of rules to determine what you the programmer meant.

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

### Beware implicit conversion

One of the biggest issues that you might begin to see from the above example is that is too easy to inadvertantly call the wrong function because C++ will also implicitly convert types. 

For example we have a class `Foo` which has a constructor.

```c++
class Foo {
public:
  Foo(int i) { /*...*/ }
};

// A function that does something with Foo
void do_foo(Foo f) {
  //.. do something to foo
}

// A piece of code that calls do_foo().
Foo i1(20);
do_foo(1);
//...

```

What you may not notice at first glance is `do_foo()` was called with `1` not `i1` but it still compiled. Why? Because the compiler took it upon itself to convert that `1` into `Foo(1)` and feed it to the function. Because of this a subtle error has crept in. So C++ has an `explicit` keyword to stop this happening:

```c++
class Foo {
public:
  explicit Foo(int i) { /*...*/ }
};

// A piece of code that calls do_foo().
Foo i1(20);
do_foo(1); // Compiler error
//...
```

But you can see it's all becoming very ornery and that's before even considering that C++ has default parameter values _and_ default constructors. So you might call a function using one signature and be calling something entirely different after the compiler resolves it.

## Rust

Rust has limited support for polymorphism. This can be very frustrating as we'll see and it may well be that some restrictions will relax in time.

But for the moment the rules are as follows:

1. Function name overloading - there is none. See section below for alternatives.
2. Coercion. Rust allows limited, explict coercion between numeric types using the `as` keyword. Otherwise see below for use on `Into` and `From` traits.
3. Parameteric - similar to C++ via generics.
4. Inclusion - there is no inheritance in Rust. The nearest thing to a virtual method in rust is a trait with an implemented function that an implementation overrides with its own. However this override is at compile time.

### Alternatives to function name overloading

If you have a few functions you can just disambiguate them, e.g.

```rust
fn new(name: &str) -> Foo { /* ... */ }
fn new_age(name: &str, age: u16) -> Foo { /* ... */ }
```

This can look very messy after a while, so there is another alternative - conversion traits.

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
