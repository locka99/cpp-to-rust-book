# Macros

Macros in C/C++ are basically little rules that are defined by a preprocessor and substituted into the code that the compiler ultimately attempts to compile.

Modern coding practice these days is to use inline functions and constants instead of macros.

But the reality is they can still be (ab)used and code often does. For example code might insert debug statements or logging which is compiled away in release mode.

Another common use is on Windows where the type TCHAR compiles to be either char or wchar_t depending on UNICODE being defined or not. Along with it go macros like USES_CONVERSION, A2CT, T2CW etc. Code should compile cleanly either way but the reality is usually it doesn't.

A classic problem would be something like this:

```c++
#define SQUARED(x) x * x
// And in code
float result = SQUARED(++x);
That would expand to
float result = ++x * ++x;
```

So the value in result would be wrong and the value in x would be incremented twice.

## Compilation errors

Consider we are compiling this structure:

```c++
// Header
struct Tooltip
#if TOOLTIP_VERSION > 4
  char buffer[128];
#else
  char buffer[64];
#endif
};
```

And in C++

```c++
Tooltip tooltip;
memset(&tooltip, 0, sizeof(tooltip));
```

If we fail to define TOOLTIP_VERSION to the same value in the implementation as in the caller, then this code may stomp all over memory because it thinks the struct is 128 bytes in one place and 64 bytes in another.

## Namespace issues

Macros aren't namespaced and in some cases this leads to problems where a macro definition collides with a well qualified symbol.
For example in Windows, TRUE is a #define for 1. But that excludes any other code that expects to compile on Windows from ever using TRUE as a const no matter how well they qualify it. Consequently code has to do workarounds such as #undef macros to make code work or using another value.

```c++
#ifdef TRUE
#define TMP_TRUE TRUE
#undef TRUE
#endif
bool value = myapp::TRUE;
#ifdef TMP_TRUE
#define TRUE TMP_TRUE
#undef TMP_TRUE
#endif
```

Ugh. But more likely we'll rename myapp::TRUE to something like myapp::MYAPP_TRUE to avoid the conflict. It's still an ugly workaround for a problem caused by inconsiderate use of macros.

Commonly used words like TRUE, FALSE, ERROR, OK, SUCCESS, FAIL are more or less unusable thanks to macros.

## How Rust helps

Rust provides developers with consts, inline attributes, and platform / architecture attributes for the purpose of conditional compilation.

Rust offers macros but they consist of a set of matching rules than must generate syntactically Rust. Macro expansion is performed by the compiler so it is capable of generating errors on the macro if the macro is in error.
