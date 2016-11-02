# Multiple Inheritance

C++ allows code to inherit from multiple classes and they in turn could inherit from other classes. This gives rise to the dreaded _diamond pattern_.

e.g. D inherits from B and C but B and C both inherit from A. So does D have two instances of A or one?

This can cause compiler errors which are only partially solved by using something called "virtual inheritance" to convince the compiler to share A between B and C.

i.e if we knew B and C could potentially be multiply inherited we might declare them with a virtual keyword in their inheritance:

```c++
class B : public virtual A {
//...
};
class C: public virtual A {
};
class D: public B, public C {
//...
};
```

When D inherits from B and C, both share the same instance of A. But that assumes the authors of A, B and C were aware of this problem arising and coded themselves with the assumption that A could be shared.

The more usual normal solution for diamond patterns is "don't do it". i.e use composition or something to avoid the problem.

## How Rust helps

Rust also does not use class inheritance so problems like diamond patterns cannot exist.

However traits in Rust can inherit from other traits, so potentially it could have diamond-like issues. But to ensure it doesn't, the base trait is implemented separately from any traits that inherit from it.

So if struct D implements traits B & C and they inherit from A, then A, B and C must have impl blocks.

```rust
trait A {
//...
}

trait B : A {
//...
}

trait C : A {
//...
}

struct D;

impl A for D {
//...
}

impl B for D {
//...
}

impl C for D {
//...
}
```
