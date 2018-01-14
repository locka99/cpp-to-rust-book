# Multithreading

Multithreading allows you to run parts of your programming concurrently, performing tasks in parallel. Every program has a *main* thread - i.e. the one your `main()` started from, in addition to which are any that you create.

Examples of reasons to use threads:

* Long running operations, e.g. zipping up a large file.
* Activity that is blocking in nature, e.g. listening for connections on a socket
* Processing data in parallel, e.g. physics, collision detection etc.
* Asynchronous activities, e.g. timers, polling operations.

In addition, if you use a graphical toolkit, or 3rd party libraries they may spawn their own threads that you do not know about. 

## Thread safety

One word you will hear a lot in multithreading is thread safety. 

By that we mean:

* Threads should not be able to modify the data at the same time. When this happens it is called a data race and can corrupt the data, causing a crash. e.g. two threads trying to append to a string at the same time.
* Threads must not lock resources in a way that could cause deadlock i.e. thread 1 obtains a lock on resource B and blocks on resource A, while thread 2 obtains a lock on resource A and blocks on resource B. Both threads are locked forever waiting for a resource to release that never will be.
* Race conditions are bad, i.e. the order of thread execution produces unpredictable results on the output from the same input.
* APIs that can be called by multiple threads must either protect their data structures or make it an explicit problem of the client to sort out.
* Open files and other resources that are accessed by multiple threads must be managed safely.

### Protecting shared data

Data should never be read at the same time it is written to in another thread. Nor should data be written to at the same time by two threads.

The common way to prevent this is either:

* Use a mutex to guard access to the data. A mutex is a special class that only one thread can lock at a time. Other threads that try to lock the mutex will wait until the lock held by another thread is relinquished
* Use a read-write lock. Similar to a mutex, it allows one thread to lock the thread for writing data, however it permits multiple threads to have read access, providing nothing is already writing to it. For data that is read more frequently than it is modified, this is a lot more efficient than just a mutex.

### Avoiding deadlock

The best way to avoid deadlock is only ever obtain a lock to one thing ever and release it as soon as you are done. But if you have to lock more than one thing, ensure the locking order is consistent between all your threads. So if thread 1 locks A and B, then ensure that thread 2 also locks A and B in that order and not B then A. The latter is surely going to cause a deadlock.

## C / C++

C and C++ predate threading to some extent so the languages have never had much built-in support for multi-threading. Instead the compiler will have code in its stdlib for threading, combined with a dependency on some operating system APIs.

A consequence of this is that C and C++ have ZERO ENFORCEMENT of any the rules mentioned above. If you data race - too bad. If you forget to write a lock in one function even if you remembered all the others - too bad. You have to discipline yourself to think concurrently and apply the proper protections where it is required. 

The consequence of not doing so may not even be felt until your software is in production and that one customer starts complaining that their server freezes about once a week. Good luck finding that bug!

### Multithreading APIs

The most common APIs would be:

* std::thread - from C++11 onwards
* POSIX threads, or pthreads. Exposed by POSIX systems such as Linux and most other Unix derivatives, e.g. OS X. There is also pthread-win32 support built over the top of Win32 threads.
* Win32 threads. Exposed by the Windows operating system.
* OpenMP. Supported by many C++ compilers.
* 3rd party libraries like Boost and Qt provide wrappers that abstract the differences between thread APIs. 

All APIs will have in common:

* Thread creation, destruction, joins (waiting on threads) and detaches (freeing the thread to do what it likes).
* Synchronization between threads using locks and barriers.
* Mutexes - mutual exclusion locks that protect shared data.
* Conditional variables - a means to signal and notify of conditions becoming true.

### std::thread

The `std::thread` represents a single thread of execution and provides an abstraction over platform dependent ways of threading.

```c++
#include <iostream>
#include <thread>

using namespace std;

void DoWork(int loop_count) {
    for (int i = 0; i < loop_count; ++i) {
        cout << "Hello world " << i << endl;
    }
    pthread_exit(NULL);
}

int main() {
    thread worker(DoWork, 100);
    worker.join();
}
```

The example spawns a thread which invokes the function and passes the parameter into it, printing a message 100 times.

### POSIX threads

The pthreads API is prefixed `pthread_` and works like so:

```c++
#include <iostream>
#include <pthread.h>

using namespace std;

void *DoWork(void *data) {
    const int loop_count = (int) data;
    for (int i = 0; i < loop_count; ++i) {
        cout << "Hello world " << i << endl;
    }
    pthread_exit(NULL);
}

int main() {
    pthread_t worker_thread;
    int result = pthread_create(&worker_thread, NULL, DoWork, (void *) 100);
    // Wait for the thread to end
    result = pthread_join(worker_thread, NULL);
}
```

This example spawns a thread which invokes DoWork with the payload of 100 which causes the function to print a message 100 times.

### Win32 Threads

Win32 threading has functions analogous to those in POSIX. They have names such as `CreateThread`, `ExitThread`, `SetThreadPriority` etc.

### OpenMP API

Open Multi-Processing (OpenMP) is an API for multi-threaded parallel processing. OpenMP relies on compiler support because you use special `#pragma` directives in your source to control thread creation and access to data.

GCC, Clang and Visual C++ have support for OpenMP so it is an option.

OpenMP is a complex standard but the use of directives can make for cleaner code than invoking threading APIs directly. The downside is it is also more opaque hiding what the software is doing, making it considerably more difficult to debug.

OpenMP is described in detail at the OpenMP [website](http://www.openmp.org/).

### Thread local storage 

Thread local storage, or TLS is static or global data which is private to every thread. Each thread holds its own copy of this data so it can modify it without fear of causing a data race.

Compilers also have proprietary ways to decorate types as thread local:

```c++
__thread int private; // gcc / clang
__declspec(thread) int private; // MSVC
```

C++11 has gained a `thread_local` directive to decorate variables which should use TLS.

```c++
thread_local int private
```

## Rust

Rust has threading support built into its language and enforced by the compiler.

We saw with C++ that you had to be disciplined to remember to protect data from race conditions. Rust doesn't give you that luxury - you MUST protect your data!

### Protecting data

#### Mutex

TODO

#### ReadWriteLock

TODO

### Thread pools

TODO

### Thread local storage

TODO