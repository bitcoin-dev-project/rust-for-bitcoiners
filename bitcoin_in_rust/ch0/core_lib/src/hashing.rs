const BLOCK_SIZE: usize = 64;
const DIGEST_SIZE: usize = 32;

struct SimpleHash {
    state: [u32; 8],
    block: [u8; BLOCK_SIZE],
    block_len: usize,
    length: u64,
}

impl SimpleHash {
    fn new() -> Self {
        SimpleHash {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            block: [0; BLOCK_SIZE],
            block_len: 0,
            length: 0,
        }
    }

    fn update(&mut self, data: &[u8]) {
        let mut i = 0;
        while i < data.len() {
            let remaining = BLOCK_SIZE - self.block_len;
            let copy_len = std::cmp::min(data.len() - i, remaining);
            self.block[self.block_len..self.block_len + copy_len]
                .copy_from_slice(&data[i..i + copy_len]);
            self.block_len += copy_len;
            self.length += copy_len as u64;
            i += copy_len;

            if self.block_len == BLOCK_SIZE {
                self.process_block();
                self.block_len = 0;
            }
        }
    }

    fn finalize(mut self) -> [u8; DIGEST_SIZE] {
        self.block[self.block_len] = 0x80;
        self.block_len += 1;

        if self.block_len > BLOCK_SIZE - 8 {
            self.process_block();
            self.block = [0; BLOCK_SIZE];
        }

        let length_bits = self.length * 8;
        self.block[BLOCK_SIZE - 8..].copy_from_slice(&length_bits.to_be_bytes());
        self.process_block();

        let mut digest = [0; DIGEST_SIZE];
        for (i, chunk) in self.state.iter().enumerate() {
            digest[i * 4..(i + 1) * 4].copy_from_slice(&chunk.to_be_bytes());
        }
        digest
    }

    fn process_block(&mut self) {
        // Simplified block processing logic
        for i in 0..8 {
            self.state[i] = self.state[i].wrapping_add(self.block[i * 4] as u32);
        }
    }
}

pub fn hash(data: &[u8]) -> [u8; DIGEST_SIZE] {
    let mut hasher = SimpleHash::new();
    hasher.update(data);
    hasher.finalize()
}
