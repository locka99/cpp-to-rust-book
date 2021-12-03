# Conditions

Conditional code is similar between C++ and Rust. You test the boolean truth of an expression and you can use boolean operators such as && and || to join expressions together.

```c++
int x = 0;
while (x < 10) {
  x++;
}
int y = 10;
bool doCompare = true;
if (doCompare && x == y) {
  printf("They match!\n");
}
```

In Rust:

```rust
let mut x = 0;
while x < 10 {
  x = x + 1;
}
let y = 10;
let do_compare = true;
if do_compare && x == y {
  println!("They match!");
}
```

The most notable difference is that Rust omits the outer braces so the code is slightly cleaner. You don't have to omit the outer braces but the compiler will issue a warning if you leave them in.

## Ternary operator

The ternary operator is that special ? : shorthand notation you can use to in C++ for simple conditionals.

```c++
int x = (y > 200) ? 10 : 0;
```

Rust does not support this notation, however you may take advantage of how a block evaluates as an expression to say this instead:

```rust
let x = if y > 200 { 10 } else { 0 };
```

So basically you can do one line conditional assignments using if and else. Also note that you could even throw in an "else if" or two if that's what you wanted to do:

```rust
let c = get_temperature();
let water_is = if (c >= 100) { "gas" } else if (c < 0) { "solid" } else { "liquid" };
```

## Conditional "if let"

One unusual feature is the "if let" pattern. This combines a test to see if something matches a pattern and if it does, to automatically assign the result to the tuple. It would be most commonly see in code that returns an enum such as a `Result` or `Option`.

For example:

```rust
fn search(name: &str) -> Option<Person> { /* ... */ }
//...
if let Some(person) = search("fred") {
  println!("You fould a person {}", person);
}
else {
  println!("Could not find person");
}
```
