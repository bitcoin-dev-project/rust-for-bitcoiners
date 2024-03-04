# Observing the fee trends using Rust

Fees in bitcoin is a hot topic now and will continue to grow in future given the fact
that fees will be the largest revenue factor for a miner than a block reward.

## Problem statement

Given a block find the transaction with the largest fee.

```rust
struct TxId(String);
// Return type is Option<_>  because some blocks have a single transaction
// which is the coinbase transaction
fn high_fee_transaction(block: Block) -> Option<TxId> {
    unimplemented!("TODO")
}
```

The ```Block``` struct has txdata field which is just a ```Vec<Transaction>```, so if
it's length is *0* then we know that we have to return None as it carries no fee. 

## Computing the Fee of a Transaction

In order to compute the fee of a transaction we need to do the following.

1. Find the total value of the inputs
1. Find the total value of the outputs
1. The difference between them is the fee.

### Computing the Total value of outputs

This step is much easier because the ```TxOut``` type has the value field which
gives the ```Amount``` of that output. So the following would suffice,

```rust
let total_output_value: u64 = tx.output.iter().map(|txout| txout.value.to_sat()).sum();
```
Read [here](https://docs.rs/bitcoin/latest/bitcoin/amount/struct.Amount.html) to look at how
to use the Amount type.


### Computing the Total value of inputs

This is a bit involved as the ```TxIn``` type has reference to the previous ```OutPoint```
which has the *txid* and *vout* of the previous transaction, *vout* refers to the index of
the output.

So to indentify it's value we have to make request with bitcoinrpc to fetch ```TxOut``` data
of that previous transaction.

```rust
let total_input_value: u64 = tx.input.iter().map(|txin| {
    let previous_outpoint = txin.previous_output;
    let previous_transaction: GetRawTransactionResult = client
        // We don't know the block_hash of the txin
        .get_raw_transaction_info(&previous_outpoint.txid, None)
        .expect("The bitcoin node does not support transaction indexing or the given block has invalid data");
    let previous_output: &GetRawTransactionResultVout =
        &previous_transaction.vout[previous_outpoint.vout as usize];
    previous_output.value.to_sat() // input value of this txin
}).sum();
```

putting it all together the entire code of this tutorial can be found [here](../blockchain_analysis/src/compute_transaction_fee.rs)
