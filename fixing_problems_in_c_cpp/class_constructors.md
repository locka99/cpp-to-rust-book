# Explicit / Implicit Class Constructors

It's not just overloading that can be a mess. C++ has a bunch of rules about implicit / explicit type conversion for single argument constructors.

For example:

```c++
class MagicNumber {
public:
    MagicNumber(int value) {}
};

void magic(const MagicNumber &m) {
  //...
}

int main() {
    //...
    magic(2016);
    return 0;
}
```

The function magic takes a `const` reference to a MagicNumber but we passed an `int`. How did it even compile? Well our MagicNumber class has a constructor that takes an `int` so the compiler helpfully implicitly converted the value for us.

If we didn't want the implicit conversion (e.g. maybe it's horribly expensive to do this without knowing), then we'd have to tack an `explicit` keyword to the constructor to negate the behaviour.

```c++
explicit MagicNumber(int value) {}
```

It demonstrates an instance where the default behavior is probably wrong. The default *should* be `explicit` and if programmers want implicit they should be required to say it.

C++11 adds to the confusion by allowing classes to declare deleted constructors which are anti-constructors that generate an error instead of code if they match.

```c++
class MagicNumber {
public:
    MagicNumber(int value) {}
    MagicNumber(double value) = delete;
};

//...
magic(2016);   // OK
magic(2016.0); // ERROR
```

## How Rust helps

Rust does not have constructors and so there is no implicit conversion during construction. And since there is no implicit conversion there is no reason to have C++11 style function delete operators either. 

You must write explicit write "constructor" functions and call them explicitly. If you want to overload the function you can use `Into<>` patterns to achieve it.

For example we might write our `MagicNumber` constructor like this:

```rust
struct MagicNumber { /* ... */ }

impl MagicNumber {
  fn new<T>(value: T) -> MagicNumber where T: Into<MagicNumber> {
    value.into()
  }
}
```

We have said here that the `new()` function takes as its argument anything that type `T` which implements the trait `Into<MagicNumber>`.

So we could implement it for `i32`:

```rust
impl Into<MagicNumber> for i32 {
   fn into(self) {
     MagicNumber { /* ... */ }
   }
}
```

Now our client code can just call `new` and providing it provides a type which implements that trait our constructor will work:

```rust
   let magic = MagicNumber::new(2016);
   // But this won't work because f64 doesn't implement the trait
   let magic = MagicNumber::new(2016.0); 
```