# Traits

**Definition**
Traits allows unification of types under certain context.

By now you would've worked wit various number types like `u16`, `i32`, `f64` etc.,
All of them behaves similarly to the programmer in general but each of them are
represented differently in the computer.

```rust
// Assume both of the computations below are executed at run-time
let x = 5 as u64 * 100 as u64;
let y = 5 as u128 * 100 as u128;

assert_eq!(x as u128, y); // TRUE, no possible loss in conversion from u64 to u128
```

In the above code snippet, eventhough both x and y have the same value semantically,
the machine code generated to compute them are different, in fact it take extra clock
cycles to compute `y`.

The compiler was able to provide the programmer with a uniform semantics but handles
all the messy details by itself. This was possible because of Traits in rust, and for this
particular example [Mul](https://doc.rust-lang.org/std/ops/trait.Mul.html) trait.

## What is a Trait in rust?

A trait is simply a list of types and function definitions.

**Example**

```rust
trait Foo {
    type Output; // Customizable

    fn do_something(self) -> Output;
}
```

In `do_something` function the parameter `self` refers to the variable of the type that is
implementing the trait.

## Implementing Add trait for U256

In the last chapter we encountered `U256` but currently we have not defined the mechanisms
to do additions like `U256 + U256`.

This can be achieved by implementing the [Add](https://doc.rust-lang.org/std/ops/trait.Add.html) trait
for U256.

```rust
/// Adding U256 + U256
impl Add for U256 {
    type Output = U256;

    // self refers to U256
    fn add(self, other: U256) -> U256 {
        let (sum_x0, carry) = self.x0.overflowing_add(other.x0);
        let (sum_x1, _carry) = self.x1.overflowing_add(other.x1 + carry as u128);

        // _carry will be ignored, like it would happen for any other primitive int type

        U256 {
            x0: sum_x0,
            x1: sum_x1,
        }
    }
}
```

What if we want to do `U256 + u128`?

```rust
/// Adding U256 + u128
impl Add<u128> for U256 {
    type Output = U256;

    // self refers to U256
    fn add(self, other: u128) -> U256 {
        let (sum_x0, carry) = self.x0.overflowing_add(other);
        let sum_x1 = self.x1 + carry as u128;

        U256 {
            x0: sum_x0,
            x1: sum_x1,
        }
    }
}
```

How about `u128 + u256`?

```rust
/// We are implementing a trait for an already existing type `u128`
impl Add<U256> for u128 {
    type Output = U256;

    // self refers to u128
    fn add(self, other: U256) -> U256 {
        let (sum_x0, carry) = self.overflowing_add(other.x0);
        let sum_x1 = other.x1 + carry as u128;

        U256 {
            x0: sum_x0,
            x1: sum_x1,
        }
    }
}
```

All this was possible because the `Add` trait was defined as follows,

```rust
trait Add<Rhs = Self> {
    type Output;

    /// Performs the `+` operation.
    ///
    /// # Example
    ///
    /// ```
    /// assert_eq!(12 + 1, 13);
    /// ```
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

That `Rhs` term in the definition allows adding numbers of different types together.
