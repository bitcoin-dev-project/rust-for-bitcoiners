use bitcoincore_rpc::{Auth, Client, RpcApi};

fn main() {
    let client = Client::new("127.0.0.1:18443", Auth::UserPass("alice".to_string(), "password".to_string())).unwrap();

    // match  client.create_wallet("trader1", None, None, None, None) {
    //     Ok(wallet) => println!("{:?}", wallet),
    //     Err(e) => println!("{}", e),
    // }


    let client = Client::new("127.0.0.1:18443/wallet/trader1", Auth::UserPass("alice".to_string(), "password".to_string())).unwrap();
    let addr = client.get_new_address(Some("mining_reward"), None).unwrap();   
    let addr = addr.assume_checked();
    println!("{addr}");

    // client.generate_to_address(100, &addr).unwrap();  // regtest specific

    // let utxos = client.list_unspent(None, None, None,  None, None).unwrap();
    // println!("{:?}", utxos);
}