// src/block/mining.rs
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use hex;
use serde_json;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::hash::calculate_hash;
use crate::utils::time::get_current_timestamp;

/// Mines a new block by finding a nonce that satisfies the difficulty requirement.
/// This function utilizes parallel processing to enhance mining performance.
///
/// # Arguments
///
/// * `index` - The index of the new block.
/// * `transactions` - A vector of transactions to include in the block.
/// * `previous_hash` - The hash of the previous block in the chain.
/// * `difficulty` - The current mining difficulty (number of leading zeros required in the hash).
///
/// # Returns
///
/// * `Block` - The newly mined block.
pub fn mine_block_parallel(index: u32, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Block {
    let timestamp = get_current_timestamp();
    let transactions_serialized = serde_json::to_string(&transactions).unwrap();
    let prefix = "0".repeat(difficulty);

    // Attempt different nonces in parallel
    let nonce = (0..u64::MAX).into_par_iter()
        .find_any(|&nonce| {
            let hash = calculate_hash(index, &timestamp, &transactions_serialized, &previous_hash, nonce);
            &hash[..difficulty] == prefix
        })
        .expect("Failed to find a valid nonce");

    let hash = calculate_hash(index, &timestamp, &transactions_serialized, &previous_hash, nonce);

    Block {
        index,
        timestamp,
        transactions,
        previous_hash,
        hash,
        nonce,
    }
}

/// Creates a new block without mining (useful for genesis block creation).
///
/// # Arguments
///
/// * `index` - The index of the new block.
/// * `transactions` - A vector of transactions to include in the block.
/// * `previous_hash` - The hash of the previous block in the chain.
/// * `difficulty` - The current mining difficulty.
///
/// # Returns
///
/// * `Block` - The newly created block.
pub fn create_new_block(index: u32, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Block {
    mine_block_parallel(index, transactions, previous_hash, difficulty)
}
