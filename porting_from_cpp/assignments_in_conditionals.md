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

## Real world

The `goto fail` example that we saw in section "Missing braces in conditionals" also demonstrates a real world dangers combining assignment and comparison into a single line:

```c++
if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
  goto fail;
```

This line is not broken for other reasons, but it's easy to see how might be, especially if this pattern were repeated all over the place. The programmer might have saved a few lines of code to combine everything in this way but at a greater risk. In this case, the risk might be inadvertantly turning the `=` into an `==`, i.e. comparing err to the function call and then comparing that to 0.

```c++
if ((err == SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
  goto fail;
```

## In Rust 

Rust does not allow assignment within expressions so they will fail to compile. This is done to prevent subtle errors with `=` being used instead of `==`.

Let's look how we might do the equivalent (but not optimal way) in Rust.

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

Here we declare a mutable `result` var and run a loop that tests a conditional block expression. The block expression assigns a value to `result` and then evaluates to `result > 0`. We also use a `match` to test the value of result since that makes sense in this context.

So this is functionally the same thing as C++. 

Can we do better? Yes since Rust also has a `while let` construct and if we change the function signature of `getResponseCode()` to return an enum such as `Option<>` or `Result<>` we can use it:

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

This code will run the loop for as long as `doSomething()` returns `Some(value)` where `value` is assigned to a variable `result`.

Rust really likes function to convey something-ness, or status in their signature so this is a good solution. Not only do we make the distinction between a bad result and a good one, but we can use `while let`.

The only form of assignment inside a conditional is the specialised and explicit `if let` and `while let` forms which are explained elsewhere.
