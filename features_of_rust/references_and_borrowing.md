# Lifetimes, References and Borrowing

C/C++ have very little enforcement of object lifetimes aside from checking to see if a variable exists in scope or not, as well as the initialisation of reference variables. This can easily lead to situations where the reference / pointer to some object is left "dangling", i.e. the object has been destroyed but it is possible to call to where it used to be causing runtime errors.

Rust takes a far stricter view of lifetimes and ownership.

1. Data moves on assignment
2. Data must implement a `Copy` trait if it wants to implicitly copy on assignment and there are strict rules on this - basically every member of the struct must be a primitive.
3. Data must implement a `Clone` trait if it wants to implicity create a clone of itself.
4. Data can be "borrowed" (with similar notation and meaning as a reference in C++) but the compiler tracks lifetimes and it is an error for a borrow to exceed the lifetime of the object it references.
5. Data can only be mutably borrowed by a single reference. It is a compile error for there to be any other references in existence at the same time. This is to prevent one place from modifying data that other places may be reading.

## Binding

When you assign an object to a variable in Rust, you are said to be binding it. i.e your variable "owns" the object for as long as it is in scope and when it goes out of scope it is destroyed.

```rust
{
  let v1 = vec![1, 2, 3, 4]; // v1 is bound to the Vec
  ...
  // v1 goes out of scope, Vec is dropped
}
```

## Moving on assignment

In C++ when I copy a value from `v1` to `v2`, I have two copies of the same data, independent of each other. This can cause some problems:

```c++
class Data {
  Data() : data_(new char[100]) {}
  ~Data() {
    delete []data_;
  }
private:
  char *data_;
}
//...
Data v1;
Data v2 = v1;
// What happens when these go out of scope?
```

The problem here is that by assigning `v1` to `v2` we now have two classes who share the same private pointer `data_` and the last to go out of scope and delete the pointer will crash. We could mitigate the problem assuming we noticed it in a couple of ways:

1. Implement a copy constructor and assignment operator to go with the destructor. This is a pattern called the *rule of three* and while it solves the issue, it complicates our simple class and brings its own issues (e.g. handling `v1 = v1` properly). 
2. Inherit from a base class with a private copy constructor. This causes the compiler to generate an error on assignment. This is how the `boost::noncopyable` works
3. Implement move on assignment. A move means that the assignment passes ownership of the data from `v1` to `v2` and the value in `v1` is invalid and will not be unwound or destroyed. This is even more complex than 1) and usually leads to the *rule of five* which is even more complex.

Rust simplifies this by treating everything as move on assignment.

```rust
struct Data {
  data: Box<[u8; 100]>
}
// ...
let v1 = Data { data: Box::new([0u8; 100])};
let v2 = v1;
// ...
```

The assignment moves the data from `v1` to `v2` and marks `v1` as invalid. If you attempt to reference `v1` any more in your code, it will generate a compile error. If there was a panic and the stack unwound, the data in `v2` would be unwound and the data in `v1` would be ignored.

Likewise, if we pass by value to a function then that also moves ownership:

```rust
{
  let v1 = Data { data: Box::new([0u8; 100])};
  we_own_it(v1);
  println!("v = {:?}", v1);
}

fn we_own_it(v: Data) {
  // ...
}
```

When we called `we_own_it(v1)` we moved ownership of the data from `v1` to the function parameter and it never came back.

If we absolutely wanted the data to come back we could do it in this somewhat clumsy inefficient way:

```rust
v1 = we_own_and_return_it(v1)
...
fn we_own_and_return_it(v: Data) -> Data {
  // ...
  v1
}
```

So we:

1. Assign the data to `v1`
2. Move the data to `v` when we call `we_own_and_return_it`
3. Return `v` as the result of the function
4. Move the data back to `v1`.

That's a lot of moving and a further section will explain borrowing.

### Variables must be bound to something

Just one more point. Variables must be bound to something. You cannot use a variable if it hasn't been initialized with a value of some kind:

```rust
let x: i32;
println!("The value of x is {}", x);
```

It is quite valid in C++ to declare variable and do nothing with it. Or conditionally do something to the variable which confuses the compiler so it only generates a warning.

```c++
int result;
{
   // The scope is to control the lifetime of a lock
   lock_guard<mutex> guard(data_mutex);
   result = do_something();
}
if (result == 0) {
  debug("result succeeded");
}
```

The Rust compiler will throw an error, not a warning, if variables are uninitialised. It will also warn you if you declare a variable and end up not using it.

## References and Borrowing

We've seen that ownership of an object is tracked by the compiler. If you assign one variable to another, ownership is bound to the assignee. The original variable is invalid and the compiler will generate errors if it is used.

Sometimes we only want to *borrow* data, use it temporarily without moving it around or the added noise in the code of reassignment.

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
