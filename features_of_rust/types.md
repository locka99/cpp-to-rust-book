# Types

## Data model

C/C++ compilers implement a *data model* that affects what the width of standard types are. The general rule is that:

`1 == sizeof(char) <= sizeof(short) <= sizeof(int) <= sizeof(long) <= sizeof(long long)`

The four common data models in C++ are:

* LP32 - `int` is 16-bit, `long` and pointers are 32-bit. This is an uncommon model, a throw-back to DOS / Windows 3.1
* ILP32 - `int`, `long` and pointers are 32-bit. Used by Win32, Linux, OS X
* LLP64 - `int` and `long` are 32-bit, `long long` and pointers are 64-bit. Used by Win64
* LP64 - `int` is 32-bit, `long` / `long long` and pointers are 64-bit. Used by Linux, OS X

As you can see, potentially everything all the way to `long long` could be a single byte, or there could be some other crazy definition. In practice however software expects one of the models above.

## Comparing C/C++ types to Rust

For this section, we'll cover the _most likely_ analogous types between Rust and C/C++.

| C/C++ | Rust | Notes
| --- | ---- | ---
| `char` | `i8` (or `u8`) | The signedness of a C++ char can be signed or unsigned - the assumption here is signed but it varies by target system.<br>A Rust `char` is not the same as a C/C++ `char` since it can hold any Unicode character. [^1]
| `unsigned char` | `u8` |
| `signed char` | `i8` |
| `short int` | `i16` |
| `unsigned short int` | `u16` |
| `(signed) int` | `i32` or `i16` | In C/C++ this is data model dependent [^2]
| `unsigned int` | `u32` or `u16` | In C/C++ this is data model dependent [^2]
| `(signed) long int` | `i32` or `i64` | In C/C++ this is data model dependent [^2]
| `unsigned long int` | `u32` or `u64` | In C/C++ this is data model dependent [^2]
| `(signed) long long int` | `i64` |
| `unsigned long long int` | `u64` |
| `size_t` | `usize` | usize holds numbers as large as the address space [^3] |
| `float` | `f32` |
| `double` | `f64` |
| `long double` | <s>f128</s> | f128 support was present in Rust but removed due to issues for some platforms in implementing it.
| `bool` | `bool` |
| `void` | `()` | The unit type (see below)

[^1] Rust's `char` type, is 4 bytes wide, enough to hold any Unicode character. This is equivalent to the belated `char32_t` that appears in C++11 to rectify the abused C++98 `wchar_t` type which on operating systems such as Windows is only 2 bytes wide. When you iterate strings in Rust you may do so either by character or `u8`, i.e. a byte.

[^2] See the next section to for a discussion on data models.

[^3] Rust has a specific numeric type for indexing on arrays and collections called `usize`. A `usize` is designed to be able to reference as many elements in an array as there is addressable memory. i.e. if memory is 64-bit addressable then usize is 64-bits in length. There is also a signed `isize` which is less used but also available.

### stdint.h / cstdint

C provides a `<stdint.h>` header that provides unambigious typedefs with length and signedess, e.g. `uint32_t`. The equivalent in C++ is `<cstdlib>`.

If you use the types defined in this header file the types become directly analogous and unambiguous between C/C++ and Rust.

| C/C++ | Rust
| --- | ----
| `int8_t` | `i8`
| `uint8_t` | `u8`
| `int16_t` | `i16`
| `uint16_t` | `u16`
| `uint32_t` | `u32`
| `int32_t` | `i32`
| `int64_t` | `i64`
| `uint64_t` | `u64`

### Machine types under the covers

C/C++ and Rust will share the same machine types for each corresponding language type and the same compiler / backend technology, i.e.:

1. Signed types are two's complement
2. IEE 754-2008 binary32 and binary64 floating points for float and double precision types.

## Integer types

### C++

C/C++ has primitive types for numeric values, floating point values and booleans. Strings will be dealt in a separate section.

Integer types (`char`, `short`, `int`, `long`) come in `signed` and `unsigned` versions.

A `char` is always 8-bits, but for historical reasons, the standards only guarantee the other types are "at least" a certain number of bits. So an `int` is ordinarily 32-bits but the standard only say it should be at *least as large* as a `short`, so potentially it could be 16-bits!

