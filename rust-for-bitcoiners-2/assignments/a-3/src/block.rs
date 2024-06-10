#![allow(unused)]

use std::collections::LinkedList as List;

struct BlockChain {
    blocks: List<Block>
}

struct Block {
    hash: String,
    id: u128,
    transactions: List<Transaction>,
}

struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
}

struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String, // to spend the output
}

struct TxOut {
    public_address: String,
    satoshis: u64, 
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests
