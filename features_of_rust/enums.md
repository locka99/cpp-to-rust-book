# Enumerations

In C++ an `enum` is a bunch of labels assigned an `int` value. i.e. it is basically a bunch of constants with scalar values.

```cpp
enum HttpResponse {
  okay = 200,
  not_found = 404,
  internal_error = 500,
};
```

C++11 extends this concept a little, allowing you to declare an `enum` that uses another kind of integral type, e.g. a `char` to hold the values.

```cpp
enum LibraryCode : char {
  checked_in = 'I',
  checked_out = 'O',
  checked_out_late = 'L'
};
```

In Rust an [`enum`](https://doc.rust-lang.org/book/enums.html) can be a scalar value just like in C++. 

```rust
enum HttpResponse {
  Ok= 200,
  NotFound= 404,
  InternalError = 500
};
```

But an enum can also hold actual data so you can convey far more information than a static value could by itself. 

```rust
enum HttpResponse {
  Ok,
  NotFound(String),
  InternalError(String, String, Vec<u8>)
}
```

You can also bind functions to the enum:

```
impl HttpResponse {
  pub fn code(&self) => {
    match *self {
      HttpResponse::Ok => 200,
      HttpResponse::NotFound(_) => 404,
      HttpResponse::InternalError(_, _, _) => 500,
    }
  }
}
```

So we might have a function that makes an http request and returns a response:

```rust
fn do_request(url: &str) -> HttpResponse {
  if url == "/invalid" {
    HttpResponse::NotFound(url.to_string())
  }
  else {
    HttpResponse::Ok
  }
}
//...
let result = do_request("/invalid");
if let HttpResponse::NotFound(url) = result {
  println!("The url {} could not be found", url);
}
```

Now our code is able to return a more meaningful response in an enum and the code is able to extract that response to print out useful information.

