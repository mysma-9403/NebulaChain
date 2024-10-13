// src/main.rs

use NebulaChain::blockchain::Blockchain;
use NebulaChain::transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    // Rejestracja walidatorów przez staking
    blockchain.stake_tokens("Alice".to_string(), 100);
    blockchain.stake_tokens("Bob".to_string(), 200);
    blockchain.stake_tokens("Charlie".to_string(), 150);

    // Dodawanie transakcji
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    blockchain.add_transaction(tx1);

    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 25.0);
    blockchain.add_transaction(tx2);

    // Kopanie bloku
    println!("Kopanie nowego bloku...");
    blockchain.mine_block();

    // Dodanie kolejnych transakcji
    let tx3 = Transaction::new("Charlie".to_string(), "Dave".to_string(), 10.0);
    blockchain.add_transaction(tx3);

    // Kopanie kolejnego bloku
    println!("Kopanie kolejnego bloku...");
    blockchain.mine_block();

    // Wyświetlenie całego łańcucha bloków
    for block in &blockchain.chain {
        println!("{:#?}", block);
    }

    // Sprawdzenie ważności łańcucha
    println!("Czy blockchain jest ważny? {}", blockchain.is_chain_valid());
}
