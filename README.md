# A Guide to Porting C\/C++ to Rust

This book is for people familiar with C or C++ who are thinking of using Rust.

Before we go into what Rust is or why it might be preferable to C/C++ _in some cases_ , let's think of software that is mission critical and must not or should not fail.

* Operating system services and daemons
* Internet of things devices
* Industrial control software
* Medical devices, imagery etc.
* High availability servers / databases / cloud storage etc.
* Avionics, telemetry, rocketry, drones etc.

All this code must run as efficiently and reliably as possible. It must run on devices for days, weeks, months or preferably years without failure. It cannot suffer intermittent freezes, erratic performance, memory leaks, crashes or other issues without impacting on its purpose.

Normally such software would be written in C or C++, but consider these _every day_ programming issues that afflict these languages:

* Dangling pointers. A program calls an invalid pointer causing a crash.
* Buffer overruns \/ underruns. Code writes beyond an allocated buffer causing memory corruption or a page exception.
* Memory leaks. Code does not free allocated memory causing the program to consume memory over time until it or the system fails.
* Data races. Multiple threads write to data at the same time causing corruption or other destabilizing behavior.

Rust stops these bad things happening **by design**. And it does so without impacting on runtime performance because all of these things are checked at compile time:

* Object lifetimes are tracked automatically to prevent memory leaks and dangling pointers.
* The length of arrays and collections is enforced.
* Data race conditions are prevented by strict enforcement of mutex / guards and object ownership.

Code that passes the compiler's checks is transformed into machine code with similar performance and speed as the equivalent C or C++. 

This is a "zero-cost" approach. The compiler enforces the rules so that there is zero runtime cost over the equivalent and correctly written program in C or C++. Safety does not compromise performance. 

In addition Rust plays well C. You may invoke C from Rust or invoke Rust from C using foreign function interfaces. You can choose to rewrite a critical section of your codebase leave the remainder alone. 

For example, the Firefox browser uses Rust to analyse video stream data - headers and such like where corrupt or malicious code could destabilize the browser or even be exploitable. 
