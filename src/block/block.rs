use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::utils::hash::calculate_hash;
use crate::utils::time::get_current_timestamp;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub validator: String
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize, validator: String) -> Block {
        let timestamp = get_current_timestamp();
        let transactions_serialized = serde_json::to_string(&transactions).unwrap();
        let hash = calculate_hash(index, &timestamp, &transactions_serialized, &previous_hash, &validator);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            validator,
        }
    }
}
