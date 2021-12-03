# Switch / Match

## C/C++ switch

A `switch` statement in C or C++ allows a condition or a variable to be compared to a series of values and for code associated with those values to executed as a result. There is also a default clause to match any value that is is not caught explicitly.

```c++
int result = http_get();
switch (result) {
case 200:
  success = true;
  break;
case 404:
  log_error(result);
  // Drop through
default:
  success = false;
  break;
}
```

Switch statements can be a source of error because behaviour is undefined when a `default` clause is not supplied. It is also possible to inadvertently forget the `break` statement. In the above example, the code explicitly "drops" from the 404 handler into the default handler. This code would work fine providing someone didn't insert some extra clauses between 404 and default...

Additionally switch statements only work on numeric values or `bool`.

## Rust

[Match](https://doc.rust-lang.org/book/match.html) is similar to a `switch` statement but it is a lot more powerful.

1. A `match` works on integers, ranges of integers, bools, enums, tuples, arrays and structs.
2. It will destructure tuples, arrays and structs.
3. It requires a default handler if necessary.

Examples:

```rust
enum Result {
  Good,
  Error(u32),
  Unknown
}

let result = doSomething();
match result {
  Result::Good => { println!("Good"); }
  Result::Error(e) => { println!("Got an error {}", e); }
  _ => { /* do nothing */ }
}
```
