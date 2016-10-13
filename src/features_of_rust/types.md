# Types

Rust has analogue types for the numeric primitives in C/C++.

Note that various C++ compilers support various data models and the standard only says that shorts, ints and longs must be "at least" a certain bit length. Therefore the rules change from one compiler and architecture to the next.

The four main models in C++ are:

* LP32 - int is 16-bit, long and pointer are 32-bit. This is an uncommon model, a throw-back to DOS / Windows 3.1
* ILP32 - int, long and pointer are 32-bit. Used by Win32, Linux, OS X
* LLP64 - ints and long are 32-bit, long long and pointer are 64-bit. Used by Win64
* LP64 - int is 32-bit, long / long long and pointer are 64-bit. Used by Linux, OS X

A basic conversion between types to Rust therefore follows

C++ | Rust
--- | ----
char | i8
unsigned char | u8
short (int) / signed short (int) | i16
unsigned short (int) | u16
int / signed int | i32 or i16 (LP32)
unsigned int | u32 or u16 (LP32)
float | f32
double | f64
size_t | usize / isize (most collections use usize)
long (int) | i32 or i64 (LP64)
unsigned long (int) | u32 or u64 (LP64)
long long (int) | i64
unsigned long long (int) | u64
bool | bool
char32_t / wchar_t | char (4 bytes!)

Note 1: that Rust's char type, is 32-bits wide, enough to hold any Unicode character. This is equivalent to the belated char32_t that appeared in C++11. On some operating systems / compilers wchar_t may be 32-bits but you can't rely on that being true, because on Windows they are 16-bits. When you iterate strings in Rust you may either iterate by character or u8, i.e. a byte.

# Arrays

An array is a fixed size list of elements allocated either on the stack or the heap.

E.g to create a 100 element array of doubles in C++:

```c++
double values[100];
```

And in Rust

```rust
let mut values = [f64; 100];
```

## Slices

A slice is a partial or full view of an array or a string. A slices is not a copy of the array, rather that it is a pointer to a portion of the array and a length.

```rust
let array = ["Mary", "Sue", "Bob", "Michael"];
println!("{:?}", array);
let slice = &array[2..];
println!("{:?}", slice);
```

This slice represents the portion of array starting from index 2.

```
["Mary", "Sue", "Bob", "Michael"]
["Bob", "Michael"]
```

## Functions of an array

One serious disadvantage of C++ arrays is there is no .len() method so if you want to specify the length then you either do something ugly like this:

```c++
const size_t num_elements = 1024;
char buffer[num_elements];
//...
// fill_buffer needs to be told how many elements there are
fill_buffer(buffer, num_elements);
```

You might also see code like this:

```c++
Element elements[100];
//...
int num_elements = sizeof(elements) / sizeof(Element);
```

In addition we can pass a slice of the array which might be the whole array or only a portion of it:

```rust
let buffer: [u8; 1024]
fill_buffer(&buffer);
//...
fn fill_buffer(elements: &[Element]) {
  println!("Number of elements = {}", elements.len());
}
```

A slice is the pointer to a part of the array and the length of the slice. It means we provide all the information it needs

TODO - lots of mess below

C++ | Rust
--- | ----
T [N] (e.g. char x[32]) or std::array<>, e.g. std::array<char, 32> | [T; N] (e.g. let x = [u8; 32])
