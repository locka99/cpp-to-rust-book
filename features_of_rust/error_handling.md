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

Exceptions have been a mixed blessing for C++. On the one hand they provide a mechanism for propagating errors and an orderly unwiding of the stack.

On the other they break the flow of the program and make it jump around in ways that not simple to follow.

Most coding guidelines would say to use exceptions sparingly for truly exceptional situations, and use return codes and other forms of error propagation for ordinary failures. The problem is that C++ has no simple way to confer error information for ordinary failures.

Rust does not support exceptions. Rust programs are expected to use a type such as `Option` or `Result` to propagate errors to their caller. In other words, the code is expected to anticipate errors and have code to deal with them.

The `Option` enum either returns something or none. It's a generic enum that specifies the type of what it may contain:

TODO Option examples

A `Result` either returns something or an error. It's a generic enum that specifies the success and error types.

TODO Result

And of course if neither of this are suitable, then code can always create its own enum type that returns what they want. Since enums can contain data, they serve the case of functions that can return different stuff.

## The ? directive (old try!() macro)

Let's say you have 2 functions `delete_user` and `find_user`. The function `delete_user` first calls `find_user` to see if the user even exists and then proceeds to delete the user or return the error code that it got from `find_user`.

```rust
fn delete_user(name: &str) -> Result<(), ErrorCode> {
  let result = find_user(name);
  if let Ok(user) = result {
     // ... delete the user
     Ok(())
  }
  else {
    Err(result.unwrap_err())
  }
}

fn find_user(name: &str) -> Result<User, ErrorCode> {
  //... find the user OR
  Err(ErrorCode::UserDoesNotExist)
}
```

We have a lot of code in `delete_user` to handle success or failure in `find_user` and throw its failure code upwards. So Rust provides a convenience `?` mark on the end of the call to a function that instructs the compiler to generate the if/else branch we hand wrote above, reducing the function to this:

```rust
fn delete_user(name: &str) -> Result<(), ErrorCode> {
  let user = find_user(name)?;
  // ... delete the user
  Ok(())
}
```

Providing you want to propogate errors up the call stack, this can eliminate a lot of messy conditional testing in the code and make it more robust.

Older versions of Rust supplied this functionality through a special `try!()` macro so you may still see crates that use this style instead:

```rust
fn delete_user(name: &str) -> Result<(), ErrorCode> {
  let user = try!(find_user(name));
  // ... delete the user
  Ok(())
}
```

## Nuclear option - panic!\(\)

If code really wants to do something equivalent to a throw / catch in C++ it may call panic!\(\).

This is NOT recommended for dealing with regular errors, only irregular ones that the code has no way of dealing with.

This macro will cause the thread to abort and if the thread is the main programme thread, the entire process will exit.

A panic!\(\) can be caught and should be if Rust is being invoked from another language. The way to catch an unwinding panic is a closure at the topmost point in the code where it can be handled.

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    panic!("Bad things");
});
```
