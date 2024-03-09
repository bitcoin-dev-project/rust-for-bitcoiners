use std::collections::HashSet;
use bitcoin::OutPoint;
use bitcoincore_rpc::Client;

use crate::utils::get_block;

pub fn build_utxo_set_10(client: &Client, start_height: u64) -> Result<HashSet<OutPoint>, bitcoincore_rpc::Error> {
    let mut utxo_set: HashSet<OutPoint> = HashSet::new();

    for height in start_height..start_height + 10 {
        let block = get_block(height, client)?;

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
    }

    Ok(utxo_set)
}
