# Notation used through this book

Code samples are given throughout this book are for C, C++, Rust and general configuration / console output.
In order to distinguish each kind they are styled as follows:

C / C++ samples are given in this style:

```c++
// C/C++
while (x < y) {
  cout << "x is less than y" << endl;
  ++x;
}
```

Rust samples are given in this style:

```rust
// Rust
if x == 20 {
  println!("Warning!");
}
```

Standard console output or script is given this style:

```
cd myproject/
cargo build
```

Most of the code samples are abbreviated in some fashion. e.g. they assume the code is running from within some function or they omit preambles. They may also assume a namespace / module to reduce the amount of noise.
