# A Guide to Porting C\/C++ to Rust

This book is for people familiar with C or C++ who are thinking of using Rust.

Before we go into what Rust is or why it might be preferable to C/C++ _in some cases_, let's think of software that is mission critical and must not or should not fail.

* Operating system services and daemons
* Internet of things devices
* Industrial control software
* Medical devices, imagery etc.
* High availability servers / databases / cloud storage etc.
* Avionics, telemetry, rocketry, drones etc.

All this code must run as efficiently and reliably as possible. It must run on devices for days, weeks, months or even years. It cannot suffer intermittent freezes, erratic performance, memory leaks, crashes or other issues without impacting on its purpose.

Now normally such software would be written in C or C++, but consider these _every day_ programming issues that those languages suffer from.

* Dangling pointers, where an invalid pointer is called
* Buffer overruns \/ underruns. Code writes beyond an allocated buffer causing memory corruption or a page exception.
* Memory leaks. Code does not free allocated memory causing the program to expand over time until the system fails.
* Data races. Multiple threads write to data at the same time causing data corruption or other destabilizing behavior.

Rust stops these bad things happening **by design**. 

* Object lifetimes are tracked automatically to prevent memory leaks and dangling pointers.
* The length of arrays and collections is enforced.
* Data race conditions are prevented by strict enforcement of mutex / guards and object ownership.
* Most enforcement happens at compile time and has zero or minimal runtime impact.

So a Rust program (which is compiled to machine code by the way), runs with similar performance and speed as C or C++ while greatly reducing the chance of errors.

In addition Rust plays well C. So you can write a dynamic or static library in Rust and use it from another language, or vice versa. So for example, you might have a Python application which has a mission / speed critical portion that you could write in Rust while leaving the rest of the code alone or port it gradually.