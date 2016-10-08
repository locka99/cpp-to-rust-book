# Fixing Problems in C/C++

This section is not so much concerned with how a programmer should write code as what the C/C++ languages allow. If the language allows a programmer to write patent nonsense, or simply become too trusting of the capabilities of the language then bad things will happen.

The C++ programming language is a very large specification, one that only grows and gets more nuanced and qualified with each release.

The problem from a programmer's perspective is understanding what things C++ allows them to do as oppose to what things they should do.

In each case we'll see how Rust might have stopped us getting into this situation in the first place.

## Copy constructor errors
In C++, imagine we have a class called PersonList:

```c++
class PersonList {
  std::Vector<Person> *personList_;
public:
  PersonList() : personList_(new std::Vector<Person>) {
  }

  ~PersonList() {
    delete personList_;
  }

  // ... Methods to add / search list
};
```

This is a fairly straightforward class that manages a list of people in some way. Each Person object is held in a vector that PersonList allocated from its constructor. The destructor for PersonList will delete this array.

Now let's see how we can create some really dangerous code:

```c++
{
  PersonList x;
  PersonList y = x;
  //...
  PersonList z;
  z = x;
} // Here be dragons!!!!
```

Well that was dangerously easy.

The PersonList had no copy constructor nor an assignment operator. So the compiler generated them for us. Lucky us since when we use them we are doomed!

The default copy constructor copies that member variable personList_ even though its a pointing to private data. So y and z will contain a personList_ that points to the same memory as x. So when z, y and x go out of scope, the same pointer will be deleted three times and the program will crash. On top of that, z allocated its own personList_ but the assignment overwrote it with the one from x so its old personList_ value just leaks.

### The Rule of Three

This is such a bad issue that it has given rise to the so-called the rule of three.

The rule says that if we explicitly declare a destructor, copy constructor or copy assignment operator in a C++ class then we probably need to implement all three of them to safely handle assignment and construction.

In order to fix this simple class we have to add a lot of bloat:

```c++
class PersonList {
  std::Vector<Person> *personList_;
public:
  PersonList() : personList_(new std::Vector<Person>) {
  }

  PersonList(const PersonList &other) :
          personList_(new std::Vector<Person>)    {
    personList_->insert(
      personList_->end(), other.personList_->begin(),
      other.personList_->end());
  }

  ~PersonList() {
    delete personList_;
  }

  PersonList & operator=(const PersonList &other) {
    // Don't forget to check if someone assigns an object to itself
    if (&other != this) {
      personList_->clear();
      personList_->insert(
        personList_->end(), other.personList_->begin(),
        other.personList_->end());
    }
    return *this;
  }

  // ... Methods to add / search list
};
```

What a mess!

The code even has to add a test to assignment in case someone writes x = x to stop the class clearing itself in preparation to adding elements from itself which would of course wipe out all its contents.

Alternatively we might disable copy / assignments by creating private constructors that prevents them being called by external code:

```c++
class PersonList {
  std::Vector<Person> *personList_;

private:
  PersonList(const PersonList &other) {}
  PersonList & operator=(const PersonList &other) { return *this }

public:
  PersonList() : personList_(new std::Vector<Person>) {
  }

  ~PersonList() {
    delete personList_;
  }
  // ... Methods to add / search list
};
```

Another alternative would be to a C++11 std::unique_ptr (or a boost scoped_ptr). A unique_ptr is a way to permit only one owner of a pointer at a time. The owner can be moved from one unique_ptr to another and the old owner becomes NULL from the move. A unique_ptr that is non-NULL deletes it from its destructor.

TODO unique_ptr example

This move is similar to the move semantics we'll see in Rust in a moment. But the object is allocated on the heap, is a pointer and is not directly analogous.

### How Rust helps

Rust does allow structs to be copied or clone unless we explicitly implement the Copy and Clone traits respectively.

Most primitive types such as ints, chars, bools etc. implement Copy so you can just assign one to another

```rust
// This is all good
let x = 8;
let y = x;
y = 20;
assert_eq!(x, 8);
```

A string cannot be copied this way but you can clone it:

```rust
let copyright = "Copyright 2017 Acme Factory".to_string();
let copyright2 = copyright.clone();
```

If we just declare a struct it cannot be copied by accident:

```rust
struct Person {
  name: String,
  age: u8
}
```

The following code will compile but you are not copying, you are moving:

```rust
let person1 = Person { name: "Tony".to_string(), age: 38 };
let person2 = person1;
```

Assignment moves ownership of the struct. from person1 to person2. It is an error to use person1 any more.

To illustrate consider this Rust which is equivalent to the PersonList we saw in C++

```rust
struct PersonList {
    pub persons: Box<Vec<Person>>,
}
```

We can see that PersonList has a vector of Person objects. A Box is what we use in Rust to hold a heap allocated object. When the Box is dropped, the item inside is also dropped and the heap memory is freed.

So this Vec of Person objects is in a Box and is on a heap. Clear?

Now let's use it.

```rust
let mut x = PersonList { persons: Box::new(Vec::new()), };
let mut y = x;
// x is not the owner any more...
x.persons.push(Person{ name: "Fred".to_string(), age: 30} );
```

The variable x is on the stack and is a PersonList but the persons member is allocated from the heap.

