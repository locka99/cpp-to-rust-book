# Switch / Match

## C++

A switch statement in C or C++ allows a condition or a variable to be compared to a series of values and for code associated with those values to executed as a result. There is also a default clause to match any value that is is not caught explicitly.

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

Switch statements can be a source of error because behaviour is undefined when a default clause is not supplied. It is also possible to inadvertently forget the break statement. In the above example, the code explicitly "drops" from the 404 handler into the default handler. This code would work fine providing someone didn't insert some extra clauses between 404 and default...

Additionally switch statements only work on numeric values (or bool)

## Rust

[Match](https://doc.rust-lang.org/book/match.html) is like a switch statement on steroids.

In C++ a switch is a straight comparison of an integer value of some kind (including chars and enums), against a list of values. If the comparison matches, the code next to it executes until the bottom of the switch statement or a break is reached.
