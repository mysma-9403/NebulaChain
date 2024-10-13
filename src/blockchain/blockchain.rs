// src/blockchain/blockchain.rs

use crate::block::Block;
use crate::transaction::Transaction;
use crate::utils::time::get_current_timestamp;
use crate::utils::hash::calculate_hash;
use chrono::prelude::*;
use crate::blockchain::difficulty::adjust_difficulty;
use crate::blockchain::validation::is_chain_valid;
use crate::staking::staking::Staking;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub difficulty: usize,
    pub adjustment_interval: usize,
    pub target_block_time: u64,
    pub staking: Staking, // Nowe pole
}

impl Blockchain {
    /// Inicjalizacja nowego blockchaina z genesis blockiem.
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            chain: vec![],
            mempool: vec![],
            difficulty: 4, // Możesz dostosować lub usunąć w PoS
            adjustment_interval: 10,
            target_block_time: 10,
            staking: Staking::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    /// Tworzy i dodaje genesis block do blockchaina.
    fn create_genesis_block(&mut self) {
        let genesis_transaction = Transaction::new("0".to_string(), "0".to_string(), 0.0);
        let genesis_block = Block::new(0, vec![genesis_transaction], "0".to_string(), self.difficulty, "genesis".to_string());
        self.chain.push(genesis_block);
    }

    /// Dodaje nową transakcję do mempoola.
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.mempool.push(transaction);
    }

    /// Dodaje stake
    pub fn stake_tokens(&mut self, validator: String, amount: u64) {
        self.staking.add_stake(validator, amount);
    }

    /// Usuwa stake
    pub fn unstake_tokens(&mut self, validator: String, amount: u64) -> bool {
        self.staking.remove_stake(validator, amount)
    }

    /// Kopie nowego bloku z aktualnym mempoolem
    pub fn mine_block(&mut self) {
        if self.mempool.is_empty() {
            println!("Brak transakcji do dodania do bloku.");
            return;
        }

        // Wybór walidatora na podstawie staku
        let validator = match self.staking.select_validator() {
            Some(v) => v,
            None => {
                println!("Brak walidatorów do kopania bloku.");
                return;
            }
        };

        // Klonowanie ostatniego bloku, aby zakończyć niezmienny borrow
        let last_block = self.chain.last().unwrap().clone();

        // Teraz, bez pożyczania niezmiennego `self`, można pożyczyć mutowalnie
        self.add_reward_transaction(validator.clone(), 50.0); // Przykładowa nagroda 50 tokenów

        let mut new_transactions = self.mempool.clone();
        new_transactions.push(Transaction::new("system".to_string(), validator.clone(), 50.0));

        let new_block = Block::new(
            last_block.index + 1,
            new_transactions,
            last_block.hash.clone(),
            self.difficulty, // Możesz dostosować lub usunąć w PoS
            validator.clone(),
        );

        println!("Blok został wykopany i dodany do łańcucha przez walidatora: {}", validator);

        // Dostosowanie trudności (opcjonalne)
        if new_block.index % self.adjustment_interval as u32 == 0 {
            self.difficulty = adjust_difficulty(&self.chain, self.adjustment_interval, self.target_block_time, self.difficulty);
        }

        self.chain.push(new_block);
        self.mempool.clear();
    }

    /// Dodaje transakcję nagrody dla walidatora
    fn add_reward_transaction(&mut self, validator: String, amount: f64) {
        let reward_tx = Transaction::new("system".to_string(), validator.clone(), amount);
        self.mempool.push(reward_tx);
    }

    /// Waliduje integralność blockchaina.
    pub fn is_chain_valid(&self) -> bool {
        is_chain_valid(&self.chain, self.difficulty)
    }
}
