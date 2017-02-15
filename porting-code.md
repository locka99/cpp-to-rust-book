# Porting Code

Before starting, the assumption at this point is you *need* to port code. If you're not sure you need to port code, then that's the question you must ask first and come to an answer. 

This section will provide a more real world C/C++ example and port it to the equivalent Rust

TODO this section is still to do

## Automation tools

### Corrode

[Corrode](https://github.com/jameysharp/corrode) is a command-line tool that can partially convert C into Rust. At the very least it may spare you some drudgery ensuring the functionality is as close to the original as possible.

Corrode will take a C file, e.g. `somefile.c` plus any arguments from `gcc` and produces a `somefile.rs` which is the equivalent code in Rust. 

It works by parsing the C code into an abstract syntax tree and then generating Rust from that.

Interestingly Corrode is written in Haskell and more interestingly is written as a [literate Haskell source](https://github.com/jameysharp/corrode/blob/master/src/Language/Rust/Corrode/C.md) - the code is a markdown document interspersed with Haskell.

### Bindgen

[Bindgen](https://github.com/servo/rust-bindgen) is a tool for generating FFI interfaces for Rust from existing C and C++ header files. You might find this beneficial if you're porting code from C / C++, or writing a new component that must work with an existing code base.

Bindgen requires that you preinstall the Clang C++ compiler in order to parse code into a structure it can digest. 

The readme documentation on the site link provides more information on installing and using the tool.

## Experiences

A number of websites offer insights of the porting process from C to Rust

1. [Porting Zopfli from C to Rust](https://github.com/carols10cents/rust-out-your-c-talk). Zopfli is a library that performs good but slow deflate algorithm. It produces smaller compressed files than zlib while still remaining compatible with it. 
2. TODO

# Tips

## Use references wherever you can

TODO references are equivalent to C++ references or to pointers in C. Passing by reference is an efficient way of passing any object greater than 64-bits in size into a function without relinquishing ownership. In some cases, it is even a good idea to return by reference since the caller can always clone the object if they need to, and more often than not they can just use the reference providing the lifetimes allow for it.

TODO you do not need to use references on intrinstic types such as integers, bools etc. and there is no point unless you intend for them to be mutable and change. 

## Learn move semantics

TODO C and C++ default to copy on assign, Rust moves on assign unless the type implements the Copy trait. This is easily one of the most mind boggling things that new Rust programmers will encounter. Code that works perfectly well in C++ will instantly fail in Rust. The way to overcome this is first use references (see previous section), and secondly don't move unless you intend for the recipient to be the new owner of an object.

TODO if you intend for the recipient to own a copy of an object then implement the Clone trait on your struct. Then you may call `.clone()` and the recipient becomes the owner of the clone instead of your copy.

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

TODO add the following to your Cargo.toml

```
[dependencies]
libc = "*"
```

TODO example of using libc