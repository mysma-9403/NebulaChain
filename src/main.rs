use NebulaChain::blockchain::Blockchain;
use NebulaChain::transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    blockchain.add_transaction(tx1);

    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 25.0);
    blockchain.add_transaction(tx2);

    println!("Mining a new block...");
    blockchain.mine_block();

    let tx3 = Transaction::new("Charlie".to_string(), "Dave".to_string(), 10.0);
    blockchain.add_transaction(tx3);

    println!("Mining another block...");
    blockchain.mine_block();

    for block in &blockchain.chain {
        println!("{:#?}", block);
    }

    println!("Is the blockchain valid? {}", blockchain.is_chain_valid());
}
