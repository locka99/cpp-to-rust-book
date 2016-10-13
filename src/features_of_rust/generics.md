# Templates / Generics

C++ offers templates as a way to write generic code using an abstract type and then specialize it by substituting one or more types into a concrete class.

This is a very powerful mechanism and the C++ library makes extensive use of it. Where templates can become a bit of a mess is that the compiler expands all of the code before compiling it. An innocuous error in a type such as the absence of a copy constructor can cause the compiler to generate a wall of indeciperahble errors from the heart of the expanded / substituted template where a problem was encountered.

## Generic Functions

TODO generic function

## Trait bounds

TODO generic trait

## Where clause

Rust has a more expressive way of defining the constraints on a generic via the where clause

TODO where clause
