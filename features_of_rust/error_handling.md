# Error Handling

## C

C supports a conventional way of error propagation - functions can return a value to indicate the success or failure of the operation.

```c
if (strcmp("hello", "world") != 0) {
  printf("Strings do not match");
}
```

In addition most C compilers offer `setjmp()` and `longjmp()`. These functions are used together to allow a program to set a place in the stack from which code further down the stack can "long jump" back to in the case of an error or some other condition. Note that C has no destructors so the code itself is responsible for freeing resources and memory it may have allocated in between the two points in the stack.

There is little in the language beyond this, however some compilers may offer `structured exception handling` (SEH) as a poor-man's version of the functionality in C++. 

Generally speaking `setjmp()`, `longjmp()` and SEH do not mix well with their C++ counterparts so if they are used at all should only be in the same language context, and not across C to C++ boundaries. If C++ code attempts to `longjmp()` it will not invoke destructors in the same manner as a try-catch block.

## C++

C++ allows C-style error handling but also allows code to throw and catch exceptions. 

As the name suggests, exceptions indicate an exceptional error but some code may use them considerably more than that.

A thrown exception interrupts the current flow of logic and allows something further up the stack to catch the exception and recover. The compiler instruments the code so that any intervening objects on the stack are destroyed as the stack is unwound. Note however that this only applies to stack allocated objects, not those which are heap allocated.

If nothing catches the throw then the thread itself will exit. Uncaught exceptions on the main thread cause the entire program to terminate.

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

### Exceptions vs Error codes

Most coding guidelines suggest using exceptions sparingly for truly exceptional situations, and use return codes and other forms of error propagation for ordinary failures.

However C++ has no simple way to confer error information for ordinary failures. Here are some common ways that they do:

* Functions that return a `bool`, an `int`, or a pointer with special meaning. e.g. `false`, `-1` or `NULL` for failure.
* Functions that return a result code or enum. This might have a `GOOD` value and a bunch of `ERROR_` values. An extreme example would be `HRESULT` used by Windows that bitpacks information about goodness, severity and origin into a result and requires macros to extract the information.
* Functions that have a special out parameter that is filled in with additional detail in the case of failure.
* Functions that provide further information about the last error in `errno()` or some similar function supplied by the library.
* Exceptions that are thrown for any failure and must be caught. 
* Exceptions that are thrown sometimes and error codes are returned other times.
* Functions that are overloaded into two forms, one that throws an exception, another that stores the error in an error parameter. The boost library has functions like this.

Since there is no consistent way to deal with errors, every library and function has its own ad hoc way to return information.

## Rust

### Panic

Rust has a limited form of exception handling called a panic which is invoked through a `panic!()` macro.

```rust
if connection_status == Status::Error {
  panic!("We should have been connected by now");
}
```

As the name suggests, a panic is a serious problem, usually in the programming logic. Panics are generally a way to fail the program, obtain a stack trace and fix the position where it happened so it doesn't happen again.

As with C++ exceptions, the panic will unwind all the way to the top of the thread and will kill the thread. If the thread is the main thread, the entire process is terminated.

You can catch most panics with a `catch_unwind()` function that takes the code to trap as a closure. For example, this code will call `simulate_panic()`, catch the problem, report the error to the program to handle.

```rust
use std::panic::catch_unwind;

if let Err(cause) = catch_unwind(|| simulate_panic()) {
  println!("Code suffered a panic, cause = {:?}", cause);
}

fn simulate_panic() {
  panic!("I failed");
}
```

The panic is a very heavy handed mechanism and is not suitable for standard error handling. It should be used for _exceptional_ circumstances, not those which are likely to occur frequently.

### Result and Option

Rust provides two enumeration types called `Result` and `Option` that allow functions to propagate results to their caller. 

These should be your every day option for error handling.

#### Result<T, E>

The result type is used by functions that return something on success or they return an error. The type is a generic, so the code decides what the type for success and error are: The `Result<T, E>` takes a success value type `T` and an error type `E`.

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

#### Option<T>

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

#### The ? directive

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

