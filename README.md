# A Guide to Porting C/C++ to Rust

This book is an introduction to Rust as well as a comparative guide for people working with C/C++. It will provide the justification on *why* you might want to switch or at least use Rust for critical portions of your software.

As a taster consider these common problems in C++.

* Dangling pointers. These problems occur when code calls a pointer (or reference) to an object that have been deleted.
* Buffer overruns / underruns. Code writes beyond an allocated buffer causing memory corruption or a page exception.
* Memory leaks. Code does not free allocated memory causing the program to expand over time and potentially fail.
* Data races. Multiple threads write to data at the same time causing data corruption or other destabilizing behavior.

Rust stops these bad things happening **by design**.

* Object lifetimes are tracked automatically to prevent memory leaks and dangling pointers.
* The length of arrays and collections is enforced.
* Data race conditions are prevented by strict enforcement of mutex / guards and object ownership.

Using Rust can make your code more reliable without compromising on performance. Most of the checks above are enforced at compile time so they come with zero cost, or at least no more cost than the same code written correctly in C++. Compiled Rust executes as fast as compiled C or C++.

In addition Rust plays well with C and C++. You can write libraries in Rust that you can call from C or C++ and you can call C or C++ libraries from Rust itself. Rust enforces safety by default but allows unsafe operations if they are explicitly marked as such.