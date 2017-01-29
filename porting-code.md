# Porting Code

Before starting, the assumption at this point is you *need* to port code. If you're not sure you need to port code, then that's the question you must ask first and come to an answer. 

This section will provide a more real world C/C++ example and port it to the equivalent Rust


## Automation

[Corrode](https://github.com/jameysharp/corrode) is a tool that can partially implement converting C into Rust. At the very least it may spare you some effort in converting code and ensuring the functionality is as close to the original as possible.

The tool is 

## Experiences

A number of websites offer insights of the porting process from C to Rust

1. [Porting Zopfli from C to Rust](https://github.com/carols10cents/rust-out-your-c-talk). Zopfli is a library that performs good but slow deflate algorithm. It produces smaller compressed files than zlib while still remaining compatible with it.



