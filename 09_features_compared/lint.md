# Lint

C/C++ compilers can issue many useful warnings but the amount of static analysis they can do is usually quite limited.

The Rust compiler performs a far more rigorous lifecycle check on data and then follows up with a lint check that inspects your code for potentially bad or erroneous

In particular it looks for:

* Dead / unused code
* Unreachable code
* Deprecated methods
* Undocumented functions
* Camel case / snake case violations
* Unbounded recursion code (i.e. no conditionals to stop recursion)
* Use of heap memory when stack could be used
* Unused extern crates, imports, variables, attributes, mut, parentheses
* Using "while true {}" instead of "loop {}"

Lint rules can be enforced more strictly or ignored by using attributes:

```rust
#[allow(rule)]
#[warn(rule)]
#[deny(rule)]
#[forbid(rule)]
```

A full list of lint rules can be found by typing "rustc -W help":

```
                         name  default  meaning
                         ----  -------  -------
                box-pointers   allow    use of owned (Box type) heap memory
           fat-ptr-transmutes  allow    detects transmutes of fat pointers
 missing-copy-implementations  allow    detects potentially-forgotten implementations of `Copy`
missing-debug-implementations  allow    detects missing implementations of fmt::Debug
                 missing-docs  allow    detects missing documentation for public members
                trivial-casts  allow    detects trivial casts which could be removed
        trivial-numeric-casts  allow    detects trivial casts of numeric types which could be removed
                  unsafe-code  allow    usage of `unsafe` code
...
```

There are a lot more checks than listed above.
