use bip39::Mnemonic;
use rand::{thread_rng, RngCore};
use secp256k1::Secp256k1;
use bitcoin::{Address, PrivateKey};

fn main() {
    generate_wallet();
}

fn generate_wallet() {
    // Generate a new mnemonic
     // Generate 128 bits of entropy
    let mut entropy = [0u8; 32];
    thread_rng().fill_bytes(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&mut entropy).unwrap();
    println!("Mnemonic Phrase: {}", mnemonic);

    // Generate a seed from the mnemonic
    let seed = mnemonic.to_seed(""); // with empty passphrase
    println!("Seed: {:?}", seed);
    // ATTENTION: Using seed to generate SecretKey doesn't work because seed is a array of size 64
    // SecretKey::from_slice(&seed) doesn't work because it expects a slice of size 32
    // Need to know what is the purpose of mnemonic.to_seed("")

    // Generate priv key from the seed
    let secp_priv_key = secp256k1::SecretKey::from_slice(&entropy).unwrap();
    let priv_key = PrivateKey::new(secp_priv_key, bitcoin::Network::Bitcoin); // generates compressed priv key

    // Generate pub key from the priv key

    // Generate segwit address from the pub key
    let secp = Secp256k1::new();
    let pub_key = priv_key.public_key(&secp);

    let segwit_address = Address::p2wpkh(&pub_key, bitcoin::Network::Bitcoin).unwrap();
    println!("Address: {}", segwit_address);
}
