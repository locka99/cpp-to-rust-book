# Functions

In C++ the standard form of a function is this:

```c++
int function_name(bool parameter1, const std::string &parameter2);
```

A function returns _something _ or it is marked void (returns nothing). The parameters to a C++ function and the return type can be the standard mix of values, pointers, references.  A function can be declared and implemented in a head as an inline function, or it may be defined in the header and implemented in the source file.

In Rust a function is like this:

```rust
fn function_name(parameter1: bool, parameter2: &str) -> i32 {
  // implementation
}
```

This Rust function is equivalent to the one we wrote in C++. The parameter types and the return type must be specified. Note how the return type is after the function, not before it.

The declaration and the implementation are one and the same thing. So unlike in C or C++ the function is defined and implemented simultaneously. 

Here is a function that adds two values together and returns them:

```rust
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

Why is there no return call? As we saw in the section on Expressions, a block can have a return value if we omit the semi-colon from the end so x + y is the result of evaluating the function block and becomes what we return.

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

Rust does not support variadic functions. However you could pass additional arguments in an array slice if the values are the same, or as a dictionary or a number of other ways.  

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
