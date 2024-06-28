use std::convert::TryInto;

const BLOCK_SIZE: usize = 32; // Size of each block in bits
const HASH_SIZE: usize = 32; // Size of the hash code in bits

struct XorHasher {
    state: [u8; HASH_SIZE],
    block_count: usize,
}

impl XorHasher {
    fn new() -> Self {
        XorHasher {
            state: [0; HASH_SIZE],
            block_count: 0,
        }
    }

    fn update(&mut self, data: &[u8]) {
        let mut offset = 0;

        while offset < data.len() {
            let mut block = [0u8; BLOCK_SIZE];
            let remaining = data.len() - offset;
            let block_size = remaining.min(BLOCK_SIZE);

            block[..block_size].copy_from_slice(&data[offset..offset + block_size]);
            self.process_block(&block);

            offset += block_size;
            self.block_count += 1;
        }
    }

    fn finalize(self) -> [u8; HASH_SIZE] {
        self.state
    }

    fn process_block(&mut self, block: &[u8; BLOCK_SIZE]) {
        for i in 0..HASH_SIZE {
            // since we have HASH_SIZE == BLOCK_SIZE this is easy
            self.state[i] ^= block[i];
        }
    }
}

fn xor_hash(data: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = XorHasher::new();
    hasher.update(data);
    hasher.finalize()
}

fn xor_hash_attack(data: &[u8]) -> Vec<u8> {
    let mut padded_data = Vec::new();
    let r = BLOCK_SIZE - (data.len() % BLOCK_SIZE);

    if r != 0 {
        let padding = vec![0; r];
        padded_data.extend_from_slice(data);
        padded_data.extend(padding);
    }
    let mut mathcing_message = Vec::new();

    for _ in 1..=3 {
        mathcing_message.extend_from_slice(&padded_data);
    }
    mathcing_message
}

#[cfg(test)]
mod tests {

    use quickcheck::QuickCheck;

    use super::*;

    #[test]
    fn test_xor_attack() {
        fn prop(data: Vec<u8>) -> bool {
            xor_hash(&data) == xor_hash(&xor_hash_attack(&data))
        }
        QuickCheck::new().quickcheck(prop as fn(Vec<u8>) -> bool);
    }

    #[test]
    fn attack_demo() {
        let data = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0];
        println!("{}", data.len());
        let attack = xor_hash_attack(&data);
        println!("{:?}", attack.len());
        println!("{:?}", xor_hash(&data));
        println!("{:?}", xor_hash(&attack));
    }
}

fn main() {
    println!("Hello, world!");
}
