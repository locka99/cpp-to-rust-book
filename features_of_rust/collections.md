# Collections

A collection is something that holds zero or more elements in some fashion that allows you to enumerate those elements, add or remove elements, find them and so forth.

* Vector - a dynamic array. Appending or removing elements from the end is cheap (providing the array is large enough to accomodate an additional item). Inserting items or removing them from any other part of the array is more expensive and involves memory movements. Generally speaking you should always reserve enough space in a vector for the most elements you anticipate it will hold. Reallocating memory can be expensive and lead to fragmentation.

* Vecdeque - a ring buffer array. Items can be added or removed from either end relatively cheaply. Items in the array are not arranged sequentially so there is a little more complexity to managing wraparound and removal than a Vector.

* LinkedList - a linked list individually allocates memory for each element making it cheap to add or remove elements from anywhere in the list. However there is a lot more overhead to iterating the list by index and much more heap allocation.

* Set - a collection that holds a unique set of items. Inserting a duplicate item will not succeed. Some sets maintain the order of insertion. Sets are useful where you want to weed out duplicates from an input.

* Map - a collection where each item is referenced by a unique key. Some maps can maintain the order of insertion.

C++ and Rust have have collections as part of their standard library as is common with modern languages.

| C | C++ | Rust
| --- | --- | ---
| - | `std::vector` | `std::vec::Vec` or `std::collections::VecDeque`
| - | `std::list` | `std::collections::LinkedList`
| - | `std::set` | `std::collections::HashSet`, `std::collections::BTreeSet`
| - | `std::map` | `std::collections::HashMap`, `std::collections::BTreeMap`

C has no standard collection classes or types. Some libraries offer collection APIs such as [glib](https://developer.gnome.org/glib/) or [cii](https://github.com/drh/cii).

## Iterators

Iterators are a reference to a position in a collection with the means to step through the collection one element at a time.

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
