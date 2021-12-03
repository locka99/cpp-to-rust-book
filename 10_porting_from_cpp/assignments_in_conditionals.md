# Assignment in Expressions

C and C++ allow you do do things like this:

```c++
int result;
while (result = getResponseCode()) {
  // In C++ result is non-zero which is evaluated as boolean true
  // In C result is non-zero which is treated as statement success
  if (result == 200) {
    //...
  }
  else if (result = 404) { // BUG!!!
    //...
  }
}
```

Our loop here is calling `doSomething()`, and continuing for as long as `result` is non-zero. Inside the loop it then further tests the value. But look at the second test, we wrote `=` instead of `==`, the language is fine about it and introduced a bug.

Some compilers may warn if a result is assigned to a constant but it is still allowed.

We saw this `goto fail` example in section "Missing braces in conditionals":

```c++
if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
  goto fail;
```

We could inadvertantly break it as easily in the other direction if we used `==` instead of `=`, i.e. comparing err to the function call and then comparing that to 0.

```c++
if ((err == SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
  goto fail;
```

## In Rust 

Rust does not allow assignment within simple expressions so they will fail to compile. This is done to prevent subtle errors with `=` being used instead of `==`.

Let's look how we might do the equivalent (but not optimal way) in Rust using a block expression.

```rust
let mut result;
while { result = getResponseCode(); result > 0 } {
  if result == 200 {
    //...
  }
  else if result == 404 {
    //...
  }
}
```

Here we declare a mutable `result` var and run a loop against a block expression. The block expression assigns a value to `result` and then evaluates as `result > 0`. We also use a `match` to test the value of result since that makes sense in this context.

So this is functionally the same thing as C++ but it's a little bit noisy.

Can we do better? Yes if change the function signature of `getResponseCode()` to return an enum such as `Option<>` or `Result<>`. Rust has a `while let` construct that allows us to test if a enum value matches a pattern and to automatically assign the payload to another value:

```rust
fn getResponseCode() -> Option<u32> { /*... */}
//...
while let Some(result) = getResponseCode() {
  if result == 200 {
    //...
  }
  else if result == 404 {
    //...
  }
}
```

This code will run the loop, calling `getResponseCode()` and if it evaluates to `Some(value)` then `value` is copied to variable `result`. If it does not match the pattern then the loop breaks.

This is good design in Rust. It is always to convey information in a function's signature. Not only do we make the distinction between a bad result and a good one, but we can use `while let`.
