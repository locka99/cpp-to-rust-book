Polymorphism

C++ allows functions to be polymorphic. That is to say, the same named function can be defined multiple times with different parameters. This is true for methods within a class too. The compiler will disambiguate functions based on calls and calculate a signature from the differing arguments to produce a _mangled_ name for the function that is unique at link time.

One of the biggest problems with C++ and polymorphism was that it was too easy to inadvertantly call the wrong function. For example, perhaps you call a function passing true or false as an argument but due to to implicit casting, the C++ compiler ends up invoking the function that takes an integer instead.



Rust does not support polymorphism. While there are valid reasons that it doesn't, it can still be very painful, especially if you have classes or functions that you need to be called with different arguments. 

The most annoying case would be for constructors, where there are different ways to construct a class. The naive workaround is to produce functions which are unique so they do not collide:

> > > > `fn new(name: &str);`
> > > >
> > > > `fn new_age(name: &str, age: u16);`
> > >
> > > Another way you can do this is with _traits_. A standard trait is called Into&lt;T&gt; where T is the type you wish to convert from. Our struct can implement the Into trait multply for set of

`impl Into<&str> for Foo {  
    fn into(v: &str) -> Foo {  
        //...  
    }  
}`

`impl Into<(&str, u16)> for Foo {  
    fn into(v: (&str, u16)) -> Foo {  
        //...  
    }  
}`





