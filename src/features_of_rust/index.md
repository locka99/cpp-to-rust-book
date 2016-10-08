# Features of Rust

## Types

Rust has analogue types for the numeric primitives in C/C++.

Note that various C++ compilers support various data models and the standard only says that shorts, ints and longs must be "at least" a certain bit length. Therefore the rules change from one compiler and architecture to the next.

The four main models in C++ are:

* LP32 - int is 16-bit, long and pointer are 32-bit. This is an uncommon model, a throw-back to DOS / Windows 3.1
* ILP32 - int, long and pointer are 32-bit. Used by Win32, Linux, OS X
* LLP64 - ints and long are 32-bit, long long and pointer are 64-bit. Used by Win64
* LP64 - int is 32-bit, long / long long and pointer are 64-bit. Used by Linux, OS X

A basic conversion between types to Rust therefore follows

C++ | Rust
--- | ----
char | i8
unsigned char | u8
short (int) / signed short (int) | i16
unsigned short (int) | u16
int / signed int | i32 or i16 (LP32)
unsigned int | u32 or u16 (LP32)
float | f32
double | f64
size_t | usize / isize (most collections use usize)
long (int) | i32 or i64 (LP64)
unsigned long (int) | u32 or u64 (LP64)
long long (int) | i64
unsigned long long (int) | u64
bool | bool
char32_t / wchar_t | char (4 bytes!)

Note 1: that Rust's char type, is 32-bits wide, enough to hold any Unicode character. This is equivalent to the belated char32_t that appeared in C++11. On some operating systems / compilers wchar_t may be 32-bits but you can't rely on that being true, because on Windows they are 16-bits. When you iterate strings in Rust you may either iterate by character or u8, i.e. a byte.

## Arrays

An array is a fixed size list of elements allocated either on the stack or the heap.

E.g to create a 100 element array of doubles in C++:

```c++
double values[100];
```

And in Rust

```rust
let mut values = [f64; 100];
```

### Slices

A slice is a partial or full view of an array or a string. A slices is not a copy of the array, rather that it holds a pointer to a portion of the array and a length.

TODO example where we have a subsection on array

### Functions of an array

One serious disadvantage of C++ arrays is there is no .len() method so if you want to specify the length then you either do something ugly like this:

```c++
const size_t num_elements = 1024;
char buffer[num_elements];
//...
// fill_buffer needs to be told how many elements there are
fill_buffer(buffer, num_elements);
```

You might also see code like this:

```c++
Element elements[100];
//...
int num_elements = sizeof(elements) / sizeof(Element);
```

In addition we can pass a slice of the array which might be the whole array or only a portion of it:

```rust
let buffer: [u8; 1024]
fill_buffer(&buffer);
//...
fn fill_buffer(elements: &[Element]) {
  println!("Number of elements = {}", elements.len());
}
```

A slice is the pointer to a part of the array and the length of the slice. It means we provide all the information it needs

TODO - lots of mess below

C++ | Rust
--- | ----
T [N] (e.g. char x[32])
std::array<>, e.g. std::array<char, 32> | [T; N] (e.g. let x = [u8; 32])

## Strings

Strings in C++ can be a bit messy and to understand why requires some backstory.

### What is a character exactly?

Traditionally in C++, a char type is 8-bits. Strictly speaking it's a signed type, but the values essentially represent the values 0-255. Standards like US-ASCII used the first 7-bits (0-127) to assign values to upper and lower case letters in the English alphabet, numbers, punctuation marks and certain control characters. Other standards like EBDIC also assign values to upper and lower case letters, number, punctuation marks, but different values! By default, the encoding the compiler choose is what it sees fit - most compilers would use ASCII but some mainframes may still use EBDIC.

The problem is that the world uses MANY symbols, and they can't all fit in 256 values.

So in time, some operating systems came up with a hack - the OS set itself to a "code page" which defined how values mapped onto symbols. Normally the upper 128-255 values were repurposed according to the code page in effect. Thus most languages only had a handful of additional symbols that could be specified in this range. But CJK (Chinese-Japanese-Korean) languages as well as Thai, Arabic etc. have thousands of symbols so some languages require 2-byte symbols. For example Microsoft's code page 932 implemented Shift JIS (Japanese) where some symbols are two bytes.

Obviously this was rapidly becoming a mess. Software that rendered properly in one language didn't on another. In some cases, the fact that characters could be multiple bytes broke code which attempted to count bytes or split strings.

### Unicode to the rescue

The Unicode standard defines every printable symbol with a unique 32-bit value, a code point. Most symbols fall in the first 16-bits, the Basic Multilingual Plane (BMP) some Chinese characters still did not fit and China has mandated all software must to support all 32-bits.

C++ has a wide character called wchar_t that should correspond to a code point but on Windows it's only 16 bits wide, i.e. UTF-16. On gcc, it's a compiler switch. Normally this is fine, but some characters don't reside in 0-65535 and must be escaped. So C++11 adds explicit char16_t and char32_t types and corresponding versions of string called std::u16string and std::u32string.

So now C++ has 4 character types. Great huh?

Character type | Encoding
-------------- | --------
char | ad hoc, ASCII, EBDIC, UTF-8, ???
wchar_t | UTF-16 or UTF-32
char16_t | UTF-16
char32_t | UTF-32

#### C++

C and C++ never had a string primitive type, instead it uses a pointer to an array of chars which are zero-byte terminated.

The char type is a byte wide. The std::string template wraps a char pointer and provides methods for modifying the string in a safe manner.