The variable x is bound to a PersonList on the stack. The vector is created in the heap. If we assign x to y then we could have two stack objects sharing the same pointer on the heap in the same way we did in C++.

But Rust stops that from happening. When we assign x to y, the compiler will do a bitwise copy of the data in x, but it will bind ownership to y.  When we try to access the in the old var Rust generates a compile error.

```
error[E0382]: use of moved value: `*x.persons`
   |
10 | let mut y = x;
   |     ----- value moved here
11 | x.persons.push(Person{});
   | ^^^^^^^^^ value used here after move
   |
   = note: move occurs because `x` has type `main::PersonList`, which does not implement the `Copy` trait
```

Rust has stopped the problem that we saw in C++. Not only stopped it but told us why it stopped it - the value moved from x to y and so we can't use x any more.

Sometimes we DO want to copy / duplicate an object and for that we must implement a trait to tell the compiler that we want that.

The Copy trait allows us to do direct assignment between variables. You can only implement Copy by deriving it:

But this will create an error:

```rust
#[derive(Copy)]
struct Person {
  name: String,
  age: u8
}
```

A struct can be copied if all its members can be copied and in this case "name" cannot be. The "name" field is of type String and that does not implement the Copy trait. However it implements the Clone trait.

A Clone trait can be derived or explicitly implemented. We can derive it if every member of the struct can be cloned which in the case of Person it can:

```rust
#[derive(Clone)]
struct Person {
  name: String,
  age: u8
}
...
let x = Person { /*...*/ };
let y = x.clone();
```

Now that Person derives Clone, we can do the same for PersonList because all its member types implement that trait - a Person can be cloned, a Vec can be cloned, and a Box can be cloned:

```rust
#[derive(Clone)]
struct PersonList {
    pub persons: Box<Vec<Person>>,
}
```

And now we can clone x into y and we have two independent copies.

```rust
//...
let mut x = PersonList { persons: Box::new(Vec::new()), };
let mut y = x.clone();
// x and y are two independent lists now, not shared
x.persons.push(Person{ name: "Fred".to_string(), age: 30} );
y.persons.push(Person{ name: "Mary".to_string(), age: 24} );
```

In summary, Rust stops us from getting into trouble by treated assigns as moves when a non-copyable variable is assigned from one to another. But if we want to be able to clone / copy we can make our intent explicit and do that too.

C++ just lets us dig a hole and fills the dirt in on top of us.

## Missing braces in conditionals

I expect every programmer has encountered an error like this and spent hours trying to figure out why it wasn't working.

```c++
const bool result = fetch_files();
if (result) {
  process_files()
}
else
  print_error()
  return false;

// Now cleanup and
cleanup_files();
return true;
```

The reason of course was the else statement wasn't enclosed in braces so the wrong code was executed. The compiler might spot dead code in this instance but that may not always be the case. Even if it did, it might only issue a warning instead of an error.

