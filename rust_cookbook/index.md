# Rust Cookbook

## Numbers

### Convert a number to a string

Let's say you have an integer you want to turn into a string.

In C++ you might do one of the following:

```c++
const int value = 17;
std::string value_as_string;

// Nonstandard C itoa() (also not thread safe)
value_as_string = itoa(value);

// OR _itoa()
char buffer[16];
_itoa(value, buffer, 10);

// OR
sprintf(buffer, "%d", value);

// OR
stringstream ss;
ss << value;
value_as_string = ss.str();

// OR (boost)
value_as_string = boost::lexical_cast<std::string>(ivalue);
```

All of these have issues be it standards compliance, thread safety, sensitivity to changes to the type of "value", clumsy code, or dependency on third party libs.

Rust makes it far easier because numeric primitives implement a trait called ToString. The ToString trait has a to_string() function. So to convert the number to string is as simple as this:

```rust
let value = 17u32;
let value_as_string = value.to_string();
```

The same is true for a floating point number:

```rust
let value = 100.00345f32;
let value_as_string = value.to_string();
```

### Convert a number to a string with precision / padding

In C you would add precision of padding using printf operations:

```c++
double value = 1234.66667;
char result[32];
sprintf(result, "%08.2d", value);
```

In C++ you could use the C way (and to be honest it's easier than what is written below), or you can set padding and precision through an ostream:

```c++
// TODO validate
double value = 1234.66667;
ostringstream ss;
ss << setfill('0') << setw(8) << setprecision(2) << value;
```

In Rust you can use format!() [https://doc.rust-lang.org/std/fmt/] for this purpose and it is similar to printf / sprintf:

```rust
let value = 1234.66667;
let value_as_string = format!("{:08.2}", value);
println!("value = {}", value_as_string);
```

Output

```
value = 01234.67
```

### Convert a number to a localized string

Some locales will use dots or commas for separators. Some languages will use dots or commas for the decimal place. In order to format these strings we need to make use of the locale.

TODO

### Convert a string to a number

In C / C++ a number might be converted from a string to a number in a number of ways

```c++

int value = atoi(value_as_str);

```

TODO

In Rust we have a &str containing a number:

```rust
let value_as_str = "12345";
```

Any type that implements a trait called FromStr can take its type from a string. All the standard primitive types implement FromStr so we can simply say this:

```rust
let value_as_str = "12345";
let value = i32::from_str(value_as_str).unwrap();
```

Note the unwrap() at the end - the FromStr::from_str() returns the value inside a Result<value, error>, to allow for the possibility that the string cannot be parsed. Production code should test for errors before calling unwrap() or it will panic.

Another way to get the string is to call parse() on the &str or String itself. In this case, you use a slightly odd looking syntax nicknamed 'turbofish' which looks like this:

```rust
let value_as_str = "12345";
let value = value_as_str.parse::<i32>().unwrap();
```

The string's implementation of parse() is a generic that works with any type implementing FromStr. So calling parse::<i32> is equivalent to calling i32::from_str().

Note one immediate advantage of Rust is it uses string slices. That means you could have a long, comma separated string and use slices to parse numbers straight out of the middle of it without constructing intermediate copies.


### Converting between numeric types

Converting between numeric types is as easy as using the "as" keyword.

```rust
let f = 1234.42f32;
let i = f as i32;
println!("Value = {}", i);
```

The result in i is the integer part of f.

```
Value = 1234
```

## Strings

Rust comes with some very powerful functions that are attached to every &str and String type. These mostly correspond to what you may be used to on the std::string class and in boost string algorithms.

Most find / match / trim / split string operations in Rust are efficient because they neither modify the existing string, nor return a duplicate to you. Instead they return slices, i.e. a pointer and a length into your existing string to denote the range that is the result.

It is only operations that modify the string contents themselves such as creating upper or lowercase versions that will return a new copy of a string.

### Trimming a string

Spaces, tabs and other Unicode characters defined as whitespace can be trimmed from a string.

All strings have access to the following functions

```rust
fn trim(&self) -> &str
fn trim_left(&self) -> &str
fn trim_right(&self) -> &str
```

Note the signatures of these functions - they are not mutable. The functions return a slice of the string that excludes the leading and / or trailing whitespace removed. In other words it is not duplicating the string, nor is it modifying the existing string. Instead it is just telling you what the trimmed range is within the &str you're already looking at.

So

```rust
let untrimmed_str = " this is test with whitespace    \t";
let trimmed_str = untrimmed_str.trim();
println!("Trimmed str = \"{}\"", trimmed_str);
```

Yields:

```
Trimmed str = "this is test with whitespace"
```

Also be aware that trim_left() and and trim_right() above are affected by the directionality of the string.

Most strings read from left-to-right, but strings in Arabic or Hebrew are read right-to-left and will start with a control character that sets their base direction right-to-left. If that character is present, trim_left() actually trims from the right and trim_right() trims from the left.

### Get the length of a string

Every &str and String has a len() function.

```rust
let message = "All good things come to those who wait";
println!("Length = {}", message.len());
```

Note that len() is the length in bytes. If you want the number of characters you need to call message.chars().count(), e.g.

```rust
let message = "文字列の長さ";
assert_eq!(message.chars().count(), 6);
```

### Splitting a string
TODO

### Tokenizing a string

TODO

### Joining strings together

TODO

### Getting a substring

TODO

### Converting a string between upper and lower case

Strings have these functions for converting between upper and lower case:

```rust
fn to_lowercase(&self) -> String
fn to_uppercase(&self) -> String
```

These functions will return a new String that contains the upper or lower case version of the input. Upper and lower case are defined by Unicode rules. Languages that have no upper or lowercase strings may return the same characters.

### Doing a case insensitive compare

TODO

### Using regular expression matches

TODO

## Date and Time

### Get the current date and time

TODO time_rs

### UTC

TODO explain what UTC is and why maintaining time in UTC is vital Epochs etc.
TODO preamble about what an epoch is, the Unix epoch and other epochs

### Setting a timer

TODO setting a timer

### System time vs UTC

TODO the reason timers might be set in system uptime vs timers being set in UTC. Answer because users and NTP can change the UTC time wherease system time is relative to bootup. So setting a timer to run 10s from now will always work against system time where setting a timer to run 10s from now in UTC could fail if the OS sets time back by an hour.

### Formatting a date as a string

TODO standard date formatting UTC
TODO example

### Parsing a date from a string

TODO parsing a date from a string's
TODO example

### Performing date / time arithmetic

## Collections

### Creating a static array

An array primitive consists of a type and a length. e.g. a 16 kilobyte array of bytes can be created and zeroed like this:

```rust
let values: [u8; 16384] = [0; 16384];
```

The variable specifies the type and length and the assignment operator assigns 0 to every element.

The type, length and values can be initialized implicitly in-place like this:

```rust
let my_array = [ "Cat", "Dog", "Fish", "Donkey", "Albatross" ];
println!("{:?}", my_array);
```

This is an array of 5 &str values. The compiler will complain if we try to mix types in the array.
We could also declare the array and manipulate it:

```rust
let mut my_array: [&'static str; 5] = [""; 5];
// Set some values
my_array[0] = "Cat";
my_array[1] = "Dog";
my_array[2] = "Fish";
my_array[3] = "Donkey";
my_array[4] = "Albatross";
println!("{:?}", my_array);
```

Note in this case we declared the array, each element received an empty value. Then our code programmatically set the new element value.
The latter form would obviously be useful for arrays that change. The latter would be useful for arrays which do not.

### Creating a dynamic vector

A vector is a linear array of values. Unlike an array which has a fixed length, a vector can grow or shrink over time.

A vector can be created using the vec! macro like this:

```rust
let mut my_vector = vec![1984, 1985, 1988, 1995, 2001];
```

This creates a mutable Vec and prepopulates it with 5 values. Note how the vec! macro can use square brackets for its arguments. We could have used round brackets and it would have meant the same.

A new Vec can also be made using Vec::new() or Vec::with_capacity(size)

```rust
let mut my_array = Vec::new();
my_array.push("Hello");
let my_presized_array = Vec::with_capacity(100);
```

It is strongly recommended you use Vec::with_capacity() to create a vector with enough capacity for maximum number of elements you expect the vector to contain. It prevents the runtime from having to reallocate and copy data if you keep exceeding the existing capacity. It also significantly reduces heap fragmentation.

### Removing values from a vector

Sometimes you want to strip out values from a list which match some predicate. In which case there is a handy function for that purpose.
TODO `.retain`

### Sorting a vector

A vector can be sorted by the natural sort order of the elements it contains:

```rust
let mut values = vec![ 99, -1, 3, 555, 76];
values.sort();
println!("Values = {:?}", values);
```

Sorting is done using the Ord trait and calling Ord::cmp() on the elements to compare them to each other.

Comparison can also be done through a closure and Vec::sort_by()

TODO `.sort_by`
TODO `.sort_by_key`

### Stripping out duplicates from a vector

Assuming your vec is sorted, you can strip out consecutive duplicate entries using dedup().
This function won't work and the result will be undefined if your vector is not sorted.
TODO .dedup

### Creating a linked list

A linked list is more suitable than a vector when items are likely to be inserted or removed from either end or from points within the list.

`std::collections::LinkedList`

### Creating a hash set

A hash set is a unique collection of objects. It is particularly useful for removing duplicates that might occur in the input.
`std::collections::HashSet`

### Creating a hash map

A hash map consists of a key and a value. It is used for look up operations
`std::collections::HashMap`

### Iterating collections

TODO

### Iterator adaptors

TODO

An adaptor turns the iterator into a new value

`.enum`
`.map(X)`
`.take(N)`
`.filter(X)`

### Consuming iterators

A consumer is a convenience way of iterating a collection and producing a value or a set of values from the result.

`.collect()`

`.find()` will return the first matching element that matches the closure predicate. TODO

`.fold()` is a way of doing calculations on the collection. It takes a base value, and then calls a closure to accumulate the value upon the result of the last value. TODO
Processing collections

## Localization

### Unicode considerations

TODO

### Externalizing strings

TODO

### Building strings from parameters

TODO

### Creating a localization file

TODO

## Logging

## Files and streams

Rust comes with two standard modules:

* std::io contains various stream related traits and other functionality.
* std::fs contains filesystem related functionality including the implementation of IO traits to work with files.

### Creating a directory

A directory can be created with `std::fs::DirBuilder`, e.g.

```
let result = DirBuilder::new().recursive(true).create("/tmp/work_dir");
```

### File paths

Windows and Unix systems have different notation for path separators and a number of other differences. e.g. Windows has drive letters, long paths, and network paths called UNCs.

Rust provides a PathBuf struct for manipulating paths and a Path which acts like a slice and can be the full path or just a portion of one.

TODO simple example of a path being created

TODO simple example of a Path slice in actively

TODO simple example of relative path made absolute

Windows has a bunch of path prefixes so std::path::Prefix provides a way to accessing those.

TODO example of a path being made from a drive letter and path

### Opening a file

A `File` is a reference to an open file on the filesystem. When the struct goes out of scope the file is closed. There are static functions for creating or opening a file:

```rust
use std::io::prelude::*;
use std::fs::File;

let mut f = try!(File::open("myfile.txt"));
TODO
```

Note that File::open() opens a file read-only by default. To open a file read-write, there is an OpenOptions struct that has methods to set the behaviour of the open file - read, write, create, append and truncate.

e.g. to open a file with read/write access, creating it if it does not already exist.

```rust
use std::fs::OpenOptions;

let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("myfile.txt");
```

### Writing to a file

TODO simple example of opening file to write

### Reading lines from a file

TODO simple example of opening file text mode, printing contents

## Threading

Rust actively enforces thread safety in your code. If you attempt to pass around data which is not marked thread safe (i.e. implements the Sync trait), you will get a compile error. If you use code which is implicitly not thread safe such as Rc<> you will get a compile error.

This enforcement means that Rust protects against data race conditions, however be aware it cannot protect against other forms of race conditions or deadlocks, e.g. thread 1 waits for resource B (held by thread 2) while thread 2 waits for resource A (held by thread 1).

### Creating a thread

Creating a thread is simple with a closure.

TODO

### Waiting for a thread to complete

TODO

### Using atomic reference counting

Rust provides two reference counting types. Type Rc<> is for code residing on the same thread and so the reference counting is not atomic. Type Arc<> is for code that runs on different threads and the reference counting is atomic.

An Arc<> can only hold a Sync derived object. Whenever you clone an Arc<> or its lifetime ends, the counter is atomically incremented or decremented. The last decrement to zero causes the object to be deleted.

TODO example

### Locking a shared resource

Message passing is a preferable way to prevent threads from sharing state but its not always possible.

Therefore Rust allows you to create a mutex and lock access to shared data. The guard that locks / unlocks the mutex protects the data and when the guard goes out of scope, the data is returned.

This style of guard is called TODO

### Data race protection

Rust can guarantee that protection from data races, i.e. more than one thread accessing / writing to the same data at the same time.

However even Rust cannot protect against the more general problem of race conditions. e.g. if two threads lock each other's data, then the code will deadlock. This is a problem that no language can solve.

### Waiting for multiple threads to finish

TODO

### Sending data to a thread

Any struct that implements the Send trait is treated safe to send to another thread. Of course that applies to

### Receiving data from a thread

A thread can receive messages and block until it receives one. Thus it is easy to create a worker thread of some kind.

TODO

## Networking

### Connecting to a server

TODO

###Listening to a socket

TODO

## Interacting with C

### Using libc functions and types

### Calling a C library

### Generating a dynamic library

### Calling Win32 functions

## Common design patterns

### Singleton

A singleton has one instance ever in your application.
TODO

### Factory

TODO

### Observer

TODO

### Facade

TODO

### Flyweight

TODO

### Adapter

An adapter is where we present a different interface to a client calling the adapter than the interface the code is implemented in. This might be done to make some legacy code conform to a new interface, or to manage / hide complexity which might leak out into the client.

As Rust is a relatively new language you are most likely to use an adapter pattern to wrap some existing code in C. A common use for the adapter in C++ is to wrap up a C library in RAII classes or similar.

TODO
