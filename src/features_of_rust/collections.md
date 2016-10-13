# Collections

C has no standard collection classes or types. Users wanting collections might have resorted to using [glib](https://developer.gnome.org/glib/) or [cii](https://code.google.com/archive/p/cii/downloads).

C++ and Rust have have collections as part of their standard library as is common with modern languages.

| C | C++ | Rust
| - | std::vector | std::vec::Vec or std::collections::VecDeque
| - | std::list | std::collections::LinkedList
| - | std::set | std::collections::HashSet, std::collections::BTreeSet
| - | std::map | std::collections::HashMap, std::collections::BTreeMap
