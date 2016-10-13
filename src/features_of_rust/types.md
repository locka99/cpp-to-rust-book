# Types

C/C++ and Rust have mostly analogous primitive types:

| C/C++ | Rust | Notes
| --- | ----
| char32_t / wchar_t | char (4 bytes!) | See Note 1
| char | i8 |
| unsigned char | u8 |
| short (int) / signed short (int) | i16 |
| unsigned short (int) | u16 |
| int / signed int | i32 or i16 (LP32) | See Note 2
| unsigned int | u32 or u16 (LP32) | See Note 2
| float | f32 |
| double | f64 |
| size_t | usize / isize | size_t is a #define whereas usize / isize are part of the language and hold numbers as large as the address space |
| long (int) | i32 or i64 (LP64) | See Note 2
| unsigned long (int) | u32 or u64 (LP64) | See Note 2
| long long (int) | i64 |
| unsigned long long (int) | u64 |
| bool | bool |

Note 1: that Rust's char type, is 32-bits wide, enough to hold any Unicode character. This is equivalent to the belated char32_t that appeared in C++11. On some operating systems / compilers wchar_t may be 32-bits but you can't rely on that being true, because on Windows they are 16-bits. When you iterate strings in Rust you may either iterate by character or u8, i.e. a byte.

Note 2: There are various data models used by C/C++ compilers. The standard only says that shorts, ints and longs must be "at least" a certain bit length, not what that length must be. Therefore the size of a short, int, long could differ from one compiler and architecture to the next.

The four data models in C++ are:

* LP32 - int is 16-bit, long and pointer are 32-bit. This is an uncommon model, a throw-back to DOS / Windows 3.1
* ILP32 - int, long and pointer are 32-bit. Used by Win32, Linux, OS X
* LLP64 - ints and long are 32-bit, long long and pointer are 64-bit. Used by Win64
* LP64 - int is 32-bit, long / long long and pointer are 64-bit. Used by Linux, OS X

The recommended way to avoid these issues is to use the explicitly sized and signed typedefs in [<stdint.h>](http://www.cplusplus.com/reference/cstdint/) to avoid these issues. But random code may or may not do that or still get it wrong. e.g. code might coerce a pointer into a 32-bit int which won't work on a 64-bit platform.

Rust avoids the length ambiguity by explicitly spelling out the length of the type.

# Arrays

An array is a fixed size list of elements allocated either on the stack or the heap.

E.g to create a 100 element array of doubles in C++:

```c++
// Stack
double values[100];
// Heap
double *values = new double[100];
delete []values;
```

And in Rust:

```rust
// Stack
let mut values: [f64; 100] = [0f64; 100];
// Heap
let mut values: Box<[f64; 100]> = Box::new([0f64; 100]);
```

Note how Rust provides a shorthand to initialise the array with zeroes or any other value.


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
