use rand::rngs::OsRng;
use bitcoin::secp256k1::{Secp256k1, Message};
use bitcoin::secp256k1::hashes::{sha256, Hash};

fn demo() {
    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    let digest = sha256::Hash::hash("Hello World!".as_bytes());
    let message = Message::from_digest(digest.to_byte_array());

    let sig = secp.sign_ecdsa(&message, &secret_key);
    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
}