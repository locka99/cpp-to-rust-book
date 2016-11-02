# Assignment in Conditionals

The omission of an `=` in an `==` condition turns it into an assignment that evaluates to true:

```c++
int result = getResponseCode();
if (result = 200) { // BUG!
  // Success
}
else {
  //... Process error
}
```

So here, result was assigned the value 200 rather than compared to the value 200. Some compilers may issue a warning for these cases, but an error would be better.

Developers might also try to reverse the left and right hand side to mitigate the issue:

```c++
if (200 = result) { // Compiler error
  // Success
}
else {
  // ... Process error
}
```

Now the compiler will complain because the value of result is being assigned to a constant which makes no sense. This may work if a variable is compared to a constant but arguably it makes the code less readable and wouldn't help if the left and right hand sides were both assignable so their order didn't matter.

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

## How Rust helps

This code just won't compile:

```rust
let mut result: i32;
if result = 200 { // Compile Error
  //...
}
```

The only form of assignment inside a conditional is the specialised and explicit `if let` and `while let` forms which are explained pages 55 and 58 respectively.
