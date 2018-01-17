# Lambda Expressions / Closures

## Lambdas in C++

A lambda expression, or lambda is a feature introduced in C++11 for creating an anonymous function that can be declared and passed around from within the scope of the call itself.

This can be particularly useful when you want to sort, filter, search or otherwise do some trivial small action without the bother of declaring a function and making it work.

In C++ a lambda looks like this:

```c++
float values[10] = { 9, 3, 2.1, 3, 4, -10, 2, 4, 6, 7 };
std::sort(values, values + 10, [](float a, float b) {
  return a < b;
});
```

This lambda is passed to a std::sort function to sort an array of values by some criteria.

A C++ lambda can (but doesn't have to) capture variables from the enclosing scope if it wishes and it can specify capture clauses in the [ ] section that define how capture is made. Captures can by value or reference, and can explicitly list the variables to capture, or specify to capture everything by reference or assignment. A lambda that captures variables effectively becomes a closure.

```c++
auto v1 = 10.;
auto v2 = 2.;
// Capture by value
auto multiply = [v1, v2]() { return v1 * v2; };
// Capture by reference
auto sum = [&v1, &v2]() { return v1 + v2; };
cout << multiply() << endl;
cout << sum() << endl;
v1 = 99; // Now v1 in sum() references 99
cout << multiply() << endl;
cout << sum() << endl;
```

We can see from the output that multiply() has captured immutable copies of the values, whereas sum() is sensitive to changes to the variables:

```
20
12
20
101
```

Note that C++ lambdas can exhibit dangerous behaviour - if a lambda captures references to variables that go out of scope, the lambda's behaviour is undefined. In  practice that could mean the application crashes.

## Closures in Rust

Rust implements closures. A closure is a lambda with access to its enclosing environment. i.e. by default it can access any variable that is in the function it was declared within however it then owns that variable.

Here is a sorting algorithm similar to the one in C++.

```rust
use std::cmp::Ord;
let mut values = [ 9.0, 3.0, 2.1, 3.0, 4.0, -10.0, 2.0, 4.0, 6.0, 7.0 ];
values.sort_by(|a, b| a.partial_cmp(b).unwrap());
println!("values = {:?}", values);
```

But a closure can access a variable around it and borrow ownership. Borrowing means I can't change the value to something else because the closure relies on the value not changing. If I want to change the value I have to ensure the closure goes out of scope to free the borrow:

```rust
let mut x = 100;
{
  let square = || x * x;
  println!("square = {}", square());
}
x = 200;
```

Alternatively you can `move` variables used by the closure so it owns them. Since our closure is using an integer, move implies copy so our `square` closure has its own `x` assigned the value `100`. Even if we change `x` to `200`, the function will still yield the same value

```rust 
let mut x = 100;
let square = move || x * x;
println!("square = {}", square()); // 10000
x = 200;
println!("square = {}", square()); // 10000
```

So a closure is basically a lambda that captures everything around it.

This is the equivalent to the C++ code above:

```rust
let mut v1 = 10.0;
let v2 = 2.0;
let multiply = move || v1 * v2;
let sum = |x: &f64, y: &f64| x + y;
println!("multiply {}", multiply());
println!("sum {}", sum(&v1, &v2));
v1 = 99.0;
println!("multiply {}", multiply());
println!("sum {}", sum(&v1, &v2));
```

This will yield the same results as the C++ code. The only difference was that rather than binding a lambda to references, we passed the reference values in as parameters to the closure.
