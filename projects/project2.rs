use std::fs::{self, File};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

enum State {
    Initiated,
    Confirmed,
}

struct Transaction {
    from: String,
    to: String,
    amount: f64,
    state: State,
}

struct Ledger {
    transactions: Vec<Transaction>,
}

impl Ledger {
    fn new() -> Ledger {
        Ledger { transactions: Vec::new() }
    }

    fn add_transaction(&mut self, from: String, to: String, amount: f64) {
        self.transactions.push(Transaction { from, to, amount });
    }

    fn save_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        for transaction in &self.transactions {
            writeln!(file, "{},{},{}", transaction.from, transaction.to, transaction.amount)?;
        }
        Ok(())
    }

    fn load_from_file(path: &str) -> io::Result<Ledger> {
        let file = File::open(Path::new(path))?;
        let reader = BufReader::new(file);
        let mut ledger = Ledger::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let from = parts[0].to_string();
                let to = parts[1].to_string();
                let amount: f64 = parts[2].parse().expect("Invalid amount");
                ledger.add_transaction(from, to, amount);
            }
        }
        Ok(ledger)
    }
}

fn main() -> io::Result<()> {
    let mut ledger = Ledger::new();
    ledger.add_transaction("Alice".to_string(), "Bob".to_string(), 50.0);

    ledger.save_to_file("ledger.csv")?;

    let loaded_ledger = Ledger::load_from_file("ledger.csv")?;
    println!("Transactions in Ledger:");
    for transaction in loaded_ledger.transactions {
        println!("{} -> {}: {}", transaction.from, transaction.to, transaction.amount);
    }

    Ok(())
}
