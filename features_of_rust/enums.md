# Enumerations

In C++ an `enum` is a bunch of labels assigned an `int` value.

```c++
enum HttpResponse {
  okay = 200,
  not_found = 404,
  internal_error = 500,
};
```

C++11 extends this concept a little, allowing you to declare an `enum` that uses another integer type, e.g. a `char` to hold the values.

In Rust an [`enum`](https://doc.rust-lang.org/book/enums.html) can hold actual data so you can convey far more information than a static value could by itself.

```rust
enum HttpResponse {
  Ok,
  NotFound(String),
  InternalError(String, String, Vec<u8>)
}
```

So we might have a function that makes an http request and returns a response:

```rust
fn do_request(url: &str) -> HttpResponse {
  if url == "/invalid" {
    return HttpResponse::NotFound(url.to_string());
  }
   HttpResponse::Ok
}
let result = do_request("/invalid");
if let HttpResponse::NotFound(url) = result {
  println!("The url {} could not be found", url);
}
```

Now our code is able to return a more meaningful response in an enum and the code is able to extract that response to print out useful information.
