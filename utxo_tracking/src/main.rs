use bitcoincore_rpc::{RpcApi, Client, Auth};
use bitcoin::blockdata::transaction::OutPoint;
use std::{collections::HashSet, env};

fn get_block(client: &Client, block_height: u64) -> Result<bitcoin::Block, bitcoincore_rpc::Error> {
    let block = client.get_block_hash(block_height)?;
    client.get_block(&block)
}

fn track_utxo_set_10(client: &Client, start_height: u64) -> Result<(), bitcoincore_rpc::Error> {
    let mut utxos: HashSet<OutPoint> = HashSet::new();

    for height in start_height..start_height + 10 {
        let block = get_block(client, height)?;

        for tx in block.txdata {
            // For the first block, add all UTXOs
            if height == start_height {
                for (vout_index, _) in tx.output.iter().enumerate() {
                    let outpoint = OutPoint::new(tx.txid(), vout_index as u32);
                    utxos.insert(outpoint);
                }
            } else {
                // For subsequent blocks, update the UTXO set
                // Remove spent outputs
                for input in tx.input.iter() {
                    utxos.remove(&input.previous_output);
                }
                // Add new outputs
                for (vout_index, _) in tx.output.iter().enumerate() {
                    let outpoint = OutPoint::new(tx.txid(), vout_index as u32);
                    utxos.insert(outpoint);
                }
            }
        }
    }

    dbg!("{}", utxos);

    Ok(())
}

fn main() {
    // Load your local environment variables to access the bitcoin node via rpc
    dotenv::dotenv().ok();
    let rpc_url = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL must be set");
    let rpc_user = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_password = env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let rpc_client = Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap();

    track_utxo_set_10(&rpc_client, 831200).unwrap();
}
