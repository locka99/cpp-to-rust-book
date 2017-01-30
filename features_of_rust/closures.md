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

Rust implements closures. A closure is a lambda with access to its enclosing environment. i.e. by default it can make reference to any variable that is in the function it was declared within. So a closure is basically a lambda that captures everything around it.

Unlike a C++ capture however, the closure is directly referencing the outer variables and is subject to the same lifetime & borrowing rules that any other code is.

TODO closure example

If need be, ownership of variables can be moved to the closure. This may be necessary if the closure lives longer than the code around it does.

TODO move semantics