The wchar_t type is for wide strings and is either 2 or 4 bytes wide and is compiler / platform specific. In Microsoft Visual C++ it is an unsigned short (corresponding to Win32's Unicode API), in gcc it can be 32-bits or 16-bits according to the compile flags. There is a corresponding  std::wstring.

#### Rust
Rust has been rather fortunate in that Unicode existed before it did and therefore it doesn't have any legacy baggage. It can choose to be UTF-8 encoding internally and expose 32-bit chars.
Rust only has str and std::String but they handle all cases.

#### Comparison

C++ | Rust
--- | ----
char * or wchar_t *
C++11 - char16_t \*, char32_t \* | &str
std::string, std::wstring
std::u16string std::u32string | std::String

### char * vs str

The closest C or C++ has to a string primitive is a char pointer, i.e. a string points to an arbitrary sequence of non zero bytes.  A zero byte terminates the string. The same applies for wider chars, except of course they require 2 or 4 bytes.

```c++
// The traditional way
char *my_str = "Hello"; // Terminates with \0
// or
char my_str[] = "Hello"; // Terminates with \0
// or wide string with L prefix
wchar_t hello_chinese = L"\u4f60\u597d";
// C11 and C++11 add UTF string literal prefixes
auto hello_8  = u8"\u4f60\u597d"; // UTF-8 encoded
auto hello_16 =  u"\u4f60\u597d"; // UTF-16
auto hello_32 =  U"\u4f60\u597d"; // UTF-32
```

Rust would use a str for all of these and does not need to specify a width or escape its Unicode characters.

```rust
let my_str = "Hello";
let hello_chinese = "你好";
```

We can instantly see it removes the mess of character width and literal prefixes that C and C++ have to suffer under.

Type inference creates a reference to a string slice, a &str. A string slice is a pointer to the data and a length (in bytes) of the portion to view. The string itself doesn't move and &str is read-only.

The str has functions for iterating over the string in bytes / characters, splitting, find a pattern etc.

```rust
let my_str = "Hello"; // v is a &’static str
println!("My string is {} and it is {} bytes long", v, v.len());
```

Note len() is the length in bytes because strings are UTF-8 encoded. A single character may be encoded as 1, 2, 3, or 4 bytes. It may not be the number of characters a human would actually see.

```rust
let my_str = "你好";
println!("Number of bytes = {}", my_str.len());
println!("Number of chars = {}", my_str.chars().count());
```

```
Number of bytes = 6
Number of chars = 2
```

You can do a split on a &str to produce a left and a right slice like this:

```rust
let (part1, part2) = "Hello".split_at(3);
println!("Part 1 = {}", part1);
println!("Part 2 = {}", part2);
```

```
Part 1 = Hel
Part 2 = lo
```

### std::basic_string (C++) vs std::String (Rust)

The standard C++ library also has template class std::basic_string that acts as a wrapper around the various character types and can be used for manipulating a string of any width. This template is specialised as
std::string, std:wstring, std::u16string, std::u32string.

```c++
std::string my_str = "Hello";
my_str += " world";

// C++11 also allows some type inference with autos
auto s1 =   "Hello"s; // std::string
auto s2 = u8"Hello"s; // std::string, forces UTF-8 encoding
auto s3 = L"Hello"s;  // std::wstring
auto s4 = u"Hello"s;  // std::u16string
auto s5 = U"Hello"s;  // std::u32string
```

In Rust, the std::String type serves the same purpose:

```rust
let v = String::from("Hello");
v.push_str(" world");
```

Using it is fairly simple

```rust
let mut v = String::from("This is a String");
v.push_str(" that we can modify");
```

To add two Strings together

```rust
let b = String::from(" Bananas");
let mut result = String::new();
result.push_str("Apples ");
result.push('&'); // Push a char
result.push_str(b.as_str());
println!("result = {}", result);
```

Strings are always valid UTF-8.

Internally a String has a pointer to the data, its length and a capacity (max size). If you intend to expand a string, then you should ensure the String has sufficient capacity to accommodate its longest value otherwise you may cause it to reallocate itself excessively.

Strings will never shrink their capacity unless you explicitly shrink_to_fit(). This means if you use a temporary string in a loop, it's probably best to place it outside the loop and reserve space to make it efficient.

```rust
let mut v = String::with_capacity(100);
// or
let mut v = String::new();
v.reserve_exact(100);
```

Strings also have all the methods of str thans to implementing Deref trait.

### Formatting strings

Rust allows objects to be formatted as strings based upon the formatting traits they implement
TODO description of {} meaning and C / C++ equivalent

C++ | Rust (pattern / Formatting trait)
--- | ---------
| {} / Display
| {:X}
| {:o}
| TODO

### OsString / OsStr

Rust recognises there are times when you need to pass or receive a string from a system API.

In this case you may use OsString which cheaply allows interchange between Rust and a system dependent representations of strings. On Windows it will return UTF-16 strings, on Linux / Unix systems it will return UTF-8.

An OsStr is a slice onto OsString.

## Variables

### Type Inference

C++11 has type inference, previous versions of C++ do not. Type inference allows the compiler to figure out the type of a variable based upon the value being assigned to it.

Boolean and numeric types are fairly easy to understand providing you are explicit as you need to be.

Where C++ gets really messy is for arrays and strings. Recall that strings are not primitive types in the strong sense within C or C++.

```c++
auto x = true; // bool
auto y = 42;   // int
auto z = 100.; // double
auto s = std::string("Now is the window of our discontent"); // char string
auto s = U"Battle of Waterloo"; // char32_t pointer to UTF-32 string literal
```

Strings are covered elsewhere, but essentially there are many kinds of strings and C++/C has grown a whole bunch of string prefixes to deal with them all.

Arrays are a more interesting problem. The auto keyword has no easy way to infer array type so is one hack workaround to assign a templatized array to an auto and coerce it.

```c++
template <typename T, int N> using raw_array = T[N];
auto a = raw_array<int, 5>{};
```

In Rust, the equivalent is let and it has no problem with arrays:

```rust
let x = true; // x: bool
let y = 42; // y: i32
let z = 100.0; // z: f64
let v = vec![10, 20, 30]; // v: Vec<i32>
let s = "Now is the winter of our discontent".to_string(); // s: String
let s2 = "Battle of Waterloo"; // s2: &str
let a = [i32; 5]
```

## Literals

### Strings

See the separate section on strings. (TODO ref)

### Booleans

Booleans are true or false.

### Numbers

Numbers are a decimal value followed by an optional type.

```rust
123i32;
123u32;
123_u32;
0usize;
```

There are also hex, octal and binary representations:

```rust
0xff_u8;
0o70_i16;
0b111_111_11001_0000_i32;
```

Floating point numbers are similar.

```rust
let a = 100.0f64;
let b = 0.134f64;
let c = 2.3f32; // But 2.f32 is not valid (note 1)
let d = 12E+99_E64;
```

Note 1: One quirk with floating point numbers is the decimal point is used for float assignments but it's also used for member and function invocation. So you can't say 2.f32 since it thinks you are referencing f32 on 2. Instead syntax requires you to say 2.f32 or alter how you declare the type, e.g. "let v: f32 = 2.;".

## Pointers

### In C++

A pointer is a variable that points to an address in memory. The pointer may be assigned to point somewhere else, and it might be assigned NULL. Since a pointer may be NULL, or set to point to garbage, as well as suffering dangling pointer issues, it is only used when a reference is not possible.

### In Rust:

The use of raw pointers tend to be reserved for a few use cases such as interacting with C/C++ where allocated memory may need to be extracted from a Box or placed into a Box outside the normal lifetime rules. Normally it is sufficient to use references only.

## References

### In C++
A reference is also a variable that points to an address but unlike a pointer, it cannot be reassigned and it cannot be NULL. Therefore a reference is generally assumed to be safer than a pointer. It is still possible for the a reference to become dangling, assuming the address it referenced is no longer valid.

### In Rust
A reference is also lifetime tracked by the compiler.

## Tuples

A tuple is bunch of values held in parenthesis. Functions may return a tuple too.

```rust
fn get_last_mouse_click() -> (i32, i32) {
  (100, 20)
}
let (x, y) = get_last_mouse_click();
```

## Collections
Most languages allow elements to be added collection classes - vectors, stacks, linked lists, sets, maps etc.
TODO collections

## Structs / Classes

### C++

A class and a struct in C++ are largely the same thing from an implementation standpoint. They both hold fields and they both can have methods attached to the class (static) or instance level. It is only the default access level (public for struct, private for class) which is different and some rules about templates that only apply to classes.

But from a psychological perspect a struct tends to be used to hold public data with few or no methods that is passed around. A class tends to be something more self contained with methods that are called to access or manage private fields.

These are equivalents:

```c++
struct Foo { // as a struct
private:
};

class Foo { // As a class
};

// Or the other way around

struct Bar {
};

class Bar {
public:
};
```

Classes can also use an access specifier to inherit from a base class. So a class may choose to publically or private inherit from another class depending on whether it wants those methods to be visible to callers, or subclasses.

Classes and structs may have special constructor and destructor methods which are described in sections below.

```c++
class Size {
public:
  Size(int width, int height);

  int width_;
  int height_;

  int area() const;
};
```

Then in the .cpp file you might implement the constructor and method:

```C++
Size::Size(int width, int height) : width_(width), height_(height) {}

int Size::area() { return width_ * height_; }
```

### Rust

Rust only has structs. A struct consists of a definition which specifies the fields and their access level (public or not), and an implementation section which specifies functions bound to the struct.

```rust
struct Size {
  pub width: i32;
  pub height: i32;
}
```

An impl section follows containing the associated functions:

```rust
impl Size {
  pub fn new(width: i32, height: i32) -> Size {
    Size { width: width, height: height, }
  }

  pub fn area(&self) -> i32 {
    self.width * self.height
  }
}
```

The new() function here is a convenience method that returns a struct preinitialised with the arguments supplied. The area() function specifies a &self argument and returns an area calculation. Any function that supplies a &self, or &mut self can be called from the variable bound to the struct.

```rust
let size = Size::new(10, 20);
println!("Size = {}", size.area());
```

The "self" keyword much the same way as C++ uses "this", as a reference to the struct from which the function was invoked. If a function modifies the struct it must say &mut self, which indicates the function modifies the struct.

There is no inheritance in Rust. Instead, a struct may implement zero or more traits. A trait describes some kind of behavior that can be associated with the struct and described further later on in this chapter.

### Constructors

In C++ all classes have implicit or explicit constructors. Either the compiler generates them or you do, or a mix of both.

An implicit default constructor, copy constructor and assignment operator will be created when a class does not define its own. We saw on page 73 why this could be really bad news.

What becomes obvious from reading there is a lot of noise and potential for error in C++ merely. There would be even more if raw pointers were used instead of a std::unique_ptr here.

In Rust, things are simpler, and we'll see how it shakes out errors.

First off, let's declare our equivalent struct in Rust:

```rust
struct Person {
  pub name: String,
  pub age: i32,
  pub credentials: Option<Credentials>,
}
```

Since credentials are optional, we wrap in an Option object, i.e. credentials might be None or it might be Some(Credentials).
Any code anywhere in the system can instantiate a Person simply be declaring an instance:

```rust
let person = Person { name: String::from("Bob"), age: 20, credentials: None }
```

In Rust you cannot create a struct without initialising all its members so we cannot have a situation where we don't know what is in each field - it MUST be set by our code.

But declaring the struct is a bit clumsy, especially if the struct is created in lots of places. So can write function that behaves like a constructor in C++.

Instead you implement a static method in the impl of the Struct which returns an initialised struct, e.g.

```rust
impl Person {
  pub fn new(name: String, age: String) -> Person {
    Person { name: name.clone(), age: age, credentials: None }
  }
}
```

Note that Rust does not support overloads. So if we had multiple "constructor" methods, they would each have to have unique names, e.g. if we had reasons to create an empty person then perhaps we'd have a new_empty() method filled in with the default values.

Finally what about copying the Person? There are two ways to do this, the first is to implement the Copy trait which means assignment is implicit, but is what we want? Do we really want to make copies of a struct by accident?
Instead we probably want to implement Clone instead to add a clone() method and require an explicit call in order to create a copy. But the compiler can derive clone() providing all the members of the struct implement the Clone trait.

```rust
#[derive(Clone)]
struct Person {
  pub name: String,
  pub age: i32,
  pub credentials: Option<Credentials>, // Credentials must implement Clone
}

impl Person {
  pub fn new(name: String, age: String) -> Person {
    Person { name: name.clone(), age: age, credentials: None }
  }
}


//...

let p = Person::new(String::from("Michael"), 20);
let p2 = p.clone();
```

What we can see is that Rust's construction and clone() behavior is basically declarative.
We saw how C++ has all kinds of rules and nuances to construction, copy construction and assignment which make it complicated and prone to error.  

### Destructors
A C++ destructor is a specialized method called when your object goes out of scope or is deleted.

```c++
class MyClass {
public:
  MyClass() : someMember_(new Resource()) {}
  ~MyClass() {
     delete someMember_;
  }

private:
  Resource *someMember_;
}
```

In C++ you can declare a class destructor to be called when the object is about to be destroyed.

We saw on page 96 how you must ensure you use a virtual destructor your class inherits from another class. Otherwise you might end up calling the destructor on the base class but not the thing derived from it.

Since Rust does not do inheritance and does not have constructors, the manner in which you cleanup is different and simpler. Instead of a destructor you implement the Drop trait.

```rust
impl Drop for Shape {
    fn drop(&mut self) {
        println!("Shape dropping!");
    }
}
```

The compiler recognizes this trait. If you implement this trait then the compiler knows to call your drop() function prior to destroying your struct. It’s that simple.

Occasionally there might be a reason to explicitly drop a struct before it goes out of scope. Perhaps the resources held by the variable should be freed as soon as possible to release a resource which is in contention. Whatever the reason, the answer is to call drop like this:

```rust
{
  let some_object = SomeObject::new();
  //...
  // Ordinarily some_object might get destroyed later,
  // but this makes it explicitly happen here
  drop(some_object);
  //...
}
```

### Access specifier rules

A C++ class can hide or show methods and members to any other class, or to things that inherit from itself using the public, private and protected keywords:

* public – can be seen by any code internal or external to the class
* private – can only be used by code internal to the class. Not even subclasses can access these members
* protected – can be used by code internal to the class and by subclasses.

A class may designate another function or class as a friend which has access to the private and protected members of a class.

Rust makes things somewhat simpler.

If you want something to access a member of a struct (including modifying it if its mutable), then mark it pub.

TODO example

If you want something to be able to call a function on your struct you mark it pub.

TODO example

If you want a struct to be visible outside your module you mark it pub

TODO example

### Methods

And then it has methods that you bind to the struct contained within an impl block:

```rust
impl Shape {
  fn area(&self) -> i32 {
    self.width * self.height
  }
  fn set(&mut self, width: i32, height: i32) {
    self.width = width;
    self.height = height;
  }
}
```

Note how the first parameter is a reference to self  which is the struct instance itself. In one method we pass a immutable reference to self because it doesn’t need to modify the struct. In the second we pass a mutable reference so we can modify the struct.

Unlike C++, all access to the struct has to be qualified. In C++ you don't have to say "this->foo" to access a member foo. Rust requires code to say unambiguously "self.foo".

### Static methods

Static methods are merely functions in the impl block that do not have self as their first parameter, e.g.

```rust
impl Circle {
   fn pi() -> f64 { std::f64::consts:PI }
}
//...
let pi = Circle::pi();
```

You can attach functions to the class or the instance depending on the first argument being &self or not.

### Traits

C++ allows one class to inherit from another. Generally this is a useful feature although it can get pretty complex if you implement multiple inheritance, particularly the dreaded diamond pattern.

As we’ve found out, Rust doesn’t have classes at all – they’re structs with bound functions.  So how do you inherit code? The answer is you don’t.

Instead your struct may implement traits which are a bit like partial classes.

A trait is declared like so:

```rust
trait HasCircumference {
  fn circumference() -> i32;
}
```

And then structs can implement the trait by declaring it

```rust
impl HasCircumference for Size {
  fn circumference() -> i32 {
    2 * width + 2 * height
  }
}
```

TODO traits partial implementation.

### Lifetimes

C++ doesn't really care how long objects live. If you maintain a pointer to some object from your class, it won't care if the pointer is valid or not. It won't care if the object has long gone and you're holding onto garbage.

Rust does care and carefully tracks the lifetime of objects to ensure that you cannot reference something that no longer exists.

Occasionally this causes problems for structs and classes where a struct holds a reference to some other struct but is unsure about the life time.

TODO lifetimes

### Lifetime can be omitted / elided in most cases

TODO explain WTF elision means - to omit. Essentially if the compiler can work out the lifetimes of structs for itself, then your does not need to say what those lifetimes are.

TODO example of elided lifetime versus specific

When you do not have to specifically say anything about the lifetime because the compiler figures it out, it is said to be elided. Why it's called elision when omit would be a more commonly understood word is anyone's guess. Elide = omit, remember that.

https://ericlippert.com/2013/01/24/five-dollar-words-for-programmers-elision/

## Directives / Attributes

C++ has various ways to give directives during compilation:

* Compile flags that control numerous behavious
* #pragma statements in code
* #define with ubquitous #ifdef / #else / #endif blocks
* Keywords inline, const, volatile etc.. These hint the code and allow the compiler to make decisions that might change its output or optimization.

Rust uses a notation called attributes. Attributes are enclosed in a #[ ] block and provide compiler directives that allow:

* Mark unit tests
* Conditional compilation for target OS
* Enable / disable lint rules
* Enable / disable compiler features
* Change the entry point function from main to something else
* Conditional compilation for target architecture, OS, family, endianess, pointer width,
* Inline hinting
* Derivation of certain traits
* Enabling compiler features

An attribute applies to the next item it is declared before:

```rust
#[test]
fn this_is_a_test() {
  //...
}
```

It can also be expressed as #![foo] which applies to the thing its contained by. Attributes can also have name=value pairs as part of the directive.

### Linking to native libraries

TODO

C++ | Rust
--- | ----
\#pragma (comment, "somelib") | #[link(name = "somelib")]
- | #[link(name = "somelib", kind = "static")]

Default "kind" is "dynamic" library but "static" can be specified.

### Inlining code

Inlining happens where your function logic is inserted in-place to the code that invokes it. It tends to happen when the function does something trivial such as return a value or execute a simple conditional. The overhead of duplicating the code is outweighed by the performance benefit.

Inlining is achieved in C++ by declaring and implementing a function, class method or template method in a header or marking it with the inline keyword.

In Rust, inlining is only a hint. Rust recommends not forcing inlning, rather leaving it as a hint for the LLVM compiler to do with as it sees fit.

C++ | Rust
--- | ----
Explicitly with "inline" or implicitly through methods implemented in class / struct | #[inline], #[inline(always)], #[inline(never)]

Note that it may be better to leave inlining alone and trust the link-time optimisation in LLVM.

TODO inline example

## Variable bindings, ownership and moving

When you assign an object to a variable in Rust, you are said to be binding it. i.e your variable "owns" the object for as long as it is in scope and when it goes out of scope it is destroyed.

```rust
{
  let v1 = vec![1, 2, 3, 4]; // Vec is created
  ...
} // v1 goes out of scope, Vec is dropped
```

So variables are scoped and the scope is the constraint that affects their lifetime. Outside of the scope, the variable is invalid.

In this example, it is important to remember the Vec is on the stack but the pointer it allocates to hold its elements is on the heap. The heap space will also be recovered when the Vec is dropped.

If we assign v1 to another variable, then all the object ownership is moved to that other variable:

```rust
{
  let v1 = vec![1, 2, 3, 4];
  let v2 = v1;
  ...
  println!("v1 = {:?}", v1); // Error!
}
```

This may seem weird but it's worth remembering a serious problem we saw in C++, that of Copy constructor errors. It is too easy to duplicate a class and inadvertantly share private date or state across multiple instances.

We don't want to objects v1 and v2 to shared internal state and in Rust they don't. So Rust copies the data from v1 to v2 and marks v1 as invalid. If you attempt to reference v1 any more in your code, it will generate a compile error. This compile error will indicates that ownership was moved to v2.

Likewise, if we pass the value to a function then that also moves ownership:

```rust
{
  let v1 = vec![1, 2, 3, 4];
  we_own_it(v1);
  println!("v = {:?}", v1);
}

fn we_own_it(v: Vec<i32>) {
  // ...
}
```

When we call we_own_it() we moved ownership of the object to the function and it never came back.
Therefore the following call using v1 is invalid. We could call a variation of the function called  we_own_and_return_it() which does return ownership:

```rust
v1 = we_own_and_return_it(v1)
...
fn we_own_and_return_it(v: Vec<i32>) -> Vec<i32> {
  // ...
  v1
}
```

But that's pretty messy and there is a better way described in the next section called borrowing.

These move assignments look weird but it is Rust protecting you from the kinds of copy constructor error that is all too common in C++. If you assign a non-Copyable object from one variable to another you move ownership and the old variable is invalid.

If you truly want to copy the object from one variable to another so that both hold independent objects you must make your object implement the Copy trait.  Normally it's better to implement the Clone trait which works in a similar way but through an explicit clone() operation.

### Variables must be bound to something
Another point. Variables must be bound to something. You cannot use a variable if it hasn't been initialized with a value of some kind:

```rust
let x: i32;
println!("The value of x is {}", x);
```
A C++ compiler might issue a warning or catch the error with strict flags, but by default it doesn't care.

Rust will also warn you if you declare a variable and end up not using it.

## Comments

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

## References and borrowing

We've seen that ownership of an object is tracked by the compiler. If you assign one variable to another, ownership of the object is said to have moved to the assignee. The original variable is invalid and the compiler will generate errors if it is used.

Unfortunately this extends to passing values into functions and this is a nuisance.
But variable bindings can be borrowed. If you wish to loan a variable to a function for its duration, you can pass a reference to the object:

```rust
{
  let mut v = Vec::new(); // empty vector
  fill_vector(&mut v);
  // ...
  println!("Vector contains {:?}", v);
}
//...
fn fill_vector(v: &mut Vec<i32>) {
  v.push(1);
  v.push(2);
  v.push(3);
}
```

Here we create an empty vector and pass a mutable reference to it to a function called fill_vector(). The compiler knows that the function is borrowing v and then ownership is returned to v after the function returns.

## Expressions

An expression is something that evaluates to something. Just like C++ more or less...

```rust
let x = 5 + 5; // expression evaluates to 10
```

Where it gets more interesting is that a block is an expression too and you can return a value from it.

```rust
let x = {
   let pi = 3.141592735;
   let r = 5;
   2 * pi * r
};
```

Note how the last line inside the block is not terminated with a semicolon. So the result of the block expression is 2 * pi * r which is assigned to x. If we’d put a semicolon on the end of that line, the expression would evaluate to nothing.

You could even do complex matching in your block and conditionally assign the output:

```rust
let result = {
  match server_state {
    ServerState::waiting => { "Waiting" }
    ServerState::running => { "Running" }
    ServerState::stopped => { "Stopped" }
  }
}.to_string();
println!("The server state is {}", result);
```

In this instance, the match returns a &str from each match and we then call to_string() to turn it to a String before binding it to the variable result.
More normally you will see this in function blocks or closures. A trivial function can just omit the return statement:

```rust
pub fn add_values(x: i32, y: i32) -> i32 {
  x + y
}
```

Another case you might see is by using Rust's equivalent to a C++ ternary operator:

```rust
let x = if y / 2 == 4 { true } else { false };
```

## Conditions

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

### Ternary operator

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

### Conditional "if let"

One unusual feature is the "if let" pattern. This combines a test to see if something matches a pattern and if it does, to automatically assign the result to the tuple. It would be most commonly see in code that returns an enum such as a Result or

```rust
if let Some(person) = search("fred") {
  println!("You fould a person {}", person);
}
else {
  println!("Could not find person");
}
```

### The try!() macro
At first glance this unfortunately named macro might seem to have something to do with a try-catch block. But don't be fooled. It's really a convenience to cut out some lines of code when processing calls to functions that return a Result.

Sometimes you have code of this pattern:

```rust
fn my_code() -> Result<int, String> {
  let result = my_other_code();
  if let Err(err) = result {
    return Err(err);
  }
  Ok(result.unwrap());
}
```

The try!() macro simplifies the code by testing if the call to my_other_code() was is_ok or is_err. If it was an error it returns Err() for you and if it wasn't it returns the unwrapped ok value.
So the above code reduces to this:

```rust
fn my_code() -> Result<int, String> {
  let result = try!(my_other_code());
  Ok(result);
}
```

## Enumerations
In C++ an enum is a bunch of labels assigned an integer value.

```c++
enum HttpResponse {
  okay = 200,
  not_found = 404,
  internal_error = 500,
};
```

C++11 extends this concept a little, allowing you to declare an enum that uses another integer type, e.g. a char to hold the values.
In Rust enums can takevalues so you can convey far more information than a static value could by itself.

```rust
enum HttpResponse {
  Ok,
  NotFound(String),
  InternalError(String, String, Vec<u8>)
}
```

So we might have a function that makes an http request and returns a response:

```rust
fn do_request(url: &str) -> HttpResponse {
  if url == "/invalid" {
    return HttpResponse::NotFound(url.to_string());
  }
   HttpResponse::Ok
}
let result = do_request("/invalid");
if let HttpResponse::NotFound(url) = result {
  println!("The url {} could not be found", url);
}
```

Now our code is able to return a more meaningful response in an enum and the code is able to extract that response to print out useful information.

## Switch / Match

TODO

Match is like a switch statement on steroids.

In C++ a switch is a straight comparison of an integer value of some kind (including chars and enums), against a list of values. If the comparison matches, the code next to it executes until the bottom of the switch statement or a break is reached.

## Casting
Casting is the act of coercing one type to be another, or dynamically producing the equivalent value in the other type.

TODO

For example, converting a double to int or vice versa involves changing the

C++ has a range of cast operators that turn a pointer or type of one kind into a pointer of type of another kind. It also has const_cast<> which allows code to violate const enforcement even in the times it might be enforced.

The equivalent of casting in Rust is the "as" command. You may cast primitives from kind to another, and you can cast

## Loops

The three segments of the for statement allow:
* Zero or more variables to be initialized (yes it can be empty)
* Zero or more conditions to be true for the loop to continue
* Zero or more actions to perform on each iteration.

### Infinite

An infinite loop is one that never ends. The typical way to do this in C++ is to test against an expression that always evaluates to true:

```c++
while (true) {
  poll();
  do_work();
}
```

Rust has an explicit infinite loop that runs indefinitely:

```rust
loop {
  poll();
  do_work();
}
```

Rust recommends using this form when an infinite loop is required. Note that an infinite loop can still be broken out of using a break statement.

## While loop

C++ has conditional while() {} and do { } while() forms.

```c++
while (!end) {
  std::string next = getLine();
  end = next == "END";
};
```

A while loop in Rust looks pretty similar

```rust
while request_count < 1024 {
  process_request();
  request_count = request_count + 1;
}
```

The do-while form in C++ will execute the loop body at least once because the condition is only tested after each iteration.

```c++
int i = 0;
do {
  i = rand();
} while (i < 20);
```

Rust has no equivalent to the do-while loop form. It can be simulated but it looks a bit inelegant:

```rust
let mut i = 0;
loop {
  i = i + 1;
  if i >= 20 { break; }
}
```

### While let loop

Just as there is an "if let" which tests and assigns a value that matches a pattern, there is also a "while let" equivalent:

```rust
let mut iterator = vec.into_iter();
while let Some(value) = iterator.next() {
  process(value);
}
```

This loop will break when the iterator returns None.

### For loop - Iterating a range

A C++ loop consists of an initialising expression, a condition expression and a a loop expression separated by semicolons. So a loop that iterates from 0 to 100 looks like this:

```c++
for (int i = 0; i < 100; i++ ) {
  cout << "Number " << i << endl;
}
```

Rust for loops are quite different from C++ because Rust allows software to iterate over a defined range:

```rust
for i in 0..100 {
  println!("Number {}", i);
}
```

Every iterable item also implements an enumerate() function that returns a tuple. The first item is the zero based index of the item in the range and the second is the value.

So for example:

```rust
for (i, x) in (30..50).enumerate() {
   println!("Iteration {} is value {}", i, x);
}
```

### For loop - Iterating collections

C++ introduces the concept of iterators to its collection classes. An iterator is something that can increment or decrement to traverse a collection.

So to iterate a collection from one end to the other, an iterator is assigned with the collection's begin() iterator and incremented until it matches the end() iterator.

```c++
for (std::vector<string>::const_iterator i = my_list.begin(); i != my_list.end(); ++i ) {
  cout << "Value = " << *i << end;
}
```

C++11 introduces a range based loop which simplifies the syntax when iterating over arrays and collections:

```c++
std::vector values;
...
for (const auto & v: values) {
  ...
}

int x[5] = { 1, 2, 3, 4, 5 };
for (int y : x) {
  ...
}
```

### Break and Continue

If you need to exit a loop or start the next iteration early then you use the break and continue keywords.

TODO C++ example

These keywords work the same in C++ and Rust - they operate on the the innermost loop. A continue will start on the next iteration while a break will terminate the loop.

TODO Rust example

### Labels
As break and continue only work on the inner most loop there will be occasions where they do not work as desired, e. If you need to break or continue an outer loop, you can label each loop and refer to the label in the break / continue to indicate what you're breaking.

TODO example

## Generics
Templates in C++ are class or functions with parameterized values that are expanded by the compiler and compiled. One of the dangers of this approach is the user only sees errors in the expanded code which more often than not is meaningless noise.

Rust takes an approach somewhat closer to generics in languages such as Java. Code may define traits or functions that take parameters but the errors that you see are in against the generic instead of an expanded and substituted version.

TOD O.. examples

In addition, a generic can place constraints on the parameters that say what kind of thing is an acceptable value to the generic. For example a generic function may only apply to types that implement the Display trait so we can enforce that:
TODO... Display trait generic

## Functions

In C++ the standard form of a function is this:

```c++
int do_something(bool parameter1, const std::string &parameter2);
```

A function is either void (returns nothing), or it returns something. The parameters to a C++ function and the return type can be the standard mix of values, pointers, references.  A function can be declared and implemented in one block, however if it is called from an external file, or before its implementation then it must be forward declared in a header.

In Rust a function is like this:

```rust
fn do_something(parameter1: bool, parameter2: &str) -> i32 {
  // implementation
}
```

This Rust function is equivalent to the one we wrote in C++. The parameter types and the return type must be specified.

The declaration and the implementation are the same thing. There is no need for forward declaration since Rust only resolves function calls after the file has been completely parsed.

This function adds two values together and returns them:

```rust
fn add(x: i32, y: i32) -> i32 {
  x + y
}
```

Why is there no return call? As we saw in the section on Expressions, a block can have a return value if we omit the semi-colon from the end so x + y is the result of evaluating the function block and becomes what we return.

There are occasions were you explicitly need the return keyword. Typically you do that if you want to exit the function before you get to the end

```rust
fn process_data(number_of_times: ui32) -> ui32 {
  if number_of_times == 0 {
    return 0;
  }
  let mut result : ui32 = 0;
  for i in number_of_times {
    result += i;
  }
  result
}
```

### Variable arguments

C++ functions can take a variable number of arguments with the ... ellipsis pattern. This is used in functions such as print, scanf etc.

```c++
void printf_like(const char *pattern, ...);
```

Rust does not support variadic functions. However you could pass additional arguments in an array slice if the values are the same, or as a dictionary or a number of other ways.  

TODO Rust example of array slice

Another option is to write your code as a macro. Macros can take any number of expressions so you are able to write code that takes variable arguments. This is how macros such println!, format! and vec! work.

### Default arguments

C++ arguments can have default values.

```c++
std::vector<Record> fetch_database_records(int number_to_fetch = 100);
```

A function defines what its name is, what types it takes and what value (if any) it returns.

### Function overloading

C++ functions can be overloaded, e.g.

```c++
std::string to_string(int x);
std::string to_string(float x);
std::string to_string(bool x);
```

Rust does not support overloading. As an alternative, each variation of the function would have to be named uniquely.

### C++11 alternative syntax

C++11 introduces a new syntax which is slightly closer to Rust's in style.

```c++
auto function_name(type parameter1, type parameter2, ...) -> return-type;
```

This form was created to allow C++ function declarations to more closely to resemble lambda functions in some scenarios and to help with decltype return values.

## Error Handling

C++ allows code to throw and catch exceptions.

```c++
void do_something() {
  if (!read_file()) {
    throw std::runtime_error("read_file didn't work!");
  }
}
...
try {
  do_something();
}
catch (std::exception e) {
   std::cout << "Caught exception -- " << e.what() << std::endl;
}
```

Exceptions have been a mixed blessing for C++. On the one hand they provide a mechanism for propagating errors in some way. On the other they break the flow of the program and make it jump around in ways that not simple to follow.

They also really don't cross library boundaries, and it is easy for code to use them excessively or not at all.

Most coding guidelines would say to use exceptions sparingly for truly exceptional situations, and use return codes and other forms of error propagation for ordinary failures. The problem is that C++ has no simple way to confer error information.

Rust does not support exceptions. Rust programs are expected to use a type such as Option or Result to propagate errors to their caller. In other words, the code is expected to anticipate errors and have code to deal with them.

The Option enum either returns something or none. It's a generic enum that specifies the type of what it may contain:

TODO Option examples

A Result either returns something or an error. It's a generic enum that specifies the success and error types.

TODO Result

And of course if neither of this are suitable, then code can always create its own enum type that returns what they want. Since enums can contain data, they serve the case of functions that can return different stuff.

### Nuclear option - panic!()

If code really wants to do something equivalent to a throw / catch in C++ it may call panic!().

This is NOT recommended for dealing with regular errors, only irregular ones that the code has no way of dealing with.

This macro will cause the thread to abort and if the thread is the main programme thread, the entire process will exit.

A panic!() can be caught and should be if Rust is being invoked from another language. The way to catch an unwinding panic is a closure at the topmost point in the code where it can be handled.

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    panic!("Bad things");
});
```

## Lambda expressions / Closures

### Lambdas in C++

A lambda expression, or lambda is a C++11 feature for creating an anonymous function that can be declared and passed to a function from within the scope of the call itself. This can be particularly useful when you want to sort, filter, search or otherwise do some trivial small action without the bother of declaring a function and making it work.

In C++ a lambda looks like this:

```c++
float values[10] = { 9, 3, 2.1, 3, 4, -10, 2, 4, 6, 7 };
std::sort(values, values + 10, [](float a, float b) {
  return a < b;
});
```

Here we sort an array of values using a lambda to do the comparison.

A C++ lambda can (but doesn't have to) capture variables from the enclosing scope if it wishes and it can specify capture clauses in the [ ] section that define how capture is made. A lambda that captures variables effectively becomes a closure.

TODO capture clause

Prior to C++11 there was no lambda support however Boost provided a poor-man's version lambda function called a binding - basically a call to a function preloaded with arguments so it could be copied around and invoked.

TODO boost::bind

### Closures in Rust

Rust doesn't implement lambdas, it implements closures. What's the difference?

A lambda is an anonymous function and a closure is an anonymous function with access to its enclosing environment. So a closure is a form by which a lambda may be implemented.
Rust's lambdas have access to their enclosing environment so they are closures. When the closure is called it borrows the binding for any variable it accesses in that scope.

TODO closure example

If need be, ownership of variables can be moved to the closure. This may be necessary if the closure lives longer than the code around it does.
TODO move semantics

## Generics / Templates

C++ offers templates as a way to write generic code using an abstract type and then specialize it by substituting one or more types into a concrete class.

This is a very powerful mechanism and the C++ library makes extensive use of it. Where templates can become a bit of a mess is that the compiler expands all of the code before compiling it. An innocuous error in a type such as the absence of a copy constructor can cause the compiler to generate a wall of indeciperahble errors from the heart of the expanded / substituted template where a problem was encountered.

### Generic Functions

TODO generic function

### Trait bounds

TODO generic trait

### Where clause

Rust has a more expressive way of defining the constraints on a generic via the where clause

TODO where clause

## Lint

The Rust compiler contains a lint check that extends beyond the syntactic correctness of your code and looks for potential errors that you may not notice.

In particular it looks for:

* Dead / unused code
* Unreachable code
* Deprecated methods
* Undocumented functions
* Camel case / snake case violations
* Unbounded recursion code (i.e. no conditionals)
* Unused extern crates, imports, variables, attributes, mut, parentheses
* Using while true{} instead of loop
* Lint rules can be enforced more strictly or ignored by using attributes:
* TODO

```rust
#[allow(rule)]
#[warn(rule)]
#[deny(rule)]
#[forbid(rule)]
```

Where rule is an underscored version of the rules supported by the compiler. A full list of lint rules can be found by typing "rustc -W help".

There are a lot more than are listed here.

## Macros

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

### Rust macros

Macros in Rust are pretty complex. Depending on what your opinion is of macros, this is either a good or a bad thing. If you think macros are to be discouraged then the more complex the better.

Firstly lets point out some differences in Rust macros compared to those in C or C++:

* Rust macros are hygenic. That is to say if macro contains variables, their names do not conflict with, hide, otherwise interfere with named variables from the scope they're used from.
* The pattern supplied in between the brackets of the macro are tokenized and designated as parts of the Rust language. identifiers, expressions etc. In C / C++ you can #define a macro to be anything you like whether it is garbage or syntactically correct. Furthermore you can call it from anywhere you like because it is preprocessed even before the compiler sees the macro.
* Rust macros are rule based with each rule having a left hand side pattern "matcher" and a right hand side "substitution"
* Rust macros must produce syntactically correct code.
* Rust macros can be exported by crates and used in other code providing the other code elects to enable macro support from the crate. This is a little messy since it must be signalled with a #[macro_export] directive.

Here is a simple macro demonstrating repetition called hello_x!(). It will take a comma separated list of expressions and say hello to each one of them.

```rust
macro_rules! hello_x {
  ($($name:expr),*) => (
    $(println!("Hello {}", $name);)*
  )
}
...
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

