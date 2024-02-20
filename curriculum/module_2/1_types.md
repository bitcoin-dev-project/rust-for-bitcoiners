# Data Types

## Structs

In Rust structs are product types, think about Cartesian product between sets.
It is a collection of *n* types. At any point in time the value of a struct is the
combination of all the *n* types.
This is very similar to a simple class in Java or struct in C.

**Example**

```rust
struct FieldElement {
    n: u128,
    p: u128,
}
```

## Enums

If you want to specify a Type which can be one of *n* types then enum can be used.
It can be as simple as C enum which can be used to represent choices or can be as powerful as
Algebraic Data Types found in Haskell.
Enums are called Sum types because at any point the value is one of them *n* types.

**Examples**

Simple enum to represent choices from [rust-bitcoin](https://docs.rs/bitcoin/latest/bitcoin/network/enum.Network.html)
```rust
// At any point in time a user can connect one of the following
enum Network {
    Bitcoin,
    Testnet,
    Signet,
    Regtest,
}
```

Complex enum example from [utxo-teleport](https://github.com/utxo-teleport/teleport-transactions/blob/master/src/wallet/api.rs)

```rust
enum UTXOSpendInfo {
    SeedCoin {
        path: String,
        input_value: u64,
    },
    SwapCoin {
        multisig_redeemscript: ScriptBuf,
    },
    TimelockContract {
        swapcoin_multisig_redeemscript: ScriptBuf,
        input_value: u64,
    },
    HashlockContract {
        swapcoin_multisig_redeemscript: ScriptBuf,
        input_value: u64,
    },
    FidelityBondCoin {
        index: u32,
        input_value: u64,
    },
}
```

Most frequently encountered enums are *Result* and *Option*.

### Option

It is used to denote the presence or absence of a value. Sometimes the result of
a computation may not produce a value or in other words it is not defined for certain
inputs, in that case *Option* is used to denote the value. In other programming languages
the commonly used trick is null pointers or other run time exceptions which are hard
to predict.

### Result

Some computation may result in a failure. For example you are trying to connect to a bitcoin
node with rpc and you failed to provide correct username or password, this is an error not
an absence of value. In that case Result is used.

That's why the return type of ```Client::new(url, Auth::user(username, password))``` of RpcApi trait
in *rust-bitcoincore_rpc* module is a result.

