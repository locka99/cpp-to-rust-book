# A Guide to Porting C\/C++ to Rust

This book is for people familiar with C or C++ who are thinking of using Rust.

Before we go into what Rust is or why it might be preferable to C/C++ _in some cases_, let's think of software that is mission critical and must not or should not fail.

* Operating system services and daemons
* Internet of things devices
* Industrial control software
* Medical devices, imagery etc.
* High availability servers / databases / cloud storage etc.
* Avionics, telemetry, rocketry, drones etc.

All this code must run as efficiently and reliably as possible. It must run on devices for days, weeks, months or preferably years without failure. It cannot suffer intermittent freezes, erratic performance, memory leaks, crashes or other issues without impacting on its purpose.

Normally such software would be written in C or C++, but consider these _every day_ programming issues that those languages suffer from.

* Dangling pointers, where an invalid pointer is called causing a crash.
* Buffer overruns \/ underruns. Code writes beyond an allocated buffer causing memory corruption or a page exception.
* Memory leaks. Code does not free allocated memory causing the program to expand over time until the system fails.
* Data races. Multiple threads write to data at the same time causing data corruption or other destabilizing behavior.

Rust stops these bad things happening **by design**. And it does so without impacting on runtime performance.

* Object lifetimes are tracked automatically to prevent memory leaks and dangling pointers.
* The length of arrays and collections is enforced.
* Data race conditions are prevented by strict enforcement of mutex / guards and object ownership.
* Most enforcement happens at compile time and has zero or minimal runtime impact.

So a Rust program (which is compiled to machine code), runs with similar performance and speed as C or C++ while massively reducing the chance of errors. Any errors are caught at compile time, well before they can hit the software deployed in the field.

In addition Rust plays well C. So if you have a large code base you can choose to refactor a part of it. You are not forced to rewrite everything in one go. The Firefox browser is following this strategy to replace parts of itself over time for code written in Rust while still relying on older code written in C or C++.
