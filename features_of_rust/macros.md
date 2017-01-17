# Macros

Macros in C++ are very prone to error and so have been deprecated in favour of constants and inline functions.

Even so, they are frequently used in these roles:

* To set a command-line flag or directive, e.g. the compiler might define WIN32 so code can conditionally compile one way or another according to its presence.
* For adding guard blocks around headers to prevent them being #include'd more than once. Most compilers implement a "#pragma once directive" which is an increasingly common alternative
* For generating snippets of boiler plate code (e.g. namespace wrappers), or things that might be compiled away depending on #defines like DEBUG being set or not.
* For making strings of values and other esoteric edge cases

Writing a macro is easy, perhaps too easy:

```c++
#define PRINT(x) \
  printf("You printed %d", x);
```

This macro would expand to printf before compilation but it would fail to compile or print the wrong thing if x were not an integer.

## Rust macros

Macros in Rust are quite a complex topic but they are more powerful and safer than the ones in C++.

* Rust macros are hygenic. That is to say if a macro contains variables, their names do not conflict with, hide, or otherwise interfere with named variables from the scope they're used from.
* The pattern supplied in between the brackets of the macro are tokenized and designated as parts of the Rust language. identifiers, expressions etc. In C / C++ you can #define a macro to be anything you like whether it is garbage or syntactically correct. Furthermore you can call it from anywhere you like because it is preprocessed even before the compiler sees the macro.
* Rust macros are either declarative and rule based with each rule having a left hand side pattern "matcher" and a right hand side "substitution". Or they're procedural and actualy rust code turns an input into an output (see section below).
* Macros must produce syntactically correct code.
* Declarative macros can be exported by crates and used in other code providing the other code elects to enable macro support from the crate. This is a little messy since it must be signalled with a #[macro_export] directive.
* With all that said, macros in Rust are very complex to read - too complex to read, and generally speaking should be used as sparingly as possible.

Here is a simple declarative macro demonstrating repetition called hello_x!(). It will take a comma separated list of expressions and say hello to each one of them.

```rust
macro_rules! hello_x {
  ($($name:expr),*) => (
    $(println!("Hello {}", $name);)*
  )
}
// The code can supply as many arguments it likes to this macro
hello_x!("Bob", "Sue", "John", "Ellen");
```

Essentially the matcher matches against our comma separate list and the substitution generates one println!() with the message for each expression.

```
Hello Bob
Hello Sue
Hello John
Hello Ellen
```

What if we threw some other expressions into that array?

```rust
hello_x!("Bob", true, 1234.333, -1);
```

Well that works too:

```
Hello Bob
Hello true
Hello 1234.333
Hello -1
```

What about some illegal code:

```rust
hello_x!(Aardvark {});
```

We get a meaningful error originating from the macro.

```
error[E0422]: `Aardvark` does not name a structure
  |
8 | hello_x!(Aardvark {});
  |          ^^^^^^^^
<std macros>:2:27: 2:58 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
<anon>:5:7: 5:35 note: in this expansion of println! (defined in <std macros>)
<anon>:8:1: 8:23 note: in this expansion of hello_x! (defined in <anon>)
```

## Real world example - vec!()

Rust comes with a lot of macros for reducing some of the leg work of tedious boiler plate. For example the vec!() macro is a way to declare a std::Vec and prepopulate it with some values.

Here is the actual vec! macro source code taken from the Rust source:

```rust
macro_rules! vec {
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n)
    );
    ($($x:expr),*) => (
        <[_]>::into_vec(box [$($x),*])
    );
    ($($x:expr,)*) => (vec![$($x),*])
}
```

It looks complex but we will break it down to see what it does. Firstly it has a match-like syntax with three branches that expand to anything that matches the left hand side:

### First branch

The first matcher matches a pattern such as `1; 100`. The value `1` goes into `$elem`, the value `100` goes into `$n`:

```rust
($elem:expr; $n:expr) =>  (
        $crate::vec::from_elem($elem, $n)
    );
```

The `$crate` is a special value that resolves to the module crate which happens to be std.

So this expands to this:

```rust
let v = vec!(1; 100);
// 1st branch matches and it becomes this
let v = std::vec::from_elem(1, 100);
```

### Second branch

The second matcher contains a glob expression - zero or more expressions separated by comma (the last comma is optional). Each matching expression ends up in `$x`:

```rust
($($x:expr),*) => (
        <[_]>::into_vec(box [$($x),*])
    );
```

So we can write:

```rust
let v = vec!(1, 2, 3, 4, 5);
// 3nd branch matches and it becomes this
let v = <[_]>::into_vec(box [1, 2, 3, 4, 5]);
```

The box keyword tells Rust to allocate the supplied array on the heap and moves the ownership by calling a helper function called into_vec() that wraps the memory array with a Vec instance. The <[\_]>:: at the front is a turbo-fish notation to make the into_vec() generic function happy.

### Third branch

The third branch is a little odd and almost looks the same as the second branch. But take at look the comma. In the last branch it was next to the asterisk, this time it is inside the inner $().

```rust
($($x:expr,)*) => (vec![$($x),*])
```

The matcher matches when the the comma is there and if so recursively calls vec!() again to resolve to the second branch matcher:

Basically it is there so that there can be a trailing comma in our declaration and it will still generate the same code.

```rust
// 3rd branch matches this
let v = vec!(1, 2, 3, 4, 5,);
// and it becomes this
let v = vec!(1, 2, 3, 4, 5);
// which matches 2nd branch to become
let v = <[_]>::into_vec(box [1, 2, 3, 4, 5]);
```

## Procedural Macros

So far we've talked about declarative macros that expand out to be Rust code based upon how they pattern match the rules defined by the macro.

A second kind of macro is the *procedural macro*. A procedural macro is a plugin written in Rust that is compiled and loaded by the compiler to produce arbitrary Rust code as its output.

A procedural macro can therefore be thought of as a code generator but one that forms part of the actual compiler. Procedural macros can be particularly useful for:

* Serialization / deserialization (e.g. the [serde](https://github.com/serde-rs/serde) module generates code for reading and writing structs to a variety of formats - JSON, YAML, TOML, XML etc.)
* Domain Specific Languages (e.g. embedded SQL, regular expressions etc).
* Aspect oriented programming (e.g. extra debugging, performance metrics etc)
* New lint and derive rules

For more information look at this section on [compiler plugins](https://doc.rust-lang.org/book/compiler-plugins.html) in the Rust book.
