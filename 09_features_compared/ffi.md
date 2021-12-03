# Foreign Function Interface

Rust doesn't work in a vaccum and was never intended as such. Instead it was always assumed that it would need to call other code and other code would need to call it,

* Call other libraries via their entry points
* Produce C ABI libraries in Rust that can be called by code written in another language. e.g. C, C++, Python, Ruby etc.

To that end it has the Foreign Function Interface, the means to define external functions, expose its own functions without name mangling and to invoke unsafe code that would otherwise be illegal in Rust.

## Calling out to C libraries

Rust supports the concept of a foreign function interface which is a definition of an external function or type that is resolved at link time.

For example, we might wish to link to a library called foo.lib, and invoke a command foo_command().

```rust
#[link(name = "foo")]
extern {
  fn foo_command(command: *mut u8)
}
```

To call this function we have to turn off safety checks first because we are stepping out of the bounds of Rust's lifetime enforcement. To do this we wrap the call in an unsafe block to disable the safety checks:

```rust
pub fn run_command(command: &[u8]) {
  unsafe {
    foo_command(command.as_ptr());
  }
}
```

Note how we can use unsafe features like pointers inside of this unsafe block. This allows interaction with the outside world while still enforcing safety for the rest of our code.

## Making Rust code callable

The converse is also possible. We can produce a C ABI library from Rust that can be invoked by some other code.

For example, imagine we have some code written in Python. The code works fine but it is not performant and the bottle neck is in just one portion of the code, e.g. some file operation like a checksum. We want our code to consist of a make_checksum() and a release_checksum().

```rust
extern crate libc;

use std::ffi::CString;
use std::ptr;
use libc::{c_char, c_void, malloc, memset, strcpy, free};

#[no_mangle]
pub extern "C" fn make_checksum(filepath: *const c_char) -> *mut c_char {
    // Your code here
    if filepath == ptr::null() {
      return ptr::null_mut::<c_char>()
    }

    unsafe {
        // Imagine our checksum code here...
        let result = malloc(12);
        memset(result, 0, 12);
        strcpy(result as *mut c_char, CString::new("abcdef").unwrap().as_ptr());
        return result as *mut c_char;
    }
}

#[no_mangle]
pub extern "C" fn release_checksum(checksum: *const c_char) {
    unsafe {
        free(checksum as *mut c_void);
    }
}
```

Now in Python we can invoke the library simply:

```Python
import ctypes

checksum = ctypes.CDLL("path/to/our/dll");
cs = checksum.make_checksum("c:/somefile");
...
checksum.release_checksum(cs)
```

The [FFI specification](https://doc.rust-lang.org/book/ffi.html) goes into a lot more detail than this and explains concepts such as callbacks, structure packing, stdcall, linking and other issues that allow full interoperability.

## libc

Rust maintains a crate called [libc](https://github.com/rust-lang/libc) which holds types and functions corresponding to C.

A dependency to libc would be added to the `Cargo.toml` of your project:

```
[dependencies]
libc = "0.2.17"
```

And the file that uses the functions would contain a preamble such as this saying what types and functions it calls:

```rust
extern crate libc;

use libc::{c_char, malloc, free, atoi};
```

## Other libraries

There are also crates that have the definitions of structures, types and functions.

* [WinAPI](https://github.com/retep998/winapi-rs) bindings for Win32 programming APIs.
* [OpenSSL](https://github.com/sfackler/rust-openssl) bindings for OpenSSL
