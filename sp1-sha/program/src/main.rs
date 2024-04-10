//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use bincode;
use sha2_v0_10_8::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use sp1_types::Output;

pub fn main() {
    let mut output_hashes: Vec<Vec<u8>> = Vec::new();
    for i in 0..1000{
        let sha_input = [1u8;32];
        output_hashes.push(Sha256::digest(&sha_input).to_vec());
    };
    let output: Output = Output{
        sha256_hashes: output_hashes
    };
    sp1_zkvm::io::commit(&output);
}