What about some illegal

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

### Real world example - vec!()
The vec! macro is a real world macro which allows us to declare a Vec and prefill it in a simple declarative way.

Here is the actual vec! macro source code:

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

It looks complex but we will break it down to see what it does. Firstly it has a match-like syntax with three conditions.

#### First branch

The first matcher allows us to create an array of elements all with the same value.

We can see the matcher looks for a pattern consisting of an expression, a semi-colon and another expression like our call.

```rust
($elem:expr; $n:expr) =>  (
        $crate::vec::from_elem($elem, $n)
    );
```

This matches to something like this:

```rust
let v = vec!(1; 100);
```

The first expression goes into a value $elem, the second expression goes into $n.

The substitution block looks like this:

```rust
(
        $crate::vec::from_elem($elem, $n)
);
```

So substituting the values we supply and substituting $crate for std this becomes the following in the code:

```rust
let v = std::vec::from_elem(1, 100);
```

#### Second branch

The second matcher contains a glob expression - zero or more expressions separated by comma (the last comma is optional)

```rust
($($x:expr),*) => (
        <[_]>::into_vec(box [$($x),*])
    );
```

So we can write:

```rust
let v = vec!(1, 2, 3, 4, 5);
```

When the matcher runs it evalutes the values and produces this code:

```rust
<[_]>::into_vec(box [1, 2, 3, 4, 5]);
```
The box keyword tells Rust to allocate the supplied array on the heap and moves the ownership by calling a helper function called into_vec() that wraps the memory array with a Vec instance. The <[\_]>:: at the front is a turbo-fish notation to make the into_vec() generic function happy.

