# Type Mismatching

Consider two methods. Both are called evaluate() and they are overloaded. The main() method calls evaluate("Hello world"). What version is called in the compiled code?

```c++
#include <iostream>
#include <string>

using namespace std;

void evaluate(bool value) {
    cout << "Evaluating a bool " << value << endl;
}

void evaluate(const std::string &value) {
    cout << "Evaluating a string " << value << endl;
}

int main() {
    evaluate("Hello world");
    return 0;
}
```

It may surprise you to know that the bool version is called and the compiler doesn't even complain about it either:

```
Evaluating a bool 1
```

This is an example of bad type inference. A string literal (a char *) should be turned into a std::string (a C++ string has a constructor that takes char *) but the compiler chose to treat it as a bool instead.

On other occasions the compiler might spot ambiguity and complain but the blurred lines between types in C++ combined with overloading lead to errors:
Here is another example where the compiler is a little more useful by generating an error, but in doing so it demonstrates the limits of overloading

```c++
bool evaluate(bool value);
bool evaluate(double value);
```

These overloaded methods should be distinct but they're not distinct enough as far as the compiler is concerned.

In summary, blurred and confusing rules about types in C++ can cause unexpected errors that can propagate to runtime.

## How Rust helps

In Rust the functions cannot be overloaded in this manner.

Rust is also more strict about type coercion - if you have a bool you cannot pass it to a function that takes an integer.

Nor can you pass an integer of one size to a function taking an integer of another size.

```rust
fn print_i32(value: i32) {
   println!("Your value is {}", value);
}
let value = 20i16; // 16-bit int
print_i32(value);
```

This will yield an error:

```
error[E0308]: mismatched types
  |
7 | print_i32(value);
  |           ^^^^^ expected i32, found i16
```

You must use an explicit numeric cast to turn the value into the type the function expects:

```rust
print_i32(value as i32);
```
