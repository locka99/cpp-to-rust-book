# Memory Management

Structures that you declare in C++ or Rust reside on the stack or they reside in the heap.

## Stack

The stack is a memory reserved by the operating system for each thread in your program. Stack is reserved for local variables based upon their predetermined size by moving a stack pointer register forward by that amount. When the local variables go out of scope, the stack pointer reduces by the same amount.

```rust
// Stack allocated
double pi = 3.141592735;
{
  // Stack pointer moves as values goes in and out of scope
  int values[20] = { 0, 1, 2, 3, ... , 19, 20 };
}
```

In C-style languages it is normal for the stack in each thread to be a single contiguous slab of memory that represents the "worst case" scenario for your program i.e. you will never need any more stack than the thread allocated at start. If you do exceed the stack, then you cause a stack overflow.

Some languages support the concept of split or segmented stack. In this case, the stack is a series of "stacklets" joined together by a linked list. When the stack is insufficient for the next call, it allocates another stacklet.

The gcc can support a segmented stack, but it greatly complicates stack unwinding when an exception is thrown and also when calls are made across linker boundaries, e.g. between a segmented-stack aware process and a non segmented stack dynamic library.

## Stack Overflows

The main worry from using the stack is the possibility of a stack overflow, i.e the stack pointer moves out of the memory reserved for the stack and starts trampling on other memory.

This can occur in two common ways in isolation or combination:

* Deeply nested function calls, e.g. a recursive function that traverses a binary tree, or a recursive function that never stops
* Exhausting stack by using excessive and\/or large local variables in functions, e.g. lots of 64KB byte arrays.

### C++

Some C++ compilers won't catch an overflow at all. They have no guard page and thus allow the stack pointer to just grow whereever memory takes it until the program is destabilized and crashes.

The gcc compiler has support segmented stacks but as described earlier not without issue.

The MSVC compiler adds a guard page and stack pointer checks when when the stack pointer could advance more than a page in a single jump and potentially miss the guard page.

### Rust

Rust used to support a segmented stack as a means of detecting memory violation but since 1.4 has replaced it with a guard page at the end of the stack space. If the guard page is touched by a memory write, it will generate a segmentation fault that halts the thread. Guard pages open up a small risk that the stack could grow well in excess of the guard and it might take some time for a write to the guard to generate a fault.

Rust aspires to support stack probe code generation on all platforms at which point it is likely to use that in addition to a guard page. A stack probe is additional generated code on functions that use more than a page of space for local variables to test if they exceed the stack.

Rust reduces the risk stack overflows in some indirect ways. It's easy in C++ through inheritance or by calling a polymorphic method inadvertently set off a recursive loop

## Heap

Heap is a memory that the language runtime requests from the operating system and makes available to your code through memory allocation calls:

```c++
char * v = (char *) malloc(128);
memset(v, 0, 128);
strcpy(v, "Hello world");
//...
free(string);

double *values = new double[10];
for (int i = 0; i < 10; i++) {
  values[i] = double(i);
}
delete []values;
```

Allocation simply means a portion of the heap is marked as in-use and the code is provided with a pointer to the reserved area to do what it likes with. Free causes the portion to be returned to its free state, coalescing with any free areas that it resides next to in memory.

A heap can grow and code might create multiple heaps and might even be compelled to in order control problems such as heap fragmentation.

## Boxing objects

To allocate memory on the heap in Rust you must put a struct in a box. For example to create a 1k block of bytes:

```rust
let x: Box<[u8]> = Box::new([0; 1024]);
```

Many structs in std:: and elsewhere will have a stack based portion and also use use heap internally to hold their buffers.

## Heap fragmentation

Heap fragmentation happens when contiguous space in the heap is limited by the pattern of memory allocations that it already contains. When this happens a memory allocation can fail and the heap must be grown to make it succeed. In systems which do not have virtual memory \/ paging, memory exhaustion caused by fragmentation can cause the program or even the operating system to fail completely.

The easiest way to see fragmentation is with a simple example. We'll pretend there is no housekeeping structures, guard blocks or other things to get in the way. Imagine a 10 byte heap, where every byte is initially free.

Now allocate 5 bytes for object of type A. The heap reserves 5 bytes and marks them used.

![](/assets/aaaaa-----.png)

Now allocate 1 byte for object of type B. This is also marked used.

![](/assets/aaaaaab.png)

Now free object A. The the portion of heap is marked unused. Now we have a block of 5 bytes free and a block with 4 bytes free.

![](/assets/-----b----.png)

Now allocate 2 bytes for object of type C. Now we have a block of 3 bytes free and a block with 4 bytes free.

![](/assets/cc---b----.png)

Now allocate 5 slots for object of type A - Oops we can't! The heap has 7 bytes free but they are not contiguous. At this point the runtime would be forced to grow the heap, i.e. ask the operating system for another chunk of memory at which point it can allocate 5 bytes for A.

![](/assets/cc---baaaaa---.png)

The above assumes the heap is a contiguous, or that memory paging makes it seem so. On some systems, it might be that the heap is a linked list of chunks, in which case the allocated space for A would have to reside be in a single chunk, the newly allocated portion above. 

This is also an exagerated example, but it demonstrates how heap can have space, but not enough to fufilly allocations without growing.

Software running in embedded devices are particularly vulnerable to fragmentation because they do not have virtual memory, have low physical memory and normally have to run for days, weeks or years at a time.

One major problem for C++ is that heap fragmentation is almost impossible to avoid. The standard template library allocates memory for virtually all string and collection work, and if a string \/ collection grows then it may have to reallocate more memory.

The only way to mitigate the issue is to choose the best collection, and to reserve capacity wherever possible.

```c++
std::vector<double> values;
values.reserve(10);
for (int i = 0; i < 10; i++) {
  values.push_back(double(i));
}
```

Rust also has this issue and strings \/ collections have methods to reserve capacity. But as a consequence of its design it prefers the stack over the heap. Unless you explicitly allocate memory by putting it into a Box, Cell or RefCell you do not allocate it on the heap.

## RAII

RAII stands for Resource Acquisiton Is Initalization. It's a programming pattern that ties access to some resource the object's lifetime

C++ classes allow a pattern called RAII \(\). A class constructor acquires some resource, the destructor releases that resource. As soon as the class goes out of scope, the resource is released.

**TODO **C++ example

Rust is inherently RAII and enforces it through lifetimes. When an object goes out of scope, the thing it holds is released. Rust also allows the programmer to explicitly drop a struct earlier than its natural lifetime if there is a reason to.

RAII is most commonly seen for heap allocated memory but it can apply to files, system handles etc.

**TODO **Rust example

