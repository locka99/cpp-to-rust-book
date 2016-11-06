# Debugging Rust

Rust compiles to machine code - executable code - in just the same manner as C\/C++. A compiled binary may contain symbolic information that allows a debugger to "step" through the code, set a "breakpoint" and do the normal manner of debugging operations that you might expect.

The main thing to be aware of debugging Rust is knowing which backend your compiled executable came through. On Windows, the choice is between MSVC and GNU. Depending on your choice you must debug within Visual Studio or use gdb. On other platforms you may have the choice of using gdb or lldb.

