# Structs

## C++

A `class` and a `struct` in C++ are largely the same thing from an implementation standpoint. They both hold fields and they both can have methods attached to the class (`static`) or instance level. 

```c++
class Foo {
public:
   // Methods and members here are publicly visible
   double calculateResult();
protected:
   // Elements here are only visible to ourselves and subclasses
   virtual double doOperation(double lhs, double rhs);
private:
   // Elements here are only visible to ourselves
   bool debug_;
};
```

The default access level is `public` for struct and `private` for class. Some rules about templates only apply to classes.

From a psychological perspect a `struct` tends to be used to hold public data that is largely static and/or passed around. A `class` tends to be something more self contained with methods that are called to access or manage private fields.

So these are equivalents:

```c++
struct Foo { // as a struct
private:
};

class Foo { // As a class
};

// Or the other way around

struct Bar {
};

class Bar {
public:
};
```

Classes can also use an access specifier to inherit from a base class. So a class may specify `public`, `protected` or `private` when deriving from another class depending on whether it wants those methods to be visible to callers, or subclasses.

Classes and structs may have special constructor and destructor methods which are described in sections below.

```c++
class Size {
public:
  Size(int width, int height);

  int width_;
  int height_;

  int area() const;
};
```

Then in the .cpp file you might implement the constructor and method:

```C++
Size::Size(int width, int height) : width_(width), height_(height) {}

int Size::area() { return width_ * height_; }
```

## Rust

Rust only has structs. A `struct` consists of a definition which specifies the fields and their access level (public or not), and an `impl` section which contains the implementation of functions bound to the struct.

```rust
struct Size {
  pub width: i32,
  pub height: i32,
}
```

An `impl` section follows containing the associated functions (`new`) and methods (`area`):

```rust
impl Size {
  pub fn new(width: i32, height: i32) -> Size {
    Size { width: width, height: height, }
  }

  pub fn area(&self) -> i32 {
    self.width * self.height
  }
}
```

The `new()` function here is an associated function that returns a struct preinitialised with the arguments supplied. The `area()` method specifies a `&self` argument and returns an area calculation. Any method (with `&self`, or `&mut self` as the first argument)  can be called from the variable bound to the struct.

```rust
let size = Size::new(10, 20);
println!("Size = {}", size.area());
```

The `self` keyword works in much the same way as C++ uses `this`, as a reference to the struct from which the function was invoked. If a function modifies the struct it must say `&mut self`, which indicates the function modifies the struct.

There is no inheritance in Rust. Instead, a `struct` may implement zero or more traits. A trait describes some kind of behavior that can be associated with the struct and described further later on in this chapter.

The above impelementation can also be written with the following syntactic sugar:

```rust
struct Size {
  pub width: i32,
  pub height: i32,
}


impl Size {
  pub fn new(width: i32, height: i32) -> Self {
      Self {width, height}
  }

  pub fn area(&self) -> i32 {
      self.width * self.height
  }
}
```

Note using `Self` as a type replacer and struct fields inference.

## Constructors

In C++ all classes have implicit or explicit constructors. Either the compiler generates them or you do, or a mix of both.

An implicit default constructor, copy constructor and assignment operator will be created when a class does not define its own. We saw on page 73 why this could be really bad news.

What becomes obvious from reading there is a lot of noise and potential for error in C++. There would be even more if raw pointers were used instead of a `std::unique_ptr` here.

In Rust, things are simpler, and we'll see how it shakes out errors.

First off, let's declare our equivalent struct in Rust:

```rust
struct Person {
  pub name: String,
  pub age: i32,
  pub credentials: Option<Credentials>,
}
```

Since credentials are optional, we wrap in an `Option` object, i.e. credentials might be None or it might be `Some(Credentials)`.
Any code anywhere in the system can instantiate a `Person` simply be declaring an instance:

```rust
let person = Person { name: String::from("Bob"), age: 20, credentials: None }
```

In Rust you cannot create a struct without initialising all its members so we cannot have a situation where we don't know what is in each field - it MUST be set by our code.

But declaring the struct is a bit clumsy, especially if the struct is created in lots of places. So can write function that behaves like a constructor in C++.

Instead you implement a static method in the impl of the Struct which returns an initialised struct, e.g.

```rust
impl Person {
  pub fn new(name: String, age: i32) -> Person {
    Person { name: name.clone(), age: age, credentials: None }
  }
}
```

Note that Rust does not support overloads. So if we had multiple "constructor" methods, they would each have to have unique names.

Finally what is we wanted to copy the `Person` struct?

By default Rust does not allow copying on user-defined structs. Assigning a variable to another variable moves ownership, it doesn't copy.

There are two ways to make a user-defined struct copyable

1. implement the `Copy` trait which means assignment is implicit, but is what we want? Do we really want to make copies of a struct by accident?
2. implement `Clone` instead to add a `clone()` method and require an explicit call to `clone()` order to duplicate the struct a copy.

