# Let's Start Simple

The usual introduction to any language is "Hello, World!".  A simple program that prints that message out to the console.

Here is how we might write it for C:

```c++
#include <stdio.h>

int main(int argc, char *argv[]) {
  printf("Hello, World!\n");
  return 0;
}
```

We could write it the same way for C++, or we could use the C++ stream classes if we preferred:

```c++
#include <iostream>

using namespace std;

int main(int argc, char *argv[]) {
  cout << "Hello, World!" << endl;
  return 0;
}
```

And here is the equivalent in Rust:

```rust
fn main() {
  println!("Hello, World!");
}
```

There are some obvious points of similarity that we can observe:

* C/C++ and Rust follow the convention of having a `main()` function as the entry point into code. Note that Rust's main doesn't return anything. It's effectively a void method.
* There is a general purpose print statement.
* The general structure in terms of main, use of { } and semi-colons is mostly the same. In both languages a block of code is enclosed in curly braces, and a semi-colon is used as a separator between statements.
* Rust looks a little bit more terse than either C or C++ because it automatically includes references to part of its standard runtime that it refers to as its "prelude".

The `println!()` is actually a macro that expands into code that writes to the standard output. We know it's a macro because it ends in a ! character but you may treat it like a function call for now. We'll see how Rust macros differ to those in C/C++ later.

## Compiling our code

Open a command prompt and set up your compiler environments.

If you were using gcc, you’d compile your code like this:

```bash
gcc hw.cpp -o hw
```

If you were using Microsoft Visual C++ you'd compile like this:

```bash
cl /o hw.exe hw.cpp
```

To compile in Rust you invoke the rustc compiler.

```bash
rustc hw.rs
```

And to run either

```bash
./hw (or .\hw.exe)
Hello, World!
```

Again there are points of similarity:

* There is a shell command that compiles the code and creates an executable from it.
* The binary runs in the same way.

A less obvious point of similarity is that Rust shares its code generation backend with gcc-llvm and clang. Rustc outputs llvm bitcode which is compiled \(and optimized\) into machine code via LLVM. This means the resulting executable is very similar in form to that output by C++ compilers. That includes the symbolic information it supplies for debugging purposes. A rust executable can be debugged in gdb, lldb or Microsoft Visual Studio depending on the target platform.

```bash
rustc -O hw.rs
```

