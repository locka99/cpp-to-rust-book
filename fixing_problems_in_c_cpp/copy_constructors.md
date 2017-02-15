# Copy Constructor / Assignment Operators

In C++ you can construct one instance from another via a constructor and also by an assignment operator. In some cases a constructor will be used instead of an assignment:

```c++
PersonList x;
PersonList y = x; // Copy constructor, not assignment
PersonList z;
z = x; // Assignment operator
```

By default C++ generates all the code to copy and assign the bytes in one class to another without any effort. Lucky us! 

So our class PersonList might look like this:

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

Except we're not lucky, we just got slimed. The default byte copy takes the pointer in `personList_` and makes a copy of it. Now if we copy `x` to `y`, or assign `x` to `z` we have three classes pointing to the same private data! On top of that, `z` allocated its own `personList_` during its default constructor but the byte copy assignment overwrote it with the one from `x` so its old `personList_` value just leaks.

Of course we might be able to use a `std::unique_ptr` to hold our pointer. In which case the compiler would generate an error. But it might not always be that simple. `personList_` may have been opaquely allocated by an external library so have no choice but to manage its lifetime through the constructor and destructor.

## The Rule of Three

This is such a terrible bug enabling problem in C++ that it has given rise to the so-called the Rule of Three[^1].

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

Another alternative would be to use noncopyable types within the class itself. For example, the copy would fail if the pointer were managed with a C++11 `std::unique_ptr` \(or Boost's `boost::scoped_ptr`\).

Boost also provides a `boost::noncopyable` class which provides yet another option. Classes may inherit from noncopyable which implements a private copy constructor and assignment operator so any code that tries to copy will generate a compile error.

[^1] The Rule of Three has become the Rule of Five(!) in C++11 because of the introduction of move semantics.

## How Rust helps

Structs cannot be copied by default. If you assign a struct from one variable to another, ownership moves with it. The old variable is invalid and the compiler will complain if you access it.

If you want to copy a struct you must explicitly implement either the `Copy` and / or `Clone` trait on it.

Most primitive types such as ints, chars, bools etc. implement `Copy` so you can just assign one to another

```rust
// This is all good
let x = 8;
let y = x;
y = 20;
assert_eq!(x, 8);
```

But a `String` cannot be copied this way. A string has an internal heap allocated pointer so the struct cannot implement Copy. If you assign a String variable to another you move ownership, i.e. the original variable is no longer able to call functions or fields on the struct. 

But you can explicitly clone a `String` in which case the clone operation will make a duplicate of the allocated memory so that both strings are independent of each other:

```rust
let copyright = "Copyright 2017 Acme Factory".to_string();
let copyright2 = copyright.clone();
```

If we just declare a struct it can neither be copied nor cloned by default.

```rust
struct Person {
  name: String,
  age: u8
}
```

The following code creates a `Person` object and assigns it to `person1`. When `person1` is assigned to `person2`, ownership of the data also moves:

```rust
let person1 = Person { name: "Tony".to_string(), age: 38u8 };
let person2 = person1;
```

Attempting to use `person1` after ownership moves to `person2` will generate a compile error:

```rust
println!("{}", person1.name); // Error, use of a moved value
```

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

### Implementing a Copy / Clone

Sometimes we DO want to copy / duplicate an object and for that we must implement a trait to tell the compiler that we want that.

The Copy trait allows us to do direct assignment between variables. You can only implement Copy by deriving it and you can only derive it if all the members of the struct also derive it:

```rust
#[derive(Copy)]
struct PersonKey {
  id: u32,
  age: u8,
}
```

But this will create an error because a String cannot be copied, it must be cloned.:

```rust
#[derive(Copy)]
struct Person {
  name: String,
  age: u8
}
```

But we can still implement Clone. A `Clone` trait can be derived or explicitly implemented. We can derive it if every member of the struct can be cloned which in the case of Person it can:

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

In summary, Rust stops us from getting into trouble by treated assigns as moves when a non-copyable variable is assigned from one to another. But if we want to be able to clone / copy we can make our intent explicit and do that too.

C++ just lets us dig a hole and fills the dirt in on top of us.
