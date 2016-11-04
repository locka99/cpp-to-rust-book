# A Guide to Porting C\/C++ to Rust

This book is for people with C\/C++ who might be considering porting to Rust. It provides an introduction to Rust, supplies reasons you might want to fix and goes through some of the problems that you may have in your existing code that Rust would outright prevent.

As a taster consider these common problems in C++.

* Dangling pointers. These problems occur when code calls a pointer \(or reference\) to an object that have been deleted.
* Buffer overruns \/ underruns. Code writes beyond an allocated buffer causing memory corruption or a page exception.
* Memory leaks. Code does not free allocated memory causing the program to expand over time and potentially fail.
* Data races. Multiple threads write to data at the same time causing data corruption or other destabilizing behavior.

Rust stops these bad things happening **by design**.

* Object lifetimes are tracked automatically to prevent memory leaks and dangling pointers.
* The length of arrays and collections is enforced.
* Data race conditions are prevented by strict enforcement of mutex \/ guards and object ownership.

These checks are enforced at compile and run time so they come with zero cost, or at least no more cost than the same code written correctly in C\/C++. And Rust compiles to machine code similar to C\/C++ with no runtime overheads such as garbage collection.

In addition Rust plays well with other languages. You can write static or dynamic libraries that you can link to other languages and you can call static or dynamic libraries written in other languages.

