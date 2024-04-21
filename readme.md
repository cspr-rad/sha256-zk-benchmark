# Sha256 benchmarks in SP1, Risc0 and Risc0 groth 16

This repository includes different crates for computing sha256 hashs in Risc0 and SP1 execution environments.

The focus of the benchmarks is primarily on proof size.

# SP1 circuit

```rust
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
```

sha256 library used: `sha2-v0-10-8` = { git = "https://github.com/sp1-patches/RustCrypto-hashes", package = "sha2", branch = "patch-v0.10.8" }

See [here](https://github.com/sp1-patches/RustCrypto-hashes)

# Risc0 circuit

```rust
    // read the input
    let rounds: u32 = env::read();
    let mut output_hashes: Vec<Digest> = Vec::new();
    // test sha256
    for i in 0..rounds {
        let sha_input = [1u8; 32];
        let sha_hash: Digest = *Impl::hash_bytes(&sha_input);
        output_hashes.push(sha_hash);
    }
    let output: Output = Output {
        sha256_hashes: output_hashes,
    };
    env::commit(&output);
```

sha256 library used: `use risc0_zkvm::sha::{Digest, Impl, Sha256};`

See [here](https://crates.io/crates/risc0-zkvm)

# SP1 proof size benchmark 

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| 1.44 MB | 1.52 MB | 2.81 MB | x |

# SP1 proof speed benchmark
| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| 1s | 4s | 27s | x |

# Risc0 proof size benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| 215.6 KB | 216.2 KB | 256.7 KB | x |

# Risc0 proof speed benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| 19s | 19s | 78s | x |

# Risc0-Groth16 proof size benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| 485 B | 773 B |  3653 B | x |

# Risc0-Groth16 proof speed benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| 223s | 223s | 286s | x |


Disclaimer: I had to run the SP1 benchmarks on my Macbook, on Ubuntu I encountered this error:

Machine used for SP1: `M2 Macbook Air, 8GB Ram`.

The reason why a different machine was used for the SP1 benchmarks is [this issue](https://github.com/jdx/mise/issues/1630) related to `curve25519-dalek v4.1.2`.

The machine used for Risc0 and Risc0-groth16 was a 32GB Ram, 8-core x86-64 Ubuntu server.
