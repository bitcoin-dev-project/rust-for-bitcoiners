use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

fn connect_to_node(address: &str) -> Result<(), std::io::Error> {
    match TcpStream::connect(address) {
        Ok(_) => {
            println!("Successfully connected to node at {}", address);
            Ok(())
        },
        Err(e) => {
            println!("Failed to connect to node at {}: {}", address, e);
            Err(e)
        },
    }
}

fn main() {
    let addresses = vec![
        "127.0.0.1:8333", // Simulated Bitcoin node address
        "127.0.0.1:18333", // Another simulated Bitcoin node address
    ];

    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for address in addresses {
        let results_clone = Arc::clone(&results);
        let address_clone = address.to_owned();

        let handle = thread::spawn(move || {
            let result = connect_to_node(&address_clone);
            let mut results_locked = results_clone.lock().unwrap();
            results_locked.push((address_clone, result));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results_locked = results.lock().unwrap();
    for (address, result) in results_locked.iter() {
        match result {
            Ok(_) => println!("Connection to {} was successful.", address),
            Err(e) => println!("Connection to {} failed with error: {}", address, e),
        }
    }
}
