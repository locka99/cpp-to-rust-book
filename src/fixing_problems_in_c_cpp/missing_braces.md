# Missing Braces in Conditionals

I expect every programmer has encountered an error like this and spent hours trying to figure out why it wasn't working.

```c++
const bool result = fetch_files();
if (result) {
  process_files()
}
else
  print_error()
  return false;

// Now cleanup and
cleanup_files();
return true;
```

The reason of course was the else statement wasn't enclosed in braces so the wrong code was executed. The compiler might spot dead code in this instance but that may not always be the case. Even if it did, it might only issue a warning instead of an error.

The problem can be especially annoying in deeply nested conditions where a misplaced brace can attach to the wrong level.
This problem has lead real-world security issues. For example here is the infamous ["goto fail"](https://www.imperialviolet.org/2014/02/22/applebug.html) bug that occured in some Apple products. This (intentional?) bug occured during an SSL handshake and was exploitable. :

```c++
static OSStatus
SSLVerifySignedServerKeyExchange(
   SSLContext *ctx, bool isRsa, SSLBuffer signedParams,
   uint8_t *signature, UInt16 signatureLen)
{
  OSStatus        err;
  //...

  if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
    goto fail;
  if ((err = SSLHashSHA1.update(&hashCtx, &signedParams)) != 0)
    goto fail;
    goto fail;
  if ((err = SSLHashSHA1.final(&hashCtx, &hashOut)) != 0)
    goto fail;
  //...

fail:
  SSLFreeBuffer(&signedHashes);
  SSLFreeBuffer(&hashCtx);
  return err;
}
```

Note how the "goto fail" is repeated twice and not bound to the condition but is indented as if it was. The code would jump straight into the fail label and return with an err indicating success (since the prior SHA1 update had succeeded). If conditionals

## How Rust helps

Rust requires if-else expressions and loops to be associated with blocks.

So this code won't compile:

```rust
let mut x: i32 = do_something();
if x == 200 {
  // ...
}
else
  println!("Error");
```

If you try you will get an error like this.

```
rustc 1.13.0-beta.1 (cbbeba430 2016-09-28)
error: expected `{`, found `println`
  |
8 |   println!("Error");
  |   ^^^^^^^
  |
help: try placing this code inside a block
  |
8 |   println!("Error");
  |   ^^^^^^^^^^^^^^^^^^
error[E0425]: unresolved name `do_something`
  |
3 | let mut x: i32 = do_something();
  |                  ^^^^^^^^^^^^ unresolved name
```
