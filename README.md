# A Guide to Porting C\/C++ to Rust

This book is for people familiar with C or C++ who are thinking of using Rust.


The first question is obvious why? We'll go into that in more detail but let's think of software that is mission critical and must not or should not fail.

* Operating system services and daemons
* Internet of things devices
* Industrial control software
* Medical devices, imagery etc.
* High availability servers / databases / cloud storage etc.
* Avionics, telemetry, rocketry, drones etc.

All this code that has to run as efficiently and reliably as possible. It is running on constrained environments and it should work reliably for days, weeks, months or even years with the minimal of errors. It cannot suffer sudden freezes or expanding / collapsing memory behavior due to garbage collection.

In some cases, no general purpose programming would meet the needs of the system. Some hardware such as programmable logic controllers will use specialised languages like IEC1131 programming languages. 

This document talks about like for like. Where you might have used C or C++ but want extra reliability.

Consider these common problems in C++.

* Dangling pointers. These problems occur when code calls a pointer \(or reference\) to an object that have been deleted.
* Buffer overruns \/ underruns. Code writes beyond an allocated buffer causing memory corruption or a page exception.
* Memory leaks. Code does not free allocated memory causing the program to expand over time and potentially fail.
* Data races. Multiple threads write to data at the same time causing data corruption or other destabilizing behavior.

Rust stops these bad things happening **by design**.

* Object lifetimes are tracked automatically to prevent memory leaks and dangling pointers.
* The length of arrays and collections is enforced.
* Data race conditions are prevented by strict enforcement of mutex / guards and object ownership.
* Most enforcement happens at compile time and has zero or minimal runtime impact.

These checks are enforced at compile and run time so they come with zero cost, or at least no more cost than the same code written correctly in C\/C++. And Rust compiles to machine code similar to C\/C++ with no runtime overheads such as garbage collection.

In addition Rust plays well with other languages. You can write static or dynamic libraries that you can link to other languages and you can call static or dynamic libraries written in other languages.

In addition Rust plays well with C and C++. You can write libraries in Rust that you can call from C or C++ and you can call C or C++ libraries from Rust itself. Rust enforces safety by default but allows unsafe operations if they are explicitly marked as such.
