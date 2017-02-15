# Types

Rust has mostly analogous primitive types with C/C++.

| C/C++ | Rust | Notes
| --- | ---- | ---
| `char` | `i8` | A Rust `char` is not the same as a C/C++ `char` [^1]
| `unsigned char` | `u8` |
| `short int` | `i16` |
| `unsigned short int` | `u16` |
| `int` | `i32` or `i16` | In C/C++ this is data model dependent [^2]
| `unsigned int` | `u32` or `u16` | In C/C++ this is data model dependent [^2]
| `long int` | `i32` or `i64` | In C/C++ this is data model dependent [^2]
| `unsigned long int` | `u32` or `u64` | In C/C++ this is data model dependent [^2]
| `long long int` | `i64` |
| `unsigned long long int` | `u64` |
| `size_t` | `usize` | usize holds numbers as large as the address space [^3] |
| `float` | `f32` |
| `double` | `f64` |
| `long double` | <s>f128<s> | f128 support was present in Rust but removed due to issues for some platforms in implementing it.
| `bool` | `bool` |
| `void` | `()` | The unit type (see below)

[^1] Rust's `char` type, is 4 bytes wide, enough to hold any Unicode character. This is equivalent to the belated `char32_t` that appears in C++11 to rectify the abused `wchar_t` type which on operating systems such as Windows is only 2 bytes. When you iterate strings in Rust you may do so either by character or `u8`, i.e. a byte.

[^2] See the next section to for a discussion on data models.

[^3] Rust has a specific numeric type for indexing on arrays and collections called `usize`. A `usize` is designed to be able to reference as many elements in an array as there is addressable memory. i.e. if memory is 64-bit addressable then usize is 64-bits in length. There is also a signed `isize` which is less used but also available.

## Data model

C/C++ compilers implement a *data model* that affects what width the standard types are.

The four data models in C++ are:

* LP32 - `int` is 16-bit, `long` and pointers are 32-bit. This is an uncommon model, a throw-back to DOS / Windows 3.1
* ILP32 - `int`, `long` and pointers are 32-bit. Used by Win32, Linux, OS X
* LLP64 - `int` and `long` are 32-bit, `long long` and pointers are 64-bit. Used by Win64
* LP64 - `int` is 32-bit, `long` / `long long` and pointers are 64-bit. Used by Linux, OS X

C ships with a special  `<stdint.h>` header (which is called `<cstdint.h>` in C++) that provides explicit length typedefs, e.g. `uint32_t`.

## C/C++ types compared to Rust

C/C++ and Rust will share the same machine types for each corresponding language type and the same compiler / backend technology, i.e.:

1. Signed types are two's complement
2. IEE 754-2008 binary32 and binary64 floating points for float and double precision types.

The `<stdint.h>` / `<cstdint.h>` typedefs are also directly analogous.

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

## Integer types

### C++

C/C++ has primitive types for numeric values, floating point values and booleans. Strings will be dealt in a separate section.

Integer types (`char`, `short`, `int`, `long`) come in `signed` and `unsigned` versions.

A `char` is always 8-bits, but for historical reasons, the standards only guarantee the other types are "at least" a certain number of bits. So an `int` is ordinarily 32-bits but the standard only say it should be at *least as large* as a `short`, so potentially it could be 16-bits!

More recent versions of C and C++ provide a [`<cstdint.h>`](http://www.cplusplus.com/reference/cstdint/) (or `<stdint.h>` for C) with typedefs that are unambiguous about their precision.

Even though `<stdint.h>` can clear up the ambiguities, code frequently sacrifices correctness for terseness. It is not unusual to see an `int` used as a temporary incremental value in a loop:

```c++
string s = read_file();
for (int i = 0; i < s.size(); ++i) {
  //...
}
```

While `int` is unlikely to fail for most loops in a modern compiler supporting ILP32 or greater, it is still technically wrong. In a LP32 data model incrementing 32767 by one would become -32768 so this loop would never terminate if `s.size()` was a value greater than that.

This loop should be using the same type returned from `string::size()` which is an opaque unsigned integer type called `size_type`. This is usually a typedef for `std::size_t` but not necessarily. Thus we have a type mismatch. A `string` has an iterator which could be used instead but if you need the index for some reason, but it can messy:

```c++
string s = read_file();
for (string::iterator i = s.begin(); i != s.end(); ++i) {
  string::difference_type idx = std::distance(s.begin(), i);
  //...
}
```

Now we've swapped from one opaque type `size_type` to another called `difference_type`. Ugh.

C/C++ types can also be needlessly wordy such as `unsigned long long int`. Again, this sort of puffery encourages code to make bad assumptions, use a less wordy type, or bloat the code with typedefs. The best action is of course to use `<cstdint.h>` / `<stdint.h>` if it is available.

## Rust

Rust benefits from integer types that unambiguously denote their signedness and width in their name - `i16`, `u8` etc.

They are also extremely terse making it easy to declare and use them. For example a `u32` is an unsigned 32-bit integer. An `i64` is a signed 64-bit integer.

Types may be inferred or explicitly prefixed to the value:

```rust
let v1 = 1000;
let v2 : u32 = 25;
let v3 = 126i8;
let v4 = 99.3333f64;
let v5 = v4 as f32;
let f1 = true;
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

A f128 did exist for a period of time but was removed to portability complexity and maintenance issues. Note how `long double` is treated (or not) according to the compiler and target platform. At some point Rust might get a f128 or f80 but at this time does not have such a type.

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

This link describes the [gory details](https://github.com/rust-lang/rfcs/blob/master/text/1216-bang-type.md) of this pattern and proposes compiler support for a special `!` notation. Until this lands as a feature you are better off to say `()`.

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

Values may also be split out of a tuple easily too.

```rust
let (x, y, _) = (11, 200, -33);
println!("x = {}, y = {}", x, y);
```

In this example, we can directly assign the values from some tuple directly to `x` and `y`. The underscore `_` indicates we're not interested in the 3rd value.

Tuples can be particularly useful with code blocks. For example, lets say we want to get some values from a 

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

## Arrays

An array is a fixed size list of elements allocated either on the stack or the heap.

E.g to create a 100 element array of `double` values in C++:

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
let mut values = [0f64; 100];
// Heap
let mut values = Box::new([0f64; 100]);
```

Note how Rust provides a shorthand to initialise the array with zeroes or any other value. The C++ code above would be pointing at garbage unless the code explicitly set it to something. 

Rust actually *forces* you to initialise an array to something. Attempting to declare an array without assigning it a value is a compiler error.

## Slices

A slice is a partial or full view of an array or a string. A slice is not a copy of the array, rather that it is a pointer and length that represent a segment of the array.

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

One serious disadvantage of C++ arrays is there is no `.len()` method so the length has to be passed in to any function that wishes to manipulate it.

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
