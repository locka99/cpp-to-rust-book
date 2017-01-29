# Porting Code

Before starting, the assumption at this point is you *need* to port code. If you're not sure you need to port code, then that's the question you must ask first and come to an answer. 

This section will provide a more real world C/C++ example and port it to the equivalent Rust

TODO this section is still to do

## Automation

[Corrode](https://github.com/jameysharp/corrode) is a command-line tool that can partially convert C into Rust. At the very least it may spare you some drudgery ensuring the functionality is as close to the original as possible.

Corrode will take a C file, e.g. `somefile.c` plus any arguments from `gcc` and produces a `somefile.rs` which is the equivalent code in Rust. 

It works by parsing the C code into an abstract syntax tree and then generating Rust from that.

Interestingly Corrode is written in Haskell, so perhaps a future project will allow Haskell code to be converted to Rust!

## Experiences

A number of websites offer insights of the porting process from C to Rust

1. [Porting Zopfli from C to Rust](https://github.com/carols10cents/rust-out-your-c-talk). Zopfli is a library that performs good but slow deflate algorithm. It produces smaller compressed files than zlib while still remaining compatible with it. 