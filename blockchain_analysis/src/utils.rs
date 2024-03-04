use bitcoincore_rpc::{Client, RpcApi};
pub fn get_block(
    block_height: u64,
    client: &Client,
) -> Result<bitcoin::Block, bitcoincore_rpc::Error> {
    let block = client.get_block_hash(block_height)?;
    client.get_block(&block)
}
