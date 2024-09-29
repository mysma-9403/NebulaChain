use sha2::{Digest, Sha256};
use hex;

pub fn calculate_hash(index: u32, timestamp: &str, transactions_serialized: &str, previous_hash: &str, nonce: u64) -> String {
    let input = format!("{}{}{}{}{}", index, timestamp, transactions_serialized, previous_hash, nonce);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}