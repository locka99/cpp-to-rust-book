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

This is a very powerful mechanism and the C++ library makes extensive use of it. Where templates can become a bit of a mess is that the compiler expands all of the code before compiling it. An innocuous error in a type such as the absence of a copy constructor can cause the compiler to generate a wall of indecipherable errors.

## Generic Functions

Rust's equivalent to a template is called a generic. A generic works not from types but from traits.

TODO generic function

## Trait bounds

TODO generic trait

## Where clause

Rust has a more expressive way of defining the constraints on a generic via the where clause

TODO where clause
