# Privacy concerns of a Transaction and it's implication in Coinselection

Typically when a bitcoin transaction is created using bitcoin client or a wallet
for a single user one can conclude that all the inputs in the transaction belong
to a single user. We might not able to deduce who the user is but it's clear that
all the addresses in that partucular transaction inputs belong to a single user.

## Counting number of unique addresses in a transaction using rust

In our analysis we are going to fetch a block of certain height and analyze the transactions
within it.

The following code snippet demonstrates how to do this using [rust-bitcoincore_rpc crate](https://crates.io/crates/bitcoincore-rpc).

```rust
fn get_block(client: &Client, block_height: u64) -> Result<bitcoin::Block, bitcoincore_rpc::Error> {
    let block = client.get_block_hash(block_height)?; // fetch the hash of the block
    client.get_block(&block) // return the complete block with another rpc call
}
```

The following is the function that we want to complete

```rust
fn txs_sort_unique_input_addrs(client: &Client, block_height: u64) -> Result<Vec<(Transaction, u32)>, bitcoincore_rpc::Error> {

    // Fetching the basic data necessary and intializing the variables
    let block = get_block(client, block_height)?;
    let txs = block.txdata;
    
    let mut result: Vec<(Transaction, u32)> = vec![];
}
```

We need to keep track of unique addresses, the datastructure that helps us achieve that goal is a
Hashset.
Now we iterate through every transaction and compute the unique addresses used in each of them, using
HashSet.

```rust
for tx in txs.iter() {
    let mut input_addrs: HashSet<Address> = HashSet::new();

    // For coinbase transactions the input Vec will be empty so it will be safely ignored.

    // Iterate through every input
    for input in tx.input.iter() {
        // Do more processing to extract address from the input
        let address = extract_from_TxIn(input);
        input_addrs.insert(address);
    }
    result.push((tx.clone(), input_addrs.len() as u32));
}
```

The ```extract_from_TxIn``` function looks like this,

```rust
fn extract_from_TxIn(input: TxIn) -> bitcoin::Address {
    // TODO

}

// In bitcoin crate TxIn is defined like the following
struct TxIn {
    pub previous_output: OutPoint,
    pub script_sig: ScriptBuf, // It is not possible to extract address directly from here using current implementations
    pub sequence: Sequence,
    pub witness: Witness,
}

// OutPoint is defined as follows
struct OutPoint {
    /// The referenced transaction's txid.
    pub txid: Txid,
    /// The index of the referenced output in its transaction's vout.
    pub vout: u32,
}
```

So in order to extract address from the transaction input we have to get information
about the transaction where the input was the output (TxOut).
The following code demonstrates how to do it,

```rust
let prev_tx: Transaction = client.get_raw_transaction(&input.previous_output.txid, Some(&block_hash)).unwrap();
let prev_tx_output = &prev_tx.output[input.previous_output.vout as usize];

// TxOut Struct is defined like the following
struct TxOut {
    /// The value of the output, in satoshis.
    pub value: Amount,
    /// The script which must be satisfied for the output to be spent.
    pub script_pubkey: ScriptBuf, // We can extract address from here
}
```

From the script_pubkey of the previous output it is possible to extract the address of the current
transaction input using the following,

```rust
let prev_tx_script = &prev_tx_output.script_pubkey;
let input_address = bitcoin::Address::from_script(prev_tx_script, Network::Bitcoin).expect("Failed to parse address from scriptPubKey");
```

Putting it all together the entire code can be found [here](../blockchain_analysis/src/privacy.rs)
