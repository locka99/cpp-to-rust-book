# C++

C and C++ have a preprocessor to do conditional compilation.

```c++
#if defined(FOO_WINDOWS)
string data_path = "c:\\ProgramData\\Foo";
#elif defined(FOO_LINUX)
string data_path = "/var/cache/lib/foo";
#else
#error Unsupported platform
#endif
```

In this example, the preprocessor will strip lines seen by the compiler using the `#if`, `#elif`, `#else` directives depending on whether `FOO_WINDOWS`, `FOO_LINUX` or nothing at all has been defined. These values could be `#define`d above in another header file, or could have been supplied from the command line, or could have come from the preprocessor itself.

```
gcc -DFOO_WINDOWS somefile.cpp
```

Conditional compilation on C++ can get _really_ hard due to the large number of compilers and supported platforms. It is not uncommon for libraries to try and isolate the mess into a file such as a `config.h`. Many open source programs even have a large number of settings and options which require configuration to be generated, e.g. via a `./configure` script.

# Rust

Rust uses attributes to control conditional compilation. The `cfg` attribute allows the compiler to test whether some configuration / feature holds true and to generate code for it.

```rust
#[cfg(windows)]
const data_path: &'static str = r#"c:\ProgramData\Foo"#;
#[cfg(linux)]
const data_path: &'static str = "/var/cache/lib/foo";

fn main() {
    println!("Path = {}", data_path);    
}
```

## cfg!

TODO