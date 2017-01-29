# Memory Allocation

Allocated memory is memory that is requested from a portion of memory called a heap, used for some purpose and returned to the free space when it is no longer required.

There are 3 ways to heap allocate memory in C++:

* Through C functions like `malloc`, `calloc` and `free` for buffers, arrays.
* Through `new` / `delete` for C++ class instances
* Through `new[]` and `delete[]` for buffers, arrays of C++ classes

If we fail to free / delete memory that we've allocated, the program will leak memory. If we free / delete memory we've already deallocated, the program may crash. If we free a C++ class with a C `free()` the program may leak memory because any member variables will not be destroyed properly. If we fail to call the correct constructor and destructor pair the program may leak / crash.

A cottage industry of tools has sprung up just to try and debug issues with memory leaks, crashes and so forth. Tools like Valgrind etc. specialise in trying to figure out who allocated something without freeing it.

For example, what's wrong with this?

```c++
std::string *strings = new std::string[100];
//...
delete strings;
```

Oops we allocated an array of strings with `new[]` but called `delete` instead of `delete[]`. So instead of deleting an array of strings we called delete on the first member. 99 of those string's destructors will never be called.
We should have written:

```c++
delete []strings;
```

But the compiler doesn't care and so we have created a potentially hard-to-find bug.

Some of the problems with memory allocation can be mitigated by wrapping pointers with scoped or shared pointer classes. But there are even problems which can prevent them from working.

It's not a good idea to allow memory allocation to cross a library boundary. So many libraries provide new / free functions through their API. Issues about balancing calls apply to them too.

## How Rust helps

During normal safe programming Rust has no explicit memory allocation or deallocation. We simply declare an object and it continues to exist until its lifetime goes out of scope (i.e. nothing refers to it any more). This is NOT garbage collection since the compile tracks the lifetime of the object and generates code to automatically deletes it at the point it is no longer used. The compiler also knows if we enclose an object's declaration inside a cell, box, rc or similar construct that the object should be allocated on the heap and otherwise it should go on the stack.

Allocation / deallocation is only available in unsafe programming. We would not only ordinarily do this except when we are interacting with an external library or function call and explicitly tag the section as unsafe.
