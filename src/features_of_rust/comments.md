# Comments

Rust comments are pretty much like C++ except they may contain Unicode since .rs files are UTF-8 encoded:

```rust
/*
 This is a comment
*/

// This a comment with Unicode, 你好
```


Anything that uses triple slash notation is parsed by a tool called rustdoc (which you can also invoke indirectly via "cargo doc") to produce documentation:

```rust
/// This is a comment that becomes documentation for do_thing below
pub fn do_thing() {}
/// Returned by server if the resource could not be found
pub const NOT_FOUND = 404;
```

Rustdoc uses Markdown notation for its notation.

This means you can write sections, code sections, links etc. into your comments as you might with Markdown.

TODO markdown example.

Rust has special sections in Markdown to describe behaviors of the struct

See here for [full documentation](https://doc.rust-lang.org/book/documentation.html)
