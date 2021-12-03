# Enumerations

## Scalar values

In C++ an `enum` is a list of scalar values to an integer value. i.e. it is basically a bunch of constants with values.

```cpp
enum HttpResponse {
  okay = 200,
  not_found = 404,
  internal_error = 500,
};
```

C++11 extends this concept a little, allowing you to declare an `enum` that uses another kind of integral type, e.g. a `char` to hold the values.

```cpp
enum LibraryCode : char {
  checked_in = 'I',
  checked_out = 'O',
  checked_out_late = 'L'
};
```

In Rust an [`enum`](https://doc.rust-lang.org/book/enums.html) can be a scalar value just like in C++. 

```rust
enum HttpResponse {
  Ok = 200,
  NotFound = 404,
  InternalError = 500
};
```

## As a typed union

C/C++ have a construct called a `union` that describes a variable that can contain one of many possible fields:

```c
union VariantValue {
  char *s;
  uint8_t u8;
  uint16_t u16;
  double f64;
}
```

So this `VariantValue` can hold a pointer to a string, or an unsigned byte, or an unsigned word, or a double precision float. It is up to the caller and callee to know from the context which value is used.

It is quite common for a union to be wrapped up in a struct that has a type field describing what the payload is, for example:

```c
enum VariantType {
  VT_STRING,
  VT_UINT8,
  VT_UINT16,
  VT_DOUBLE
};

struct Variant {
  VariantType vt;
  VariantValue value;
}
```

This `Variant` has a `VariantType` enumeration that says which field in `VariantValue` should be read. We can then proceed to use the code something like this:

```c++
Variant v;
v.type = VT_STRING;
v.value.s = "Hello World";
printValue(v);

void printValue(const Variant &v) {
  switch (v.vt) {
    case VT_STRING: 
      std::cout << v.value.s << endl;
      break;
    case VT_UINT8:
      std::cout << v.value.u8 << endl;
      break;
    case VT_UINT16:
      std::cout << v.value.u16 << endl;
      break;
    case VT_DOUBLE:
      std::cout << v.value.f64 << endl;
      break;
  }
}
```

Rust provides all of the above functionality through `enum`. Instead of the above we could declare an `enum` in Rust where values can hold a payload:

```rust
enum Variant {
  String(String),
  UInt8(u8),
  UInt16(u16),
  Double(f64)
};
```

So here, the enum `String` has a payload value of a `String` type. This `Variant` type can be easily initialised and the value inferred:

```rust
let v = Variant::String("Hello World".into());
printValue(&v);

fn printValue(v: &Variant) {
  match v {
    Variant::String(ref v) => println!("{}", v),
    Variant::UInt8(v) => println!("{}", v),
    Variant::UInt16(v) => println!("{}", v),
    Variant::Double(v) => println!("{}", v),
  }
}
```

This code is functionally the same as C++, but it is a lot terser and a lot safer. Essentially the variant is constructed in a single line and passed into the function. The `match` extracts the payload according to the match pattern and calls the handler. 

Note: you'll see in the match on `Variant::String(ref v)` that it uses a reference to the value. This is because otherwise it would move the value from the `Variant` which would cause a compiler error. The variant is supplied to the function as a reference and we cannot move values out of it. It is fine for the other branches of the match because numeric types have an intrinsic `Copy` trait and don't need to move to do this.

### Rust has union too

Rust does have a `union` keyword that works very much like C/C++. 

```rust
union VariantValue {
  s: String,
  uint8: u8,
  uint16: u16,
  double: f64,
}
```

But since Rust does not know what the type of the value is, you are required to use `unsafe` blocks anywhere you read or write values to the union:

```rust
let v = VariantValue{ s: "Hello World!".into };

unsafe {
  // This code has to know v contains a string
  println!("Value of value is {}", v.s);
}
```

Ordinarily you'll probably only ever use `union` from Rust if you're talking with C or C++.