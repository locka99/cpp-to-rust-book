# Variables

## C++

### Type Inference

C++11 implements type inference, previous versions of C++ do not. Type inference allows the programmer to assign a value to an `auto` typed variable and let the compiler infer the type based on the assignment. The
`auto` keyword is a convenience because it allows the compiler to figure out the type in a great deal of cases.

Boolean and numeric types are fairly easy to understand providing the code is as explicit as it needs to be.

```c++
auto x = true; // bool
auto y = 42;   // int
auto z = 100.; // double
```

Where C++ gets messy is for references, arrays and strings. Recall that strings are not primitive types in the strong sense within C or C++ so auto requires they be explicitly defined or the type will be wrong.

```c++
// C++11
auto s = std::string("Now is the window of our discontent"); // char string
// C++17 
auto s = "Now is the window of our discontent"s; // Notation to use a std::string
// Unicode 32-bit
auto s = U"Battle of Waterloo"; // char32_t pointer to UTF-32 string literal
```

Strings are covered elsewhere, but essentially there are many kinds of strings and C++/C has grown a whole bunch of string prefixes and suffixes to deal with them all.

Arrays are a more interesting problem. The `auto` keyword has no easy way to infer array type so is one hack workaround to assign a templatized array to an `auto` and coerce it.

```c++
template <typename T, int N> using raw_array = T[N];
auto a = raw_array<int, 5>{};
```

References are also a hidden danger. If you intend to assign a reference to an auto, you must ensure the auto is a reference too, otherwise you may inadvertantly make a copy of the thing
you are assigning.

```c++
std::vector<int> & dataPoints();

//...
auto points = dataPoints(); // Makes a copy
auto &points = dataPoints(); // References the original
```

## Rust

Rust, variables are bound with a `let` command. Binding is basically assignment, but with the added proviso that a variable can be unbound if its
value is moved somewhere else. Using an unbound variable results in a compiler error.

The `let` may specify the variable's type, or it may also use type inference to infer it from the value it is assigned with.

```rust
let x = true; // x: bool
let y = 42; // y: i32
let z = 100.0; // z: f64
let v = vec![10, 20, 30]; // v: Vec<i32>
let s = "Now is the winter of our discontent".to_string(); // s: String
let s2 = "Battle of Waterloo"; // s2: &str
let a1: [i32; 5] = [1, 2, 3, 4, 5];
```

Rust has no problem with using type inference in array assignments:

```rust
let a2 = ["Mary", "Fred", "Sue"];
```

Note that all array elements must be the same type, inference would generate a compiler error for an array like this:

```rust
// Compile error
let a3 = ["Mary", 32, true];
```

References in Rust are more obvious than in C++ - if you assign a reference to another variable, then the type is inferred to be a reference too.

```
let x = 10;   // i32
let rx = &x;  // &i32
let rx2 = rx; // &i32
```

## Scope rules

Scope rules in C, C++ and Rust are fairly similar - the scope that you declare the item determines its lifetime.


### Shadowing variables

One very useful feature of Rust is that you can declare the same named variable more than once in the same scope or nested scopes and the compiler doesn't mind. In fact you'll use this feature a lot. 

This is called *shadowing* and works like this:

```rust
let result = do_something();
println!("Got result {:?}", result);
if let Some(result) = result {
  println!("We got a result from do_something");
}
else {
  println!("We didn't get a result from do_something");
}

let result = do_something_else();
//...
```

This example uses the variable name `result` 3 times. First to store the result of calling `do_something()`, then to extract some value `Foo` from  `Option<Foo>` and a third time for calling something else. We could have assigned `result` to `result2` and then later on assigned the value `do_something_else()` to `result3` but we didn't need to because of shadowing. 

## Pointers

### In C++

A pointer is a variable that points to an address somewhere in memory. The pointer's _type_ indicates to the compiler what to expect at the address but there is no enforcement to ensure that the address actually holds that type. A pointer might might be assigned `NULL` \(or `nullptr` in C++11\) or may even be garbage if nothing was assigned to it.

```c++
char *name = "David Jones";

int position = -1;
find_last_index("find the letter l", 'l', &position);
```

Generally pointers are used in situations where references cannot be used, e.g. functions returning allocated memory or parent / child collection relationships where circular dependencies would prevent the use of references.

C++11 deprecates `NULL` in favour of new keyword `nullptr` to solve a problem with function overloading.

```c++
void read(Data *data);
void read(int value);
// Which function are we calling here?
read(NULL);
```

Since `NULL` is essentially `#define NULL 0` and 0 is an integer, we call the wrong function by accident. So C++ introduces an explicit `nullptr` for this purpose.

```c++
read(nullptr);
```

### In Rust:

Rust supports pointers, normally called *raw* pointers however you will rarely use them unless you need to interact with C API or similar purposes.

A pointer looks fairly similar to that of C++:

```rust
// This is a reference coerced to a const pointer
let age: u16 = 27;
let age_ptr: *const u16 = &age;

// This is a mut reference coerced to a mutable pointer
let mut total: u32 = 0;
let total_ptr: *mut u32 = &mut total;
```

Although you can make a pointer outside of an unsafe block, many of the functions you might want to perform on pointers are unsafe by definition and must be inside `unsafe` blocks.

The documentation in full is [here](https://doc.rust-lang.org/std/primitive.pointer.html).

## References

### In C++

A reference is also a variable that points to an address but unlike a pointer, it cannot be reassigned and it cannot be `NULL`. Therefore a reference is generally assumed to be safer than a pointer. It is still possible for the a reference to become dangling, assuming the address it referenced is no longer valid.

### In Rust

A reference is also lifetime tracked by the compiler.

## Tuples

A tuple is list of values held in parenthesis. They're useful in cases where transient or ad-hoc data is being passed around and you cannot be bothered to write a special struct just for that case. 

### In C++

C++ does not natively support tuples, but C++11 provides a template for passing them around like so:

```c++
#include <tuple>

std::tuple<int, int> get_last_mouse_click() {
  return std::make_tuple(100, 20);
}

std::tuple<int, int> xy = get_last_mouse_click();
int x = std::get<0>(xy);
int y = std::get<1>(xy);
```

### In Rust

Tuples are part of the language and therefore they're far more terse and easy to work with.

```rust
fn get_last_mouse_click() -> (i32, i32) {
  (100, 20)
}
// Either
let (x, y) = get_last_mouse_click();
println!("x = {}, y  = {}", x, y);
// or
let xy = get_last_mouse_click();
println!("x = {}, y  = {}", xy.0, xy.1);
```
