# Error Handling

C++ allows code to throw and catch exceptions.

```c++
void do_something() {
  if (!read_file()) {
    throw std::runtime_error("read_file didn't work!");
  }
}
...
try {
  do_something();
}
catch (std::exception e) {
   std::cout << "Caught exception -- " << e.what() << std::endl;
}
```

Exceptions have been a mixed blessing for C++. On the one hand they provide a mechanism for propagating errors in some way. On the other they break the flow of the program and make it jump around in ways that not simple to follow.

They also really don't cross library boundaries, and it is easy for code to use them excessively or not at all.

Most coding guidelines would say to use exceptions sparingly for truly exceptional situations, and use return codes and other forms of error propagation for ordinary failures. The problem is that C++ has no simple way to confer error information.

Rust does not support exceptions. Rust programs are expected to use a type such as `Option` or `Result` to propagate errors to their caller. In other words, the code is expected to anticipate errors and have code to deal with them.

The `Option` enum either returns something or none. It's a generic enum that specifies the type of what it may contain:

TODO Option examples

A `Result` either returns something or an error. It's a generic enum that specifies the success and error types.

TODO Result

And of course if neither of this are suitable, then code can always create its own enum type that returns what they want. Since enums can contain data, they serve the case of functions that can return different stuff.

## Nuclear option - panic!()

If code really wants to do something equivalent to a throw / catch in C++ it may call panic!().

This is NOT recommended for dealing with regular errors, only irregular ones that the code has no way of dealing with.

This macro will cause the thread to abort and if the thread is the main programme thread, the entire process will exit.

A panic!() can be caught and should be if Rust is being invoked from another language. The way to catch an unwinding panic is a closure at the topmost point in the code where it can be handled.

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    panic!("Bad things");
});
```
