# Types

## C++

C/C++ has primitive types for numeric values, floating point values and booleans. Strings will be dealt in a separate section.

### Integer types

Integer types (char, short, int, long) come in signed and unsigned versions.

A char is always 8-bits, but for historical reasons, the standards only guarantee the other types are "at least" a certain number of bits. So an "int" is ordinarily 32-bits but the standard only say it should be at least as large as a short, so potentially it could be 16-bits!

More recent versions of C and C++ include a [<stdint.h> or <cstdint.h>](http://www.cplusplus.com/reference/cstdint/) with typedefs that are unambiguous about their precision.

C/C++ compilers implement a *data model* that affects what width the standard types are.

The four data models in C++ are:

* LP32 - int is 16-bit, long and pointer are 32-bit. This is an uncommon model, a throw-back to DOS / Windows 3.1
* ILP32 - int, long and pointer are 32-bit. Used by Win32, Linux, OS X
* LLP64 - ints and long are 32-bit, long long and pointer are 64-bit. Used by Win64
* LP64 - int is 32-bit, long / long long and pointer are 64-bit. Used by Linux, OS X

Even though stdint.h can clear up the ambiguities, code frequently sacrifices correctness for terseness. It is not unusual to see an "int" used as a temporary incremental value in a loop:

```c++
string s = read_file();
for (int i = 0; i < s.size(); ++i) {
  //...
}
```

This loop is not using negative values so it shouldn't use a signed integer, but writing "int" is easier than writing "unsigned int", or "size_t" for that matter. While int is unlikely to fail for most loops in a modern compiler supporting ILP32 or greater, it is still technically wrong.

C/C++ types can also be needlessly wordy such as "unsigned long long int". Again, this sort of puffery encourages code to take short cuts, or bloat the code with typedefs or potentially use the wrong type altogether. The best action is of course to use stdint.h if it is available.

### Real types

C/C++ has float, double and long double precision floating point types. A floating point number can represent, with varying degrees of precision, real numbers including fractional portions.

* float
* double - "at least as much precision as a float"
* long double - "at least as much precision as a double"

The C and C++ standards are vague on what precision these values represent. In most compilers however a float is a 32-bit single precision value, and a double is an 64-bit double precision value. The most common machine representation is the [IEEE 754-2008 format](https://en.wikipedia.org/wiki/IEEE_floating_point).

The "[long double](https://en.wikipedia.org/wiki/Long_double)" has proven quite problematic for compilers. Despite expectations it is not normally a quadruple precision value. Some compilers such as gcc may offer 80-bit extended precision on x86 processors with a floating point unit but it is implementation defined behaviour. The Microsoft Visual C++ compiler treats it with the same precision as a double. Other architectures may treat it as quadruple precision. The fundamental problem with "long double" is that most desktop processors would not have the ability to perform 128-bit floating point operations in hardware so a compiler must implement code in software.

Some GPU C-derived shader languages may also support a "half" precision 16-bit float (for interpolating values between 0 and 1 for example) but it is not part of the C/C++ standard.

### Booleans

A boolean in C/C++ can have the value true or false, however it can be promoted to an integer (0 = false, 1, true) and even be incremented with ++ to be true although it cannot be decremented to false !?

Inverting true with a ! becomes false and vice versa.

```c++
!false == true
!true == false
```

## Rust

Rust benefits from integer types that unambiguously denote their signedness and width in their name.

They are also extremely terse making it easy to declare and use them. For example a u32 is an unsigned 32-bit integer. An i64 is a signed 64-bit integer.

Types may be inferred or explicitly prefixed to the value:

```rust
let v1 = 1000;
let v2 : u32 = 25;
let v3 = 126i8;
let v4 = 99.3333f64;
let v5 = v4 as f32;
let f1 = true;
```

The C/C++ data model affects what the equivalent type is for Rust in some cases.

| C/C++ | Rust | Notes
| --- | ---- | ---
| char  | i8 | A Rust char is not the same as a C/C++ char [^notechars]
| unsigned char  | u8 |
| signed short int | i16 |
| unsigned short int | u16 |
| (signed) int | i32 or i16 | Data model dependent.
| unsigned int | u32 or u16 | Data model dependent.
| (signed) long int | i32 or i64 | Data model dependent.
| unsigned long int | u32 or u64 | Data model dependent.
| (signed) long long int | i64 |
| unsigned long long int | u64 |
| size_t | usize | usize holds numbers as large as the address space [^usize] |
| float | f32 |
| double | f64 |
| long double | <s>f128</s> | f128 support was present in Rust but removed due to issues for some platforms in implementing it.
| bool | bool |

[^notechars] Rust's char type, is 4 bytes wide, enough to hold any Unicode character. This is equivalent to the belated char32_t that appears in C++11 to rectify the abused wchar_t type which on operating systems such as Windows is only 2 bytes. When you iterate strings in Rust you may do so either by character or u8, i.e. a byte.

[^usize] Rust has a specific numeric type for indexing on arrays and collections called usize. A usize is designed to be able to reference as many elements in an array as there is addressable memory. i.e. if memory is 64-bit addressable then usize is 64-bits in length.

Typically both C/C++ and Rust will share the same machine types for each corresponding language type, i.e.:

1. Signed types are two's complement
2. IEE 754-2008 binary32 and binary64 floating points for float and double precision types.

The stdint.h typedefs are directly analogous.

| C/C++ | Rust
| --- | ----
| int8_t | i8
| uint8_t | u8
| int16_t | i16
| uint16_t | u16
| uint32_t | u32
| int32_t | i32
| int64_t | i64
| uint64_t | u64

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
