# Building UTXO Set

## What is an UTXO

UTXO stands for unspent transaction output. In a bitcoin transaction
there are inputs and outputs.
An input to a transaction has to be an output of another transaction unless the transaction
is a coinbase transaction (block reward) which has no inputs at all.
In bitcoin a UTXO is essentially a transaction out which has not been included in another
transactions input.
So inorder to validate a transaction one must make sure that the input is a UTXO i.e., it
has not been used as an input in anyother transaction and the provided *scriptsig* should
unlock the *scriptpubkey*.

## How it is represented in rust-bitcoin crate

Transaction output has the information about the amount being locked onto a script pubkey.
In rust-bitcoin crate this is how Transaction Output is represented.

```rust
pub struct TxOut {
    /// The value of the output, in satoshis.
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf,
}
```

Every transaction has a sequence of `TxOut`. So any `TxOut` can be uniquely
identified by transaction id and the index of that output in that transaction.
That's exactly how `OutPoint` is defined in rust-bitcoin,

```rust
pub struct OutPoint {
    /// The referenced transaction's txid.
    pub txid: Txid,
    /// The index of the referenced output in its transaction's vout.
    pub vout: u32,
}
```

## How to build the UTXO set?

When a bitcoin node is started, it starts downloading the block data from
it's peers one by one. It maintains a UTXO set in RAM in a HashSet like datastructure,
and by reading each transaction it validates it and updates the UTXO set accordingly.
This is a very time consuming process and proposals like [Assume UTXO](https://bitcoinops.org/en/topics/assumeutxo/), have been published.

### How big is the UTXO set?

The blocks are stored in the harddisk and currently they are around 500+GB, while UTXO data according to
my research can fit comfortably in a computer with 2GB RAM.
The size of the UTXO set is influenced by the coinselection algorithm used by bitcoin wallets.
The detailed analysis can be found in [Murch's Thesis](https://murch.one/erhardt2016coinselection.pdf).
[rust-coinselect](https://github.com/Bitshala-Incubator/rust-coinselect) is in the early stages of development, feel free to contribute.

# Problem Statement

Given a `block_height`, consider it as the genesis block and build the utxo set for the 10 blocks
starting from `block_height`.

```rust
pub fn build_utxo_set_10(client: &Client, start_height: u64) -> Result<HashSet<OutPoint>, bitcoincore_rpc::Error> {
    // TODO
}
```

Initialize the HashSet for maintaining the active utxos.
```rust
    let mut utxo_set: HashSet<OutPoint> = HashSet::new();
```

Starting from the `block_height` to `block_height + 10` fetch each block.
```rust
    for height in start_height..start_height + 10 {
        let block = get_block(height, client)?;
        // TODO
    }
```

A block has a sequence of transactions and for each of them we have to process `utxo_set` based
on it's input and output.
We have to remove the previous out of the input from the `utxo_set` and add the outputs to the set,
except when block refers to the block at `block_height` because that's how we want to do.

```rust
        for tx in block.txdata {
            // For the first block, add all utxo_set
            if height == start_height {
                for (vout_index, _) in tx.output.iter().enumerate() {
                    let outpoint = OutPoint::new(tx.txid(), vout_index as u32);
                    utxo_set.insert(outpoint);
                }
            } else {
                // For subsequent blocks, update the UTXO set
                // Remove spent outputs
                for input in tx.input.iter() {
                    utxo_set.remove(&input.previous_output);
                }
                // Add new outputs
                for (vout_index, _) in tx.output.iter().enumerate() {
                    let outpoint = OutPoint::new(tx.txid(), vout_index as u32);
                    utxo_set.insert(outpoint);
                }
            }
        }
```

### Note Rust specific syntax

```rust
/*Line 92 and 103*/ utxo_set.insert(outpoint); // outpoint is moved into the utxo_set (HashSet)
/*Line 98*/ utxo_set.remove(&input.previous_output); // utxo_set borrows the input.previous_output value
```

Why is that?
Think about the consequences of the design and why they have been done this way.
