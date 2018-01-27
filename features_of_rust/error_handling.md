# Error Handling

C++ allows code to throw and catch exceptions. As the name suggests, exceptions indicate an exceptional error. An exception is thrown to interrupt the current flow of logic and allows something further up the stack which to catch the exception and recover the situation. If nothing catches the throw then the thread itself will exit.

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

Most coding guidelines would say to use exceptions sparingly for truly exceptional situations, and use return codes and other forms of error propagation for ordinary failures. However C++ has no simple way to confer error information for ordinary failures and exceptions can be complicated to follow and can cause their own issues.

Rust does not support exceptions. Rust programs are expected to use a type such as `Option` or `Result` to propagate errors to their caller. In other words, the code is expected to anticipate errors and have code to deal with them.

The `Option` enum either returns `None` or `Some` where the `Some` is a payload of data. It's a generic enum that specifies the type of what it may contain:

```rust
enum Option<T> {
   None
   Some(T)
}
```

For example, we might have a function that searches a database for a person's details, and it either finds them or it doesn't.

```rust
struct Person { /* ... */}

fn find_person(name: &str) {
   let records = run_query(format!("select * from persons where name = {}", sanitize_name(name)));
   if records.is_empty() {
      None
   }
   else {
      let person = Person::new(records[0]);
      Some(person)
   }
}
```

The `Result` enum either returns a value of some type or an error of some type.

```rust
enum Result<T, E> {
  Ok(T),
  Err(E)
}
```

So we might have a function `set_thermostat` for setting the room temperature. 

```rust
fn set_thermostat(temperature: u16) -> Result<(), String> {
   if temperature < 10 {
     err(format!("Temperature {} is too low", temperature))
   }
   else if temperature > 30 {
     err(format!("Temperature {} is too high", temperature))
   }
   else {
     Ok(())
   }
}
// ...
let result = set_thermostat();
if result.is_ok() {
  // ...
}
```

This function will return a unity `()` value for success, or a `String` for failure.

## The ? directive

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

Older versions of Rust used a special `try!()` macro for this same purpose (not to be confused with `try-catch` in C++) which does the same thing. So if you see code like this, it would be the same as above.

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
