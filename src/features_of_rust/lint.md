# Lint

C/C++ compilers can issue a lot of warnings but they usually supplemented by code analysis tools that attempt to perform deeper inspection for problems that a compiler would not pick up.

The Rust compiler contains a lint check that extends beyond the syntactic correctness of your code and looks for potential errors that you may not notice.

In particular it looks for:

* Dead / unused code
* Unreachable code
* Deprecated methods
* Undocumented functions
* Camel case / snake case violations
* Unbounded recursion code (i.e. no conditionals)
* Unused extern crates, imports, variables, attributes, mut, parentheses
* Using while true{} instead of loop
* Lint rules can be enforced more strictly or ignored by using attributes:
* TODO

```rust
#[allow(rule)]
#[warn(rule)]
#[deny(rule)]
#[forbid(rule)]
```

Where rule is an underscored version of the rules supported by the compiler. A full list of lint rules can be found by typing "rustc -W help".

There are a lot more than are listed here.
