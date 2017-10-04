# Loops

## C++

### For loops

A `for` loop in C/C++ consists of 3 expression sections housed in the `for()` section and a block of code to execute:

The three segments of a for statement allow:

* Zero or more variables to be initialized \(can be empty\)
* Zero or more conditions to be true for the loop to continue \(can be empty\)
* Zero or more actions to perform on each iteration \(can be empty\).

So this is a valid for loop:

```cpp
// Infinite
for (;;) {
}
```

So is this:

```cpp
for (int i = 10, j = 0; (j = i * i) <= 100; i--) {
  //...
}
```

This is clearly a convoluted and somewhat confusing loop because it mixes assignment and conditional tests into the terminating text, but it is one which is entirely legal.

#### Iterating a range

A C++ loop consists of an initialising expression, a condition expression and a a loop expression separated by semicolons. So a loop that iterates from 0 to 100 looks like this:

```cpp
for (int i = 0; i < 100; i++ ) {
  cout << "Number " << i << endl;
}
```

#### Iterating collections

C++ introduces the concept of iterators to its collection classes. An `iterator` is something that can increment or decrement to traverse a collection.

So to iterate a collection from one end to the other, an iterator is assigned with the collection's `begin()` iterator and incremented until it matches the `end()` iterator.

```cpp
for (std::vector<string>::const_iterator i = my_list.begin(); i != my_list.end(); ++i ) {
  cout << "Value = " << *i << end;
}
```

C++11 introduces a range based loop which simplifies the syntax when iterating over arrays and collections:

```cpp
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

### Infinite Loop

An infinite loop is one that never ends. The typical way to do this in C++ is to test against an expression that always evaluates to true or an empty for loop:

```cpp
while (true) {
  poll();
  do_work();
}
// Or with a for loop
for (;;) {
  poll();
  do_work();
}
```

### While Loop

C++ has conditional `while() {}` and `do { } while()` forms. The former tests the expression before it even runs while the latter runs at least once before testing the expression.

```cpp
while (!end) {
  std::string next = getLine();
  end = next == "END";
}
```

The do-while form in C++ will execute the loop body at least once because the condition is only tested after each iteration instead of before.

```cpp
int i = 0;
do {
  i = rand();
} while (i < 20);
```

### Break and Continue

If you need to exit a loop or start the next iteration early then you use the `break` and `continue` keywords. The break keyword terminates the loop, the continue, causes the loop to proceed to the next iteration.

```cpp
bool foundAdministrator = false;
for (int i = 0; i < loginCredentials; ++i) {
   const LoginCredentials credentials = fetchLoginAt(i);
   if (credentials.disabled) {
     // This user login is disabled so skip it
     continue;
   }
   if (credentials .isAdmin) {
     // This user is an administrator so no need to search rest of list
     foundAdministrator = true;
     break;
   }
   // ...   
}
```

## Rust

### For loop

#### Iterating a range

Rust `for` loops are quite different from C++ because Rust allows software to iterate over a defined range:

```rust
for i in 0..100 {
  println!("Number {}", i);
}
```

Every iterable item also implements an `enumerate()` function that returns a tuple. The first item is the zero based index of the item in the range and the second is the value.

So for example:

```rust
for (i, x) in (30..50).enumerate() {
   println!("Iteration {} is value {}", i, x);
}
```

### For loop - Iterating collections

TODO

### Break and Continue

Rust also has `break` and `continue` keywords and they operate in a similar fashion - they operate on the innermost loop. A `continue` will start on the next iteration while a `break` will terminate the loop.

TODO Rust example

#### Labels

As `break` and `continue` only work on the inner most loop there will be occasions where they do not work as desired, e. If you need to break or continue an outer loop, you can label each loop and refer to the label in the break / continue to indicate what you're breaking.

TODO example

### Infinite Loop

Rust has an explicit infinite `loop` that runs indefinitely:

```rust
loop {
  poll();
  do_work();
}
```

Rust recommends using this form when an infinite loop is required. Note that an infinite loop can still be broken out of using a `break` statement.

### While Loop

A `while` loop in Rust looks pretty similar to one written in C/C++. The main difference is that parentheses are not necessary around the conditional test.

```rust
while request_count < 1024 {
  process_request();
  request_count = request_count + 1;
}
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

Just as there is an `if let` which tests and assigns a value that matches a pattern, there is also a `while let` equivalent:

```rust
let mut iterator = vec.into_iter();
while let Some(value) = iterator.next() {
  process(value);
}
```

This loop will break when the iterator returns `None`.

