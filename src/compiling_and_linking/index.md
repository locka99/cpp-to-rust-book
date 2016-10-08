# Compiling and Linking in More Detail

## Your main() entry point

Rust has a main function just like C/C++ which is usually called main() .

It doesn’t take any arguments unlike C++ and it doesn’t return anything but neither of these are problems.

Note Rust can override the use of main() as the default entry point but you can override it using a special #[start] directive.

### Processing command-line arguments

In C/C++, the entry point takes argc, and argv arguments. Argc is the number of arguments and argv is an array of char * pointers that specify those arguments.

These are used to access the command-line parameters that the program was invoked with.

Rust doesn't expose them this way. Instead you can access the command-line parameters from std::env::args(). Namespacing is covered later, but std::env::args() means we are invoking the function called args() which resides inside a module env which resides inside a module std.

The function args() returns the parameters in a string array. As with C++, the first element of the array at index 0 is the command itself:

```rust
fn main() {
    for argument in std::env::args() {
        println!("{}", argument);
    }
}
```

We can see some clear advantages to how Rust supplies args:

* You don't need a separate argc, parameter. You get an array, the array defines its own length.
* You can access arguments from anywhere in your program, not just from the main(). In C++ you would have to pass your args around from one place to another. In Rust you can simply ask for them from anywhere.

### Exit code

If you want to exit with a code, you set it explicitly:

```rust
fn main() {
    //... my code
    std::os::set_exit_status(1);
}
```

When main() drops out, the runtime cleans up and returns the code to the environment. Again there is no reason the status code has to be set in main(), you could set it somewhere else and panic!() to cause the application to exit.

## Optimized compilation

In a typical edit / compile / debug cycle there is no need to optimize code and so Rust doesn't optimize

Aside from slowing down compilation it may obfuscate the code so that backtraces and debugging may not point at the proper lines of code in the source.

If you want to optimize your code, add a -O argument to rustc:

```
rustc -O hw.rs
```

The act of optimization will cause Rust to invoke the LLVM optimizer prior to linking. This will produce faster executable code at the expense of compile time.

## Incremental compilation

Incremental compilation is also important for edit / compile / debug cycles. Incremental compilation only rebuilds those parts of the code which have changed through modification to minimize the amount of time it takes to rebuild the product.

Rust has a different incremental compilation model to C++.

* C++ doesn't support incremental compilation per se. That function is left to the make / project / solution tool. Most maintain a current list of what file depends on what so if file foo.h changes then the makefile knows to invoke the compiler over foo.cpp and main.cpp and relink.
* In Rust incremental compilation is at the crate level - that if any file in a crate changes then the crate as a whole has to be rebuilt. Thus larger code bases tend to be split up into crates to reduce the incremental build time.

There is a recognition that this model kind of sucks especially if you have a large crate with lots of code.

The rust compiler is getting [incremental per-file compilation support](https://blog.rust-lang.org/2016/09/08/incremental.html
) in addition to per-crate.

At the time of writing this support is experimental because it is tied to refactoring the compiler for other reasons to improve performance and optimization but will eventually be enabled and supported by rustc and cargo.

## Managing a project

In C++ we would use a makefile or a solution file of some kind to manage a real world project and build it.

For small programs we might run a script or invoke a compiler directly but as our program grows and takes longer to build, we would have to use a makefile to maintain our sanity.

A typical makefile has rules that say what files are our sources, how each source depends on other sources (like headers), what our final executable is and a bunch of other mess about compile and link flags that must be maintained.

There are lots of different makefile solutions which have cropped up over the years but a simple gmake might look like one:

```
SRCS = main.o pacman.o sprites.o sfx.o
OBJS = $(SRCS:.cpp=.o)
EXE = pacman
$(EXE): $(OBJS)
	$(CC) $(CFLAGS) -o $(EXE) $(OBJS)
.cpp.o:
	$(CC) $(CFLAGS) -c $< -o $@
```

When you invoke "make", the software will check all the dependencies of your target, looking at their filestamps and determine which rules need to be invoked and which order to rebuild your code.

Rust makes things a lot easier – there is no makefile! Consider this main.rs for a pacman game:

```rust
mod pacman;

fn main() {
  let mut game = pacman::Game::new();
  game.start();
}
```

If we save this file and type "rustc main.rs" the compiler will notice the reference to "mod pacman" and will search for a pacman.rs (or pacman/mod.rs) and compile that too. It will continue doing this with any other modules referenced along the way.

In other words you could have a project with 1000 files and compile it as simply as "rustc main.rs". Anything referenced is automatically compiled and linked.

Okay, so we can call rustc, but what happens if our code has dependencies on other projects. Or if our project is meant to be exported so other projects can use it?

### Cargo
Rust recognizes that very few pieces of code have zero dependencies so it provides a package manager and dependency management tool for you called Cargo.

Cargo can fetch dependencies, build them, build and link your code, run unit tests, install binaries, produce documentation and upload versions of your project to a repository.

The easiest way to create a new project in Rust is to use the "cargo" command to do it

```
cargo new hello_world –bin
```

Creates this

```
hello_world/
  .git/ (git repo)
  .gitignore
  Cargo.toml
  src/
    main.rs
```

Building the project is then simply a matter of this:

```
cargo build
```

If you want to build for release you add a --release argument. This will invokes the rust compiler with optimizations enabled:

```
cargo build --release
```

If we wanted to build and run unit tests in our code we could write

```
cargo test
```

### Crates and external dependencies

Cargo doesn't just take care of building our code.
It created a Cargo.toml manifest in out project root. We could edit that file to say we have a dependency on an external library:

```
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Joe Blogs <jbloggs@somewhere.com>"]

[dependencies]
time = "0.1.35"
```

Now when we run "cargo build", it will fetch "time" from crates.io and dependencies that "time" has and build each in turn automatically. It does this efficiently so iterative builds do not incur a penalty. External crates are download and built in your .cargo home directory.

To use our external crate we declare it in the main.rs of our code, e.g.

```rust
extern crate time;
///
fn main() {
  let now = time::PreciseTime::now();
}
```

#### Cargo.lock

Also note that cargo maintains a Cargo.lock file in our root directory.

This file was generated when we did the "cargo build". It provides a manifest of what packages our project pulled in, their version, their source url and any dependencies they had their of their own.

This means if we invoke "cargo build" again the tool can exactly reproduce the same dependency configuration even from a clean configuration.
