# Lambda Expressions / Closures

## Lambdas in C++11

A [lambda expression](https://msdn.microsoft.com/en-us/library/dd293608.aspx) is an anonymous function that can be declared and passed around from within the scope of the call itself.

A lambda can be particularly useful when you want to sort, filter, search or otherwise do some trivial small action without the bother of declaring and maintaining a separate function.

In C++ a lambda looks like this:

```c++
float values[10] = { 9, 3, 2.1, 3, 4, -10, 2, 4, 6, 7 };
std::sort(values, values + 10, [](float a, float b) {
  return a < b;
});
```

This lambda is passed to a std::sort function to sort an array of values by some criteria.

A C++ lambda can (but doesn't have to) capture variables from the enclosing scope if it wishes and it can specify capture clauses in the `[]` section that define how capture is made. Captures can be by value or reference, and can explicitly list the variables to capture, or specify to capture everything by reference or assignment. A lambda that captures variables effectively becomes a closure.

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

We can see from the output that `multiply()` has captured copies of the values in `v1` and `v2`, whereas `sum()` captures by reference and so it is sensitive to changes in the variables:

```
20
12
20
101
```

A capture can also specify a default capture mode by specifying `=` in the capture clause or by reference `&` and then specify capture behaviour for specific variables.

So our captures above could be simplified to:

```c++
// Capture by value
auto multiply = [=]() { return v1 * v2; };
// Capture by reference
auto sum = [&]() { return v1 + v2; };
```

Note that C++ lambdas can exhibit dangerous behaviour - if a lambda captures references to variables that go out of scope, the lambda's behaviour is undefined. In practice that could mean the application crashes.

## Closures in Rust

Rust implements closures. A closure is like a lambda except it automatically captures anything it references from the enclosing environment. i.e. by default it can access any variable that is in the enclosing scope.

However it is important to note that using captured variables also limits the closure to the lifetime of the variables it uses. i.e. We could not spawn a thread that uses variables from its enclosing environment because the thread could outlive them.

Here is the same sort snippet we saw in C++ expressed as Rust. This closure doesn't borrow anything from its enclosing scope but it does take a pair of arguments to compare two values for sorting. The `sort_by()` function repeatedly invokes the closure to sort the array.

```rust
use std::cmp::Ord;
let mut values = [ 9.0, 3.0, 2.1, 3.0, 4.0, -10.0, 2.0, 4.0, 6.0, 7.0 ];
values.sort_by(|a, b| a < b );
println!("values = {:?}", values);
```

A closure that uses a variable from the enclosing scope borrows it by default. That means the borrowed variable can't change while the closure is in scope. To change the value we must ensure the closure goes out of scope to free the borrow, e.g. with a block:

```rust
let mut x = 100;
{
  let square = || x * x;
  println!("square = {}", square());
}
x = 200;
```

Remember how the closure cannot exist outside the lifetime of the variables it uses from its enclosing environment. If this is a limitation you can `move` ownership of variables used by the closure so it owns a bitwise copy of them and they become inaccessible from the outerscope. Since our closure was accessing an integer, the move becomes an implicit copy. So our `square` closure has its own `x` assigned the value `100`. Even if we change `x` in the outer scope to `200`, the closure has its own independent copy.

```rust 
let mut x = 100;
let square = move || x * x;
println!("square = {}", square()); // 10000
x = 200;
println!("square = {}", square()); // 10000
```

This is the equivalent to the C++ code above that used lambda expressions to bind to copies and references:

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

This will yield the same results as the C++ code. The main difference here is that rather than binding our closure to a reference, we passed the reference values in as parameters to the closure.
