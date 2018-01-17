# Collections

C++ and Rust have have collections as part of their standard library as is common with modern languages.

| C | C++ | Rust
| --- | --- | ---
| - | `std::vector` | `std::vec::Vec` or `std::collections::VecDeque`
| - | `std::list` | `std::collections::LinkedList`
| - | `std::set` | `std::collections::HashSet`, `std::collections::BTreeSet`
| - | `std::map` | `std::collections::HashMap`, `std::collections::BTreeMap`

C has no standard collection classes or types. Users wanting collections might have resorted to using [glib](https://developer.gnome.org/glib/) or [cii](https://code.google.com/archive/p/cii/downloads).


## Iterators

Iterators are a position and reference to a collection with the means to advance through the collection one element at a time.

### C++

C++11 provides a shorthand way of iterating a collection:

```c++
std::vector<char> values;
for (const char &c : values) {
    // do something to process the value in c
}
```

Iterators are more explicit in C++98 and before and the code in C++11 is basically equivalent to this:

```c++
std::vector<char> values;

for (std::vector<char>::const_iterator i = values.begin(); i != values.end(); ++i) {
    const char &c = *i;
    // do something to process the value in c
}
```

This is quite verbose, but essentially each collection type defines a mutable `iterator` and immutable `const_iterator` type and calling `begin` returns an iterator to the beginning of the collection. Calling the `++` operator overload on the iterator causes it to advance to the next element in the collection. When it hits the exclusive value returned by `end` it has reached the end of the collection.

Obviously with an indexed type such as a `vector` you could also reference elements by index, but it is far more efficient to use iterators for other collection types.

#### Processing collections

C++ provides a number of utility templates in <algorithm> for modifying sequences in collections on the fly. 

### Rust

Rust also has iterators which work in a similar fashion to C++ - incrementing their way through collections. 

TODO

TODO chaining iterators together

TODO mapping one collection to another collection
