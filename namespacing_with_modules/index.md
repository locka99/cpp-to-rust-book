# Namespacing With Modules

In C++ a namespace can be used to split your functions, variables and classes into logical blocks and allow the compiler to disambiguate them from other functions, variables and classes that might otherwise have the same name.
Namespacing in C++ is completely optional which means some code doesn’t do it at all while other code does.
e.g.

```c++
// Namespacing is usually a good idea
namespace myapp {
  void error() {
    //...
  }
  const int SOME_VALUE = 20;
  void doSomething(int value) {
    //...
  }
}
```

The equivalent in Rust is a module and serves a similar purpose.  Unlike C++ you don't need to explicitly create a module because each file is a module in its own right.

If a function bar() is in a file called foo.rs, the function can be referenced foo::bar(). It's implied and you don't have to do anything except name your file in a meaningful way.

But if you want an explicit module you may write it like so in the file it is being used from:

```rust
mod myapp {
  pub fn error() { /* ... */ }
  pub const SOME_VALUE = 20;
  pub fn doSomething(value: i32) { /* ... */ }
}
```

And to call a module we just apply the mod as a qualifier, much how its done in C++.

```rust
myapp::doSomething(myapp::SOME_VALUE);
```

You could also just bring in the whole of the mod if you like:

```rust
use myapp::*;
doSomething(SOME_VALUE);
```

Or just the types and functions within it that you use:

```rust
use myapp::{doSomething, SOME_VALUE}
doSomething(SOME_VALUE);
// Other bits can still be referenced by their qualifying mod
myapp::error();
```

Modules can be nested too, e.g. myapp.rs might declare a mod "helpers" within itself which can be referenced by fully qualifying it myapp::helpers::.

## Splitting modules across files

Namespacing with modules is pretty easy, But sometimes you might have lots of files in a module and you don't want the outside world to see a single module namespace.

In these cases you’re more likely to use the myapp/mod.rs form. In this instance the mod.rs file may pull

in subordinate files

```rust
// myapp/mod.rs
mod helpers;
mod gui;

#[cfg(test)]
mod tests

// Perhaps we want the outside world to see myapp::Helper
pub use helpers::Helper;
```

In this example, the module pulls in submodules helpers and gui. It also references tests if the unit tests are being built. It also calls "pub use helpers::Helper" which exposes that struct to the outside world as myapp::Helper.
