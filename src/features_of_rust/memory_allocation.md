# Memory allocation

This section is concerned with memory allocation, i.e. creating objects that reside on the heap and not on the stack, and the manner in which they are created and are destroyed.

## C++

C and C++ have various standard ways to allocate memory:

1. malloc/calloc() and free()
2. new and delete (C++ only)
3. new[] and delete[] for arrays (C++ only)
4. System functions for creating heaps, allocating memory from them. Under the covers all of the above calls map onto system functions at the discretion of the runtime.


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

The use of malloc()/free() is generally deprecated in C/C++ since it does not invoke constructors or destructors on classes or any of their members.

An allocation must be matched by the corresponding free action so immediately we can see scope for error here:

1. Not using the correct new & delete pair, e.g. calling delete instead of delete[]
2. Forgetting to free memory causing a leak.
3. Freeing memory more than once
4. Calling a dangling pointer, i.e. a pointer which refers to freed memory.

```c++
Node *nodes = new Node[100];
// Oops, potential memory leak - only destructor of first node was invoked
delete nodes;
```

C/C++ do not enforce lifetimes on memory allocation. C++ has smart pointers which impose a life time on objects

```
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

There are *many* other solutions to managing memory. The lack of any concrete lifetimes means every library has its own concept of ownership which is usually different from one to the next. Boost for example has scoped_ptr and shared_ptr which work in a similar fashion to those in C++11.

## Rust

As you can guess by now Rust tends to be a lot more strict about allocation that C/C++. Lifetimes of objects are tracked and enforced by the compiler and that includes memory allocated objects.

In normal safe programming there is no explicit new / delete so there is no way to forget to free an object. There are no pointers either so code cannot call a dangling pointer.

1. A Box is a managed pointer that holds a heap allocated object. A box cannot be cloned, so there is only one owner at any time.
2. A Cell is a mutable memory location - it can hold any kind of copyable type and the value within it can be changed.
3. A RefCell is a mutable memory location that can hold a reference

Rust requires most heap allocated memory to be contained by one or more of the structs below. The struct manages the lifetime and access to the object inside ensuring the lifetime is managed correctly.

### Box<T>

A Box is a managed pointer. It holds a reference to some data but there can only be one valid reference to the box itself. Essentially, that means you can pass the box around from one place to another and whoever binds to it last can open it. Everyone else’s binding is invalid will generate a compile error.

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

Rust implements Rc<> and Arc<> for the purpose of reference counting objects that need to be shared and used by different parts of code. Rc<> is a single threaded reference counted wrapper, while Arc<> is atomic reference counted wrapper. You use one or the other depending on whether threads are sharing the object.

A reference counted object is usually wrapping a Box, Cell or Refcell. So multiple structs can hold a reference to the same object.

### Rc<T>

From std::rc::Rc. A reference counted object can be held by multiple owners at a time. Each own holds a cloned Rc<T> but the T contents are shared. The last reference to the object causes the contents to be destroyed.

### Arc<T>

From std::sync::Arc. An atomic reference counted object that works like Rc<T> except it uses an atomically incremented counter which makes it thread safe. There is more overhead to maintain an atomic reference count. If multiple threads access the same object they are compelled to use Arc<T>
