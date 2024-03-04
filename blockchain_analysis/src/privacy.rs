use std::collections::HashSet;

use bitcoin::{Address, Network, Transaction};
use bitcoincore_rpc::{Client, RpcApi};

use crate::utils::*;

pub fn txs_sort_unique_input_addrs(
    client: &Client,
    block_height: u64,
) -> Result<Vec<(Transaction, u32)>, bitcoincore_rpc::Error> {
    let block_hash = client.get_block_hash(block_height)?;
    println!("block hash {}", block_hash);
    let block = get_block(block_height, client)?;
    let txs = block.txdata;

    let mut result: Vec<(Transaction, u32)> = vec![];
    for tx in txs.iter() {
        let mut input_addrs: HashSet<Address> = HashSet::new();
        for input in tx.input.iter() {
            let prev_tx =
                client.get_raw_transaction(&input.previous_output.txid, Some(&block_hash))?;
            let prev_tx_output = &prev_tx.output[input.previous_output.vout as usize];
            let prev_tx_script = &prev_tx_output.script_pubkey;
            let input_address = Address::from_script(prev_tx_script, Network::Bitcoin)
                .expect("Failed to parse address from scriptPubKey");
            input_addrs.insert(input_address);
        }
        result.push((tx.clone(), input_addrs.len() as u32));
    }
    result.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(result)
}
