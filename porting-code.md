# Porting Code

Before starting, the assumption at this point is you *need* to port code. If you're not sure you do to port code, then maybe you don't. After all, if your C/C++ code works fine, then why change it?

TODO This section will provide a more real world C/C++ example and port it to the equivalent Rust

## Automation tools

### C2Rust

The [C2Rust project](https://github.com/immunant/c2rust) is developing a tool that translates C to semantically equivalent Rust.

There is an online demo of it working [here](https://c2rust.com/).

As C is unsafe by default, it means the Rust equivalent is also unsafe by default, but it can be useful as a starting point for converting code.

### Corrode

[Corrode](https://github.com/jameysharp/corrode) is a command-line tool that can partially convert C into Rust. At the very least it may spare you some drudgery ensuring the functionality is as close to the original as possible.

Corrode will take a C file, e.g. `somefile.c` plus any arguments from `gcc` and produces a `somefile.rs` which is the equivalent code in Rust. 

It works by parsing the C code into an abstract syntax tree and then generating Rust from that.

Interestingly Corrode is written in Haskell and more interestingly is written as a [literate Haskell source](https://github.com/jameysharp/corrode/blob/master/src/Language/Rust/Corrode/C.md) - the code is a markdown document interspersed with Haskell.

### Bindgen

[Bindgen](https://github.com/servo/rust-bindgen) is a tool for generating FFI interfaces for Rust from existing C and C++ header files. You might find this beneficial if you're porting code from C / C++, or writing a new component that must work with an existing code base.

Bindgen requires that you preinstall the Clang C++ compiler in order to parse code into a structure it can digest. 

The readme documentation on the site link provides more information on installing and using the tool.

### CBindgen

[CBindgen](https://github.com/eqrion/cbindgen) works in the opposite direction of bindgen - it produces C header files from Rust structures and functions.

So if you have Rust code that you wish to call from C, then you can generate the appropriate header files that enable you to do this.

The headers can be produced from the command line like so:

```
cargo install cbindgen
cbindgen -o bindings.h
```

Alternatively you can create a `build.rs` that automatically does this when you build your crate:

```rust
extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}
```

A `cbindgen.toml` allows you to configure things that go into the header file. See the site link for more information.


## Experiences

A number of websites offer insights of the porting process from C to Rust

1. [Porting Zopfli from C to Rust](https://github.com/carols10cents/rust-out-your-c-talk). Zopfli is a library that performs good but slow deflate algorithm. It produces smaller compressed files than zlib while still remaining compatible with it. 
2. TODO

# Tips

## Use references wherever you can

TODO references are equivalent to C++ references or to pointers in C. Passing by reference is an efficient way of passing any object greater than 64-bits in size into a function without relinquishing ownership. In some cases, it is even a good idea to return by reference since the caller can always clone the object if they need to, and more often than not they can just use the reference providing the lifetimes allow for it.

TODO you do not need to use references on intrinstic types such as integers, bools etc. and there is no point unless you intend for them to be mutable and change. 

## Move semantics

C and C++ default to copy on assign. Unless you explicitly implement a move constructor on objects, the default is copy. Copying is dangerous because it allows two data structures to potentially point to the same data, e.g. a raw file handle. Even if code implements a move constructor, the compiler will not care if you reference the old 
object so you are required to put the object into a valid but safe state.

Rust by default will move on assign unless the type implements a `Copy` trait. Only simple primitives or structs comprised of simple primitives can implement or derive the `Copy` trait.

```rust
let x = 100;
let y = x; // This is a copy
println!("x = {}", x);

let x = "Hello world".to_string();
let y = x; // This is a move
println!("x = {}", x); // compile error
```

And unlike C++, you CANNOT access the old object once you have assigned the value to a new one. The compiler will generate an error if you try.

If you need to copy data in Rust you must implement / derive the `Clone` trait which allows you to explicitly invoke `.clone()` to make a copy of an object:

```rust
let x = "Hello world".to_string();
let y = x.clone();
println!("x = {}", x);

#[derive(Clone)]
struct MyData {
  name: String;
}
///...
let x = MyData { name: "Fred".to_string() };
let y = x.clone();
```

## Use modules to naturally arrange your source code

TODO

## Using composition and traits

TODO Rust does not allow you to inherit one struct from another. The manner of overcoming this.

## Using Cargo.toml to create your build profiles

## Use Rust naming conventions and formatting

C and C++ have never had 

# Foreign Function Interface

TODO for now read the [FFI omnibus](http://jakegoulding.com/rust-ffi-omnibus/).

## Leaving function names unmangled

TODO attribute no_mangle

## libc

TODO Rust provides a crate with bindings for C library functions. If you find yourself receiving a pointer allocated with malloc you could free it with the corresponding call to free() via the bindings.

TODO add the following to your `Cargo.toml`

```
[dependencies]
libc = "*"
```

TODO example of using libc