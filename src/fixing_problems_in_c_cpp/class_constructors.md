# Explicit / Implicit Class Constructors

It's not just overloading that be a mess. C++ has a bunch of rules about implicit / explicit type conversion for single argument constructors.

For example this would work:

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

The function magic takes a const reference to a MagicNumber but we passed an int. How did it work? Well our MagicNumber class has a constructor that takes an int so the compiler helpfully implicitly converted the value for us.

If we didn't want the implicit conversion (e.g. maybe it's horribly expensive to do this without knowing), then we'd have to tack an "explicit" keyword to the constructor to basically turn off that behaviour.

```c++
explicit MagicNumber(int value) {}
```

It demonstrates that the default behavior is probably wrong. The default should be explicit and if programmers want implicit they should be required to say it.
C++11 adds to the confusion by allowing classes to declare deleted constructors which generate an error instead of code if they match.

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

Rust does not have constructors and therefore no implicit conversion.

The only form of implicit coercion it has is if for mutable references and certain kinds of raw pointers.
