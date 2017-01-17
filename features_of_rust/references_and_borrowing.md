# Lifetimes, References and Borrowing

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

## Variables must be bound to something
Another point. Variables must be bound to something. You cannot use a variable if it hasn't been initialized with a value of some kind:

```rust
let x: i32;
println!("The value of x is {}", x);
```
A C++ compiler might issue a warning or catch the error with strict flags, but by default it doesn't care.

The Rust compiler will throw an error. Uninitialised values are errors. It will also warn you if you declare a variable and end up not using it.

## References and Borrowing

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
