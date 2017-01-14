# Expressions

An expression is something that evaluates to something. Just like C++ more or less...

```rust
let x = 5 + 5; // expression evaluates to 10
```

## Blocks are expressions too

Where it gets more interesting is that a block is an expression. This is legal code:

```rust
let x = {};
println!("x = {:?}", x);
```

What does x get assigned? In this case the block was empty so x was assigned with the value of ().

```
x = ()
```

So far so useless. But we can change what the block expression evaluates to:
```rust
let x = {
    let pi = 3.141592735;
    let r = 5.0;
    2.0 * pi * r
};
println!("x = {}", x);
```

Now x assigned with the result of the last line which is an expression.

Note how the last line inside the block is not terminated with a semicolon. That becomes the result of the block expression. If we’d put a semicolon on the end of that line, the expression would evaluate to ().

### Use in functions

A trivial function can just omit the return statement:

```rust
pub fn add_values(x: i32, y: i32) -> i32 {
  x + y
}
```

### You can use return too

Sometimes you might explicitly need to use the return statement. The block expression evaluates at the end of the block so if you need to bail early you could just use return.

```rust
pub fn find(value: &str) -> i32 {
  if value.len() == 0 {
    return -1;
  }
  database.do_find(value)
}
```

### Even more complex cases

In C or C++ it would not be uncommon to see code like this:

```c++
std::string result;
switch (server_state) {
  case WAITING:
    result = "Waiting";
    break;
  case RUNNING:
    result = "Running";
    break;
  case STOPPED:
    result = "Stopped";
    break;
  }
}
```

We want to assign a value to result from a switch so each branch of the switch has an assignment action. Aside from looking a bit clunky it introduces the possibility of error since we might forget to assign, or add a break, or omit one of the values.

In Rust we can assign the result of from a match because each match condition is a block expression.

```rust
let result = match server_state {
    ServerState::WAITING => "Waiting",
    ServerState::RUNNING => "Running",
    ServerState::STOPPED => "Stopped",
};
```

Not only is this half the length it reduces the scope for error. The compiler will assign the block expression's value to the variable result. It will also check that each match block returns the same kind of type (so you can't return a float from one match and strings from others). It will also generate an error if the ServerState enum had other values that our match didn't handle.

### Ternary operator

The ternary operator in C/C++ is an abbreviated way to perform an if/else expression condition, usually to assign the result to a variable.

```c++
bool x = (y / 2) == 4 ? true : false;
```

Rust has no such equivalent to a ternary operator but it can be accomplished using block expressions.

```rust
let x = if y / 2 == 4 { true } else { false };
```

Unlike C/C++ you could add additiona else ifs, matches or anything else to that providing each branch returns the same type.
