enum Network {
    Mainnet = 5, // 5
    Testnet, // 6
    Signet, // 7
    Regtest,
}
// isize is the default integer type for discriminant

fn get_network_port(network: Network) -> u16 {
    match network {
        Network::Mainnet => 8333,
        Network::Testnet => 18333,
        Network::Signet => 38333,
        Network::Regtest => 18444,
    }
}

type Hash = [u8; 256]; // A type synonym

enum Address {
    P2pksh { hash: Hash, network: Network }, // 0
    P2sh { hash: Hash, network: Network }, // 1
    // ... other address
}

enum Example {
    Var1, // size = 0 
    Integer(i64). // size = 64
    Person(i32, i64, i128), // size = 224, actual size = 256
    // discriminant = i32, max_size = 224, total = 256 = 32 bytes
}

fn get_script_pubkey(address: Address) -> Vec<u8> {
    match address {
        Address::P2pksh {hash, network} => todo!(),
        Address::P2sh {hash, network} => todo!(),
    }
}

/**

fn (a: A) {
    a.do_something(); // where it exists?
}

**/

fn main() {
    println!("Hello, world!");
}
