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

Rust uses attributes to control conditional compilation. The `cfg()` attribute allows the compiler to test whether some configuration / feature holds true and to generate code for it.

```rust
#[cfg(windows)]
const data_path: &'static str = r#"c:\ProgramData\Foo"#;
#[cfg(linux)]
const data_path: &'static str = "/var/cache/lib/foo";

fn main() {
    println!("Path = {}", data_path);    
}
```

In this example, the `data_path` constant is either declared one way, another way or not at all in which case the program will be in error.

The `cfg()` attribute takes a range of different predefined values, and can be extended using features.

Say for example our `Cargo.toml` defined a feature:

```
[features]
default = []
webservice = ["actix-web"]
```

So we have a project which can optionally build something with a command such as `cargo build --features webservice`.

Our code might contain conditional tests to that:

```rust
#[cfg(features = webservice)]
extern crate actix_web;

#[cfg(features = webservice)]
fn start_webservice() {
    //...
}
```

## cfg!

Sometimes it would be more useful to have inline configuration testing, which is where the `cfg!` macro is used.