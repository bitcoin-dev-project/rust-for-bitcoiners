use std::collections::HashSet;

use bitcoin::{Address, Network, Transaction};
use bitcoincore_rpc::{Client, RpcApi};

use crate::utils::*;

pub fn txs_sort_unique_input_addrs(
    client: &Client,
    block_height: u64,
) -> Result<Vec<(Transaction, u32)>, bitcoincore_rpc::Error> {
    let block = get_block(block_height, client)?;
    let txs = block.txdata;

    let mut result: Vec<(Transaction, u32)> = vec![];
    for tx in txs.iter() {
        if tx.is_coinbase() {
            continue;
        }
        let mut input_addrs: HashSet<Address> = HashSet::new();

        for input in tx.input.iter() {
            if let Ok(prev_tx) = client.get_raw_transaction(&input.previous_output.txid, None) {
                let prev_tx_output = &prev_tx.output[input.previous_output.vout as usize];
                let prev_tx_script = &prev_tx_output.script_pubkey;
                if let Ok(input_address) = Address::from_script(prev_tx_script, Network::Bitcoin) {
                    input_addrs.insert(input_address);
                } else {
                    println!("`Address::from_script` only recognizes p2sh, p2pkh and their respective segwit variants");
                }
            } else {
                println!("Unknown transaction id of an input {}", &input.previous_output.txid);
            }
        }
        result.push((tx.clone(), input_addrs.len() as u32));
    }
    result.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(result)
}
