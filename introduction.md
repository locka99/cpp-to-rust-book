# Introduction

We'll cover some of the problems with C / C++ later in detail, but think how many bugs in C / C++ are caused by the following:

* Dangling pointers - writing to an address that contains garbage or is invalid causing a page exception.
* Buffer overruns / underruns - code that writes beyond the ends of the allocated buffer causing memory corruption or a page exception.
* Memory leaks - code that doesn't free allocated objects properly and eventually the system runs out of memory.
* Data races - code doesn't protect its data structures from concurrent access causing a race condition when one thread reads while another is writing.

These problems apply to C and C++ but of course C++ piles its own problems on top that compound these issues.
Rust tries to stop a lot of the bad things happening **by design**.

* It tracks the lifetime of objects preventing memory leaks, dangling pointer calls, ownership problems.
* It prevents buffer overflows by enforcing the length of arrays and other collections.
* It prevents data race conditions in threads by using mutex / guards to protect data.
* Most of these enforcements have no additional runtime cost over the same (correctly written) code in C++.
* The compiler catches violations and generates efficient, correct machine code.
* It offers modern conveniences of other high level languages - type  inference, collections, build & package management.
* It can still invoke C libraries and system APIs when necessary.

Simply put, using Rust can make your code more reliable without compromising on performance.

## Active Development

TODO

The Rust team releases a new version of Rust approximately every 6 weeks. This means Rust receives code and speed improvements over time.

Most releases focus on marking APIs as stable, improving code optimization and compile times.

## Open source and free

Rust is dual licensed under the Apache 2.0 and MIT open source licenses. The full copyright message is viewable [online](https://github.com/rust-lang/rust/blob/master/COPYRIGHT).

Essentially the license covers your right to modify and distribute the Rust source code. Note that Rust generates code for LLVM so LLVM also has its own software license (TODO link).

What you compile with Rust (or LLVM) is not affected by the open source license. So you may compile, execute and distribute proprietary code without obligation to these licenses.

## Is Rust for everybody?

No of course not. Performance and safety are only two things to consider when writing software. Sometimes you want something quick and dirty. Other times you can tolerate a few crashes.

If you have code that's written that does what you want then why throw that away?

Or if your code spends much of its life waiting for other things to happen - network requests, database queries and so on then performance may not be a big deal.

But you may find there is partial benefit to moving some of your code to Rust. For example, your C++ software might work great but it has to deal with a lot of user-generated data so perhaps you want to reimplement that code path in Rust for extra safety.




Console output / commands is given in this style

Most of the code samples are abbreviated in some fashion. e.g. they assume the code is running from within some function or they omit preambles. They may also assume a namespace / module to reduce the amount of noise.
