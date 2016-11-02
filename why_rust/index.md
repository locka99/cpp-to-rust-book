# Why Rust?

The C and C++ are the defacto languages for producing very fast close-to-the-metal software. The problem is not with that but with their safe-by-design. 

It is trivial \(by accident\) to write code that is in error such as causing a memory leak. It is easy \(by malice\) to exploit badly written code to force it into error. It easy with the best testing in the world for some of these errors to only manifest themselves when the code is in production.

At best, bugs are a costly burden for developers to find and fix, not just in time and dollars but also their reputation. At worst, the bug could causes catastrophic failure but more ordinarily leaves code unstable or vulnerable to hacking.

Rust is a language that produces machine code that is comparable in performance as C\/C++ but enforces a safe-by-design philosophy. Simply put, the language and the compiler try to stop errors from happening in the first place. For example the compiler rigorously enforces lifetime tracking on objects and generates errors on violations. Most of these checks and guards are done at compile time so there is a zero-cost at runtime.

Some examples of this safe-by-design philosophy:

* Variable \(binding\) is immutable by default. This is the opposite of C++ where mutable is the default and we must explicitly say const to make something immutable. Immutability extends to the &self reference on struct functions.
* Lifetime tracking. The Rust compiler will track the lifetime of objects and can generate code to automatically drop them when they become unused. It will generate errors if lifetime rules are violated.
* Borrowing \/ Variable binding. Rust enforces which variable "owns" an object at any given time, and tracks values that are moved to other variables. It enforces rules about who may hold a mutable or immutable reference to it. It will generate errors if the code tries to use moved variables, or obtain multiple mutable references to it.
* There is no NULL pointer in safe code. All references and pointers are valid because their lifetimes and borrowing are tracked.
* Rust uses LLVM for the backend so it generates optimized machine code.
* Lint checking is builtin, e.g. style enforcement for naming conventions and code consistency.
* Unit tests can be integrated into the code and run automatically
* Modules \(equivalent to namespaces C++\) are automatic meaning we implicitly get them by virtue of our file structure.

## Don't C++11 \/ C++14 get us this?

Yes and no. C++11 and C++14 certainly bring in some long overdue changes. Concurrency primitives \(threads at last!\), move semantics, pointer ownership and other beneficial things all come in with these latest standards. Conveniences such as type inference, lambdas et al also come in.

And perhaps if you program the right subset of features and diligently work to avoid pitfalls of C++ in general then you are more likely to create safe code.

But what is the _right_ subset?

* if you use someone else's library - are they using the right subset?
* if one subset is right then why does C++ still contain all the stuff that is wrong?
* Why are all the things which are patently unsafe \/ dangerous still allowed?
* Why are certain dangerous default behaviors such as default copy constructors not flipped to improve code safety?

We could argue that C++ doesn't want to break existing code by introducing change that requires code to be modified. That's fair enough but the flip-side is that future code is almost certainly going to be broken by this decision. Perhaps it would be better to inflict a little pain for some long term gain.

## Unsafe programming \/ C interoperability

Rust recognizes you may need to call an external libraries, e.g. in a C library or a system API.

Therefore it provides an unsafe keyword that throws some of the safety switches when it is necessary to talk to the outside world.

This allows you consider the possibility of porting code partially to Rust while still allowing some of it to remain as C.

