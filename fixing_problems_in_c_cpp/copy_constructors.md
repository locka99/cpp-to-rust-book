# Copy Constructor \/ Assignment Operators

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

Well that was easy. And dangerous.

By default C++ allows us to copy and assign one class to another so that we make multiple copies of the same data. The compiler generated a copy constructor and assignment operator for us even though PersonList doesn't say anything about copy or assignment. Lucky us!

Except copying doesn't work the way it should for managed data.

The default copy constructor copies that member variable `personList_` even though its a pointing to private data. So `y` and `z` will contain a `personList_` that points to the same memory as `x`. So when `z`, `y` and `x` go out of scope, the same pointer will be deleted three times and the program might crash. On top of that, `z` allocated its own `personList_` but the assignment overwrote it with the one from `x` so its old `personList_` value just leaks.

## The Rule of Three

This is such a bad issue that it has given rise to the so-called the rule of three.

The rule says that if we explicitly declare a destructor, copy constructor or copy assignment operator in a C++ class then we probably need to implement all three of them to safely handle assignment and construction. In other words the burden for fixing C++'s default and dangerous behaviour falls onto the developer.

So let's fix the class:

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

We've added a copy constructor and an assignment operator to the class to handle copying safely. The code even had to check if it was being assigned to itself in case someone wrote `x = x`. Without that test, the receiving instance would clear itself in preparation to adding elements from itself which would of course wipe out all its contents.

Alternatively we might disable copy \/ assignments by creating private constructors that prevents them being called by external code:

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

Another alternative would be to a C++11 `std::unique_ptr` \(or a `boost::scoped_ptr`\). A `unique_ptr` is a way to permit only one owner of a pointer at a time. The owner can be moved from one `unique_ptr` to another and the old owner becomes `NULL` from the move. A `unique_ptr` that holds a non-NULL pointer will delete it from its destructor.

TODO unique\_ptr example

This move is similar to the move semantics we'll see in Rust in a moment. But the object is allocated on the heap, is a pointer and is not directly analogous.

## How Rust helps

Rust does allow structs to be copied or clone unless we explicitly implement the `Copy` and `Clone` traits respectively.

Most primitive types such as ints, chars, bools etc. implement `Copy` so you can just assign one to another

```rust
// This is all good
let x = 8;
let y = x;
y = 20;
assert_eq!(x, 8);
```

A `String` cannot be copied this way. If you assign a String variable to another you move ownership, i.e. the original variable is no longer able to call functions or fields on the struct. But you can explicitly clone a `String`:

```rust
let copyright = "Copyright 2017 Acme Factory".to_string();
let copyright2 = copyright.clone();
```

If we just declare a struct it also be copied by accident:

```rust
struct Person {
  name: String,
  age: u8
}
```

The following code will compile but you are not copying, you are moving:

```rust
let person1 = Person { name: "Tony".to_string(), age: 38u8 };
let person2 = person1;
println!("{}", person1.name); // Error, use of a moved value
```

Assignment moves ownership of the struct from person1 to person2. It is an error to use person1 any more.

To illustrate consider this Rust which is equivalent to the PersonList we saw in C++

```rust
struct PersonList {
    pub persons: Box<Vec<Person>>,
}
```

We can see that PersonList has a vector of Person objects. A `Box` is what we use in Rust to hold a heap allocated object. When the `Box` is dropped, the item inside is also dropped and the heap memory is freed.

So this `Vec` of Person objects is in a `Box` and is on a heap. Clear?

Now let's use it.

```rust
let mut x = PersonList { persons: Box::new(Vec::new()), };
let mut y = x;
// x is not the owner any more...
x.persons.push(Person{ name: "Fred".to_string(), age: 30u8} );
```

The variable `x` is on the stack and is a PersonList but the persons member is allocated from the heap.

The variable `x` is bound to a PersonList on the stack. The vector is created in the heap. If we assign `x` to `y` then we could have two stack objects sharing the same pointer on the heap in the same way we did in C++.

But Rust stops that from happening. When we assign `x` to `y`, the compiler will do a bitwise copy of the data in x, but it will bind ownership to `y`.  When we try to access the in the old var Rust generates a compile error.

    error[E0382]: use of moved value: `*x.persons`
       |
    10 | let mut y = x;
       |     ----- value moved here
    11 | x.persons.push(Person{});
       | ^^^^^^^^^ value used here after move
       |
       = note: move occurs because `x` has type `main::PersonList`, which does not implement the `Copy` trait

Rust has stopped the problem that we saw in C++. Not only stopped it but told us why it stopped it - the value moved from x to y and so we can't use x any more.

Sometimes we DO want to copy \/ duplicate an object and for that we must implement a trait to tell the compiler that we want that.

The Copy trait allows us to do direct assignment between variables. You can only implement Copy by deriving it:

But this will create an error:

```rust
#[derive(Copy)]
struct Person {
  name: String,
  age: u8
}
```

A `struct` can be copied if all its members can be copied and in this case `name` cannot be. The field is of type `String` that does not implement the `Copy` trait. However `String` implements the `Clone` trait.

A `Clone` trait can be derived or explicitly implemented. We can derive it if every member of the struct can be cloned which in the case of Person it can:

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

Now that Person derives `Clone`, we can do the same for PersonList because all its member types implement that trait - a Person can be cloned, a Vec can be cloned, and a Box can be cloned:

```rust
#[derive(Clone)]
struct PersonList {
    pub persons: Box<Vec<Person>>,
}
```

And now we can clone `x` into `y` and we have two independent copies.

```rust
//...
let mut x = PersonList { persons: Box::new(Vec::new()), };
let mut y = x.clone();
// x and y are two independent lists now, not shared
x.persons.push(Person{ name: "Fred".to_string(), age: 30} );
y.persons.push(Person{ name: "Mary".to_string(), age: 24} );
```

In summary, Rust stops us from getting into trouble by treated assigns as moves when a non-copyable variable is assigned from one to another. But if we want to be able to clone \/ copy we can make our intent explicit and do that too.

C++ just lets us dig a hole and fills the dirt in on top of us.

