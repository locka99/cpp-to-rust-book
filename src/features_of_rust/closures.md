# Lambda Expressions / Closures

## Lambdas in C++

A lambda expression, or lambda is a C++11 feature for creating an anonymous function that can be declared and passed to a function from within the scope of the call itself. This can be particularly useful when you want to sort, filter, search or otherwise do some trivial small action without the bother of declaring a function and making it work.

In C++ a lambda looks like this:

```c++
float values[10] = { 9, 3, 2.1, 3, 4, -10, 2, 4, 6, 7 };
std::sort(values, values + 10, [](float a, float b) {
  return a < b;
});
```

Here we sort an array of values using a lambda to do the comparison.

A C++ lambda can (but doesn't have to) capture variables from the enclosing scope if it wishes and it can specify capture clauses in the [ ] section that define how capture is made. A lambda that captures variables effectively becomes a closure.

TODO capture clause

Prior to C++11 there was no lambda support however Boost provided a poor-man's version lambda function called a binding - basically a call to a function preloaded with arguments so it could be copied around and invoked.

TODO boost::bind

## Closures in Rust

Rust doesn't implement lambdas, it implements closures. What's the difference?

A lambda is an anonymous function and a closure is an anonymous function with access to its enclosing environment. So a closure is a form by which a lambda may be implemented.
Rust's lambdas have access to their enclosing environment so they are closures. When the closure is called it borrows the binding for any variable it accesses in that scope.

TODO closure example

If need be, ownership of variables can be moved to the closure. This may be necessary if the closure lives longer than the code around it does.
TODO move semantics
