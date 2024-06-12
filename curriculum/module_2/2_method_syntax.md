# Method call syntax

Rust is not an object oriented programming language because it does not
support inheritcance, which is a bad practice because the saying goes like
"Prefer Composition over inheritance".

If you are familiar with OOP languages you would've benefitted mightily from
the method call syntax as IDE's can implement auto-completion for fields and
methods.

So in Rust we get the benefits of method call syntax without the cons of spaghetti
code which is the eventual result of preferring inheritance.

Also inheritance affects performance and memory usage negatively.

## How to achieve method call syntax in rust?

Let's review the `bitcoin` crate to understand this.

```rust
pub struct Amount(u64);

impl Amount {
    /// The zero amount.
    pub const ZERO: Amount = Amount(0);
    /// Exactly one satoshi.
    pub const ONE_SAT: Amount = Amount(1);
    /// Exactly one bitcoin.
    pub const ONE_BTC: Amount = Self::from_int_btc(1);
    /// The maximum value allowed as an amount. Useful for sanity checking.
    pub const MAX_MONEY: Amount = Self::from_int_btc(21_000_000);

    /// Create an [Amount] with satoshi precision and the given number of satoshis.
    pub const fn from_sat(satoshi: u64) -> Amount { Amount(satoshi) }

    /// Gets the number of satoshis in this [`Amount`].
    pub fn to_sat(self) -> u64 { self.0 }

    /// Convert from a value expressing bitcoins to an [Amount].
    pub fn from_btc(btc: f64) -> Result<Amount, ParseAmountError> {
        Amount::from_float_in(btc, Denomination::Bitcoin)
    }
    ...
}
```

### using method call syntax

```Rust
// Accessing fields, called `associated constants`
let zero = Amount::ZERO;
let btc = Amount::ONE_BTC;

// Using `associated functions`
let btc_amount = Amount::from_btc(100.021);
let btc_u64_method_style = btc.to_sat(); // this is the preferred way of using associated functions with self
// while using the to_sat method the `btc: Amount` is passed to the self argument

let btc = Amount::ONE_BTC;
let btc_u64_func_style = Amount::to_sat(btc); // Not used widely, but legal
assert_eq!(btc_u64_method_style, btc_u64_func_style); // TRUE
```

Full source code can be found [here](https://docs.rs/bitcoin-units/0.1.1/src/bitcoin_units/amount.rs.html#856).

The `pub` keyword suggests that the entity following can be used from anywhere, similar
to `public` in Java or C#.

There are *associated constants* like `ZERO`, `ONE_SAT` etc.,

### Why u64 is chosen?

Let's review why `u64` is chosen but not `u32` or `u128` etc.,
The limit on the number of bitcoins in circulation is `21_000_000` which is
`21_000_000 * 100_000_000` satoshis. One can quickly verify using python interpreter that
`2^50 < 21_000_000 * 100_000_000 < 2^51`. Which means we need at least 51 bits to store all possible
satoshi values, hence 64 bit unsigned integer is used.

### Encapsulation

You might wonder what is the purpose of this `Amount` type, it is just a wrapper over `u64` right?
Because this is how we achieve encapsulation in Rust.
A library should make sure that the types defined are used as intended. `u64` will behave like
an unsigned integer, but satoshis should have the behaviours as defined in the bitcoin network
and ecosystem.

The method call syntax ensures that the user gets to know everything on how to use the `Amount` type
just by examining the function syntax and documentation, dig deeper into the code if necessary.
