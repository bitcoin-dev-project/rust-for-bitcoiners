use bitcoincore_rpc::{bitcoin::taproot::NodeInfo, Auth, Client, RpcApi};

fn main() {
    let client = Client::new("127.0.0.1:18443", Auth::UserPass("alice".to_string(), "password".to_string())).unwrap();

    // let wallet = client.create_wallet("wallet", None, None, None, None).unwrap();
    // println!("{:?}", wallet);
    let addr = client.get_new_address(Some("mining_reward"), None).unwrap();   
    let addr = addr.assume_checked();

    client.generate_to_address(100, &addr).unwrap();

    let utxos = client.list_unspent(None, None, None,  None, None).unwrap();
    println!("{:?}", utxos);
}