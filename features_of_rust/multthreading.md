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

C and C++ predate threading to some extent so until C++11 the languages have had little built-in support for multi-threading and what there was tended to be compiler specific extensions.

A consequence of this is that C and C++ have ZERO ENFORCEMENT of thread safety. If you data race - too bad. If you forget to write a lock in one function even if you remembered all the others - too bad. You have to discipline yourself to think concurrently and apply the proper protections where it is required. 

The consequence of not doing so may not even be felt until your software is in production and that one customer starts complaining that their server freezes about once a week. Good luck finding that bug!

### Multithreading APIs

The most common APIs would be:

* `<thread>`, `<mutex>` - from C++11 onwards
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
}

int main() {
    thread worker(DoWork, 100);
    worker.join();
}
```

The example spawns a thread which invokes the function and passes the parameter into it, printing a message 100 times.

### std::mutex

C++ provides a family of various `mutex` types to protect access to shared data.

The mutex is obtained by a `lock_guard` and other attempts to obtain the mutex are blocked until the lock is relinquished.

```c++
#include <iostream>
#include <thread>
#include <mutex>

using namespace std;

mutex data_guard;
int result = 0;

void DoWork(int loop_count) {
	for (auto i = 0; i < loop_count; ++i) {
		lock_guard<mutex> guard(data_guard);
		result += 1;
	}
}

int main() {
	thread worker1(DoWork, 100);
	thread worker2(DoWork, 150);
	worker1.join();
	worker2.join();
	cout << "result = " << result << endl;
}
```

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

We saw with C++ that you had to be disciplined to remember to protect data from race conditions. 

Rust doesn't give you that luxury -

1. Any data that you share must be protected in a thread safe fashion
2. Any data that you pass between threads must be marked thread safe

### Spawning a thread

Spawning a thread is easy enough by calling `spawn`, supplying the closure you want to run in the context of your new thread.

```rust
use std::thread;

thread::spawn(move || {
  println!("Hello");
});
```

Alternatively you can supply a function to `spawn` which is called in the same manner.

```rust
fn my_thread() {
  println!("Hello");
}
//...
thread::spawn(my_thread);
```

If you supply a closure then it must have a lifetime of `'static` because threads can outlive the thing that created them. i.e. they are detached by default. 

A closure can make use of move values that are marked `Send` so the compiler allows ownership to transfer between threads.

Likewise function / closure may also return a value which is marked `Send` so the compiler can transfer ownership between the terminating thread and the thread which calls `join` to obtain the value.

So the thread above is detached. If we wanted to wait for the thread to complete, the `spawn` returns a `JoinHandle` that we can call `join` to wait for termination.

```rust
let h = thread::spawn(move || {
  println!("Hello");
});
h.join();
```

If the closure or function returns a value, we can use `join` to obtain it.

```rust
let h = thread::spawn(move || 100 * 100);
let result = h.join().unwrap();
println!("Result = {}", result);
```

### Data race protection in the compiler

Data races are bad news, but fortunately in Rust the compiler has your back. You MUST protect your shared data or it won't compile.

The simplest way to protect your data is to wrap the data in a mutex and provide each thread instance with a reference counted copy of the mutex.

```rust
let shared_data = Arc::new(Mutex::new(MySharedData::new()));

// Each thread we spawn should have a clone of this Arc
let shared_data = shared_data.clone();
thread::spawn(move || {
  let mut shared_data = shared_data.lock().unwrap();
  shared_data.counter += 1;
});
```

Here is a full example that spawns 10 threads that each increment the counter.

```rust
struct MySharedData {
  pub counter: u32,
}

impl MySharedData {
  pub fn new() -> MySharedData {
    MySharedData {
	  counter: 0
	}
  }
}

fn main() {
  spawn_threads();
}

fn spawn_threads() {
  let shared_data = Arc::new(Mutex::new(MySharedData::new()));
  
  // Spawn a number of threads and collect their join handles
  let handles: Vec<JoinHandle<_>> = (0..10).map(|_| {
	let shared_data = shared_data.clone();
    thread::spawn(move || {
	  let mut shared_data = shared_data.lock().unwrap();
	  shared_data.counter += 1;
	})
  }).collect();
  
  // Wait for each thread to complete
  for h in handles {
    h.join();
  }
  
  // Print the data
  let shared_data = shared_data.lock().unwrap();
  println!("Total = {}", shared_data.counter);
}
```

So the basic strategy will be this:

1. Every thread will get it's own atomic reference to the mutex. 
2. Each thread that wishes to access the shared must obtain a lock on the mutex.
3. Once the lock is released, the next waiting thread can obtain access.
3. The compiler will enforce this and generate errors if ANYTHING is wrong.

### Read Write Lock

A read write lock works much like a mutex - we wrap the shared data in a `RwLock`, and then in an `Arc`.

```rust
let shared_data = Arc::new(RwLock::new(MySharedData::new()));
```

Each thread will then either need to obtain a read lock or a write lock on the shared data.

```rust
let shared_data = shared_data.read().unwrap();
// OR
let mut shared_data = shared_data.write().unwrap();
```

The advantage of a `RwLock` is that many threads can concurrently read the data, providing nothing is writing to it. This may be more efficient in many cases.

### Sending data between threads using channels

TODO mpsc channel

### Thread local storage

As with C++ you may have reason to use thread local storage

```rust
thread_local! {
  // TODO
}
```