More recent versions of C and C++ provide a [`<cstdint>`](http://www.cplusplus.com/reference/cstdint/) (or `<stdint.h>` for C) with typedefs that are unambiguous about their precision.

Even though `<stdint.h>` can clear up the ambiguities, code frequently sacrifices correctness for terseness. It is not unusual to see an `int` used as a temporary incremental value in a loop:

```c++
string s = read_file();
for (int i = 0; i < s.size(); ++i) {
  //...
}
```

While `int` is unlikely to fail for most loops in a modern compiler supporting ILP32 or greater, it is still technically wrong. In a LP32 data model incrementing 32767 by one would become -32768 so this loop would never terminate if `s.size()` was a value greater than that.

But look again at this snippet. What if the file read by `read_file()` is outside of our control. What if someone deliberately or accidentally feeds us a file so large that our loop will fail trying to iterate over it? In doing so our code is hopelessly broken.

This loop should be using the same type returned from `string::size()` which is an opaque unsigned integer type called `size_type`. This is usually a typedef for `std::size_t` but not necessarily. Thus we have a type mismatch. A `string` has an iterator which could be used instead but perhaps you need the index for some reason, but it can messy:

```c++
string s = read_file();
for (string::iterator i = s.begin(); i != s.end(); ++i) {
  string::difference_type idx = std::distance(s.begin(), i);
  //...
}
```

Now we've swapped from one opaque type `size_type` to another called `difference_type`. Ugh.

C/C++ types can also be needlessly wordy such as `unsigned long long int`. Again, this sort of puffery encourages code to make bad assumptions, use a less wordy type, or bloat the code with typedefs.

## Rust

Rust benefits from integer types that unambiguously denote their signedness and width in their name - `i16`, `u8` etc.

They are also extremely terse making it easy to declare and use them. For example a `u32` is an unsigned 32-bit integer. An `i64` is a signed 64-bit integer.

Types may be inferred or explicitly prefixed to the value:

```rust
let v1 = 1000;
let v2 : u32 = 25;
let v3 = 126i8;
```

Rust also has two types called `usize` and `isize` respectively. These are equivalent to `size_t` in that they are as large enough to hold as many elements as there is addressable memory. So in a 32-bit operating system they will be 32-bits in size, in a 64-bit operating system they will be 64-bits in size.

Rust will not implicitly coerce an integer from one size to another without explicit use of the `as` keyword.

```
let v1 = 1000u32;
let v2: u16 = v1 as u16;
```

## Real types

### C++

C/C++ has float, double and long double precision floating point types and they suffer the same vagueness as integer types.

* `float`
* `double` - "at least as much precision as a `float`"
* `long double` - "at least as much precision as a `double`"

In most compilers and architectures however a float is a 32-bit single precision value, and a double is an 64-bit double precision value. The most common machine representation is the [IEEE 754-2008 format](https://en.wikipedia.org/wiki/IEEE_floating_point).

#### Long double

The [`long double`](https://en.wikipedia.org/wiki/Long_double) has proven quite problematic for compilers. Despite expectations that it is a quadruple precision value it usually isn't. Some compilers such as gcc may offer 80-bit extended precision on x86 processors with a floating point unit but it is implementation defined behaviour.

The Microsoft Visual C++ compiler treats it with the same precision as a `double`. Other architectures may treat it as quadruple precision. The fundamental problem with `long double` is that most desktop processors do not have the ability in hardware to perform 128-bit floating point operations so a compiler must either implement it in software or not bother.

#### Math functions

The `<math.h>` C header provides math functions for working with different precision types.

```c++
#include <math.h>

const double PI = 3.1415927;
double result = cos(45.0 * PI / 180.0);
//..
double result2 = abs(-124.77);
//..
float result3 = sqrtf(9.0f);
//
long double result4 = powl(9,10);
```

Note how different calls are required according to the precision, e.g. sinf, sin or sinl. C99 supplies a "type-generic" set of macros in `<tgmath.h>` which allows sin to be used regardless of type.

C++11 provides a `<cmath>` that uses specialised inline functions for the same purpose:

```c++
#include <cmath>
float result = std::sqrt(9.0f);
```

### Rust

Rust implements two floating point types - `f32` and `f64`. These would be analogous to a 32-bit `float` and 64-bit `double` in C/C++.

```rust
let v1 = 10.0;
let v2 = 99.99f32;
let v3 = -10e4f64;
```

Unlike in C/C++, the math functions are directly bound to the type itself providing you properly qualify the type.

```rust
let result = 10.0f32.sqrt();
//
let degrees = 45.0f64;
let result2 = angle.to_radians().cos();
```

Rust does not have a 128-bit double. A `f128` did exist for a period of time but was removed to portability, complexity and maintenance issues. Note how `long double` is treated (or not) according to the compiler and target platform. 

At some point Rust might get a f128 or f80 but at this time does not have such a type.

## Booleans

A `bool` (boolean) type in C/C++ can have the value `true` or `false`, however it can be promoted to an integer type (0 == `false`, 1 == `true`) and a bool even has a ++ operator for turning false to true although it has no -- operator!?

But inverting true with a ! becomes false and vice versa.

```c++
!false == true
!true == false
```

Rust also has a `bool` type that can have the value `true` or `false`. Unlike C/C++ it is a true type with no promotion to integer type

## void / Unit type

C/C++ uses `void` to specify a type of nothing or an indeterminate pointer to something.

```c++
// A function that doesn't return anything
void delete_directory(const std::string &path);

// Indeterminate pointer use
struct file_stat {
  uint32_t creation_date;
  uint32_t last_modified;
  char file_name[MAX_PATH + 1];
};

// malloc returns a void * which must be cast to the type need
file_stat *s = (file_stat *) malloc(sizeof(file_stat));
// But casting is not required when going back to void *
free(s);
```

The nearest thing to `void` in Rust is the Unit type. It's called a Unit type because it's type is `()` and it has one value of `()`.

Technically `void` is absolutely nothing and `()` is a single value of type `()` so they're not analogous but they serve a similar purpose.

When a block evaluates to nothing it returns `()`. We can also use it in places where we don't care about one parameter. e.g. say we have a function `do_action()` that succeeds or fails for various reasons. We don't need any payload with the Ok response so specify `()` as the payload of success:

```rust
fn do_action() -> Result<(), String> {
 //...
 Result::Ok(())
}

let result = do_action();
if result.is_ok() {
 println!("Success!");
}
```

### Empty enums

Rust *does* have something closer (but not the same as) `void` - empty enumerations.

```rust
enum Void {}
```

Essentially this enum has no values at all so anything that assigns or matches this nothing-ness is unreachable and the compiler can issue warnings or errors. If the code had used `()` the compiler might not be able to determine this.

## Tuples

A tuple is a collection of values of the same or different type passed to a function or returned by one as if it were a single value.

C/C++ has no concept of a tuple primitive type, however C++11 can construct a tuple using a template:

```c++
std::tuple<std::string, int> v1 = std::make_tuple("Sally", 25);
//
std::cout << "Name = " << std::get<0>(v1)
          << ", age = " << std::get<1>(v1) << std::endl;
```

Rust supports tuples as part of its language:

```rust
let v1 = ("Sally", 25);
println!("Name = {}, age = {}", v1.0, v1.1);
```

As you can see this is more terse and more useful. Note that the way a tuple is indexed is different from an array though, values are indexed via .0, .1 etc.

Tuples can also be returned by functions and assignment operators can ignore tuple members we're not interested in.

```rust
let (x, y, _) = calculate_coords();
println!("x = {}, y = {}", x, y);
//...
pub fn calculate_coords() -> (i16, i16, i16) {
  (11, 200, -33)
}
```

In this example, the calculate_coords() function returns a tuple containing three `i16` values. We assign the first two values to `x` and `y` respectively and ignore the third by passing an underscore. The underscore tells the compiler we're aware of the 3rd value but we just don't care about it.

Tuples can be particularly useful with code blocks. For example, let's say we want to get some values from a piece of code that uses a guard lock on a reference counted service. We can lock the service in the block and return all the values as a tuple to the recipients outside of the block:

```rust
let protected_service: Arc<Mutex<ProtectedService>> = Arc::new(Mutex::new(ProtectedService::new()));
//...
let (host, port, url) = {
  // Lock and acquire access to ProtectedService
  let protected_service = protected_service.lock().unwrap();
  let host = protected_service.host();
  let port = protected_service.port();
  let url = protected_service.url();
  (host, port, url)
}

```

This code is really neat - the lock allows us to obtain the values, the lock goes out of scope and the values are returned in one go.

## Arrays

An array is a fixed size list of elements in a contiguous memory location that can be referenced by an index. Arrays can be allocated either on the stack or the heap.

E.g to create a 100 element array of `double` values in C++ / C using the language features:

```c++
// Stack (uninitialized)
double values[100]; // ?,?,?,?,?,...
// Stack with assignment
double values[100] = [1, 2, 3]; // 1,2,3,?,?,?,?,...
// Heap
double *values = new double[100]; // ?,?,?,?,?,...
delete []values;
// C99 initialized arrays
double values[100] = { }; // 0,0,0,0,0,...
double values[100] = {1, 2, 3}; // 1,2,3,0,0,0,0...
// C99 initialized arrays with designators
double values[100] = {1, 2, 3, [99] = 99}; // 1,2,3,0,0,0,...,0,99
// C++ doesn't need the assignment
double values[100] {1, 2, 3}; // 1,2,3,0,0,0,0...
```

As can be seen, arrays have evolved a lot to resolve issues using uninitialized data but it is also leads to a lot of variation in how they are defined. Designators can be be incredibly powerful.

The language also doesn't help you know what the size of an array is, so you will often see code like this:

```c++
// Number of elements is the size of the entire array divided by the size of one element
int len = sizeof(values) / sizeof(values[0]);
```

But this isn't the end of it because C++ also defines `std::array` which is slightly more convenient for having `size()`, `empty()`, `begin()`, `end()` etc. making it similar to other kinds of collection:

```c++
#include <array>
//...
std::array values {1, 2, 3};
for (int i = 0; i < values.size(); i++) {
  //...
}
```

Rust has a less powerful syntax than is possible with initialized arrays in C++ but it is also less ambiguous:

```rust
// Stack
let values = [0f64; 100]; // 100 elements initialised to 0
let values = [1f64, 2f64, 3f64]; // 3 elements 1,2,3
// Heap
let values = Box::new([0f64; 100]);
```

Note how Rust provides a shorthand to initialise the array with the same value or assigns the array with every value. Initialisation in C and C++ is optional but it is more expressive in that portions of the array can be set or not set using enclosed list syntax.

But Rust *forces* you to initialise an array to something, ensuring the content of the array is predictable. Attempting to declare an array without assigning it a value is a compiler error.

In addition, a Rust array coerces to be a slice `&[T]`, so methods like `len()`, `is_empty()`, `get()`, `swap()`, `reverse()` are all instantly available:

```rust
// Reverse the order of values in this array in-place
let mut values = [1, 2, 3, 4];
values.reverse();
println!("Values = {:?}", values);
```

### Multi-dimensional arrays

## Slices

A slice is a runtime view of a part of an array or string. A slice is not a copy of the array / string rather that it is a reference to a portion of it. The reference holds a pointer to the starting element and the number of elements in the slice. 

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

### Size of the array 

C and C++ basically give no easy way to know the length of the array unless you encapsulate the array with a `std::array` or happen to remember it from the code that declares it.

```c++
// C++11
std::array<Element, 100> elements;
std::cout << "Size of array = " << elements.size() << std::endl;
```

The `std::array` wrapper is of limited use because you cannot pass arrays of an unknown size to a function. Therefore even with this template you may pass the array into a function as one argument and its size as another.

Alternatively you might see code like this:

```c++
const size_t num_elements = 1024;
char buffer[num_elements];
//...
// fill_buffer needs to be told how many elements there are
fill_buffer(buffer, num_elements);
```

Or like this

```c++
Element elements[100];
//...
int num_elements = sizeof(elements) / sizeof(Element);
```

In Rust, the array has a function bound to it called `len()`. This always provides the length of the array. In addition if we take a slice of the array, that also has a `len()`.

```rust
let buffer: [u8; 1024]
println!("Buffer length = {}", buffer.len());

fill_buffer(&buffer[0..10]);
//...
fn fill_buffer(elements: &[Element]) {
  println!("Number of elements = {}", elements.len());
}
```
