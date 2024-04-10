//! A simple script to generate and verify the proof of a given program.
use bincode;
use serde::{Deserialize, Serialize};
use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
use sp1_types::Output;

use chrono::{DateTime, TimeZone, Utc};

fn benchmark(rounds: u32){
    let mut stdin = SP1Stdin::new();
    stdin.write(&rounds);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");
    println!(
        "Proof size: {:?}",
        &bincode::serialize(&proof).unwrap().len()
    );

    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    /*proof
        .save("proof-with-io.json")
        .expect("saving proof failed");
    */
}


fn main() {
    let test_vectors: Vec<u32> = vec![10, 100, 1000];
    for rounds in test_vectors{
        let start_time: i64 = Utc::now().timestamp();
        benchmark(rounds);
        let runtime = Utc::now().timestamp() - start_time;
        println!("[SP1 Benchmark] Total runtime: {:?}, rounds of sha256: {:?}", runtime, rounds);
    }
}
