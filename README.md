# A Guide to Porting C/C++ to Rust

The book is an introduction to Rust as well as a comparative guide for people working with C/C++. The book also provides some justification as to *why* you might want to switch or at least use Rust for critical portions of your software.

As a taster consider these common problems in C++.

* Dangling pointers - writing to an address that contains garbage or is invalid causing a page exception.
* Buffer overruns / underruns - code that writes beyond the ends of the allocated buffer causing memory corruption or a page exception.
* Memory leaks - code that doesn't free allocated objects properly and eventually the system runs out of memory.
* Data races - code doesn't protect its data structures from concurrent access causing a race condition when one thread reads while another is writing.

Rust stops these bad things happening **by design**.

* It tracks the lifetime of objects preventing memory leaks, dangling pointer calls, ownership problems.
* It prevents buffer overflows by enforcing the length of arrays and other collections.
* It prevents data race conditions in threads by using mutex / guards to protect data.
* Most of these enforcements have no additional runtime cost over the same (correctly written) code in C++.
* The compiler catches violations and generates efficient, correct machine code.
* It offers modern conveniences of other high level languages - type  inference, collections, build & package management.
* It can still invoke C libraries and system APIs when necessary.

Simply put, using Rust can make your code more reliable without compromising on performance.
