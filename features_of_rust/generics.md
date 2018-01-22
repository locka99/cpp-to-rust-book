# Templates / Generics

C++ offers templates as a way to write generic code using an abstract type and then specialize it by substituting one or more types into a concrete class.

```c++
template <typename T>
inline void debug(const T &v) {
  cout << "The value of object is " << v << endl;
}
//...
debug(10);
```

This template uses the type of the parameter (int this case 10) to create an inline function that prints out the value of that type:

```
The value of object is 10
```

Classes can also be made from templates:

```c++
template <class T>
class Stack {
private:
  vector<T> elements;
public:
  void push(const T &v) {
    // ...
  }
  T pop() {
    // ...
  }
}
//...
Stack<double> doubleStack;
```

This class implements a simple stack using a template to indicate the type of object it contains.

This is a very powerful mechanism and the C++ library makes extensive use of it. 

Where templates can become a bit of a mess is that the templates are inline and the compiler will expand out anything you call before attempting to compile it. 

An innocuous error such as using a type that has no default copy constructor in a collection can cause the compiler to go nuts and output a wall of indecipherable errors. 

## Generic Functions

Rust's equivalent to a template is called a generic. A generic generalizes a function or a trait so it works with different types that match the criteria.

So the Rust equivalent of the `debug()` function in C++ would be this.

```rust
use std::fmt;

fn debug<T>(data: T) where T: fmt::Display {
  println!("The value of object is {}", data);
}
//...
debug(10);
```

Here we describe a function that takes a generic type `T` where the constraint is that `T` must implement the trait `std::fmt::Display`. Any struct that implements this trait can passed into the call. Since integer types implement the trait, we can just call it directly as `debug(10)` and the compiler is happy.

## Generic structs

Similarly we can use generics on a struct. So the equivalent in Rust of the C++ template class `Stack` is this:

```rust
struct Stack<T> {
  elements: Vec<T>
}

impl<T> Stack<T> {
  fn new() -> Stack<T> { Stack { elements: Vec::new() } }

  fn push(v: T) {
    //...
  }

  fn pop() -> Option<T> {
    //...
    None
  }
}
//...
let double_stack: Stack<f64> = Stack::new();
```

## Where clause

The `where` clause can be added to impose constraints on what generic type must do to be allowed to be supplied to the generic function or struct.

For example we might have a function that takes a closure as an argument. A closure is a function and so we want to define the shape that the closure will take.

So:

```rust
fn compare<T, F>(a: T, b: T, f: F) -> bool 
  where F: FnOnce(T, T) -> bool 
{
  f(a, b)
}

let comparer = |a, b| a < b;
let result = compare(10, 20, comparer);
```

Here we have defined a `compare()` function that takes a couple of values of the same type. The `where` clause states that the function must take two values of the same type and return a boolean. The compiler will ensure any closure we pass in matches that criteria, as indeed our `comparer` closure does.