use crate::block::Block;
use crate::utils::hash::calculate_hash;

pub fn is_chain_valid(chain: &Vec<Block>, difficulty: usize) -> bool {
    for i in 1..chain.len() {
        let current = &chain[i];
        let previous = &chain[i - 1];

        let transactions_serialized = serde_json::to_string(&current.transactions).unwrap();
        if current.hash != calculate_hash(
            current.index,
            &current.timestamp,
            &transactions_serialized,
            &current.previous_hash,
            current.nonce,
        ) {
            return false;
        }

        if current.previous_hash != previous.hash {
            return false;
        }

        if &current.hash[..difficulty] != "0".repeat(difficulty) {
            return false;
        }
    }
    true
}
