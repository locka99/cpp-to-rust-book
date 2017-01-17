# Forward Declarations

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

## How Rust helps

In Rust forward declarations are unnecessary. The struct and function’s definition reside in a .rs and can be referenced with a use directive.
