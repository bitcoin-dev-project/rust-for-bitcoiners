extern crate serde;
extern crate serde_json;
extern crate bincode;

use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct BitcoinTransaction {
    txid: String,
    version: i32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    previous_output: String,
    script_sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    value: u64, // Satoshis
    script_pubkey: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let transaction = BitcoinTransaction {
        txid: "sample_txid".into(),
        version: 1,
        inputs: vec![
            Input {
                previous_output: "sample_previous_txid".into(),
                script_sig: "sample_script_sig".into(),
            },
        ],
        outputs: vec![
            Output {
                value: 1000, // Example value in Satoshis
                script_pubkey: "sample_script_pubkey".into(),
            },
        ],
    };

    // Serialize to JSON
    let json = serde_json::to_string(&transaction)?;
    println!("JSON serialized transaction: {}", json);

    // Deserialize from JSON
    let deserialized: BitcoinTransaction = serde_json::from_str(&json)?;
    println!("Deserialized transaction: {:?}", deserialized);

    // Optionally, serialize to binary format using bincode
    let binary = bincode::serialize(&transaction)?;
    println!("Binary serialized transaction: {:?}", binary);

    // And deserialize from binary
    let binary_deserialized: BitcoinTransaction = bincode::deserialize(&binary)?;
    println!("Binary deserialized transaction: {:?}", binary_deserialized);

    Ok(())
}
