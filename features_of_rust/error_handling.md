# Error Handling

## C++

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

Most coding guidelines would say to use exceptions sparingly for truly exceptional situations, and use return codes and other forms of error propagation for ordinary failures.

However C++ has no simple way to confer error information for ordinary failures. Here are some common ways they may work:

* Functions that return a `bool`, an `int`, or a pointer with special meaning. e.g. `false`, `-1` or `NULL` for failure.
* Functions that return a result code or enum. This might have a `GOOD` value and a bunch of `ERROR_` values. An extreme example would be `HRESULT` used by Windows that bitpacks information about goodness, severity and origin into a result and requires macros to extract the information.
* Functions that have a special out parameter that is filled in with additional detail in the case of failure.
* Functions that provide further information about the last error in `errno()` or some similar function supplied by the library.
* Exceptions that are thrown for any failure and must be caught. 
* Exceptions that are thrown sometimes and error codes are returned other times.
* Functions that are overloaded into two forms, one that throws an exception, another that stores the error in an error parameter. The boost library has functions like this.

Since there is no consistent way to deal with errors, every library and function has its own ad hoc way to return information.

## Rust

Rust provides two enumeration types called `Result` and `Option` that allow functions to propagate any errors to their caller. The intention is that there are no magic numbers that a function may return become part of the function signature.

It also provides a `panic!()` macro that you can use for unexpected state and other failings in your code. A panic is similar to an exception except there are limits on how you can catch it.


### Result<T, E>

The type `Result<T, E>` takes a success value type `T` and an error type `E`.

```rust
enum Result<T, E> {
  Ok(T),
  Err(E)
}
```

So perhaps we have `validate_files()` function that either succeeds or it returns with an error. We can define it like so:

```rust
enum ErrorResultCode {
  ResourcesNotFound(Vec<String>),
  DataCorrupted,
  PermissionDenied
}

fn validate_files() -> Result<(), ErrorResultCode> { /* ... */ }
//...
match validate_files() {
  Ok(_) => { println!("Success"); }
  Err(err) => {
    match err {
      ErrorResultCode::ResourcesNotFound(resources) => {
        println!("Fail resources not found");
        resources.for_each(|resource| println!("Not found {}", resource));
      }
      ErrorResultCode::DataCorrupted => { println!("Fail data corrupted"); }
      ErrorResultCode::PermissionDenied => { println!("Fail permission denied"); }
    }
  }
}

```

The return code `Result<(), ErrorResultCode>` means calling the function will either return:

* `Ok(T)` where the payload is the `()` unity type/value. i.e. when we succeed we get back nothing more of interest.
* `Err(E)` where the payload is `ErrorResultCode` which we can inspect further if we want to.

### Option<T>

The `Option` enum either returns `None` or `Some(T)` where the `Some` contains a type `T` payload of data.

This type is particularly useful for functions that either return something or nothing, e.g. a database query.

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

This is NOT recommended for dealing with regular errors, only irregular ones that the code has little or no way of dealing with.

This macro will cause the thread to abort and if the thread is the main programme thread, the entire process will exit.

A panic!\(\) can be caught _in some situations_ and should be if Rust is being invoked from another language. The way to catch an unwinding panic is a closure at the topmost point in the code where it can be handled.

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    panic!("Bad things");
});
```
