# Rust's standard library

The core functionality in Rust is provided by a module called `std`. This is the standard runtime library.

As with its C++ namesake, everything can be referenced through a `std::` namespace prefix or via a `use std::{foo}` import.

The most commonly used parts of `std` are implicitly brought in as if you had typed this at the top of your code:

```rust
extern crate std;
use std::prelude::*;
```

The [`std::prelude`](https://doc.rust-lang.org/beta/std/prelude/) contains the most commonly used parts of std that you can just implicitly reference. For example:

* `String` and `ToString` trait
* Iterators traits of various kinds - `Iterator`, `IntoIterator` etc.
* `Result<>` and `Option<>` enums
* Conversion traits `AsRef`, `AsMut`, `Into`, `From`
* `Vec` heap allocated vector
* Common traits such as `Drop`, `Fn`, `FnMut`, `FnOnce`, `Box`, `Clone`, `Copy`, `Send`, `Sized`, `Sync`, `PartialEq`, `PartialOrd` etc.
* Macros such as `println!`, `format!`, `assert!` etc.

There are various sub-modules under std that concern themselves with aspects of development. Here are just some of them:

1. clone – the `Clone` trait
2. cmp – `Eq`, `Ord`, `PartialEq`, `PartialOrd` traits. These traits are used for equality and ordering functionality.
3. collections - contains the standard collection types for sequences, maps, sets, and miscellaneous. e.g. `Vec` and `HashMap` are members of this module.
4. env – environmental helpers - command line arguments, status codes, environment variables, temporary folder
5. fmt – utilities for formatting and printing strings
6. fs - filesystem manipulation
7. io – Read and Write traits that are implemented by streams / buffers in file system and networking, stdio functionality
8. mem – memory primitives
9. net – networking
10. path – path manipulation
11. process – spawn, fork, exec etc.

Anything not brought in by `std::prelude` can be pulled in through a `use` statement:

```
use std::collections::BTreeMap;
```

## C / C++ lib to Rust lib cross reference

TODO

Note that Rust's std namespace contains a lot of stuff not in the standard C or C++ libraries and a lot of things are not directly analogous.
For example the standard C / C++ library have very little to say about sockets, or path manipulation, or atomically incrementing numbers, or creating threads.

C | C++ | Rust
--- | --- | ---
T [S], e.g. char foo[20] | std::array (C++11) | [T; S], e.g. let foo: [u8; 20] = [0; 20]
char * or char[] with functions such as strcmp, strcpy, strstr, strdup etc. Plus wide equivalents to these. | std::string, std::wstring, std::u16string (C++11), std::u32string (C++11) | &str or String as appropriate
- | std::vector | std::vec::Vec or std::collections::VecDeque
- | std::list | std::collections::LinkedList
- | std::set | std::collections::HashSet, std::collections::BTreeSet
- | std::map | std::collections::HashMap, std::collections::BTreeMap
fopen, fclose, fread / fwrite, fseek etc. | std::ofstream, std::ifstream, std::fstream | TODO
Math functions such as cos, sin, tan, acos, asin, atan, pow, abs, log, log10, floor, ceil are defined in <math.h> | - | Math functions are direction accessible from f64. f32 types., e.g. 1.0f64.cos().

Note that because due to the decimal point being used on a float, you have to prefix f32 or f64 to literals when you call them so the compiler can figure out what you're doing.

## Standard Traits

Commonly used traits are defined in `std` and in some cases can be derived automatically via compiler directives.

In others they cause the compiler to generate additional code for you such as the `Drop` trait (described in class destructor section)

### Drop
The `Drop` trait allows you do something when an object is dropped, such as add additional logging or whatever.

### Copy
A struct implementing a Copy trait can be copied through assignment, i.e. if you assign a to b then a and b now how copies of the object, independent of each other.
The Copy trait really only useful when you have small amounts of data that represent a type or value of some kind.
TODO copy example, e.g. `struct PlayingCard { suit: Suit, rank: Rank }`
If you find yourself with a type that is larger, or contains heap allocated memory then you should use clone.

### Clone
A struct implementing the Clone trait has a .clone() method. Unlike Copy you must explicitly .clone() the instance to create another.
TODO clone example

### Eq, PartialEq
TODO equality

### Ord, PartialOrd
TODO ordering
