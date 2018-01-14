# Multithreading

Multithreading allows you to run parts of your programming concurrently. Every program has a *main* thread - i.e. the one your `main()` started from. In addition, if you use a graphical toolkit, or timers, or 3rd party libraries they might well spawn their own threads.

## Thread safety

One word you will hear a lot in multithreading is thread safety. 

By that we mean:

* No two threads should be able to modify the same data at the same time. This is called a data race. Data races are bad news because data can be corrupted, potentially causing a crash.
* No two threads should obtain mutually exclusive locks to each other's resources that could cause deadlock i.e. thread one obtains a lock on B and waits on A, while thread two obtains a lock on A and waits on B. When this happen both threads are locked forever and a program can freeze.
* No race conditions. Where the order of thread execution produces unpredictable results. This can be caused by failing to manage control the inputs and outputs of threads, or failing to wait correctly for threads to complete. For example if the main thread terminates without terminating a worker thread, the worker may not clean up properly.
* APIs that can be called by multiple threads must ensure to protect their data structures to prevent any of the problems above from arising. Sometimes APIs take the easy way out and stuff all their context into a context object and it is up to the caller to ensure the thread safety.

Therefore there is a necessary discipline required for multithreading or bad things will happen.

### Protecting shared data

Data should never be read at the same time it is written to in another thread. Nor should data be written to at the same time by two threads.

The common way to prevent this is either:

* Use a mutex to guard access to the data. A mutex is a special class that only one thread can lock at a time. Other threads that try to lock the mutex will wait until the lock held by another thread is relinquished

* Use a read-write lock. Similar to a mutex, it allows one thread to lock the thread for writing data, however it permits multiple threads to have read access, providing nothing is writing to it. Therefore it's more suitable for data which is read more frequently than it is modified.

### Avoiding deadlock

The best way to avoid deadlock is only ever obtain a lock to one thing ever. But if you have to lock more than one thing, ensure the locking order is consistent between all your threads. So if thread 1 locks A and B, then ensure that thread 2 also locks A and B in that order.

## C / C++

C and C++ predate threading to some extent so the languages have never had much built-in support for multi-threading. Instead the compiler will have code in its stdlib for threading, combined with a dependency on some operating system APIs.

The most common APIs would be:

* POSIX threads, or pthreads. Exposed by POSIX systems such as Linux and most other Unix derivatives, e.g. OS X.
* Win32 threads. Exposed by the Windows operating system
* Some compilers may also expose ad hoc functions which map onto the OS mechanism
* 3rd party libraries like Boost and Qt will abstract away the differences behind cross-platform thread classes. 

All APIs will have in common:

* Thread creation, destruction and joins (waiting on threads)
* Sychronization between threads using locks and barriers
* Mutexes. Mutual exclusion locks to protect shared data.
* Conditional variables - means to signal and notify of conditions becoming true

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

TODO