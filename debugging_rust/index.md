# Debugging Rust

Rust compiles in to machine code the same as C and benefits from sharing the same ABI and compiler backend formats as C/C++.

Consequently, it is possible to debug compiled Rust in the same way as C/C++.

```
c:\dev\visu>rustup show
Default host: x86_64-pc-windows-msvc

stable-x86_64-pc-windows-msvc (default)
rustc 1.13.0 (2c6933acc 2016-11-07)
```

## Enabling backtrace

If your code is crashing because of a panic!() you can get a backtrace on the console by setting the `RUST_BACKTRACE` environment variable.

```
# Windows
set RUST_BACKTRACE=1
# Unix/Linux
export RUST_BACKTRACE=1
```

## Microsoft Visual Studio

If you have the MSVC toolchain the LLVM backend will generate a .pdb file and binaries will be compatible with the standard MSVC runtime.

To debug your code:

1. Open Visual Studio
2. Choose File | Open | Project/Solution...
3. Select the compiled executable
4. Open a source file to debug and set a breakpoint
5. Click the "Start" button

## GDB

GDB can be invoked directly from the command line or through a plugin / IDE. From the command line it's a

TODO

## LLDB

TODO