But the compiler can derive clone() providing all the members of the struct implement the Clone trait.

```rust
#[derive(Clone)]
struct Person {
  pub name: String,
  pub age: i32,
  pub credentials: Option<Credentials>, // Credentials must implement Clone
}

impl Person {
  pub fn new(name: String, age: String) -> Person {
    Person { name: name.clone(), age: age, credentials: None }
  }
}

//...

let p = Person::new(String::from("Michael"), 20);
let p2 = p.clone();
```

What we can see is that Rust's construction and `clone()` behavior is basically declarative.
We saw how C++ has all kinds of rules and nuances to construction, copy construction and assignment which make it complicated and prone to error.  

## Destructors
A C++ destructor is a specialized method called when your object goes out of scope or is deleted.

```c++
class MyClass {
public:
  MyClass() : someMember_(new Resource()) {}
  ~MyClass() {
     delete someMember_;
  }

private:
  Resource *someMember_;
}
```

In C++ you can declare a class destructor to be called when the object is about to be destroyed. You have to use a virtual destructor if your class inherits from another class in case a caller calls `delete` on the base class.

Since Rust does not do inheritance and does not have constructors, the manner in which you cleanup is different and simpler. Instead of a destructor you implement the `Drop` trait.

```rust
impl Drop for Shape {
    fn drop(&mut self) {
        println!("Shape dropping!");
    }
}
```

The compiler recognizes this trait. If you implement this trait then the compiler knows to call your `drop()` function prior to destroying your struct. It’s that simple.

Occasionally there might be a reason to explicitly drop a struct before it goes out of scope. Perhaps the resources held by the variable should be freed as soon as possible to release a resource which is in contention. Whatever the reason, the answer is to call `drop` like this:

```rust
{
  let some_object = SomeObject::new();
  //...
  // Ordinarily some_object might get destroyed later,
  // but this makes it explicitly happen here
  drop(some_object);
  //...
}
```

## Access specifier rules

A C++ class can hide or show methods and members to any other class, or to things that inherit from itself using the public, private and protected keywords:

* `public` – can be seen by any code internal or external to the class
* `private` – can only be used by code internal to the class. Not even subclasses can access these members
* `protected` – can be used by code internal to the class and by subclasses.

A class may designate another function or class as a friend which has access to the private and protected members of a class.

Rust makes things somewhat simpler.

If you want a struct to be visible outside your module you mark it `pub`. If you do not mark it `pub` then it is only visible within the module and submodules.

```rust
pub struct Person { /* ... */ }
```

If you want public access a member of a struct (including modifying it if its mutable), then mark it `pub`.

```rust
pub struct Person {
  pub age: u16,
}
```

If you want something to be able to call a function on your struct you mark it `pub`.

```rust
impl Person {
  pub fn is_adult(&self) -> bool {
    self.age >= 18
  }
}
```

## Functions

Functions can be bound to a struct within an `impl` block:

```rust
impl Shape {
  pub fn new(width: u32, height: u32) -> Shape {
    Shape { width, height }
  }
  
  pub fn area(&self) -> i32 {
    self.width * self.height
  }

  pub fn set(&mut self, width: i32, height: i32) {
    self.width = width;
    self.height = height;
  }
}
```

Functions that start with a `&self` / `&mut self` parameter are bound to instances.  Those without are bound to the type. So the `new()` function can be called as `Shape::new()`.

Where `&self` is provided, the function is invoked on the instance. So for example:

```rust
let shape = Shape::new(100, 100);
let area = shape.area();
```

Where `&mut self` is provided it signifies that the function mutates the struct.

Unlike C++, all access to the struct has to be qualified. In C++ you don't publishing_interval: Double, lifetime_count: UInt32, max_keep_alive_count: UInt32, max_notifications_per_publish: UInt32, priority: Bytehave to say `this->foo()` to call foo() from another member of the class. Rust requires code to say unambiguously `self.foo()`.

## Static functions

Static functions ("associated functions") are merely functions in the `impl` block that do not have `&self` or `&mut self` as their first parameter, e.g.

```rust
impl Circle {
   fn pi() -> f64 { std::f64::consts:PI }
}
//...
let pi = Circle::pi();
```

In other words they're not bound to an instance of a type, but to the type itself. For example, `Circle::pi()`.

## Traits

C++ allows one class to inherit from another. Generally this is a useful feature although it can get pretty complex if you implement multiple inheritance, particularly the dreaded diamond pattern.

As we’ve found out, Rust doesn’t have classes at all – they’re structs with bound functions.  So how do you inherit code? The answer is you don’t.

Instead your struct may implement traits which are a bit like partial classes.

A trait is declared like so:

```rust
trait HasCircumference {
  fn circumference(&self) -> f64;
}
```

