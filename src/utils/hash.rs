use sha2::{Digest, Sha256};
use hex;

pub fn calculate_hash(index: u32, timestamp: &str, transactions: &str, previous_hash: &str, validator: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(timestamp);
    hasher.update(transactions);
    hasher.update(previous_hash);
    hasher.update(validator);
    let result = hasher.finalize();
    hex::encode(result)
}