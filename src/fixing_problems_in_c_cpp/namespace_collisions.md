# Namespace Collisions

C code has no namespaces at all and namespaces in C++ are optional. The C++ language makes them easy to declare but there is no compunction for any code to bother or to do so in anything but the most perfunctory way.

By default all C++ code resides in a global namespace:

```c++
void hello() {
  // My function hello
}

int main() {
  // Main entry point
}
```

The function hello() is part of the global namespace. Calls to hello() could be replaced with calls to ::hello(). The problem of course is that the more code we write into the global namespace, or the more libraries we pull in that have no namespaces, the more chance there is of collisions.  

C has learned to live without namespaces. Most C code tends to prefix all their functions and structs to avoid collisions, e.g sqlite3_exec() is a function belonging to SQLite3 and uses the prefix because exec() by itself is too common. So the prefix acts as a pseudo namespace. But this adds noise to our code and would be unnecessary if namespaces were supported and enforced.

Of course C++ does have namespaces, but code has to choose to use them and it has to use them correctly. It is easy to abuse them because the compiler doesn’t really care what we do in a header, for example this is never a good idea:

```c++
// Inside of foo.h...
using namespace std;
```

Any file that #includes foo.h is inadvertently setting the compiler to automatically look up unscoped types and functions against std which may not be what the code wants at all.

Namespacing requires code enclose the namespaced portion in a block.

```c++
namespace application {
  // stuff in here belongs to application::
}
```

Nested namespacing is also possible but it can look really messy

```c++
namespace application { namespace gui {
  // stuff in here belongs to application::gui::
} }
```

If we forget to close a brace it becomes very easy to make C++ throw up a wall of incoherent errors.

## How Rust helps

In Rust every file is implicitly a module (equivalent to a namespace). We don't need to explicitly declare a namespace although that option exists too if we wish.
