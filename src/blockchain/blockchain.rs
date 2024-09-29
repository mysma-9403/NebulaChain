// src/blockchain/blockchain.rs
use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::time::get_current_timestamp;
use crate::utils::hash::calculate_hash;
use chrono::prelude::*;
use crate::blockchain::difficulty::adjust_difficulty;
use crate::blockchain::validation::is_chain_valid;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub difficulty: usize,
    pub adjustment_interval: usize,
    pub target_block_time: u64,
}

impl Blockchain {
    /// Initializes a new blockchain with a genesis block.
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: vec![],
            mempool: vec![],
            difficulty: 4,
            adjustment_interval: 10,
            target_block_time: 10,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    /// Creates and adds the genesis block to the blockchain.
    fn create_genesis_block(&mut self) {
        let genesis_transaction = Transaction::new("0".to_string(), "0".to_string(), 0.0);
        let genesis_block = Block::new(0, vec![genesis_transaction], "0".to_string(), self.difficulty);
        self.chain.push(genesis_block);
    }

    /// Adds a new transaction to the mempool.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.push(transaction);
    }

    /// Mines a new block with the current mempool transactions.
    pub fn mine_block(&mut self) {
        if self.mempool.is_empty() {
            println!("No transactions to add to the block.");
            return;
        }

        let last_block = self.chain.last().unwrap();
        let new_block = Block::mine_block_parallel(
            last_block.index + 1,
            self.mempool.clone(),
            last_block.hash.clone(),
            self.difficulty,
        );

        println!("Block has been mined and added to the chain.");

        // Adjust difficulty if necessary
        if new_block.index % self.adjustment_interval as u32 == 0 {
            self.difficulty = adjust_difficulty(&self.chain, self.adjustment_interval, self.target_block_time, self.difficulty);
        }

        self.chain.push(new_block);
        self.mempool.clear();
    }

    /// Validates the integrity of the blockchain.
    pub fn is_chain_valid(&self) -> bool {
        is_chain_valid(&self.chain, self.difficulty)
    }
}
