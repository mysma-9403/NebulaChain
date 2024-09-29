use my_blockchain::blockchain::Blockchain;
use my_blockchain::transaction::Transaction;

#[test]
fn test_genesis_block() {
    let blockchain = Blockchain::new();
    let genesis = &blockchain.chain[0];
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.previous_hash, "0");
    assert_eq!(genesis.transactions.len(), 1);
    assert_eq!(genesis.transactions[0].sender, "0");
    assert_eq!(genesis.transactions[0].receiver, "0");
    assert_eq!(genesis.transactions[0].amount, 0.0);
}

#[test]
fn test_add_transaction() {
    let mut blockchain = Blockchain::new();
    let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    blockchain.add_transaction(tx.clone());
    assert_eq!(blockchain.mempool.len(), 1);
    assert_eq!(blockchain.mempool[0].sender, "Alice");
    assert_eq!(blockchain.mempool[0].receiver, "Bob");
    assert_eq!(blockchain.mempool[0].amount, 50.0);
}

#[test]
fn test_mine_block() {
    let mut blockchain = Blockchain::new();
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 25.0);
    blockchain.add_transaction(tx1.clone());
    blockchain.add_transaction(tx2.clone());
    blockchain.mine_block();
    assert_eq!(blockchain.chain.len(), 2);
    let mined_block = &blockchain.chain[1];
    assert_eq!(mined_block.index, 1);
    assert_eq!(mined_block.transactions.len(), 2);
    assert_eq!(mined_block.transactions[0].sender, "Alice");
    assert_eq!(mined_block.transactions[1].receiver, "Charlie");
}

#[test]
fn test_chain_validity() {
    let mut blockchain = Blockchain::new();
    let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
    blockchain.add_transaction(tx1);
    blockchain.mine_block();

    let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 25.0);
    blockchain.add_transaction(tx2);
    blockchain.mine_block();

    assert!(blockchain.is_chain_valid());

    // Wprowadzenie zmiany w transakcji
    blockchain.chain[1].transactions[0].amount = 1000.0;
    assert!(!blockchain.is_chain_valid());
}
