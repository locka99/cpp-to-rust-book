# Linker Errors

C and C++ requires you supply a list of all the .obj files that form part of your library or executable.

If you omit a file by accident you will get undefined or missing references. Maintaining this list of files is an additional burden of development, ensuring to update your makefile or solution every time you add a file to your project.

## How Rust Helps

Rust includes everything in your library / executable that is directly or indirectly referenced by mod commands, starting from your toplevel lib.rs or main.rs and working all the way down.

Providing you reference a module, it will be automatically built and linked into your binary.
