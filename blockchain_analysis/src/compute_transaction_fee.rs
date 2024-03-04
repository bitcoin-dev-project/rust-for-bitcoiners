use bitcoin::Block;
use bitcoincore_rpc::{json::{GetRawTransactionResult, GetRawTransactionResultVout}, Client, RpcApi};

struct TxId(String);
fn high_fee_transaction(block: Block, client: &Client) -> Option<TxId> {
    let txs = block.txdata;

    if txs.len() == 1 {
        return None; // Only coinbase transaction
    }

    let mut max_fee = 0;
    let mut max_fee_txid = "".to_owned();

    for tx in txs {
        // Find the total value of outputs
        let total_output_value: u64 = tx.output.iter().map(|txout| txout.value.to_sat()).sum();
        // Find the total value of inputs
        let total_input_value: u64 = tx.input.iter().map(|txin| {
            let previous_outpoint = txin.previous_output;
            let previous_transaction: GetRawTransactionResult = client
                .get_raw_transaction_info(&previous_outpoint.txid, None)
                .expect("The bitcoin node does not support transaction indexing or the given block has invalid data");
            let previous_output: &GetRawTransactionResultVout = &previous_transaction.vout[previous_outpoint.vout as usize];
            previous_output.value.to_sat() // input value of this txin
        }).sum();

        let fee = total_input_value - total_output_value;
        
        if fee > max_fee {
            max_fee = fee;
            max_fee_txid = tx.txid().to_string();
        }
    }
    Some(TxId(max_fee_txid))
}
