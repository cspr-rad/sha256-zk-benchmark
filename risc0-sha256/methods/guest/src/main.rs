#![no_main]
#![no_std]
// If you want to try std support, also update the guest Cargo.toml file
use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Digest, Impl, Sha256};
extern crate alloc;
use alloc::vec::Vec;
risc0_zkvm::guest::entry!(main);
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
struct Output {
    sha256_hashes: Vec<Vec<u8>>,
}

fn main() {
    // read the input
    let rounds: u32 = env::read();
    let mut output_hashes: Vec<Vec<u8>> = Vec::new();
    // test sha256
    for i in 0..rounds {
        let sha_input = [1u8; 32];
        let hasher = Sha256::new();
        hasher.update(&sha_input);
        output_hashes.push(hasher.to_vec());
    }
    let output: Output = Output {
        sha256_hashes: output_hashes,
    };
    env::commit(&output);
}