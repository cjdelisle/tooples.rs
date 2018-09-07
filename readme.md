# Tooples

Array functions for tuples.

In dynamic languages like javascript, you can just make an array of any types of elements
and then push and pop anything you want, to and from it. Of course, if ever you reference
the wrong item in a list or the type of the item is not what you expected it to be, your
program will do the wrong thing. However, javascript lists come with some handy functions
such as push, pop, shift, unshift and join which make them easier to deal with.

In Rust, you can have tuple types which can contain any type of element *but* tuples are
immutable, you cannot iterate over them because the elements can be of different types,
and tuples with more than 12 elements are missing some of their features.

This library implements `push()`, `pop()`, `shift()`, `unshift()` and `join()` for tuples
with 12 items or less. For every function except `join()`, the tuple is consumed and the
result is a newly created tuple.

Example:

```rust
use tooples::*

println!("{}", ("hello", "world").join(" ") );
println!("{}", (1, 1.1).join(" ") );
println!("{}", ("hello", 1.1).join(" ") );
println!("{}", ("hello", 1.1, 1, "world").join(",") );

let x = (1,"hi");
let x = x.push(3.3);
let x = x.push("test");
println!("{}", x.join(" "));

let x = x.pop();
println!("{}", x.join(" "));

let x = x.unshift("before");
println!("{:?}", x);

let x = x.shift().shift();
println!("{:?}", x);
```