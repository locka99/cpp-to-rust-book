# Casting

## C/C++ casting

Casting is the act of coercing one type to be another, or dynamically producing the equivalent value in the other type.

C++ has a range of cast operators that turn a pointer or value of one kind into a pointer or value of another kind.

* `const_cast<T>(value)` - removes the const enforcement from a value so it may be modified.
* `static_cast<T>(value)` - attempts to convert between types using implicit and user defined conversions.
* `reinterpret_cast<T>(value)` - a compiler directive to just treat the input as some other kind. It does not involve any form of conversion.
* `dynamic_cast<T>(value)` - attempts to convert a class pointer / reference to/from other classes in its inheritance hierarchy. Involves runtime checks.
* Traditional C-style cast - a C++ compiler will attempt to interpret it as a `const_cast`, a `static_cast` and a `reinterpret_cast` in varying combinations.

That's a very brief summary of casting which probably invokes more questions than it answers. Casting in C++ is very complex and nuanced. Some casts merely instruct the compiler to ignore const or treat one type as another. A static cast might involve code generation to convert a type. A dynamic cast might add runtime checks and throw exceptions.

## Coercion

Rust does not do casting so this complexity is gone. Basically if you wish to "cast" you must do one of the following.

1. A numeric type may be coerced to another numeric type using the [`as`](https://doc.rust-lang.org/book/casting-between-types.html#as) keyword.
2. Complex types can implement `Into<Foo>` or `From<Foo>` traits that allows one structured type to be turned into another.
3. Unsafe code can transmutate memory, essentially reinterpretting the contents.

```rust
let a = 123i32;
let b = a as usize;
```

## Into and From traits

Anything beyond this requires implementing the `Into<>` or `From<>` traits and making conversion an explicit action.

TODO

The compiler also does not allow code to cast away `const`-ness or treat one type as another except through `unsafe` code blocks.

# Transmutation

Rust allows some types to be [transmuted](https://doc.rust-lang.org/book/casting-between-types.html#transmute) to others. Transmute is an `unsafe` action but it allows a memory location to be treated as another type, e.g. an array of bytes as an integer.
