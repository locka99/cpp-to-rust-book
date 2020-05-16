# Rust Background

So Rust began life as a research project by Graydon Hoare in 2009 for the Mozilla foundation to solve issues related to developing in C++. It progressed until the release of version 1.0 in 2015 and has since iterated through releases every six weeks since.

## Why a

The motivating force for a new language isn't hard to see if you consider the Mozilla Firefox web browser. Like most web browsers it is:

* Written in C++. [^1]
* Has millions of lines of code.
* Complex
* Vulnerable to bugs, vulnerabilities and exploits, many of which are attributable to the language the software is written in.
* Subsystems are mostly single-threaded and therefore not well suited for many-core devices - PCs, phones, tablets etc. Implementing multi-threading to the existing engine would doubtless cause even more bugs and vulnerabilities than being single threaded.

Rust was conceived as a way to obtain C or C++ levels of performance but also remove entire classes of software problem that destabilize software and could be exploited. Code that passes the compiler phase could be guaranteed to be memory safe and therefore could be written in a way to take advantage of concurrency. It has even been coined as _fearless concurrency_ because it removes a lot of the uncertainty and doubt that comes from typical concurrent code - data race conditions and so on.

The project is hosted on [GitHub](https://github.com/rust-lang/rust). The language has been _self-hosting_ for quite some time - that is to say the Rust compiler is written in Rust, so compiling Rust happens from a compiler written in Rust. Get your head around that! But it's the same way that C and C++ compilers are these days too.

[^1] Read this [Mozilla internal string guide](https://developer.mozilla.org/en-US/docs/Mozilla/Tech/XPCOM/Guide/Internal_strings) to get a flavor of the sort of problems the browser had to overcome. A browser obviously uses a lot of temporary strings. STL strings were too inefficient / flakey from one compiler to the next and so the browser sprouted an entire tree of string related classes to solve this issue. Similar tales are told in Qt and other large libraries.

## Safe by design

Rust enforces a safe-by-design philosophy. Simply put, the language and the compiler try to stop errors from happening in the first place. 

For example the compiler rigorously enforces lifetime tracking on objects and generates errors on violations. That means you cannot hold a reference to an object which has been destroyed unlike in C++.

Most of these checks and guards are done at compile time so there is a zero-cost at runtime.

## Active Development

The Rust team releases a new version of Rust approximately every 6 weeks. This means Rust receives code and speed improvements over time.

Most releases focus on marking APIs as stable, improving code optimization and compile times.

## Open source and free

Rust is dual licensed under the Apache 2.0 and MIT open source licenses. The full copyright message is viewable [online](https://github.com/rust-lang/rust/blob/master/COPYRIGHT).

Essentially the license covers your right to modify and distribute the Rust source code. Note that Rust generates code for LLVM so LLVM also has its own software license \(TODO link\).

What you compile with Rust \(or LLVM\) is not affected by the open source license. So you may compile, execute and distribute proprietary code without obligation to these licenses.

## Is Rust for everybody?

No of course not. Performance and safety are only two things to consider when writing software.

* Sometimes it's okay for a program to crash every so often
* If you have code that's written and works then why throw that away?
* Writing new code will always take effort and will still cause application level bugs of one sort or another.
* Performance may not be a big deal especially for network bound code and a higher level language like Java, C\#, Go may suit better.
* Some people will find the learning curve extremely steep. 
* Rust is still relatively immature as a language and still has some rough edges - compilation times, optimization, complex macros.

But you may still find there is benefit to moving some of your code to Rust. For example, your C++ software might work great but it has to deal with a lot of user-generated data so perhaps you want to reimplement that code path in Rust for extra safety.

## Safe by design

Some examples of this safe-by-design philosophy:

* Variable \(binding\) is immutable by default. This is the opposite of C++ where mutable is the default and we must explicitly say const to make something immutable. Immutability extends to the &self reference on struct functions.
* Lifetime tracking. The Rust compiler will track the lifetime of objects and can generate code to automatically drop them when they become unused. It will generate errors if lifetime rules are violated.
* Borrowing / Variable binding. Rust enforces which variable "owns" an object at any given time, and tracks values that are moved to other variables. It enforces rules about who may hold a mutable or immutable reference to it. It will generate errors if the code tries to use moved variables, or obtain multiple mutable references to it.
* There is no NULL pointer in safe code. All references and pointers are valid because their lifetimes and borrowing are tracked.
* Rust uses LLVM for the backend so it generates optimized machine code.
* Lint checking is builtin, e.g. style enforcement for naming conventions and code consistency.
* Unit tests can be integrated into the code and run automatically
* Modules \(equivalent to namespaces C++\) are automatic meaning we implicitly get them by virtue of our file structure.

## Don't C++11 / C++14 / C++17 get us this?

Yes and no. C++11 and C++14 certainly bring in some long overdue changes. Concurrency primitives \(threads at last!\), move semantics, pointer ownership and other beneficial things all come in with these latest standards. Conveniences such as type inference, lambdas et al also come in.

But let's look at some of the issues

### Move semantics

Move semantics in C++ require you implement explicit move constructors on classes you wish to use then. Any class that doesn't is using copy semantics. The compiler will not care either that you leave the old class instance in a valid state as a result of your implementation. The compiler will even let you call your old class instance despite having moved data to a new instance. Move errors can be handled at runtime with a sentinel state. Simply put, move in C++ is complex and blogs [like this](https://foonathan.net/blog/2017/09/14/destructive-move.html) spend a long time explaining the gory details.

Remember in Rust you just get them for free, and they are enforced by the compiler.

### Smart pointers

Smart pointers can still be null dereferenced. Null dereferencing is undefined behaviour and can crash or compromise your code. C++ strongly prefers that you use references to avoid null dereferencing, but it doesn't check the lifetimes of references so it is still not always possible. Certainly data structures like trees and graphics virtually preclude using references.

C++ introduces an `std::optional` type that attempts to fulfill the same purpose as `Option` in Rust. There is a `nullopt`, equivalent to `None`, and `std::optional<T>(value)` to hold a value. 

### Concurrency

Concurrency requires correct programming habits - ensuring to avoid concurrency errors like data races by using guards. The compiler will not help if you do not write correct code.

#### Programming safety

Perhaps if you program the right subset of features and diligently work to avoid pitfalls of C++ in general then you are more likely to create safe code. If you search for "C++ coding guidelines" you will find no end of articles that try to minimize risky practice. 

But what is the _right_ subset?

* If you use someone else's library - are they using the right subset?
* If one subset is right then why does C++ still contain all the stuff that is outside of that?
* Why are all the things which are patently unsafe / dangerous still allowed?
* Why are certain dangerous default behaviors such as default copy constructors not flipped to improve code safety?

We could argue that C++ doesn't want to break existing code by introducing change that requires code to be modified. That's fair enough but the flip-side is that future code is almost certainly going to be broken by this decision. Perhaps it would be better to inflict a little pain for some long term gain.

## Unsafe programming / C interoperability

Rust recognizes you may need to call an external libraries, e.g. in a C library or a system API.

Therefore it provides an `unsafe` keyword that throws some of the safety switches when it is necessary to talk to the outside world.

This allows you consider the possibility of porting code partially to Rust while still allowing some of it to remain as C.

