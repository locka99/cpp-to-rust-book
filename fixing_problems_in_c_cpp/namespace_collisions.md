# Namespace Collisions

C code has no namespaces at all and namespaces in C++ are optional. The C++ language makes them easy to declare but there is no compunction for any code to bother or to do so in anything but the most perfunctory way.

By default all C++ code resides in a global namespace:

```c++
void hello() {
  // My function hello is in the global namespace, i.e. ::hello()
}

int main() {
  // Main entry point
  hello();
}
```

The function `hello()` is part of the global namespace. The call to it within `main` could be replaced with calls to `::hello()`. The problem of course is that the more code we write into the global namespace, or the more libraries we pull in that have no namespaces, the more chance there is of collisions.

C has learned to live without namespaces. Most C code uses prefixes on functions and structs to avoid collisions, e.g `sqlite3_exec()` is a function belonging to SQLite3. The prefix stops the function colliding with `exec()` which is a standard POSIX function that got there first. So the prefix acts as a pseudo namespace. But it adds noise to our code and would not be necessary if namespaces were supported and enforced.

Of course C++ does have namespaces, but code still has to choose to use them and it has to use them correctly. It is easy to abuse them because the compiler doesn’t really care what we do in a header. For example this is never a good idea:

```c++
// Inside of foo.h...
using namespace std;
//... all code after here is tainted with std
```

Any file that says `#include "foo.h"` will inadvertently tell the compiler to automatically look up unscoped types and functions against std which may not be what the code wants at all.

Namespacing requires code enclose the namespaced portion in a block.

```c++
namespace application {
  // stuff in here belongs to application::
}
//...
application::App app("my app");
```

Nested namespacing is also possible but it can look messy.

```c++
namespace application { namespace gui {
  // stuff in here belongs to application::gui::
} }
//... eg.
application::gui::Point2d point(100,100);
```

If we forget to close a brace when nesting headers it becomes very easy to make C++ throw up a wall of incoherent errors.

## How Rust helps

In Rust every file is implicitly a module (equivalent to a namespace). You cannot NOT use modules because you get them automatically.

If you have a collision between the names of crates or modules y