Here the trait `HasCircumference` has a function called `circumference()` whose signature is defined but must be implemented.

A type can implement the trait by declaring and `impl` of it.

```rust
impl HasCircumference for Size {
  fn circumference(&self) -> f64 {
    2.0 * std::f64::consts::PI * self.radius
  }
}
```

A trait may supply default function implementations. For example, a `HasDimensions` trait might implement `area()` to spare the implementor the bother of doing it.

```rust
trait HasDimensions {
  fn width(&self) -> u32;
  fn height(&self) -> u32;

  fn area(&self) -> u32 {
    self.width() * self.height()
  }
}
```

## Lifetimes

In C++ an object lives from the moment it is constructed to the moment it is destructed. 

That lifetime is implicit if you declare the object on the stack. The object will be created / destroyed as it goes in and out of scope. It is also implicit if your object is a member of another object - the lifetime is within the containing object, and the declaration order of other members in the containing object.

However, if you allocate your object via `new` then it is up to you when to `delete`. If you `delete` too soon, or forget to `delete` then you may destabilize your program.  C++ encourages using smart pointers that manage the lifetime of your object, tying it to the implicit lifetime of the smart pointer itself - when the smart pointer is destroyed, it deletes the held pointer. A more sophisticated kind of smart pointer allows multiple instances of the same pointer to exist at once, and reference counting is used so that when the last smart pointer is destroyed, it destroyes the pointer.

Even so, C++ itself will not care if you initialized a class with a reference or pointer to something that no longer lives. If you do this, your program will crash.

Let's write an `Incrementor` class which increments an integer value and returns that value.

```c++
class Incrementor {
public:
	Incrementor(int &value) : value_(value) {}
	int increment() { return ++value_; }

private:
	int &value_;
};
```

This seems fine, but what if we use it like this?

```c++
Incrementor makeIncrementor() {
  // This is a bad idea
	int value = 5;
	return Incrementor(value);
}
```

This code passes a reference to an `int` into the class constructor and returns the `Incrementor` from the function itself. But when `increment()` is called the reference is dangling and anything can happen.

## Rust lifetimes

Rust *does* care about the lifetime of objects and tracks them to ensure that you cannot reference something that no longer exists. Most of the time this is automatic and self-evident from the error message you get if you try something bad. 

The compiler also implements a *borrow checker* which tracks references to objects to ensure that:

1. References are held no longer than the lifetime of the object they refer to.
2. Only a single mutable reference is possible at a time and not concurrently with immutable references. This is to prevent data races.

The compiler will generate compile errors if it finds code in violation of its rules.

So let's write the equivalent of `Incrementor` above but in Rust. The Rust code will hold a reference to a integer `i32` and increment it from a bound function:

```rust
struct Incrementor {
  value: &mut i32
}

impl Incrementor {
  pub fn increment(&mut self) -> i32 {
    *self.value += 1;
    *self.value
  }
}
```

Seems fine, but the first error we get is:

```
2 |   value: &mut u32
  |          ^ expected lifetime parameter
```

We tried to create a struct that manages a reference, but the compiler doesn't know anything about this reference's lifetime and so it has generated a compile error.

To help the compiler overcome its problem, we will annotate our struct with a lifetime which we will call `'a`. The label is anything you like but typically it'll be a letter. 

This lifetime label is a hint on our struct that says the reference we use inside the struct must have a lifetime of at least as much the struct itself - namely that `Incrementor<'a>` and `value: &'a mut i32` share the same lifetime constraint and the compiler will enforce it.

```rust
struct Incrementor<'a> {
  value: &'a mut i32
}

impl <'a> Incrementor<'a> {
  pub fn increment(&mut self) -> i32 {
    *self.value += 1;
    *self.value
  }
}
```

With the annotation in place, we can now use the code:

```rust
let mut value = 20;
let mut i = Incrementor { value: &mut value };
println!("value = {}", i.increment());
```

Note that the annotation `'a` could be any label we like - `'increment` would work if we wanted, but obviously its more longwinded.

There is a special lifetime called `'static` that refers to things like static strings and functions which have a lifetime as long as the runtime and may therefore be assumed to always exist.

### Lifetime elision

Rust allows reference lifetimes to be elided (a [fancy word](https://ericlippert.com/2013/01/24/five-dollar-words-for-programmers-elision/) for omit) in most function signatures.

Basically, it assumes that when passing a reference into a function, that the lifetime of the reference is implicitly longer than the function itself so the need to annotate is not necessary.

```rust
fn find_person(name: &str) -> Option<Person>
// instead of
fn find_person<'a>(name: &'a str) -> Option<Person>
```

The rules for elision are described in the further reference link.

### Further reference

Lifetimes are a large subject and the documentation is [here](https://doc.rust-lang.org/book/second-edition/ch10-03-lifetime-syntax.html#lifetime-elision).
