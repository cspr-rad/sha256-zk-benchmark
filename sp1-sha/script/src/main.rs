//! A simple script to generate and verify the proof of a given program.
use bincode;
use serde::{Deserialize, Serialize};
use sp1_sdk::{ProverClient, SP1Stdin};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
use chrono::{DateTime, TimeZone, Utc};

fn benchmark(rounds: u32){
    let mut stdin = SP1Stdin::new();
    stdin.write(&rounds);
    let client = ProverClient::new();
    let mut proof = client.prove(ELF, stdin).expect("Failed to generate proof!");
    println!(
        "Proof size: {:?}",
        &bincode::serialize(&proof).unwrap().len()
    );
    client.verify(ELF, &proof).expect("verification failed");
}


fn main() {
    let test_vectors: Vec<u32> = vec![1, 10, 100, 1000];
    for rounds in test_vectors{
        let start_time: i64 = Utc::now().timestamp();
        benchmark(rounds);
        let runtime = Utc::now().timestamp() - start_time;
        println!("[SP1 SHA256 Benchmark] Total runtime: {:?}, rounds of sha256: {:?}", runtime, rounds);
    }
}
