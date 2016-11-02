# Virtual Destructors

C++ allows classes to inherit from other classes.

In some cases, such as this example, this can lead to memory leaks:

```c++
class ABase {
public:
  ~ABase() {}
};

class A : public ABase {
  std::string *value_;
public:
  A() : value_(new std::string) {}
  ~A() { delete value_; }
};

void do_something() {
  ABase *instance = new A();
  //...
  delete instance;
}
```

So here we allocate a pointer to A, assign it to "instance" which is of type ABase, do something with it and finally delete it. It looks fine but we just leaked memory! When we called "delete instance" the code invoked the destructor ~ABase() and NOT the destructor ~A() . And value_ was not deleted and the memory leaked. Even if we'd used a scoped pointer to wrap value_ it would still have leaked.

The code should have said

```c++
class ABase {
public:
  virtual ~ABase() {}
};
```

The compiler didn't care our code was in error. It just allowed us to leak for the sake of a missing keyword.

## How Rust helps

Rust also does not use inheritance so problems like ABase above cannot exist. In Rust ABase would be declared as a trait that A implements.

```rust
trait ABase {
  //...
}

struct A {
  value: String,
}

impl ABase for A {
  //...
}
```

Rust also allows our struct to implement another trait called Drop which is equivalent to a C++ destructor.

```rust
impl Drop for A {
  fn drop(&mut self) {
    println!("A has been dropped!");
  }
}
```

It allows our code to do something during destruction such as to free an open resource, log a message or whatever.
