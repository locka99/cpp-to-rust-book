# Polymorphismtion

C++ allows functions and methods including constructors to be polymorphic.

That is to say, the same named function can be overloaded with different parameters. 

```c++
class Variant {
public:
  void set(bool value);
  void set(int value);
  void set(float value);
  void set(Array *value);
};
```

One of the biggest issues that you might begin to see from the above example is that is too easy to inadvertantly call the wrong function because C++ will also coerce types. 

```c++

// Sample code
Variant v;
//...
v.set(NULL);
```

This example will call the integer overload because `NULL` evaluates to 0. One of the changes to `C++11` was to introduce an explicit `nullptr` value and type to avoid this issues.

##Ri# 

Rust does not support polymorphism. While there are valid reasons that it doesn't, it can still be very painful, especially if you have classes or functions that you need to be called with different arguments.

The most annoying case would be for constructors, where there are different ways to construct a class. The naive workaround is to produce functions which are unique so they do not collide:

```rust
fn new(name: &str);
fn new_age(name: &str, age: u16);
```

Another way you can do this is with _traits_. A standard trait is called Into&lt;T&gt; where T is the type you wish to convert from. Our struct can implement the Into trait multply for set of

```rust
impl Into<&str> for Foo {    
    fn into(v: &str) -> Foo {    
        //...    
    }    
}

impl Into<(&str, u16)> for Foo {    
    fn into(v: (&str, u16)) -> Foo {    
        //...    
    }    
}
```

Arguably this is a pretty gnarly workaround, but it does demonstrate that sometimes Rust can achieve 

