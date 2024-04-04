use bitcoincore_rpc::{Client, RpcApi};
pub fn get_block(
    block_height: u64,
    client: &Client,
) -> Result<bitcoin::Block, bitcoincore_rpc::Error> {
    // Get the block hash of the block height
    let block_hash = client.get_block_hash(block_height)?;
    // Return the block data for the respective hash
    client.get_block(&block_hash)
}
