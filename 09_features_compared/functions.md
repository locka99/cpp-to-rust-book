# Functions

In C++ the standard form of a function is this:

```c++
// Declaration
int foo(bool parameter1, const std::string &parameter2);

// Implementation
int foo(bool parameter1, const std::string &parameter2) {
  return 1;
}
```

Usually you would declare the function, either as a forward reference in a source file, or in a header. Then you would implement the function in a source file. 

If a function does not return something, the return type is `void`. If the function does return something, then there should be return statements for each exiting branch within the function.

You can forego the function declaration in two situations:

1. If the function is inline, i.e. prefixed with the `inline` keyword. In which case the function in its entireity is declared and implemented in one place. 
2. If the function is not inline but is declared before the code that calls it in the same source file. So if function `foo` above was only used by one source file, then just putting the implementation into the source would also act as the declaration

In Rust the equivalent to `foo` above is this:

```rust
fn foo(parameter1: bool, parameter2: &str) -> i32 {
  // implementation
  1
}
```

The implementation *is* the declaration there is no separation between the two. Functions that return nothing omit the `->` return section. The function can also be declared before or after whatever calls it. By default the function is private to the model (and submodules) that implement it but making it `pub fn` exposes it to other modules.

Like C++, the function must evaluate to something for each exiting branch but this is mandatory. 

Also note, that the `return` keyword is not usually unecessary. Here is a function that adds two values together and returns them with no return:

```rust
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

Why is there no `return`? As we saw in the section on Expressions, a block evaluates to a value if we omit the semi-colon from the end so `x + y` is the result of evaluating the function block and becomes what we return.

There are occasions were you explicitly need the return keyword. Typically you do that if you want to exit the function before you get to the end of the function block:

```rust
fn process_data(number_of_times: ui32) -> ui32 {
  if number_of_times == 0 {
    return 0;
  }
  let mut result : ui32 = 0;
  for i in number_of_times {
    result += i;
  }
  result
}
```

## Variable arguments

C++ functions can take a variable number of arguments with the ... ellipsis pattern. This is used in functions such as print, scanf etc.

```c++
void printf_like(const char *pattern, ...);
```

Rust does not support variadic functions (the fancy name for this ability). However you could pass additional arguments in an array slice if the values are the same, or as a dictionary or a number of other ways.  

TODO Rust example of array slice

Another option is to write your code as a macro. Macros can take any number of expressions so you are able to write code that takes variable arguments. This is how macros such println!, format! and vec! work.

## Default arguments

C++ arguments can have default values.

```c++
std::vector<Record> fetch_database_records(int number_to_fetch = 100);
```

A function defines what its name is, what types it takes and what value (if any) it returns.

## Function overloading

C++ functions can be overloaded, e.g.

```c++
std::string to_string(int x);
std::string to_string(float x);
std::string to_string(bool x);
```

Rust does not support overloading. As an alternative, each variation of the function would have to be named uniquely.

## C++11 alternative syntax

C++11 introduces a new syntax which is slightly closer to Rust's in style.

```c++
auto function_name(type parameter1, type parameter2, ...) -> return-type;
```

This form was created to allow C++ function declarations to more closely to resemble lambda functions in some scenarios and to help with decltype return values.
