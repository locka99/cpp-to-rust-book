# Poor Lifetime Enforcement

A function like is completely legal and dangerous:

```c++
std::string &getValue() {
  std::string value("Hello world");
  return value;
}
```

This function returns a reference to a temporary variable. Whoever calls it will get a reference to garbage on the stack. Even if it appears to work (e.g. if we called the reference immediately) it is only through luck that it does.

Our compiler will probably issue a warning for this trivial example but it won't stop us from compiling it.

## How Rust helps

Rust tracks the lifetime of all objects and knows when their lifetime begins and ends. It tracks references to the object, knows when it is being borrowed (being passed to a function / scope).

It generate a compiler error if it detects any violations of its lifetime / borrowing rules. So the above code would fail to compile.
