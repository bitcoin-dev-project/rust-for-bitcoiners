use bitcoin::TxIn;
use bitcoincore_rpc::{Client, RpcApi};
pub fn get_block(client: &Client, block_height: u64) -> Result<bitcoin::Block, bitcoincore_rpc::Error> {
    let block = client.get_block_hash(block_height)?;
    client.get_block(&block)
}

pub fn get_input_addr(client: &Client, txin: &TxIn) {
    let previous_output = txin.previous_output;
    let previous_transaction = client.get_transaction(&previous_output.txid, None);
}