use dns_lookup::lookup_host;
use std::{
    fs::File,
    io::{self, Error, ErrorKind, Write},
    net::{IpAddr, Ipv6Addr},
    str::FromStr,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use bitcoin::{
    consensus::{
         deserialize_partial,
        encode::{serialize_hex},
    },
    Block,
};
use bitcoin::hashes::{sha256d, Hash};
use rand::seq::SliceRandom;
use rand::thread_rng;
use tokio::time::{timeout, Duration};

const PROTOCOL_VERSION: i32 = 70015;

// Define the fields
struct NetAddress {
}

// Define the fields
struct VersionMessage {
}

const REGTEST_MAGIC: [u8; 4] = [0;4];// todo!(); 
const MAINNET_MAGIC: [u8; 4] = [0;4];// todo!(); 

fn serialize_net_address(addr: &NetAddress) -> Vec<u8> {
    todo!()
}

fn serialize_version_msg(msg: &VersionMessage) -> Vec<u8> {
    todo!()
}

fn hex_to_bytes(hex_string: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    // refer [hex to bytes conversion](https://github.com/bitcoin-dev-project/rust-for-bitcoiners/blob/main/tutorials/de_se_rialization/hex_bytes_conversions.md)
    todo!()
}

fn request_block_message(hash: &str) -> Vec<u8> {
    todo!("Given a block hash return the block message inventory for getdata command")
}

fn create_message(command: &str, payload: &[u8]) -> Vec<u8> {
    todo!("Given a command and payload prepare the message according to Message Structure section")
}

fn is_verack(data: &[u8]) -> bool {
    todo!("Check whether these bytes starts with a verack message")
}

fn randomize_slice<'a>(input: &'a [&'a str]) -> Vec<&'a str> {
    let mut rng = thread_rng();
    let mut vec = input.to_vec(); // Convert slice to vector
    vec.shuffle(&mut rng); // Shuffle the vector
    vec // Return the vector (or convert to a slice if needed)
}

async fn get_valid_ip() -> Result<(TcpStream, IpAddr), String> {
    const DNS_SEEDS: [&str; 4] = [
        "seed.bitcoin.sipa.be",
        "dnsseed.bluematt.me",
        "dnsseed.bitcoin.dashjr.org",
        "seed.bitcoinstats.com",
    ];
    todo!("Initially test with regtest with debug=net option");
    todo!("then test with your local full node");
    todo!("Then choose an ip from randomly iterating over DNS_SEEDS")
}

// bitcoin messages did not end with any special character
// So this function will keep reading from the stream until the read results in an error or 0
async fn till_read_succeeds(stream: &mut TcpStream, buffer: &mut Vec<u8>) {
    loop {
        let mut t = [0; 1024];
        if let Ok(n) = stream.read(&mut t).await {
            if n == 0 {
                return;
            }
            buffer.extend(t);
            tracing::info!("read {n} bytes");
        } else {
            tracing::error!("Error in read");
            return;
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let (mut stream, ip) = get_valid_ip().await.unwrap();

    let ip6 = match ip {
        IpAddr::V4(addr) => addr.to_ipv6_mapped(),
        IpAddr::V6(addr) => addr,
    };

    // Construct and send the version message
    let version_msg = todo!("prepare version message");


    Ok(())
}

// A bitcoin peer will be continuously sending you messages
// At some point you need to pause reading them and process the messages
// So this function reads till a specified timeout
async fn get_data_with_timeout(mut stream: &mut TcpStream, mut buffer: &mut Vec<u8>) {
    let timeout_duration = Duration::from_secs(10);
    let _ = timeout(
        timeout_duration,
        till_read_succeeds(&mut stream, &mut buffer),
    )
    .await;
}

fn get_block_payload(buffer: &[u8]) -> &[u8] {
    todo!("The bitcoin node will keep sending you messages like ping, inv etc.,");
    todo!("One of them will be your required block message");
    todo!("How will you identify that?")
}

fn starts_with_magic(buffer: &[u8]) -> bool {
    todo!("check whether the buffer strts with magic network characters")
}
