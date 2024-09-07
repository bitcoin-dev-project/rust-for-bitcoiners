# Why Option is a monad

Monad is a theoretical concept.
A type is a monad if it satisfies the [monad laws](https://wiki.haskell.org/Monad_laws).

For the following `<==>` symbol means equivalent.

#### Bind operator

`>>= <==> and_then` refer [and_then](https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then)

`return <==> Some`, think of `Some` constructor as a function which will return an Option value. 

Distinguish between a monad value and a function that returns a monad value.
The idea is that one should be able to bind a monad with a function that returns a monad value.

## Why all the fuzz?

If a type satisfies certain laws then it is very easy to work with.

