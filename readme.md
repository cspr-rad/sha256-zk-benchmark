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
```

sha256 library used: `use risc0_zkvm::sha::{Digest, Impl, Sha256};`

See [here](https://crates.io/crates/risc0-zkvm)

# SP1 proof size benchmark 

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| x | x | x | x |

# SP1 proof speed benchmark
| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| x | x | x | x |

# Risc0 proof size benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| x | x | x | x |

# Risc0 proof speed benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| x | x | x | x |

# Risc0-Groth16 proof size benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| x | x | x | x |

# Risc0-Groth16 proof speed benchmark

| 1 hash | 10 hashs | 100 hashs | 1000 hashs |
| --- | --- | --- | --- |
| x | x | x | x |

