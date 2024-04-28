#![allow(unused)]

use std::collections::LinkedList as List;

struct Block {
    hash: String,
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
}

struct TxOut {
    public_address: String,
    satoshis: u64, // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

struct BlockChain {
    blocks: List<Block>,
}
