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
  pub width: i32;
  pub height: i32;
}
```

An `impl` section follows containing the associated functions:

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

The `new()` function here is a convenience method that returns a struct preinitialised with the arguments supplied. The `area()` function specifies a `&self` argument and returns an area calculation. Any function that supplies a `&self`, or `&mut self` can be called from the variable bound to the struct.

```rust
let size = Size::new(10, 20);
println!("Size = {}", size.area());
```

The `self` keyword works in much the same way as C++ uses `this`, as a reference to the struct from which the function was invoked. If a function modifies the struct it must say `&mut self`, which indicates the function modifies the struct.

There is no inheritance in Rust. Instead, a `struct` may implement zero or more traits. A trait describes some kind of behavior that can be associated with the struct and described further later on in this chapter.

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
  pub fn new(name: String, age: String) -> Person {
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

If you want something to access a member of a struct (including modifying it if its mutable), then mark it `pub`.

TODO example

If you want something to be able to call a function on your struct you mark it `pub`.

TODO example

If you want a struct to be visible outside your module you mark it `pub`

TODO example

## Functions

And then it has functions that you bind to the struct contained within an `impl` block:

```rust
impl Shape {
  fn area(&self) -> i32 {
    self.width * self.height
  }
  fn set(&mut self, width: i32, height: i32) {
    self.width = width;
    self.height = height;
  }
}
```

Note how the first parameter is `&self` which is a reference to the struct instance. In one method we pass a immutable `&self` because it doesn’t need to modify the struct. In the second we pass a mutable `&mut self` so we can modify the struct.

Unlike C++, all access to the struct has to be qualified. In C++ you don't have to say `this->foo()` to call foo() from another member of the class. Rust requires code to say unambiguously `self.foo()`.

## Static methods

Static methods are merely functions in the `impl` block that do not have `&self` as their first parameter, e.g.

```rust
impl Circle {
   fn pi() -> f64 { std::f64::consts:PI }
}
//...
let pi = Circle::pi();
```

You can attach functions to the class or the instance depending on the first argument being &self or not.

## Traits

C++ allows one class to inherit from another. Generally this is a useful feature although it can get pretty complex if you implement multiple inheritance, particularly the dreaded diamond pattern.

As we’ve found out, Rust doesn’t have classes at all – they’re structs with bound functions.  So how do you inherit code? The answer is you don’t.

Instead your struct may implement traits which are a bit like partial classes.

A trait is declared like so:

```rust
trait HasCircumference {
  fn circumference() -> i32;
}
```

And then structs can implement the trait by declaring it

```rust
impl HasCircumference for Size {
  fn circumference() -> i32 {
    2 * width + 2 * height
  }
}
```

TODO traits partial implementation.

## Lifetimes

C++ doesn't really care how long objects live. If you maintain a pointer to some object from your class, it won't care if the pointer is valid or not. It won't care if the object has long gone and you're holding onto garbage.

Rust does care and carefully tracks the lifetime of objects to ensure that you cannot reference something that no longer exists.

Occasionally this causes problems for structs and classes where a struct holds a reference to some other struct but is unsure about the life time.

TODO lifetimes

## Lifetime can be omitted / elided in most cases

Elision means - to omit. It's a [fancy word](https://ericlippert.com/2013/01/24/five-dollar-words-for-programmers-elision/) that is used for when the compiler can work out the lifetimes of structs for itself. If the compiler can work out the lifetime then we don't need to declare a lifetime.

TODO example of elided lifetime versus specific

When you do not have to specifically say anything about the lifetime because the compiler figures it out, it is said to be elided. Why it's called elision when omit would be a more commonly understood word is anyone's guess. Elide = omit, remember that.