#### Third branch
The third branch is a little odd and almost looks the same as the second branch. But take at look the comma. In the last branch it was next to the asterisk, this time it is inside the inner $().

```rust
($($x:expr,)*) => (vec![$($x),*])
```

The matcher matches when the the comma is there and if so recursively calls vec!() again to resolve to the second branch matcher:

Basically it is there so that there can be a trailing comma in our declaration and it will still generate the same code.

```rust
let v = vec!(1, 2, 3, 4, 5,);
```

## Foreign Function Interface

Rust recognizes that you might want to call a system function or use an external library written in C.

It supports the concept of a foreign function interface which is a definition of an external function or type that is resolved at link time.

```rust
#[link(name = "foo")]
extern {
  fn foo_command(command: *mut u8)
}
```

If you call this function you have to wrap it in an unsafe block to disable the safety checks:

```rust
pub fn run_command(command: &[u8]) {
  unsafe {
    foo_command(command.as_ptr());
  }
}
```

It is even possible to expose a Rust function from a lib as a C-callable API:

```rust
#[no_mangle]
pub extern fn hello_world() {
  // Your code here
}
```

The FFI specification goes into a lot more detail than this and explains concepts such as callbacks, structure packing, stdcall, linking and other issues that allow full interoperability.
There are also crates that have the definitions of structures, types and functions for standard C, Win32, OpenGL etc.
TODO a stdc examples
TODO a Win32 example
