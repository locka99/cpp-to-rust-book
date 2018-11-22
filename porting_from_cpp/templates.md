# Templates vs Generics

## What's a template?

C++ provides a way of substituting types and values into inline classes and functions called templates. Think of it as a sophisicated substitution macro - you specify a type T in the template and this can substitute for a type `int` or something else at compile time. During compilation you'll be told if there are any errors with the type you supply. This is a very powerful feature since it allows a class to be reused for many different types.

Templates are used extensively in the C++ library, Boost and in other places. Collections, strings, algorithms and various other piece of code use templates in one form or another.

However, templates only expand into code when something actually calls the inline function. Then, if the template calls other templates, the inline code is expanded again and again until there is a large body of code which can be compiled. A small error in our code can propogate into an enormous wall of noise in the middle of some expanded template.

For example a vector takes a type it holds as a template parameter. So we can create a vector of PatientRecords.

```c++
class PatientRecord {
  std::string name_;

  PatientRecord() {}
  PatientRecord operator= (const PatientRecord &other) { return *this; }

public:
  PatientRecord(const std::string &name) : name_(name) {
  }
};
...
std::vector<PatientRecord> records;
```

So far so good. So let's add a record:

```c++
records.push_back(PatientRecord("John Doe"));
```

That works too! Now let's try to erase the record we just added:

```c++
records.erase(records.begin());
```

Boom!

```
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h: In instantiation of 'static _OI std::__copy_move<true, false, std::random_access_iterator_tag>::__copy_m(_II, _II, _OI) [with _II = PatientRecord*; _OI = PatientRecord*]':
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h:396:70:   required from '_OI std::__copy_move_a(_II, _II, _OI) [with bool _IsMove = true; _II = PatientRecord*; _OI = PatientRecord*]'
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h:434:38:   required from '_OI std::__copy_move_a2(_II, _II, _OI) [with bool _IsMove = true; _II = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; _OI = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >]'
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h:498:47:   required from '_OI std::move(_II, _II, _OI) [with _II = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; _OI = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >]'
c:/mingw/i686-w64-mingw32/include/c++/bits/vector.tcc:145:2:   required from 'std::vector<_Tp, _Alloc>::iterator std::vector<_Tp, _Alloc>::_M_erase(std::vector<_Tp, _Alloc>::iterator) [with _Tp = PatientRecord; _Alloc = std::allocator<PatientRecord>; std::vector<_Tp, _Alloc>::iterator = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; typename std::_Vector_base<_Tp, _Alloc>::pointer = PatientRecord*]'
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_vector.h:1147:58:   required from 'std::vector<_Tp, _Alloc>::iterator std::vector<_Tp, _Alloc>::erase(std::vector<_Tp, _Alloc>::const_iterator) [with _Tp = PatientRecord; _Alloc = std::allocator<PatientRecord>; std::vector<_Tp, _Alloc>::iterator = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; typename std::_Vector_base<_Tp, _Alloc>::pointer = PatientRecord*; std::vector<_Tp, _Alloc>::const_iterator = __gnu_cxx::__normal_iterator<const PatientRecord*, std::vector<PatientRecord> >; typename __gnu_cxx::__alloc_traits<typename std::_Vector_base<_Tp, _Alloc>::_Tp_alloc_type>::const_pointer = const PatientRecord*]'
..\vectest\main.cpp:22:34:   required from here
..\vectest\main.cpp:8:19: error: 'PatientRecord PatientRecord::operator=(const PatientRecord&)' is private
     PatientRecord operator= (const PatientRecord &other) { return *this; }
```

If you wade through that noise to the bottom we can see the erase() function wanted to call the assignment operator on PatientRecord, but couldn't because it was private.

But why did vector allow us to declare a vector with a class which didn't meet its requirements?

We were able to declare the vector, use the std::vector::push_back() function but when we called std::vector::erase() the compiler discovered some deeply nested error and threw these errors back at us.

The reason is that C++ only generates code for templates when it is called. So the declaration was not in violation, the push_back() was not in violation but the erase was.

## How Rust helps

Rust has a concept similar to templates called generics. A generics is a struct or trait that takes type parameters just like a template.

However but the type can be enforced by saying the traits that it must implement. In addition any errors are meaningful.

Say we want to write a generic function that clones the input value:

```rust
fn clone_something<T>(value: T) -> T {
  value.clone()
}
```

We haven't even called the function yet, merely defined it. When we compile this, we'll instantly get an error in Rust.

error: no method named `clone` found for type `T` in the current scope

```
  |
4 |   value.clone();
  |         ^^^^^
  |
  = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `clone`, perhaps you need to implement it:
  = help: candidate #1: `std::clone::Clone`
```

Rust is saying we never said what T was and because some-random-type has no method called clone() we got an error.
So we'll modify the function to add a trait bound to T. This binding says T must implement Clone:

```rust
fn clone_something<T: Clone>(value: T) -> T {
  value.clone();
}
```

Now the compiler knows T must have implement Clone it is able to resolve clone() and be happy.
Next we actually call it to see what happens:

```rust
struct WhatHappensToMe;
let x = clone_something(10);
let y = clone_something(WhatHappensToMe{});
```

We can clone the integer 10 because integers implement the Clone trait, but our empty struct WhatHappensToMe does not implement Clone trait. So when we compile it we get an error.

```
error[E0277]: the trait bound `main::WhatHappensToMe: std::clone::Clone` is not satisfied
  |
8 | let y = clone_something(WhatHappensToMe{});
  |         ^^^^^^^^^^^^^^^
  |
  = note: required by `main::clone_something`
```

In summary, Rust improves on templates by TODO

Compiling generic functions / structs even when they are unused and offer meaningful errors immediately.

Allow us to bind traits to generic types to constrain what we can pass into them.

Offer meaningful errors if we violate the requirements of the trait bounds
