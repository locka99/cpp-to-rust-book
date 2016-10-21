# Foreign Function Interface

Rust recognizes that you might want to call a system function or use an external library written in C.

## Calling out to C libraries

Rust supports the concept of a foreign function interface which is a definition of an external function or type that is resolved at link time.

```rust
#[link(name = "foo")]
extern {
  fn foo_command(command: *mut u8)
}
```

If you call this function you have to wrap it in an unsafe block to disable the safety checks:

```rust
pub fn run_command(command: &[u8]) {
  unsafe {
    foo_command(command.as_ptr());
  }
}
```

## Making Rust code callable

Imagine you have a bottleneck in some code that was written in Python. You might not wish to rewrite the entire Python application in Rust, but you might wish to port the bottleneck portion to be Rust. In which case you need to call from Python to Rust and you need to build your Rust component with a public C-callable API.

```rust
#[no_mangle]
pub extern fn hello_world() {
  // Your code here
}
```

The FFI specification goes into a lot more detail than this and explains concepts such as callbacks, structure packing, stdcall, linking and other issues that allow full interoperability.
There are also crates that have the definitions of structures, types and functions for standard C, Win32, OpenGL etc.
TODO a stdc examples
TODO a Win32 example
