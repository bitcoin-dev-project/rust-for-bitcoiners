use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: caesar_cipher <message> <shift>");
        return;
    }

    let message = &args[1];
    let shift: i32 = args[2].parse().expect("Please provide a valid number for shift");

    let encrypted_message = caesar_cipher(message, shift);
    println!("Encrypted Message: {}", encrypted_message);
}

fn caesar_cipher(message: &str, shift: i32) -> String {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut encrypted_message = String::new();

    for c in message.chars() {
        if c.is_alphabetic() {
            let is_uppercase = c.is_uppercase();
            let mut index = ALPHABET.find(c.to_ascii_lowercase()).unwrap() as i32;
            index = (index + shift) % 26;
            if index < 0 {
                index += 26;
            }
            let shifted_char = match ALPHABET.chars().nth(index as usize) {
                Some(c) => if is_uppercase { c.to_ascii_uppercase() } else { c },
                None => c,
            };
            encrypted_message.push(shifted_char);
        } else {
            encrypted_message.push(c);
        }
    }

    encrypted_message
}
