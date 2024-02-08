extern crate sha2;
extern crate sha3;

pub trait HashFunction {
    type Output;

    fn hash(data: &[u8]) -> Self::Output;
}

use sha2::{Sha256, Digest as Sha2Digest};
use sha3::{Sha3_256, Digest as Sha3Digest};

pub struct SHA256;

impl HashFunction for SHA256 {
    type Output = [u8; 32];

    fn hash(data: &[u8]) -> Self::Output {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

pub struct SHA3_256;

impl HashFunction for SHA3_256 {
    type Output = [u8; 32];

    fn hash(data: &[u8]) -> Self::Output {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        hasher.finalize().into()
    }
}

// Generics
fn hash_data<H: HashFunction>(data: &[u8]) -> H::Output {
    H::hash(data)
}

