# Headers and Sources

A header file contains definitions of classes, types, macros etc that other files need to #include in order to resolve their use of those things.

Splitting the implementation and definition across different files is an added burden for maintaining code but it can also lead to some serious errors.

* Headers used across multiple projects that have different compiler settings
* Issues with pragmas and alignment
* Issues with different #definitions that affect byte length
* Issues with different typedefs that affect byte length

Each consumer of the header must do so with the exact same settings that affect the size of every type, struct and class in the file plus any issues with packing / alignment. If these settings are not the same, it can cause instability, corruption or problems that only manifest themselves at at runtime.

Headers also make the compiler slower because source that consumes the header inevitably pulls in other headers which pull in other headers.

## Guard blocks / #pragma once

Headers will also be expanded as many times as they are `#include`'d. To prevent the expansion happening more than once per source file, they're usually protected by guard blocks.

```c++
#ifndef FOO_H
#define FOO_H
....
#endif
```

If the same header is included more than once, the second time through it is preprocessed into nothing.

### #pragma once

Most modern compilers also support a `#pragma once` directive. This allows the compiler to completely ignore an `#include` which it knows it has already included at least once before per source file. 

This is more efficient than guard blocks because the compile doesn't even bother opening or processing the file again and just skips over it. There may be situations where this is not suitable, but usually it results in faster compilation.

### Precompiled Headers

Some compilers also support precompiled headers to speed up compilation. The compiler builds a database lookup when compiling a single source file and subsequent source compiles with reference to that database. This solution can speed up compilation but it complicates the build process since one file has flags to generate the precompiled header file and other sources have flags to reference it.

## Pimpl pattern

A popular workaround for header issues is the Pimpl pattern. It is a way to separate a class into a public part and a private implementation part.

The public class is almost an interface definition in its purity that can be defined in the header with minimal dependencies. It forward references the implementation class and stores it as a member:

```c++
#pragma once

// Gory details are in the .cpp file
class ComplexThingImpl;

class ComplexThing {
  ComplexThingImpl *pimpl_;
public:
  ComplexThing();
  ~ComplexThing();

  // See note 1 below

  void somethingReallyComplex();
};
```

The constructor for the outer class would allocate the implementation class and method calls would call through to the inner.

The private implementation class is defined in the source file and can pull in as many extra headers as it needs, pragmas whatever without hurting consumers or compile times of the header.

```c++
// source file
#include "random/header.hpp"
// Lots of includes here
#include <...>
#include "more/stuff.hpp"

class  ComplexThingImpl {
  // Lots of member variables and stuff here
  // ...
public:
  void somethingReallyComplex();
}

void ComplexThingImpl::somethingReallyComplex() {
  // Lots of complex stuff here
  // ...
}

ComplexThing::ComplexThing() :
  pimpl_(new ComplexThingImpl()) {
}

ComplexThing::~ComplexThing() {
  delete pimpl_;
}

void ComplexThing:: somethingReallyComplex() {
  pimpl_->somethingReallyComplex();
}
```

This solution is known as Pimpl (private implementation) pattern and while it can work to protect consumers and speed up builds it also adds complexity and overhead to development. Instead of 2 definitions of a class to maintain (header / source) you now have 4(!) because there is a public and private impl class. Changing the signature of a method means changing it in potentially 4 places, plus the line in the public class that invokes the private counterpart.

One danger for Pimpl is that the private class is allocated from the heap. Code that uses a lot of temporary Pimpl objects could contribute to heap fragmentation.

Note 1: Remember the rule of three? That applies to this object too. The example doesn't show it but if we copy constructed or assigned ComplexThing to another instance we'd be in a heap of trouble. So on top of the issues with making PImpl work we also have to prevent the other ones. The easiest way to lock it down would be to derive from `boost::noncopyable` if you were using boost or make the copy constructor `private`, or use delete it in C++11.

## How Rust helps

In Rust the definition and the implementation are the same thing. So immediately we have exactly one thing to maintain.

Writing a function defines the function. Let's assume we have a functions.rs file

```rust
// functions.rs
pub fn create_directory_structure() {
  // Implementation
}
```

Anyone can call it as `functions::create_directory_structure()`. The compiler will validate the call is correct.

A struct's definition and its implementation are also written once. e.g. `directory.rs`

```rust
// directory.rs
pub struct Directory {
  pub path: String,
}
impl Directory {
  pub fn mkdir(&self) {
    // implementation
  }
}
```

Implementations can be defined in a private Rust module and only public structs exposed to consumers.

If we were a library crate (which we'll call `file_utils`) wishing to expose these objects to consumers we would write a top-level `lib.rs` which says what files our lib comprises of and we want to expose.

```rust
// lib.rs for file_utils
mod functions;
mod directory;
pub use functions::*;
pub use directory::Directory;
```

Now a consumer can use our crate easily:

```rust
extern crate file_utils;
use file_utils::*;
fn main() {
   create_directory_structure();
   let d = Directory { /* ... */ };
}
```
