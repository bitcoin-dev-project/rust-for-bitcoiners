# JSON serialization with serde

Let's say we want to display our data in JSON format.
A popular library that many Rust developers use is the [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) crate.

## Setup

We can add this to our Cargo.toml file like so:
```toml
[package]
name = "serde_tutorial"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0.115"
```

Note that we want to bring in the `derive` feature from `serde`.
This will allow us to `derive` the `Serialize` implementation for our structs.
We'll also bring in `serde_json` so that we can convert our struct to a JSON formatted `String`. 

## Example

Let's take an example using a struct from the Rust-Bitcoin library:

```rust
pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf,
}
```

So there are two different field types here, the `Amount` type and the `ScriptBuf` type. 
These are essentially just wrappers for other types in the form of tuple structs.
Let's include those.

```rust
pub struct Amount(u64)
pub struct ScriptBuf(Vec<u8>)

pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf,
}
```

Now we can add the `derive` attribute to `TxOut` for `Serialize` and see if this works by calling `serde_json::to_string_pretty` on our `TxOut` struct instance.
Remember to bring `serde_json` and `serde::Serialize` into scope.
You can follow along on [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a76739816ac2a17b268a58064dde24b6).

```rust
use serde_json;
use serde::Serialize;

pub struct Amount(pub u64);
pub struct ScriptBuf(pub Vec<u8>);

#[derive(Serialize)]
pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf,
}

fn main() -> () {
    let amount = Amount(1_000_000);
    let script_pubkey = ScriptBuf(vec![0, 0, 0, 0]);

    let txout = TxOut {
        amount,
        script_pubkey,
    };

    let json_txout = serde_json::to_string_pretty(&txout).unwrap();

    println!("{}", json_txout);
}
```

This, of course, won't work. We need to implement the `Serialize` trait for both the `ScriptBuf` and `Amount`. 
This trait is already implemented by default for most primitive and standard types. 
However, since these are custom types we'll need to implement the `Serialize` trait for each of them. 

# Problem Statement

Implement the `Serialize` trait for the `Amount` and `ScriptBuf` structs.

## Implementation

When implementing the `Serialize` trait, we must implement the required method, `serialize`.
This is the method signature:
`fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error>`.
It accepts a type which implements the `Serializer` trait.
This type will be passed in by `serde_json` when we call the `to_string_pretty` method.
The `Serializer` trait has several methods available that we can use to serialize various types, which can be found in the [documentation](https://docs.rs/serde/latest/serde/ser/trait.Serializer.html).

Let's start with a simple version, leveraging the methods available for the `Serializer` trait.

```rust
...

impl Serialize for Amount {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(self.0)
    }
}

impl Serialize for ScriptBuf {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(self.0.as_slice())
    }
}

...
```

Notice that we use the `serialize_u64` and `serialize_bytes` methods. 
The `serialize_bytes` method accepts a `slice` as an argument so we'll call the `.as_slice` method on the `Vec`.

This will now work and print the json output we expect:
```console
{
  "value": 1000000,
  "script_pubkey": [
    0,
    0,
    0,
    0
  ]
}
```

## Separating display from storage

However, there's another problem. We want to print the `Amount` in bitcoin and not satoshis. 
We want to store it in satoshis for internal purposes but serialize it as a bitcoin amount for display purposes. 

With `serde` we can provide custom serialization at the field level.
See [documentation](https://serde.rs/field-attrs.html#serialize_with) for various field attributes that we can add. 
This is useful when we want to serialize `Amount` differently based on its context.

We can do something like the following:

```rust
#[derive(Serialize)]
...

pub struct TxOut {
    /// The value of the output, in satoshis.
    #[serde(serialize_with = "as_btc")]
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf,
}

...
```

The `as_btc` method accepts a generic type `T` so we'll set up a new trait called `SerdeAmount` and implement the `ser_btc` method.

```rust
...

trait SerdeAmount {
    fn ser_btc(&self) -> f64;
}

impl SerdeAmount for Amount {
    fn ser_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

#[derive(Serialize)]
pub struct TxOut {
    /// The value of the output, in satoshis.
    #[serde(serialize_with = "as_btc")]
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf,
}

fn as_btc<T: SerdeAmount, S: Serializer>(t: &T, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_f64(t.ser_btc())
}

...
```

The full code is available on [Rust-Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=aa0ad5300c3c991be47b1943e3c63d21).
