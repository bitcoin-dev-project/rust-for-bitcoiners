use bitcoincore_rpc::{Auth, Client};
use std::{env, path::PathBuf, str::FromStr};

mod compute_transaction_fee;
mod privacy;
mod utils;
mod build_utxo_set;
mod profiling_transactions;

use privacy::txs_sort_unique_input_addrs;

use bitcoincore_rpc::jsonrpc::client;

fn main() {
    // Load your local environment variables to access the bitcoin node via rpc
    dotenv::dotenv().ok();
    let rpc_url = env::var("BITCOIN_RPC_UR").expect("BITCOIN_RPC_URL must be set");
    let rpc_user = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER must be set");
    let rpc_password = env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD must be set");

    let rpc_client = Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password)).unwrap();

    let cookie_auth = Auth::CookieFile(PathBuf::from_str("string").unwrap());

    // track_utxo_set_10(&rpc_client, 831200).unwrap();
    let res = txs_sort_unique_input_addrs(&rpc_client, 144008).unwrap();

    // Displaying the top 10 transactions with greatest input addresses
    for i in 0..10 {
        let r = &res[i];
        println!("Transaction = {}, uses {} addresses", r.0.txid(), r.1);
    }
}
/*
Given a block how do we find the transaction with the maximum fee?
 */
