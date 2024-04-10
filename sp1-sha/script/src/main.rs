//! A simple script to generate and verify the proof of a given program.
use bincode;
use serde::{Deserialize, Serialize};
use sp1_sdk::{SP1Prover, SP1Stdin, SP1Verifier};
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
use sp1_types::Output;

fn main() {
    // Generate proof.
    let mut stdin = SP1Stdin::new();
    let n = 186u32;
    stdin.write(&n);
    let mut proof = SP1Prover::prove(ELF, stdin).expect("proving failed");

    // Read output.
    // let journal = proof.public_values.read::<Output>();
    // println!("Public journal: {:?}", &journal);

    println!(
        "Proof size: {:?}",
        &bincode::serialize(&proof).unwrap().len()
    );

    // Verify proof.
    SP1Verifier::verify(ELF, &proof).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
