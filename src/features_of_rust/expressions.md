# Expressions

An expression is something that evaluates to something. Just like C++ more or less...

```rust
let x = 5 + 5; // expression evaluates to 10
```

Where it gets more interesting is that a block is an expression too and you can return a value from it.

```rust
let x = {
   let pi = 3.141592735;
   let r = 5;
   2 * pi * r
};
```

Note how the last line inside the block is not terminated with a semicolon. So the result of the block expression is 2 * pi * r which is assigned to x. If we’d put a semicolon on the end of that line, the expression would evaluate to nothing.

You could even do complex matching in your block and conditionally assign the output:

```rust
let result = {
  match server_state {
    ServerState::waiting => { "Waiting" }
    ServerState::running => { "Running" }
    ServerState::stopped => { "Stopped" }
  }
}.to_string();
println!("The server state is {}", result);
```

In this instance, the match returns a &str from each match and we then call to_string() to turn it to a String before binding it to the variable result.
More normally you will see this in function blocks or closures. A trivial function can just omit the return statement:

```rust
pub fn add_values(x: i32, y: i32) -> i32 {
  x + y
}
```

Another case you might see is by using Rust's equivalent to a C++ ternary operator:

```rust
let x = if y / 2 == 4 { true } else { false };
```