The problem can be especially annoying in deeply nested conditions where a misplaced brace can attach to the wrong level.
This problem has lead real-world security issues. For example here is the infamous ["goto fail"](https://www.imperialviolet.org/2014/02/22/applebug.html) bug that occured in some Apple products. This (intentional?) bug occured during an SSL handshake and was exploitable. :

```c++
static OSStatus
SSLVerifySignedServerKeyExchange(
   SSLContext *ctx, bool isRsa, SSLBuffer signedParams,
   uint8_t *signature, UInt16 signatureLen)
{
  OSStatus        err;
  //...

  if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
    goto fail;
  if ((err = SSLHashSHA1.update(&hashCtx, &signedParams)) != 0)
    goto fail;
    goto fail;
  if ((err = SSLHashSHA1.final(&hashCtx, &hashOut)) != 0)
    goto fail;
  //...

fail:
  SSLFreeBuffer(&signedHashes);
  SSLFreeBuffer(&hashCtx);
  return err;
}
```

Note how the "goto fail" is repeated twice and not bound to the condition but is indented as if it was. The code would jump straight into the fail label and return with an err indicating success (since the prior SHA1 update had succeeded). If conditionals

### How Rust helps

Rust requires if-else expressions and loops to be associated with blocks.

So this code won't compile:

```rust
let mut x: i32 = do_something();
if x == 200 {
  // ...
}
else
  println!("Error");
```

If you try you will get an error like this.

```
rustc 1.13.0-beta.1 (cbbeba430 2016-09-28)
error: expected `{`, found `println`
  |
8 |   println!("Error");
  |   ^^^^^^^
  |
help: try placing this code inside a block
  |
8 |   println!("Error");
  |   ^^^^^^^^^^^^^^^^^^
error[E0425]: unresolved name `do_something`
  |
3 | let mut x: i32 = do_something();
  |                  ^^^^^^^^^^^^ unresolved name
```

## Assignments in conditionals

The omission of an = in an == condition turns it into an assignment that evaluates to true:

```c++
int result = getResponseCode();
if (result = 200) { // BUG!
  // Success
}
else {
  //... Process error
}
```

So here, result was assigned 200 rather than compared to the value 200. Some compilers may issue a warning in some cases, but an error would be better.

Some developers might try to reverse the left and right hand side to mitigate the issue:

```c++
if (200 = result) { // Compiler error
  // Success
}
else {
  // ... Process error
}
```

Now the compiler will complain because the value of result is being assigned to a constant which makes no sense. This may work if a variable is compared to a constant but arguably it makes the code less readable and wouldn't help if the left and right hand sides were both assignable so their order didn't matter.

The "goto fail" example that we saw in section "Missing braces in conditionals" also demonstrates a real world code of code abbreviating an assignment and comparison into a single line:

```c++
if ((err = SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
  goto fail;
```

This line is not broken for other reasons, but it's easy to see how might be, especially if this pattern were repeated all over the place. The programmer might have saved a few lines of code to combine everything in this way but at a greater risk. In this case, the risk might be inadvertantly turning the = into an ==, i.e. comparing err to the function call and then comparing that to 0.

```c++
if ((err == SSLHashSHA1.update(&hashCtx, &serverRandom)) != 0)
  goto fail;
```

### How Rust helps

This code just won't compile:

```rust
let mut result: i32;
if result = 200 { // Compile Error
  //...
}
```

The only form of assignment inside a conditional is the specialised and explicit "if let" and "while let" forms which are explained pages 55 and 58 respectively.

## Class member initialisation

C++ does not require that you initialise all variables in every constructor.

* A member that is a C++ class with a default constructor doesn't need to be initialised
* A member that is a C++ class without a default constructor must be explicitly initialised.
* A member that is a references must be explicitly initialised
* Primitive types, including pointers do not have to be initialised
* Members do not have to be initialised in the order they are declared

Some compilers may issue warnings if you forget to initialise members or botch their ordering, but they will still compile the code.

TODO C++11 allows initialisation of variables in-place but this depends on every variable being initialised the same way for each overloaded constructor.

### How Rust helps

You MUST initialise all members of a struct. If your code does not initialise a struct you will get a compiler error.

This will not compile:

```rust
struct Alphabet {
  a: i32,
  b: u32,
  c: bool,
}

let a = Alphabet { a: -10, c: true };
```

If you try you will get an error like this:

```
rustc 1.13.0-beta.1 (cbbeba430 2016-09-28)
error[E0063]: missing field `b` in initializer of `main::Alphabet`
  |
9 |     let a = Alphabet { a: -10, c: true };
  |             ^^^^^^^^ missing `b`
```

## Header and source files

A header file contains definitions of classes, types, macros etc that other files need to #include in order to resolve their use of those things.

Splitting the implementation and definition across different files is an added burden for maintaining code but it can also lead to some serious errors.

* Headers used across multiple projects that have different compiler settings
* Issues with pragmas and alignment
* Issues with different #definitions that affect byte length
* Issues with different typedefs that affect byte length

Each consumer of the header must do so with the exact same settings that affect the size of every type, struct and class in the file plus any issues with packing / alignment. If these settings are not the same, it can cause instability, corruption or problems that only manifest themselves at at runtime.

Headers also make the compiler slower because source that consumes the header inevitably pulls in other headers which pull in other headers.

### Guard blocks / #pragma once

Headers will also be expanded as many times as they are #included. To prevent the expansion happening more than once per source file, they're usually protected by guard blocks.

```c++
#ifndef FOO_H
#define FOO_H
....
#endif
```

If the same header is included more than once, the second time through it is preprocessed into nothing. Some compilers support #pragma once which is neater and avoids having to open or evaluate the file a second time at all.

### #pragma once

Compilers have also tried tricks such as precompiled headers to speed up this problem but it doesn't work well in every scenario and comes with its own issues.  That one file that creates the precompile header has to have special case compile flags and every other file has to have special case compile flags to refer to the precompiled header and also dependencies on that one file.

### Pimpl pattern
A popular workaround for header issues is the Pimpl pattern. It is a way to separate a class into a public part and a private implementation part.

The public class is almost an interface definition in its purity that can be defined in the header with minimal dependencies. It forward references the implementation class and stores it as a member:

```c++
#pragma once

// Gory details are in the .cpp file
class ComplexThingImpl;

class ComplexThing {
  ComplexThingImpl *pimpl_;
public:
  ComplexThing();
  ~ComplexThing();

  // See note 1 below

  void somethingReallyComplex();
};
```

The constructor for the outer class would allocate the implementation class and method calls would call through to the inner.

The private implementation class is defined in the source file and can pull in as many extra headers as it needs, pragmas whatever without hurting consumers or compile times of the header.

```c++
// source file
#include "random/header.hpp"
// Lots of includes here
#include <...>
#include "more/stuff.hpp"

class  ComplexThingImpl {
  // Lots of member variables and stuff here
  // ...
public:
  void somethingReallyComplex();
}

void ComplexThingImpl::somethingReallyComplex() {
  // Lots of complex stuff here
  // ...
}

ComplexThing::ComplexThing() :
  pimpl_(new ComplexThingImpl()) {
}

ComplexThing::~ComplexThing() {
  delete pimpl_;
}

void ComplexThing:: somethingReallyComplex() {
  pimpl_->somethingReallyComplex();
}
```

This solution is known as Pimpl pattern and while it can work to protect consumers and speed up builds it also adds complexity and overhead to development. It's also optional and many pieces of code wouldn't do it, couldn't do it or would only do it to varying degrees.

For example one reason it couldn't be used is that the implementation class is heap allocated. Code that creates a lot of small objects on the heap could contribute to heap fragmentation.

Note 1: Remember the rule of three? That applies to this object too. The example doesn't show it but if we copy constructed or assigned ComplexThing to another instance we'd be in a heap of trouble. So on top of the issues with making PImpl work we also have to prevent the other ones. The easiest way to lock it down would be to derive from boost::noncopyable if you were using boost.

### How Rust helps

In Rust the definition and the implementation are the same thing.

TODO use ComplexThing

Writing a function defines the function. Let's assume we have a functions.rs file

```rust
// functions.rs
pub fn create_directory_structure() {
  // Implementation
}
```

Anyone can call it as functions::create_directory_structure(). The compiler will validate the call is correct.

A struct's definition and its implementation functions are written once. e.g. directory.rs

```rust
// directory.rs
pub struct Directory {
  pub path: String,
}
impl Directory {
  pub fn mkdir(&self) {
    // implementation
  }
}
```

Implementations can be defined in a private Rust module and only public structs exposed to consumers.
If we were a library crate (which we'll call file_utils) wishing to expose these objects to consumers we would write a top-level lib.rs which says what files our lib comprises of and we want to expose.

```rust
// lib.rs for file_utils
mod functions;
mod directory;
pub use functions::*;
pub use directory::Directory;
```

Now a consumer can use our crate easily:

```rust
extern crate file_utils;
use file_utils::*;
fn main() {
   create_directory_structure();
   let d = Directory { /* ... */ };
}
```

## Forward declarations

C++ prevents us from referring to a class or function which has not been defined yet. The compiler will complain even if the class or function is in the same file it is referenced from.

This means ordering matters. If our function or class is used by other files, we have to declare the function in a header. If our function is private to a source file, we have to declare it in the source file, and possibly make it static.

For classes we can make a forward reference. This acts as a hint to compiler to say a class does exist with this name and it will be told about it shortly. But it's a hack and it imposes limits on how we can use the forward declared class.

For example, DataManager below can hand out Data objects but the Data object has a reference to the DataManager. Since each class refers to each other there is no simple way to make the compiler happy except with a forward declaration.

```c++
class Data; // Forward declaration

class DataManager {
public:
  Data *getDataById(const std::string &id);
};

class Data {
public:
  Data(DataManager &dataManager);
}
```

But forward declaration compromises the design of the code. For example we couldn't hold the Data objects in a collection class:

```rust
class Data;

class DataManager {
  std::map<std::string, Data> data_;
public:
  Data *getDataById(const std::string &id);
}
```

The compiler would complain because it doesn't know anything about the constructors or size of Data. So instantly the design has to change because of a dumb compiler restriction. e.g. we might store a pointer to Data instead in the map but then we'd have to remember to delete it. So forward references increase the potential for bugs.

```c++
class Data;

class DataManager {
  // Great, now we have to remember to new / delete Data and we increase
  // memory fragmentation
  std::map<std::string, Data*> data_;
public:
  Data *getDataById(const std::string &id);
}
```

### How Rust helps

In Rust forward declarations are unnecessary. The struct and function’s definition can reside in a .rs and be referenced with a use directive.

TODO

## Namespace collisions

C code has no namespaces at all and namespaces in C++ are optional. The C++ language makes them easy to declare but there is no compunction for any code to bother or to do so in anything but the most perfunctory way.

By default all C++ code resides in a global namespace:

```c++
void hello() {
  // My function hello
}

int main() {
  // Main entry point
}
```

The function hello() is part of the global namespace. Calls to hello() could be replaced with calls to ::hello(). The problem of course is that the more code we write into the global namespace, or the more libraries we pull in that have no namespaces, the more chance there is of collisions.  

C has learned to live without namespaces. Most C code tends to prefix all their functions and structs to avoid collisions, e.g sqlite3_exec() is a function belonging to SQLite3 and uses the prefix because exec() by itself is too common. So the prefix acts as a pseudo namespace. But this adds noise to our code and would be unnecessary if namespaces were supported and enforced.

Of course C++ does have namespaces, but code has to choose to use them and it has to use them correctly. It is easy to abuse them because the compiler doesn’t really care what we do in a header, for example this is never a good idea:

```c++
// Inside of foo.h...
using namespace std;
```

Any file that #includes foo.h is inadvertently setting the compiler to automatically look up unscoped types and functions against std which may not be what the code wants at all.

Namespacing requires code enclose the namespaced portion in a block.

```c++
namespace application {
  // stuff in here belongs to application::
}
```

Nested namespacing is also possible but it can look really messy

```c++
namespace application { namespace gui {
  // stuff in here belongs to application::gui::
} }
```

If we forget to close a brace it becomes very easy to make C++ throw up a wall of incoherent errors.

### How Rust helps

In Rust every file is implicitly a module (equivalent to a namespace). We don't need to explicitly declare a namespace although that option exists too if we wish.

## Macros

Macros in C/C++ are basically little rules that are defined by a preprocessor and substituted into the code that the compiler ultimately attempts to compile.

Modern coding practice these days is to use inline functions and constants instead of macros.

But the reality is they can still be (ab)used and code often does. For example code might insert debug statements or logging which is compiled away in release mode.

Another common use is on Windows where the type TCHAR compiles to be either char or wchar_t depending on UNICODE being defined or not. Along with it go macros like USES_CONVERSION, A2CT, T2CW etc. Code should compile cleanly either way but the reality is usually it doesn't.

A classic problem would be something like this:

```c++
#define SQUARED(x) x * x
// And in code
float result = SQUARED(++x);
That would expand to
float result = ++x * ++x;
```

So the value in result would be wrong and the value in x would be incremented twice.

### Compilation errors

Consider we are compiling this structure:

```c++
// Header
struct Tooltip
#if TOOLTIP_VERSION > 4
  char buffer[128];
#else
  char buffer[64];
#endif
};
```

And in C++

```c++
Tooltip tooltip;
memset(&tooltip, 0, sizeof(tooltip));
```

If we fail to define TOOLTIP_VERSION to the same value in the implementation as in the caller, then this code may stomp all over memory because it thinks the struct is 128 bytes in one place and 64 bytes in another.

### Namespace issues

Macros aren't namespaced and in some cases this leads to problems where a macro definition collides with a well qualified symbol.
For example in Windows, TRUE is a #define for 1. But that excludes any other code that expects to compile on Windows from ever using TRUE as a const no matter how well they qualify it. Consequently code has to do workarounds such as #undef macros to make code work or using another value.

```c++
#ifdef TRUE
#define TMP_TRUE TRUE
#undef TRUE
#endif
bool value = myapp::TRUE;
#ifdef TMP_TRUE
#define TRUE TMP_TRUE
#undef TMP_TRUE
#endif
```

Ugh. But more likely we'll rename myapp::TRUE to something like myapp::MYAPP_TRUE to avoid the conflict. It's still an ugly workaround for a problem caused by inconsiderate use of macros.

Commonly used words like TRUE, FALSE, ERROR, OK, SUCCESS, FAIL are more or less unusable thanks to macros.

### How Rust helps

Rust provides developers with consts, inline attributes, and platform / architecture attributes for the purpose of conditional compilation.

Rust offers macros but they consist of a set of matching rules than must generate syntactically Rust. Macro expansion is performed by the compiler so it is capable of generating errors on the macro if the macro is in error.

##Type mismatching
Consider two methods. Both are called evaluate() and they are overloaded. The main() method calls evaluate("Hello world"). What version is called in the compiled code?
#include <iostream>
#include <string>

using namespace std;

void evaluate(bool value) {
    cout << "Evaluating a bool " << value << endl;
}

void evaluate(const std::string &value) {
    cout << "Evaluating a string " << value << endl;
}

int main() {
    evaluate("Hello world");
    return 0;
}

It may surprise you to know that the bool version is called and the compiler doesn't even complain about it either:

```
Evaluating a bool 1
```

This is an example of bad type inference. A string literal (a char *) should be turned into a std::string (a C++ string has a constructor that takes char *) but the compiler chose to treat it as a bool instead.

On other occasions the compiler might spot ambiguity and complain but the blurred lines between types in C++ combined with overloading lead to errors:
Here is another example where the compiler is a little more useful by generating an error, but in doing so it demonstrates the limits of overloading

```c++
bool evaluate(bool value);
bool evaluate(double value);
```

These overloaded methods should be distinct but they're not distinct enough as far as the compiler is concerned.

In summary, blurred and confusing rules about types in C++ can cause unexpected errors that can propagate to runtime.

### How Rust helps

In Rust the functions cannot be overloaded in this manner.

Rust is also more strict about type coercion - if you have a bool you cannot pass it to a function that takes an integer.

Nor can you pass an integer of one size to a function taking an integer of another size.

```rust
fn print_i32(value: i32) {
   println!("Your value is {}", value);
}
let value = 20i16; // 16-bit int
print_i32(value);
```

This will yield an error:

```
error[E0308]: mismatched types
  |
7 | print_i32(value);
  |           ^^^^^ expected i32, found i16
```

You must use an explicit numeric cast to turn the value into the type the function expects:

```rust
print_i32(value as i32);
```

## Explicit / implicit constructors on classes

It's not just overloading that be a mess. C++ has a bunch of rules about implicit / explicit type conversion for single argument constructors.

For example this would work:

```c++
class MagicNumber {
public:
    MagicNumber(int value) {}
};

void magic(const MagicNumber &m) {
  //...
}

int main() {
    //...
    magic(2016);
    return 0;
}
```

The function magic takes a const reference to a MagicNumber but we passed an int. How did it work? Well our MagicNumber class has a constructor that takes an int so the compiler helpfully implicitly converted the value for us.

If we didn't want the implicit conversion (e.g. maybe it's horribly expensive to do this without knowing), then we'd have to tack an "explicit" keyword to the constructor to basically turn off that behaviour.

```c++
explicit MagicNumber(int value) {}
```

It demonstrates that the default behavior is probably wrong. The default should be explicit and if programmers want implicit they should be required to say it.
C++11 adds to the confusion by allowing classes to declare deleted constructors which generate an error instead of code if they match.

```c++
class MagicNumber {
public:
    MagicNumber(int value) {}
    MagicNumber(double value) = delete;
};

//...
magic(2016);   // OK
magic(2016.0); // ERROR
```

### How Rust helps

Rust does not have constructors and therefore no implicit conversion.

The only form of implicit coercion it has is if for mutable references and certain kinds of raw pointers.

## Poor lifetime enforcement

A function like is completely legal and dangerous:

```c++
std::string &getValue() {
  std::string value("Hello world");
  return value;
}

This function returns a reference to a temporary variable. Whoever calls it will get a reference to garbage on the stack. Even if it appears to work (e.g. if we called the reference immediately) it is only through luck that it does.

Our compiler will probably issue a warning for this trivial example but it won't stop us from compiling it.

### How Rust helps

Rust tracks the lifetime of all objects and knows when their lifetime begins and ends. It tracks references to the object, knows when it is being borrowed (being passed to a function / scope).

It generate a compiler error if it detects any violations of its lifetime / borrowing rules. So the above code would fail to compile.

## Memory allocation

Allocated memory is memory that is requested from a portion of memory called a heap, used for some purpose and returned to the free space when it is no longer required.

The problems with C++ immediately start because there are 3 ways to heap allocate memory:

* Through C functions like malloc, calloc and free for buffers, arrays.
* Through new / delete for C++ class instances
* Through new[] and delete[] for buffers, arrays of C++ classes

If we fail to free / delete memory that we've allocated, the program will leak memory. If we free / delete memory we've already deallocated, the program may crash. If we free a C++ class with a C free() the program may leak memory. If we fail to call the correct constructor and destructor pair the program may leak / crash.

A cottage industry of tools has sprung up just to try and debug issues with memory leaks, crashes and so forth. Tools like Valgrind etc. specialise in trying to figure out who allocated something without freeing it.

For example, what's wrong with this?

```c++
std::string *strings = new std::string[100];
//...
delete strings;
```

Oops we allocated an array of strings with new[] but called delete without the []. So instead of deleting an array of strings we called delete with one of them. 99 of those string's destructors will never be called.
We should have written

```c++
delete []strings;
```

But the compiler doesn't care and so we have created a potentially hard-to-find bug.

Some of the problems with memory allocation can be mitigated by wrapping pointers with scoped or shared pointer classes. But there are even problems which can prevent them from working.

Many libraries also have external functions that we must call to create / destroy data that they consume. So issues about balancing calls apply to them too.

### How Rust helps

During normal safe programming Rust has no explicit memory allocation or deallocation. We simply declare an object and it continues to exist until it goes out of scope. This is NOT garbage collection since the compile tracks the lifetime of the object and automatically deletes it when it is no longer used. The compiler also knows if we enclose an object's declaration inside a cell, box, rc or similar construct that the object should be allocated on the heap and otherwise it should go on the stack.

Allocation / deallocation is only available in unsafe programming. We would not only ordinarily do this except when we are interacting with an external library or function call and explicitly tag the section as unsafe.

## Null pointers

The need to test a pointer for NULL, or blindly call a pointer that might be NULL has caused so many errors that it has even been called the [billion dollar mistake](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare)

## Virtual destructors

C++ allows classes to inherit from other classes.

In some cases, such as this example, this can lead to memory leaks:

```c++
class ABase {
public:
  ~ABase() {}
};

class A : public ABase {
  std::string *value_;
public:
  A() : value_(new std::string) {}
  ~A() { delete value_; }
};

void do_something() {
  ABase *instance = new A();
  //...
  delete instance;
}
```

So here we allocate a pointer to A, assign it to "instance" which is of type ABase, do something with it and finally delete it. It looks fine but we just leaked memory! When we called "delete instance" the code invoked the destructor ~ABase() and NOT the destructor ~A() . And value_ was not deleted and the memory leaked. Even if we'd used a scoped pointer to wrap value_ it would still have leaked.

The code should have said

```c++
class ABase {
public:
  virtual ~ABase() {}
};
```

The compiler didn't care our code was in error. It just allowed us to leak for the sake of a missing keyword.

### How Rust helps

Rust also does not use inheritance so problems like ABase above cannot exist. In Rust ABase would be declared as a trait that A implements.

```rust
trait ABase {
  //...
}

struct A {
  value: String,
}

impl ABase for A {
  //...
}
```

Rust also allows our struct to implement another trait called Drop which is equivalent to a C++ destructor.

```rust
impl Drop for A {
  fn drop(&mut self) {
    println!("A has been dropped!");
  }
}
```

It allows our code to do something during destruction such as to free an open resource, log a message or whatever.

## Exceptions

There are no hard and fast rules for when a function in C++ should throw an exception and when it should return a code. So one codebase may have a tendency to throw lots of exceptions while another might throw none at all.
Aside from that, code may or may not be exception safe. That is, it may or may not free up its resources if it suffers an exception.
Articles have been written to describe the levels of guarantees that code can aim for with [exception safety](http://www.boost.org/community/exception_safety.html).

## Constructors

You may also be advised to throw exceptions in constructors because there is no easy way to signal the object is an error otherwise except to set the new object into some kind of zombie / dead state via a flag that has to be tested.

```c++
DatabaseConn::DatabaseConn() {
  db_ = connect();
  if (db_ == NULL) {
    throw string("The database connection is null");
  }
}

// These both recover their memory
DatabaseConn db1;
DatabaseConn *db2 = new DatabaseConn();
```

But if DatabaseConn() had allocated some memory before throwing an exception, this would NOT be recovered and so ~DatabaseConn would have to clean it up.

```c++
DatabaseConn::DatabaseConn() {
  buffer_ = new char[100];
  // ... exception throwing code
}
```

DatabaseConn::~DatabaseConn() {
  if (buffer_) {
    delete[] buffer_;
  }
}
```

But if we waited until after the exception throwing to allocate memory then maybe buffer_ is not set to NULL, so we'd have to ensure we initialised it to NULL.

```c++
DatabaseConn::DatabaseConn() : buffer_(NULL) {
  // ... exception throwing code
  buffer_ = new char[100];
}
```

## Destructors

But you will be advised NOT to throw exceptions in destructors because throwing an exception during a stack unwind from handling another exception is fatal.

```c++
BadNews::~BadNews() {
    if (ptr == NULL) {
      throw string("This is a bad idea");
   }
}
```

### How Rust helps

The recommended way of dealing with errors is to use the Option and Result types to formally pass errors to your caller.

For irregular errors your code choose invoke panic!()  which is a little like an exception in that it will cause the entire thread to unwind. If the main thread panics then the process terminates.

A panic!() can be caught and recovered from in some scenarios but it is the nuclear option.

Lacking exceptions might seem a bad idea but C++ demonstrates that they come with a whole raft of considerations of their own.

## When templates go bad

C++ provides a way of substituting types and values into inline classes and functions called templates. This is a very powerful feature since it allows a class to be reused for many different types.

Templates are used extensively in the C++ library, Boost and in other places. Collections, strings, algorithms and various other piece of code use templates in one form or another.

However, templates only expand into code when something actually calls the inline function. Then, if the template calls other templates, the inline code is expanded again and again until there is a large body of code which can be compiled. A small error in our code can propogate into an enormous wall of noise in the middle of some expanded template.

For example a vector takes a type it holds as a template parameter. So we can create a vector of PatientRecords.

```c++
class PatientRecord {
  std::string name_;

  PatientRecord() {}
  PatientRecord operator= (const PatientRecord &other) { return *this; }

public:
  PatientRecord(const std::string &name) : name_(name) {
  }
};
...
std::vector<PatientRecord> records;
```

So far so good. So let's add a record:

```c++
records.push_back(PatientRecord("John Doe"));
```

That works too! Now let's try to erase the record we just added:

```c++
records.erase(records.begin());
```

Boom!

```
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h: In instantiation of 'static _OI std::__copy_move<true, false, std::random_access_iterator_tag>::__copy_m(_II, _II, _OI) [with _II = PatientRecord*; _OI = PatientRecord*]':
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h:396:70:   required from '_OI std::__copy_move_a(_II, _II, _OI) [with bool _IsMove = true; _II = PatientRecord*; _OI = PatientRecord*]'
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h:434:38:   required from '_OI std::__copy_move_a2(_II, _II, _OI) [with bool _IsMove = true; _II = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; _OI = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >]'
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_algobase.h:498:47:   required from '_OI std::move(_II, _II, _OI) [with _II = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; _OI = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >]'
c:/mingw/i686-w64-mingw32/include/c++/bits/vector.tcc:145:2:   required from 'std::vector<_Tp, _Alloc>::iterator std::vector<_Tp, _Alloc>::_M_erase(std::vector<_Tp, _Alloc>::iterator) [with _Tp = PatientRecord; _Alloc = std::allocator<PatientRecord>; std::vector<_Tp, _Alloc>::iterator = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; typename std::_Vector_base<_Tp, _Alloc>::pointer = PatientRecord*]'
c:/mingw/i686-w64-mingw32/include/c++/bits/stl_vector.h:1147:58:   required from 'std::vector<_Tp, _Alloc>::iterator std::vector<_Tp, _Alloc>::erase(std::vector<_Tp, _Alloc>::const_iterator) [with _Tp = PatientRecord; _Alloc = std::allocator<PatientRecord>; std::vector<_Tp, _Alloc>::iterator = __gnu_cxx::__normal_iterator<PatientRecord*, std::vector<PatientRecord> >; typename std::_Vector_base<_Tp, _Alloc>::pointer = PatientRecord*; std::vector<_Tp, _Alloc>::const_iterator = __gnu_cxx::__normal_iterator<const PatientRecord*, std::vector<PatientRecord> >; typename __gnu_cxx::__alloc_traits<typename std::_Vector_base<_Tp, _Alloc>::_Tp_alloc_type>::const_pointer = const PatientRecord*]'
..\vectest\main.cpp:22:34:   required from here
..\vectest\main.cpp:8:19: error: 'PatientRecord PatientRecord::operator=(const PatientRecord&)' is private
     PatientRecord operator= (const PatientRecord &other) { return *this; }
```

If you wade through that noise to the bottom we can see the erase() function wanted to call the assignment operator on PatientRecord, but couldn't because it was private.
But why did vector allow us to declare a vector with a class which didn't meet its requirements?
We were able to declare the vector, use the std::vector::push_back() function but when we called std::vector::erase() the compiler discovered some deeply nested error and threw these errors back at us.
The reason is that C++ only generates code for templates when it is called. So the declaration was not in violation, the push_back() was not in violation but the erase was.
How Rust helps
Rust has a concept similar to templates called generics. A generics is a struct or trait that takes type parameters just like a template.
However but the type can be enforced by saying the traits that it must implement. In addition any errors are meaningful.
Say we want to write a generic function that clones the input value:
fn clone_something<T>(value: T) -> T {
  value.clone()
}
We haven't even called the function yet, merely defined it. When we compile this, we'll instantly get an error in Rust.
error: no method named `clone` found for type `T` in the current scope

```
  |
4 |   value.clone();
  |         ^^^^^
  |
  = help: items from traits can only be used if the trait is implemented and in scope; the following trait defines an item `clone`, perhaps you need to implement it:
  = help: candidate #1: `std::clone::Clone`
```

Rust is saying we never said what T was and because some-random-type has no method called clone() we got an error.
So we'll modify the function to add a trait bound to T. This binding says T must implement Clone:
fn clone_something<T: Clone>(value: T) -> T {
  value.clone();
}
Now the compiler knows T must have implement Clone it is able to resolve clone() and be happy.
Next we actually call it to see what happens:
struct WhatHappensToMe;
let x = clone_something(10);
let y = clone_something(WhatHappensToMe{});
We can clone the integer 10 because integers implement the Clone trait, but our empty struct WhatHappensToMe does not implement Clone trait. So when we compile it we get an error.

```
error[E0277]: the trait bound `main::WhatHappensToMe: std::clone::Clone` is not satisfied
  |
8 | let y = clone_something(WhatHappensToMe{});
  |         ^^^^^^^^^^^^^^^
  |
  = note: required by `main::clone_something`
```

In summary, Rust improves on templates by TOD

Compiling generic functions / structs even when they are unused and offer meaningful errors immediately.

Allow us to bind traits to generic types to constrain what we can pass into them.

Offer meaningful errors if we violate the requirements of the trait bounds

## Diamond pattern multiple inheritance

C++ allows code to inherit from multiple classes and they in turn could inherit from other classes. This gives rise to the dreaded diamond pattern

e.g. D inherits from B and C but B and C both inherit from A. So does D have two instances of A or one?

This can cause compiler errors which are only partially solved by using something called "virtual inheritance" to convince the compiler to share A between B and C.

i.e if we knew B and C could potentially be multiply inherited we might declare them with a virtual keyword in their inheritance:

```c++
class B : public virtual A {
//...
};
class C: public virtual A {
};
class D: public B, public C {
//...
};
```

When D inherits from B and C, both share the same instance of A. But that assumes the authors of A, B and C were aware of this problem arising and coded themselves with the assumption that A could be shared.

The more usual normal solution for diamond patterns is "don't do it". i.e use composition or something to avoid the problem.

### How Rust helps
Rust also does not use class inheritance so problems like diamond patterns cannot exist.

However traits in Rust can inherit from other traits, so potentially it could have diamond-like issues. But to ensure it doesn't, the base trait is implemented separately from any traits that inherit from it.

So if struct D implements traits B & C and they inherit from A, then A, B and C must have impl blocks.

```rust
trait A {
//...
}

trait B : A {
//...
}

trait C : A {
//...
}

struct D;

impl A for D {
//...
}

impl B for D {
//...
}

impl C for D {
//...
}
```

## What about C?

A lot of these issues are concerning C++ and so someone might be inclined to think that therefore C does not suffer from problems.

Unfortunately is somewhat akin to arguing we don't need shoes because we have no legs. It is a valid argument on one level but one that misses the bigger issue.

So let's start out saying why C++ is a step up from C in so many ways:

* string class is safe and a huge step-up from raw string pointers
* collections including vectors are well designed and efficient
* C++ classes allow RAII style programming
* C++ has namespaces
* TODO


## Compiler warnings that should be errors

A lot of warnings in C++ should really be errors.

The simplest way to catch errors is to bump up warnings to be errors.

In Microsoft VC++ enable a high warning level, e.g. /W4 and possibly /WX to warnings into errors.

In GCC enable -Wall, -pedantic-errors and possibly -Werror to turn warnings into errors. The pedantic flag rejects code that doesn't follow ISO C and C++ standards. There are a lot of errors that can be configured - https://gcc.gnu.org/onlinedocs/gcc/Warning-Options.html#Warning-Options

However this will probably throw up a lot of noise in your compilation process and some of these errors may be beyond your means to control.

### Linker

C and C++ requires you supply a list of all the .obj files that form part of your library or executable.

If you omit a file by accident you will get undefined or missing references. Maintaining this list of files is an additional burden of development, ensuring to update your makefile or solution every time you add a file to your project.

### How Rust Helps

Rust includes everything in your library / executable that is directly or indirectly referenced by mod commands, starting from your toplevel lib.rs or main.rs and working all the way down.

Providing you reference a module, it will be automatically built and linked into your binary.

## Runtime problems

But C++ has problems in its language that can go unchecked and only appear in production.

* Calling a pure virtual function in a constructor / destructor of a base class
* Calling a dangling pointer
* Freeing memory twice
* Overflowing a buffer, e.g. being off by one with some string operation or not testing a boundary condition
* Memory leaks due to memory allocation / ownership issues
* Heap corruption
* Stack overflows

Many of these conditions could have been avoided if the compiler stopped with an error rather than producing code with the potential to break.

## Miscellanea

* Not enforcing requiring variables be initialized to a value
* Not initialising buffers with zero bytes
* Not enforcing array bounds in pointer manipulation
* Inadvertantly encouraging wrong types by making it more effort to say "unsigned int" than just "int" etc.
* Requiring code to  TODO
