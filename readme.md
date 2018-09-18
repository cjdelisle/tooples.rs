# Tooples

[![Build Status](https://travis-ci.org/cjdelisle/tooples.rs.svg?branch=master)](https://travis-ci.org/cjdelisle/tooples.rs)

## Array Functions

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

## apply_to()

This library also provides apply functions, you can call a function given a tuple of arguments.
It provides `apply_to()`, `apply_to_mut()` and `apply_to_once()` which take a `Fn`, `FnMut` and
`FnOnce` respectively. In every case, the tuple provided is by-val, it is consumed by the call
and can nolonger be used again.

Example:

```rust
let x = |a,b,c| {
    println!("{} / {} / {}", a, b, c);
};

let y = ();
let y = y.push(1);
let y = y.push(&"hello");
let y = y.push(3.2);

y.apply_to(&x);
```

## Advanced usage

You should probably define your functions by the precise tuples which they expect to take, however,
if you want to define a generic function that will take ANY tuple (up to 12 elements) and push one
particular element to the end of it, you can do so like this.

```rust
use std::time::Instant;
fn push_current_time<W>(w:W) -> Push<W,Instant> where W:CanPush<Instant> {
    w.push(Instant::now())
}
fn main() {
    let x = ();
    let x = x.push(3);
    let x = push_current_time(x);
    let x = x.push(&"hi");
    let x = push_current_time(x);
    println!("{:?}", x);
}
```

Here, instead of specifying the exact type of input tuple, you specify that it is anything which is
"capable of having an Instant pushed to it" (`where W:CanPush<Instant>`). The return type is
described as "the result of pushing Instant to a W" (`Push<W,Instant>`). You can even get more
complicated than that, but the function signatures will quickly get out of hand.

```rust
fn replace_last_item<W,X>(w:W, x:X) -> Push<Pop<W>,X> where
    W:CanPop, Pop<W>:CanPush<X>
{
    let w = w.pop();
    w.push(x)
}
```

In this example, we accept a tuple W, or more exactly, we accept a W that "can be popped from"
(`W:CanPop`) as long as the result of popping W must be able to have an X pushed to it
(`Pop<W>:CanPush<X>`). This function will replace the last item in a tuple, no matter what length
or types it contains.