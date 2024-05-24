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
        let mut block = [0u8; BLOCK_SIZE];
        let mut offset = 0;

        while offset < data.len() {
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
