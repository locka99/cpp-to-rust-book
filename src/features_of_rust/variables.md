# Variables

## C++

### Type Inference

C++11 has type inference, previous versions of C++ do not. Type inference allows the compiler to figure out the type of a variable based upon the value being assigned to it.

Boolean and numeric types are fairly easy to understand providing you are explicit as you need to be.

Where C++ gets really messy is for arrays and strings. Recall that strings are not primitive types in the strong sense within C or C++.

```c++
auto x = true; // bool
auto y = 42;   // int
auto z = 100.; // double
auto s = std::string("Now is the window of our discontent"); // char string
auto s = U"Battle of Waterloo"; // char32_t pointer to UTF-32 string literal
```

Strings are covered elsewhere, but essentially there are many kinds of strings and C++/C has grown a whole bunch of string prefixes to deal with them all.

Arrays are a more interesting problem. The auto keyword has no easy way to infer array type so is one hack workaround to assign a templatized array to an auto and coerce it.

```c++
template <typename T, int N> using raw_array = T[N];
auto a = raw_array<int, 5>{};
```

## Rust

Rust, variables are declared with a let command. The let may specify the variable's type, or it may also use type inference to infer it from the value it is assigned with.

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

TODO

## Pointers

### In C++

A pointer is a variable that points to an address in memory. The pointer may be assigned to point somewhere else, and it might be assigned NULL. Since a pointer may be NULL, or set to point to garbage, as well as suffering dangling pointer issues, it is only used when a reference is not possible.

### In Rust:

The use of raw pointers tend to be reserved for a few use cases such as interacting with C/C++ where allocated memory may need to be extracted from a Box or placed into a Box outside the normal lifetime rules. Normally it is sufficient to use references only.

## References

### In C++
A reference is also a variable that points to an address but unlike a pointer, it cannot be reassigned and it cannot be NULL. Therefore a reference is generally assumed to be safer than a pointer. It is still possible for the a reference to become dangling, assuming the address it referenced is no longer valid.

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
