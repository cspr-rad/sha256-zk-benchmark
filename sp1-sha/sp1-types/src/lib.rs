use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub sha256_hashes: Vec<Vec<u8>>,
}