#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction {
    txid: String,
    version: i32,
    locktime: u32,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    size: u32,
    weight: u64,
    fee: u64,
    status: Status,
    hex: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Vin {
    txid: String,
    vout: u32,
    prevout: Vout,
    scriptsig: String,
    scriptsig_asm: String,
    witness: Vec<String>,
    is_coinbase: bool,
    sequence: u64,
}

#[derive(Debug, Deserialize)]
struct Vout {
    scriptpubkey: String,
    scriptpubkey_type: String,
    scriptpubkey_address: String,
    value: u64,
}

#[derive(Debug, Deserialize)]
struct Status {
    confirmed: bool,
    block_height: u32,
    block_hash: String,
    block_time: u64,
}

fn main() {
  // r# string annotations are really handy to work with json literals
  let tx_str = r#"{
      "txid": "00000a2d1a9e29116b539b85b6e893213b1ed95a08b7526a8d59a4b088fc6571",
      "version": 1,
      "locktime": 0,
      "vin": [
        {
          "txid": "2e4843d552ca9487efd9e69c0359f05375b7de5449eb49510d17a25bb5b15ec0",
          "vout": 1,
          "prevout": {
            "scriptpubkey": "512065fd3d423ea46a70505248db989e7302bfbbdd64ee4193dd9a59f69894f0de48",
            "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 65fd3d423ea46a70505248db989e7302bfbbdd64ee4193dd9a59f69894f0de48",
            "scriptpubkey_type": "v1_p2tr",
            "scriptpubkey_address": "bc1pvh7n6s375348q5zjfrde38nnq2lmhhtyaeqe8hv6t8mf398smeyqnug47s",
            "value": 13413
          },
          "scriptsig": "",
          "scriptsig_asm": "",
          "witness": [
            "29783b151d376d5178451ce14f62b091059021680bff36aec2814e33ecacf130e8aa92d6da23f35be7a8c2245b8f910261d4e6a5169f79d6ff7a3f412981f486"
          ],
          "is_coinbase": false,
          "sequence": 1610616404
        }
      ],
      "vout": [
        {
          "scriptpubkey": "51204b918d31f22461021ed54e354ac9dcbbe94b98edcfd3615b76c068b08222a87f",
          "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 4b918d31f22461021ed54e354ac9dcbbe94b98edcfd3615b76c068b08222a87f",
          "scriptpubkey_type": "v1_p2tr",
          "scriptpubkey_address": "bc1pfwgc6v0jy3ssy8k4fc654jwuh055hx8delfkzkmkcp5tpq3z4pls7tx8q3",
          "value": 2908
        },
        {
          "scriptpubkey": "512065fd3d423ea46a70505248db989e7302bfbbdd64ee4193dd9a59f69894f0de48",
          "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 65fd3d423ea46a70505248db989e7302bfbbdd64ee4193dd9a59f69894f0de48",
          "scriptpubkey_type": "v1_p2tr",
          "scriptpubkey_address": "bc1pvh7n6s375348q5zjfrde38nnq2lmhhtyaeqe8hv6t8mf398smeyqnug47s",
          "value": 8503
        }
      ],
      "size": 205,
      "weight": 616,
      "fee": 2002,
      "status": {
        "confirmed": true,
        "block_height": 834552,
        "block_hash": "00000000000000000001dd0468a70c94f619251d286585cff57aeb4bd9ede330",
        "block_time": 1710355598
      },
      "hex": "01000000000101c05eb1b55ba2170d5149eb4954deb77553f059039ce6d9ef8794ca52d543482e0100000000540e0060025c0b0000000000002251204b918d31f22461021ed54e354ac9dcbbe94b98edcfd3615b76c068b08222a87f372100000000000022512065fd3d423ea46a70505248db989e7302bfbbdd64ee4193dd9a59f69894f0de48014029783b151d376d5178451ce14f62b091059021680bff36aec2814e33ecacf130e8aa92d6da23f35be7a8c2245b8f910261d4e6a5169f79d6ff7a3f412981f48600000000"
    }
  "#;

  let tx: serde_json::error::Result<Transaction> = serde_json::from_str(tx_str);

  assert!(tx.is_ok());
}
