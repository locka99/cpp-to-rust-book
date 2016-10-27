# Memory allocation

This section is concerned with memory allocation, i.e. allocating objects that reside in the heap and how they are cleaned up.

## C++

C and C++ allocate memory via matching calls to:

1. malloc/calloc and free
2. new and delete
3. new[] and delete[] for arrays

The main rule for memory allocation is that one allocation must be matched by one free. Forgetting to free memory causes a leak. Freeing more than once is an error and undefined. Calling something that is already freed is undefined behaviour but often results in a crash or some sort.

C/C++ do not enforce lifetimes on memory allocation. C++ has attempted to control memory allocation through the use of smart pointers which allow  memory to be freed when the smart pointer goes out of scope (or the stack unwinds after an exception), or for pointers to be shared and reference counted and freed when the last reference is removed.

TODO smart pointer example.

3rd party libraries have also attempted to retrofit C/C++ with garbage collection models and other forms of memory allocation which attempt to make C/C++ more resilient. For example the [Boehm garbage collector](http://www.hboehm.info/gc/) implements mark and sweep garbage collection.

## Rust

As you can guess by now Rust tends to be a lot more strict about allocation that C/C++. Lifetimes of objects are tracked and enforced by the compiler and that includes memory allocated objects.

1. Box is for holding an exclusive pointer to a heap allocated object

Rust requires most heap allocated memory to be contained by one or more of the structs below. The struct manages the lifetime and access to the object inside ensuring the lifetime is managed correctly.

### Box<T>

A Box is for allocating structs on the heap. It holds a reference to some data but there can only be one valid reference to the box itself. Essentially, that means you can pass the box around from one place to another and whoever binds to it last can open it. Everyone else’s binding is invalid will generate a compile error.

A box can be useful for abstraction since it can refer to a struct by a trait it implements allowing decoupling between types.

TODO example of a struct holding a box with a trait implemented by another struct

It can be useful for situations where one piece of code creates an object on behalf of another piece of code and hands it over. The Box makes sure that the ownership is explicit at all times and when the box moves to its new owner, so does the lifetime of the object itself.

### Cell<T>

A Cell is something that can copied with a get() or set() to overwrite its own copy. As the contents must be copyable they must implement the Copy trait.

The Cell has a zero-cost at runtime because it doesn’t have to track borrows but the restriction is it only works on Copy types. Therefore it would not be suitable for large objects or deep-copy objects.

### RefCell<T>

Somewhat more useful is the RefCell<T> but it incurs a runtime penalty to maintain read-write locks.

The RefCell holds a reference to an object that can be borrowed either mutably or immutably. These references are read-write locked so there is a runtime cost to this since the borrow must check if something else has already borrowed the reference.

Typically a piece of code might borrow the reference for a scope and then the borrow disappears when it goes out of scope. If a borrow happens before the last borrow releases, it will cause a panic.

## Reference Counting objects

Often objects are shared by different pieces of code, in which case the last reference to the object is the thing that should destroy it. This has a lifetime penalty since Rust cannot figure out what object will be last to relinquish the object. For this purpose it offers two reference counted wrappers. One is thread safe while the other is not, so it is up to the code to decide which is right for the managing the struct's lifetime.

A reference counted object is usually wrapping a Box, Cell or Refcell. So multiple structs can hold a reference to the same object.

### Rc<T>

From std::rc::Rc. A reference counted object can be held by multiple owners at a time. Each own holds a cloned Rc<T> but the T contents are shared. The last reference to the object causes the contents to be destroyed.

Most often T will actually be something such as a Cell, RefCell or a Box

### Arc<T>

From std::sync::Arc. An atomic reference counted object that works like Rc<T> except it uses an atomically incremented counter which makes it thread safe. If multiple threads access the same object they would use Arc<T>
