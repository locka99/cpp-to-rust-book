# Variables

## C++

### Type Inference

C++11 has type inference, previous versions of C++ do not. Type inference allows the programmer to assign a value to an `auto` typed variable and let the compiler infer the type based on the assignment.

Boolean and numeric types are fairly easy to understand providing the code is as explicit as it needs to be.

```c++
auto x = true; // bool
auto y = 42;   // int
auto z = 100.; // double
```

Where C++ gets really messy is for arrays and strings. Recall that strings are not primitive types in the strong sense within C or C++ so auto requires they be explicitly defined or the type will be wrong.

```c++
auto s = std::string("Now is the window of our discontent"); // char string
auto s = U"Battle of Waterloo"; // char32_t pointer to UTF-32 string literal
```

Strings are covered elsewhere, but essentially there are many kinds of strings and C++/C has grown a whole bunch of string prefixes to deal with them all.

Arrays are a more interesting problem. The `auto` keyword has no easy way to infer array type so is one hack workaround to assign a templatized array to an `auto` and coerce it.

```c++
template <typename T, int N> using raw_array = T[N];
auto a = raw_array<int, 5>{};
```

## Rust

Rust, variables are declared with a `let` command. The `let` may specify the variable's type, or it may also use type inference to infer it from the value it is assigned with.

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

## Scope rules

Scope rules in C, C++ and Rust are fairly similar - the scope that you declare the item determines its lifetime.

Note that Rust allows variables to be shadowed, i.e. you can declare the same variable more than once in a block and the second declaration masks the first.

## Pointers

### In C++

A pointer is a variable that points to an address somewhere in memory. The pointer's _type_ indicates to the compiler what to expect at the address but there is no enforcement to ensure  
that the address actually holds that type. A pointer might might be assigned NULL \(or nullptr in C++11\) or may even be garbage if nothing was assigned to it.

```c++
char *name = "David Jones";

int position = -1;
find_last_index("find the letter l", 'l', &position);
```

Generally pointers are used in situations where references cannot be used, e.g. functions returning allocated memory or parent / child collection relationships where circular dependencies would prevent the use of references.

C++11 attempts to deprecate `NULL` in favour of new keyword `nullptr` to make its intent more explicit and allow the compiler to recognize it.

### In Rust:

Rust supports pointers but only for interacting with external libraries or the operating system. As such if you wish to use them, they are available from an `unsafe` code programming block, and  [documented](https://doc.rust-lang.org/std/primitive.pointer.html).

In safe programming you would not use pointers. Instead you would use references and / or optional reference counted objects.

## References

### In C++

A reference is also a variable that points to an address but unlike a pointer, it cannot be reassigned and it cannot be `NULL`. Therefore a reference is generally assumed to be safer than a pointer. It is still possible for the a reference to become dangling, assuming the address it referenced is no longer valid.

### In Rust

A reference is also lifetime tracked by the compiler.

## Tuples

A tuple is bunch of values held in parenthesis. Functions may return a tuple too.

```rust
fn get_last_mouse_click() -> (i32, i32) {
  (100, 20)
}
let (x, y) = get_last_mouse_click();
```



