# Memory allocation

This section is concerned with memory allocation, i.e. creating objects that reside on the heap and not on the stack, and the manner in which they are created and are destroyed.

## C++

C and C++ have various standard ways to allocate memory:

1. `malloc`/`calloc`/`realloc()` and `free()` functions
2. `new` and `delete` (C++ only)
3. `new[]` and `delete[]` for arrays (C++ only)

Invoking `malloc()`/`free()` on a C++ class or struct is never a good idea since it will not call the corresponding class constructor or destructor. The `realloc()` function allocates a new piece of memory, copying the contents of an existing piece of memory before freeing the original.

```c++
// malloc / free
char *buffer = (char *) malloc(1024);
...
free(buffer);
// new / delete
Stack *stack = new Stack();
...
delete stack;
// new[] / delete[]
Node *nodes = new Node[100];
...
delete []nodes;
```

In each case the allocation must be matched by the corresponding free action so immediately we can see scope for error here:

1. Ownership rules can get messy, especially when a class is passed around a lot - who deletes the object and when?
2. Not using the correct `new` & `delete` pair, causing a memory leak. e.g. calling `delete` instead of `delete[]`
3. Forgetting to free memory at all causing a memory leak.
4. Freeing memory more than once.
5. Calling a dangling pointer, i.e. a pointer which refers to freed memory.
6. Allocating / freeing in a way that causes heap fragmentation. Reallocation can cause fragmentation to happen a lot faster.

C++ has smart pointers which manage the lifetime on objects and are a good way to programmer error:

```c++
{
  std::auto_ptr<Database> db(new Database());
  //... object is deleted when db goes out of scope
}

// C++11
{
  std::unique_ptr<Database> db(new Database());
  //... object is deleted when db goes out of scope

  std::unique_ptr<Node[]> nodes<new Node[100]);
  //... arrays of objects are supported too
}

// C++11
{
  std::shared_ptr<Database> db(new Database());
  // Reference count db
  setDatabase(db);
  //... object is deleted when last shared_ptr reference to it goes out of scope

  std::shared_ptr<Node[]> nodes<new Node[100]);
  //... arrays of objects are supported too
}
```

Unfortunately it is not always possible to use smart pointers but wherever possible they should be used.

### Other ways of allocating memory

Virtually every C and C++ library has solutions for managing memory. They all their own indivual concept of ownership which is usually different from one to the next. Boost and Qt have their own memory management "smart" pointers. Qt even requires certain objects to be deleted "later" by a message processing loop on the thread that created the object. Some libraries even adopt a COM-like model of reference counting objects with smart pointers. Most C libraries will expose an alloc and free function for creating and destroying context objects that callers pass to the API.

Memory allocation can even be overwritten and replaced in some circumstances. In C, the standard malloc / free can be substituted for another memory allocator, e.g. TCMalloc
[TCMalloc](http://goog-perftools.sourceforge.net/doc/tcmalloc.html). Or perhaps the code wants to use garbage collected memory in which case [Bohem GC](http://www.hboehm.info/gc/) is a popular library for that purpose. Boehm can also be used for leak detection since it can find objects which were never released. C++ can also [override](http://en.cppreference.com/w/cpp/memory/new/operator_new) the global or class specific new / delete operators. Some standard C++ template classes also allow memory allocation to be overridden.

## Rust

As you can guess by now Rust tends to be a lot more strict about allocation that C/C++. Lifetimes of objects are tracked and enforced by the compiler and that includes memory allocated objects.

In normal safe programming there is no explicit new / delete so there is no way to forget to free an object. There are no pointers either so code cannot call a dangling pointer or inadvertently call a null pointer.

1. A `Box` is a managed pointer that holds a heap allocated object. A box cannot be cloned, so there is only one owner at any time.
2. A `Cell` is a mutable memory location - it can hold any kind of copyable type and the value within it can be changed.
3. A `RefCell` is a mutable memory location that can hold a reference

The advantage for programmers, is that once you define the lifetime of an object properly it just comes into existence and goes away correctly. In many cases this lifetime management comes with zero runtime cost, or if there is a cost it is no more than the same code correctly written in C/C++.

Rust requires most heap allocated memory to be contained by one or more of the structs below. The struct manages the lifetime and access to the object inside ensuring the lifetime is managed correctly.

### Box<T>

A `Box` is something managed on the heap. If I create something created in a box, it's allocated by the heap.

Whoever owns the box can access it. Essentially, that means you can pass the box around from one place to another and whatever binds to it last can open it. Everyone else’s binding becomes invalid and will generate a compile error.

A box can be useful for abstraction since it can refer to a struct by a trait it implements allowing decoupling between types.

TODO example of a struct holding a box with a trait implemented by another struct

It can be useful for situations where one piece of code creates an object on behalf of another piece of code and hands it over. The Box makes sure that the ownership is explicit at all times and when the box moves to its new owner, so does the lifetime of the object itself.

### Cell<T>

A `Cell` is something that can copied with a `get()` or `set()` to overwrite its own copy. As the contents must be copyable they must implement the Copy trait.

The `Cell` has a zero-cost at runtime because it doesn’t have to track borrows but the restriction is it only works on Copy types. Therefore it would not be suitable for large objects or deep-copy objects.

### RefCell<T>

Somewhat more useful is the `RefCell<T>` but it incurs a runtime penalty to maintain read-write locks.

The `RefCell` holds a reference to an object that can be borrowed either mutably or immutably. These references are read-write locked so there is a runtime cost to this since the borrow must check if something else has already borrowed the reference.

Typically a piece of code might borrow the reference for a scope and then the borrow disappears when it goes out of scope. If a borrow happens before the last borrow releases, it will cause a panic.

## Reference Counting objects

Rust implements `Rc<>` and `Arc<>` for the purpose of reference counting objects that need to be shared and used by different parts of code. Rc<> is a single threaded reference counted wrapper, while `Arc<>` is atomic reference counted wrapper. You use one or the other depending on whether threads are sharing the object.

A reference counted object is usually wrapping a `Box`, `Cell` or `Refcell`. So multiple structs can hold a reference to the same object.

### Rc<T>

From `std::rc::Rc`. A reference counted object can be held by multiple owners at a time. Each own holds a cloned `Rc<T>` but the T contents are shared. The last reference to the object causes the contents to be destroyed.

### Arc<T>

From `std::sync::Arc`. An atomic reference counted object that works like `Rc<T>` except it uses an atomically incremented counter which makes it thread safe. There is more overhead to maintain an atomic reference count. If multiple threads access the same object they are compelled to use `Arc<T>`
