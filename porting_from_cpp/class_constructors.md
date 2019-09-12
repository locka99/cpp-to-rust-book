# Explicit / Implicit Class Constructors

It's not just overloading that can be a mess. C++ has a bunch of rules about implicit / explicit type conversion for single argument constructors.

For example:

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

The function `magic()` takes a `const MagicNumber &` yet we called it with `2016` and it still compiled.
How did it do that? Well our `MagicNumber` class has a constructor that takes an `int` so the compiler 
implicitly called that constructor and used the `MagicNumber` it yielded.

If we didn't want the implicit conversion (e.g. maybe it's horribly expensive to do this without knowing), 
then we'd have to tack an `explicit` keyword to the constructor to negate the behaviour.

```c++
explicit MagicNumber(int value) {}
```

It demonstrates an instance where the default behavior is probably wrong. The default *should* be `explicit` 
and if programmers want implicit they should be required to say it.

C++11 adds to the confusion by allowing classes to declare deleted constructors which are basically hints to the compiler to say,
"if you see something resembling this, then generate a compiler error".

For example, perhaps we only want implicit `int` constructors to match but we want to stop somebody passing in
a `double`. In that case we can make a constructor for `double` and then delete it.

```c++
class MagicNumber {
public:
    MagicNumber(int value) {}
    MagicNumber(double value) = delete;
};

void magic(const MagicNumber &m) {
  //...
}

//...
magic(2016);   // OK
magic(2016.0); // error: use of deleted function 'MagicNumber::MagicNumber(double)'
```

## How Rust helps

Rust does not have constructors and so there is no implicit conversion during construction. And since there is no
implicit conversion there is no reason to have C++11 style function delete operators either. 

This might seem counter intuitive and a bit painful, but we've just seen the problems they cause in C++.

But what does Rust let you do instead then? The answer in Rust is that a "constructor" is just any old function
that yields an instance of your struct. By convention you will normally see a `new()` function like this:

```rust
struct MagicNumber { /* ... */ }

impl MagicNumber {
  pub fn new(value: i32) -> MagicNumber { /* ... */ }
}
```

You can write as many explicit "constructor" functions but since Rust doesn't do operator overloading they must be named
explicitly, e.g. perhaps we have a `new(i32)`, `new_double(f64)` etc.  

### Using traits to simplify new()

This can get clumsy so we can use generic patterns to simplify our code.

```rust
impl MagicNumber {
  fn new<T>(value: T) -> MagicNumber where T: Into<MagicNumber> {
    value.into()
  }
}
```

We have said here that the `new()` function takes as its argument anything that type `T` which implements the trait `Into<MagicNumber>`. Then our
implementation just calls `into()`, expecting the compiler to invoke the `T:Into<MagicNumber>::into()` to yield a `MagicNumber`.

So we could implement it for `i32`:

```rust
impl Into<MagicNumber> for i32 {
   fn into(self) {
     MagicNumber { /* ... */ }
   }
}
```

Now our client code can just call `new` and providing it provides a type which implements that trait our constructor will work:

```rust
   let magic = MagicNumber::new(2016);
   // But this won't work because f64 doesn't implement Into<MagicNumber> trait
   let magic = MagicNumber::new(2016.0); 
```

### Default constructor

Sometimes we need a default constructor, i.e. we need to make instance of a struct in its default state. In C++ we'd have a constructor that takes no arguments. As
there are no constructors in Rust, we could just write a function that takes no arguments and yields the struct:

```rust
struct Point {
  x: f64,
  y: f64
}

impl Point {
  fn new() -> Point { x: 0.0, y: 0.0 }
}
```

So our `new()` just spits out a `Point` with default values.

But Rust offers a better way to do the same. Structs may implement the `Default` trait which has a single function that yields a default instance of that type:

```rust
impl Default for Point {
  fn default() -> Point { x: 0.0, y: 0.0 }
}

//...
let pt1 = Point::default();
// Or
let pt2: Point = Default::default();
```

But `Default` is implemented on all the primitives including `f64`. So we could have also initialised `x` (and similarly `y`) by saying `x: Default::default()`.

And since all the members of the struct implement `Default`, Rust also lets us just derive it for the entire struct:

```rust
#[derive(Derive)]
struct Point {
  x: f64,
  y: f64
}

//...
let pt1 = Point::default();
let pt2: Point = Default::default();
```
