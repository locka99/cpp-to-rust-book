# Compiling and Linking in More Detail

## Your main\(\) entry point

Rust has a main function just like C\/C++ which is usually called `main()`. [^1]

It doesn’t take any arguments and it doesn’t return anything unlike C\/C++. Let's see how we might do those things.

### Processing command-line arguments

In C\/C++, the entry point takes argc, and argv arguments. Argc is the number of arguments and argv is an array of char \* pointers that specify those arguments.

```c++
int main(int arcg, char **argv) {
  // our code
}
```

Processing arguments can become inordinately complex \(and buggy\) so most software will use a function like `getopt()` or `getopt_long()` to simplify the process.

Note that `getopt()` is not a standard C function and is not portable, e.g. to Windows. So immediately we see an example of problem that C\/C++ forces us to solve.

Rust doesn't process arguments this way. Instead you access the command-line parameters from `std::env::args()` from anywhere in the code. That is to say, there is a function called `args()` under the namespace `std::env` that returns the strings on the command-line. 

The function `args()` returns the parameters in a string array. As with C++, the first element of the array at index 0 is the command itself:

```rust
fn main() {
    for argument in std::env::args() {
        println!("{}", argument);
    }
}
```

Rust also supplies a [getopts](https://doc.rust-lang.org/getopts/getopts/) crate that simplifies argument processing if it is necessary.

We can see some clear advantages to how Rust supplies args:

* You don't need a separate argc, parameter. You have an array that defines its own length.
* You can access arguments from anywhere in your program, not just from the `main()`. In C++ you would have to pass your args around from one place to another. In Rust you can simply ask for them from anywhere.

### Exit code

If you want to exit with a code, you set it explicitly:

```rust
fn main() {
    //... my code
    std::os::set_exit_status(1);
}
```

When main\(\) drops out, the runtime cleans up and returns the code to the environment. Again there is no reason the status code has to be set in main\(\), you could set it somewhere else and panic!\(\) to cause the application to exit.

## Optimized compilation

In a typical edit \/ compile \/ debug cycle there is no need to optimize code and so Rust doesn't optimize unless you ask it to.

Optimization takes longer to happen and can reorder the code so that backtraces and debugging may not point at the proper lines of code in the source.

If you want to optimize your code, add a -O argument to rustc:

```
rustc -O hw.rs
```

The act of optimization will cause Rust to invoke the LLVM optimizer prior to linking. This will produce faster executable code at the expense of compile time.

## Incremental compilation

Incremental compilation is also important for edit \/ compile \/ debug cycles. Incremental compilation only rebuilds those parts of the code which have changed through modification to minimize the amount of time it takes to rebuild the product.

Rust has a different incremental compilation model to C++.

* C++ doesn't support incremental compilation per se. That function is left to the make \/ project \/ solution tool. Most builders will track a list of project files and which file depends on other files. So if file foo.h changes then the builder knows what other files depend on it and ensures they are rebuilt before relinking the target executable.
* In Rust incremental compilation is at the crate level - that if any file in a crate changes then the crate as a whole has to be rebuilt. Thus larger code bases tend to be split up into crates to reduce the incremental build time.

There is a recognition in the Rust community that the crate-level model can suck for large crates so the Rust compiler is getting [incremental per-file compilation support](https://blog.rust-lang.org/2016/09/08/incremental.html) in addition to per-crate.

At the time of writing this support is experimental because it is tied to refactoring the compiler for other reasons to improve performance and optimization but will eventually be enabled and supported by rustc and cargo.

## Managing a project

In C++ we would use a makefile or a solution file of some kind to manage a real world project and build it.

For small programs we might run a script or invoke a compiler directly but as our program grows and takes longer to build, we would have to use a makefile to maintain our sanity.

A typical makefile has rules that say what files are our sources, how each source depends on other sources \(like headers\), what our final executable is and a bunch of other mess about compile and link flags that must be maintained.

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

Rust makes things a lot easier – there is no makefile! The source code is the makefile. Each file says what other files it uses via depencies on other crates, and on other modules.

Consider this main.rs for a pacman game:

```rust
mod pacman;

fn main() {
  let mut game = pacman::Game::new();
  game.start();
}
```

If we save this file and type "rustc main.rs" the compiler will notice the reference to "mod pacman" and will search for a pacman.rs \(or pacman\/mod.rs\) and compile that too. It will continue doing this with any other modules referenced along the way.

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

Cargo doesn't just take care of building our code, it also ensures that anything our code depends on is also downloaded and built. These external dependencies are defined in a Cargo.toml in our project root.

We can edit that file to say we have a dependency on an external "crate" such as the time crate:

```
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Joe Blogs <jbloggs@somewhere.com>"]

[dependencies]
time = "0.1.35"
```

Now when we run `cargo build`, it will fetch "time" from crates.io and also any dependencies that "time" has itself. Then it will build each crate in turn automatically. It does this efficiently so iterative builds do not incur a penalty. External crates are download and built in your .cargo home directory.

To use our external crate we declare it in the main.rs of our code, e.g.

```rust
extern crate time;

fn main() {
  let now = time::PreciseTime::now();
  println!("The time is {:?}", now);
}
```

So the change to the Cargo.toml and a reference in the source is sufficient to:

1. Fetch the crate \(and any dependencies\)
2. Build the crate \(and any dependencies\)
3. Compile and link to the crate and dependencies

All that happened with a line in Cargo.toml and a line in our code to reference the crate. We didn't have to mess around figuring how to build the other library, or maintain multiple makefiles, or getting our compiler \/ linker flags right. It just happened.

#### Cargo.lock

Also note that once we build, cargo creates a Cargo.lock file in our root directory.

This file is made so that if `cargo build` is invoked again it has an exact list of what packages need to be pulled and compiled. It stops situations where the code under our feet (so to speak) moves and suddenly our project no longer builds. So if the lock file exists, the same dependency configuration can be reproduced even from a clean. If you want to force the cargo to rebuild a new lock file, e.g. after changing Cargo.toml, you can type `cargo update`.

[^1]: You can change the main entry point using a special  `#[start]` directive if you want on another function but the default is main\(\)
