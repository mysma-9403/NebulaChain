use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::utils::hash::calculate_hash;
use crate::utils::time::get_current_timestamp;
use crate::block::mining::{mine_block_parallel, create_new_block};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Mines a new block with the given parameters.
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
    /// * `Block` - The newly mined block.
    pub fn mine_block_parallel(index: u32, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Block {
        mine_block_parallel(index, transactions, previous_hash, difficulty)
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
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Block {
        create_new_block(index, transactions, previous_hash, difficulty)
    }
}
