//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use bincode;
use serde::{Deserialize, Serialize};
use sha2_v0_10_8::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
struct Output {
    sha256_hashes: Vec<Vec<u8>>,
}
pub fn main() {
    let mut output_hashes: Vec<Vec<u8>> = Vec::new();
    let rounds = sp1_zkvm::io::read::<u32>();
    for i in 0..rounds {
        let sha_input = [1u8; 32];
        let mut hasher = Sha256::new();
        hasher.update(&sha_input);
        output_hashes.push(hasher.finalize().to_vec());
    };
    let output: Output = Output {
        sha256_hashes: output_hashes,
    };
    sp1_zkvm::io::commit(&output);
}
