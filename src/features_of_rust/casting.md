# Casting

Casting is the act of coercing one type to be another, or dynamically producing the equivalent value in the other type.

C++ has a range of cast operators that turn a pointer or value of one kind into a pointer or value of another kind. It also has const_cast<> which allows code to violate const enforcement even in the times it might be enforced.

The equivalent of casting in Rust is the "[as](https://doc.rust-lang.org/book/casting-between-types.html#as)" command. You may cast values from one numeric type subject to the normal rules of data loss and truncation.

# Transmutation

Rust also allows some types to be [transmuted](https://doc.rust-lang.org/book/casting-between-types.html#transmute) to others. Transmute is an unsafe action but it allows a memory location to be treated as another type, e.g. an array of bytes as an integer.
