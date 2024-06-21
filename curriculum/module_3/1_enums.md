## Enums

If you want to specify a Type which can be one of *n* types then enum can be used.
It can be as simple as C enum which can be used to represent choices or can be as powerful as
Algebraic Data Types found in Haskell.
Enums are called Sum types because at any point the value is one of them *n* types.

**Example 1: Unit-only enum**

Simple enum to represent choices from [rust-bitcoin](https://docs.rs/bitcoin/latest/bitcoin/network/enum.Network.html)
```rust
// At any point in time a user can connect using one of the network modes
enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}
```

So the `Network` type can be one of the 4 variants.

```rust
let bitcoin: Network = Network::Bitcoin;
let testnet: Network = Network::TestNet;
/// other variants
```

### Complex enums where some of the variants can be arbitrary

The semantics are described below,

```rust
enum Example<T> {
    Constructor1(T),
    Constructor2(u32),
    Variant1,
    Variant2,
    Constructor3(String),
}
```

The value of type `Example` can be one of the two variants or can be created using one
of the three constructors by supplying appropriate values.

#### Option

```rust
enum Option<T> {
    Some(T),
    None,
}
```

It is used to denote the presence or absence of a value.
When the computation is well defined for a given input then that outcome can be represented
using `Option::Some(value)`.
Sometimes the result of a computation may not be defined for certain
inputs, in that case `Option::None` is used to denote the absence of the value.

In some cases the value is not known at compile time but can be loaded when the program
starts, in that case Option can be used along with mutable variable like shown below,

```rust
let mut some_u32: Option<u32> = Option::None;

/// Fetch some contents and do some computation ...

some_u32 = Option::Some(after_startup_computation());
```

In other programming languages the commonly used trick is null pointers in combination with
run time exceptions which are hard to debug and read.

#### Result

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

Some computation may result in a failure. For example you are trying to connect to a bitcoin
node with rpc and you failed to provide correct username or password, this is an error not
an absence of value. In that case Result is used.

That's why the return type of ```Client::new(url, Auth::user(username, password))``` of RpcApi trait
in *rust-bitcoincore_rpc* module is a result, [refer](https://docs.rs/bitcoincore-rpc/latest/bitcoincore_rpc/struct.Client.html#method.new).

In general some computations can fail due to many factors, this can be seen in `bitcoincore_rpc` crate's
`Error` type,

```rust
enum Error {
    JsonRpc(jsonrpc::error::Error),
    Hex(hex::HexToBytesError),
    Json(serde_json::error::Error),
    BitcoinSerialization(bitcoin::consensus::encode::FromHexError),
    Secp256k1(secp256k1::Error),
    Io(io::Error),
    InvalidAmount(bitcoin::amount::ParseAmountError),
    InvalidCookieFile,
    /// The JSON result had an unexpected structure.
    UnexpectedStructure,
    /// The daemon returned an error string.
    ReturnedError(String),
}
```

refer [doc](https://docs.rs/bitcoincore-rpc/latest/bitcoincore_rpc/enum.Error.html).

### Accessing Enum Values
There are different ways to work with an enum variant and access its inner value. Consider the following modified example from the documentation for the [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html#examples) trait. You can also experiment with this on [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dd936210b74c9ca00b11d3c27b546baf).
```rust
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut b = "This string will be read".as_bytes();
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let bytes_read = b.read(&mut buffer)?;
    println!("bytes_read: {}", bytes_read);
    println!("modified buffer: {:?}", buffer);

    // returns the Ok variant wrapping an empty tuple
    Ok(())
}
```

`b.read` will return a [`Result`](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read) which will wrap a `usize` for the `Ok` variant. One way to access this inner value is to append the question mark operator. This will unwrap the inner value, which is the `usize` representing the bytes successfully read. If the result is an `Error` variant, this will instead propagate that error to the calling function and then exit the function.

Let's explore triggering an error using the `read_exact` method. This will read an exact amount of bytes. So if the buffer is too large for the bytes available to read, this will propagate an error ([Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=61e0c88c6ccb08d076b37102dcda2d63)):
```rust
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut b = "This string will be read".as_bytes();
    let mut buffer = [0; 25];

    // read exactly 25 bytes or return an error
    b.read_exact(&mut buffer)?;
    println!("modified buffer: {:?}", buffer);

    // returns the Ok variant wrapping an empty tuple
    Ok(())
}
```

Run this and see what happens. Then, try modifying the string to make it longer. For example, you can change the first line to `let mut b = "This longer string will be read".as_bytes()`. See how this changes the result.

Another way to write this could be something like the following ([Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=61c22aa6a9b28fe80650b73a718145ea)):
```rust
use std::io::Read;

fn read_25_bytes(bytes: &mut &[u8]) -> std::io::Result<[u8; 25]> {
    let mut buffer = [0_u8; 25];
    
    // read exactly 25 bytes or propagate an error
    bytes.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let mut b = "This string will be read".as_bytes();

    // handle the error case
    match read_25_bytes(&mut b) {
        Ok(buffer) => println!("read successfully! returned buffer: {:?}", buffer),
        Err(e) => eprintln!("We received an error!: {}", e)
    }
}
```

### Further reading

* Highly recommend to read the [reference manual](https://doc.rust-lang.org/reference/items/enumerations.html) on enums.
* Option [docs](https://doc.rust-lang.org/std/option/).
* Result [docs](https://doc.rust-lang.org/std/result/).
