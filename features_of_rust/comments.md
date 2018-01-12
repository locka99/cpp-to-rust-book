# Comments

Rust comments are similar to C++ except they may contain Unicode because .rs files are UTF-8 encoded:

```rust
/*
 This is a comment
*/

// This a comment with Unicode, 你好
```

But in addition anything that uses triple slash `///` notation can be parsed by a tool called `rustdoc` to produce documentation:

```rust
/// This is a comment that becomes documentation for do_thing below
pub fn do_thing() {}
/// Returned by server if the resource could not be found
pub const NOT_FOUND = 404;
```

Documentation beyond a triple-slash comment is in Markdown format, just like this guide. That means you have a human readable language for writing rich-text documentation and if it's not enough you can resort to HTML via tags.

The easiest way to produce documentation is to invoke `cargo doc`.

See here for [full documentation](https://doc.rust-lang.org/book/documentation.html)

