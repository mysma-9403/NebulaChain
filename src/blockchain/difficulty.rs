use chrono::prelude::*;
use crate::block::Block;

pub fn adjust_difficulty(chain: &Vec<Block>, adjustment_interval: usize, target_block_time: u64, current_difficulty: usize) -> usize {
    if chain.len() < adjustment_interval + 1 {
        return current_difficulty;
    }

    let latest = chain.last().unwrap();
    let latest_timestamp = DateTime::parse_from_rfc3339(&latest.timestamp).unwrap().timestamp();
    let prev_adjustment = &chain[chain.len() - adjustment_interval - 1];
    let prev_timestamp = DateTime::parse_from_rfc3339(&prev_adjustment.timestamp).unwrap().timestamp();
    let actual_time = (latest_timestamp - prev_timestamp) as u64;

    let expected_time = target_block_time * adjustment_interval as u64;

    if actual_time < expected_time / 2 {
        let new_difficulty = current_difficulty + 1;
        println!("Trudność zwiększona do {}", new_difficulty);
        new_difficulty
    } else if actual_time > expected_time * 2 && current_difficulty > 1 {
        let new_difficulty = current_difficulty - 1;
        println!("Trudność zmniejszona do {}", new_difficulty);
        new_difficulty
    } else {
        println!("Trudność pozostaje na poziomie {}", current_difficulty);
        current_difficulty
    }
}
