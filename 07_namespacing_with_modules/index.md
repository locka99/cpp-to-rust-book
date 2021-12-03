# Namespacing With Modules

C++ namespaces allow you to group your functions, variables and classes into logical blocks and allow the compiler to disambiguate them from other functions, variables and classes that might otherwise have the same name.

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
//... somewhere else in the code
myapp::doSomething(100);
```

Namespacing in C++ is completely optional which means some code may use nest namespaces while other code may be content to cover its entire codebase with a single namespace. Some code might even put its code into the global namespace. Other code might control the use of namespaces with macros.

The equivalent to a namespace in Rust is a module and serves a similar purpose.  Unlike C++ though you get namespacing automatically from the structure of your files. Each file is a module in its own right.

So if we may have a file myapp.rs

```rust
// myapp.rs
pub fn error() { /* ... */ }
pub const SOME_VALUE: i32 = 20;
pub fn doSomething(value: i32) { /* ... */ }
```

Everything in myapp.rs is automatically a module called myapp. That means modules are implicit and you don't have to do anything except name your file something meaningful.

```rust
use myapp;
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

But if you want an explicit module you may also write it in the code. So perhaps myapp doesn't justify being a separate file.

```
// main.rs
mod myapp {
  pub fn error() { /* ... */ }
  pub const SOME_VALUE = 20;
  pub fn doSomething(value: i32) { /* ... */ }
}
```

Modules can be nested so a combination of implicit modules \(from file names\) and explicit modules can be used together.

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

In this example, the module pulls in submodules `helpers` and `gui`. Neither is marked as `pub mod` so they are private to the module.

However the module also says `pub use helpers::Helper` which allows the outside to reference `myapp::Helper`. Thus a module can act as a gatekeeper to the things it references, keeping them private or selectively making parts public.

We haven't mentioned the other module here `tests`. The attribute `#[cfg(test)]` indicates it is only pulled in when a unit test executable is being built. The `cfg` attribute is used for [conditional compliation](https://doc.rust-lang.org/book/conditional-compilation.html).

## Using a module

Modules can be used once they are defined.

```rust
use helpers::*;
```

Note that the use command is relative to the toplevel `main` or `lib` module. So if you declare a `mod helpers` at the top, then the corresponding `use helpers` will retrieve it. You can also use relative `use` commands with the `super` and `self` keywords.

// TODOs

## Module aliasing

TODO

## External crates

TODO

