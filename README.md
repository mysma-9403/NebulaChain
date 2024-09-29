# Rust Blockchain Implementation

A simple and modular blockchain implementation in Rust, demonstrating the core principles of blockchain technology, including block creation, transaction management, mining (Proof of Work), difficulty adjustment, and chain validation. This project is designed with clean code practices, making it easy to understand, maintain, and extend.

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Installation](#installation)
- [Usage](#usage)
- [Running Tests](#running-tests)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Block Structure:** Each block contains an index, timestamp, list of transactions, previous hash, current hash, and nonce.
- **Transaction Management:** Simple transactions with sender, receiver, and amount.
- **Mining (Proof of Work):** Implements a Proof of Work algorithm to secure the blockchain.
- **Difficulty Adjustment:** Automatically adjusts mining difficulty based on the time taken to mine recent blocks.
- **Chain Validation:** Ensures the integrity and validity of the blockchain.
- **Parallel Processing:** Utilizes the Rayon library for parallel computation during mining to enhance performance.
- **Unit and Integration Tests:** Comprehensive tests to ensure the reliability of blockchain operations.

## Architecture

The project is structured into several modules, each responsible for a specific aspect of the blockchain:

- **`blockchain`**: Manages the overall blockchain, including adding transactions, mining blocks, and validating the chain.
- **`block`**: Defines the block structure and mining logic.
- **`transaction`**: Handles transaction creation and management.
- **`utils`**: Provides utility functions for hashing and time management.

This modular approach promotes separation of concerns, making the codebase easier to navigate and maintain.

## Installation

### Prerequisites

- **Rust:** Ensure that you have Rust installed. If not, download it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Steps

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/rust-blockchain.git
   cd rust-blockchain

2. **Build the Project:**
   ```bash
   cargo build --release
This will compile the project and its dependencies.

## Usage

After building the project, you can run the blockchain application to interact with it.

1. **Running the Application**

   ```bash
   cargo run --release
 
2. **Example Output**

   ```bash
   Mining a new block...
   Block has been mined and added to the chain.
   Mining another block...
   Block has been mined and added to the chain.
   Block {
       index: 0,
       timestamp: "2024-04-27T12:34:56Z",
       transactions: [
           Transaction {
               sender: "0",
               receiver: "0",
               amount: 0.0,
           },
       ],
       previous_hash: "0",
       hash: "0000abcd1234...",
       nonce: 123456,
   }
   Block {
       index: 1,
       timestamp: "2024-04-27T12:35:10Z",
       transactions: [
           Transaction {
               sender: "Alice",
               receiver: "Bob",
               amount: 50.0,
           },
           Transaction {
               sender: "Bob",
               receiver: "Charlie",
               amount: 25.0,
           },
       ],
       previous_hash: "0000abcd1234...",
       hash: "0000efgh5678...",
       nonce: 789012,
   }
   Block {
       index: 2,
       timestamp: "2024-04-27T12:35:25Z",
       transactions: [
           Transaction {
               sender: "Charlie",
               receiver: "Dave",
               amount: 10.0,
           },
       ],
       previous_hash: "0000efgh5678...",
       hash: "0000ijkl9012...",
       nonce: 345678,
   }
   Is the blockchain valid? true

3. **Interacting with the Blockchain**

The current implementation demonstrates adding transactions and mining blocks automatically. To extend functionality, you can modify the *main.rs* file to accept user input or integrate with a network layer.

## Running Tests

The project includes unit and integration tests to ensure the correctness of blockchain operations.

1. **Running All Tests**
    ```bash
    cargo test
 

2. **Test Coverage**

- Genesis Block Creation: Verifies the creation of the genesis block with correct initial values.
- Transaction Addition: Ensures that transactions are correctly added to the mempool.
- Block Mining: Tests the mining process and the addition of new blocks to the chain.
- Chain Validation: Confirms the integrity of the blockchain and detects tampering.

3. **Example Test Output**
   ```bash
   running 4 tests
   test tests::test_add_transaction ... ok
   test tests::test_chain_validity ... ok
   test tests::test_genesis_block ... ok
   test tests::test_mine_block ... ok
   
   test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

## Project Structure

   ```bash
      rust-blockchain/
      ├── Cargo.toml
      └── src/
          ├── main.rs
          ├── lib.rs
          ├── blockchain/
          │   ├── mod.rs
          │   ├── blockchain.rs
          │   ├── difficulty.rs
          │   └── validation.rs
          ├── block/
          │   ├── mod.rs
          │   ├── block.rs
          │   └── mining.rs
          ├── transaction/
          │   ├── mod.rs
          │   └── transaction.rs
          ├── utils/
          │   ├── mod.rs
          │   ├── hash.rs
          │   └── time.rs
          └── tests/
              └── integration_tests.rs

```
## Module Breakdown

- main.rs: Entry point of the application. Handles high-level interactions and orchestrates blockchain operations.
- lib.rs: Defines the library interface, exposing modules and their functionalities.
- blockchain/:
  - blockchain.rs: Core logic for managing the blockchain.
  - difficulty.rs: Logic for adjusting mining difficulty.
  - validation.rs: Functions for validating the blockchain's integrity.
- block/:
  - block.rs: Defines the Block structure and associated methods.
  - mining.rs: Contains mining-related functionalities.
- transaction/:
  - transaction.rs: Defines the Transaction structure and methods.
- utils/:
  - hash.rs: Utility functions for hashing.
  - time.rs: Utility functions for handling timestamps.
- tests/:
  - integration_tests.rs: Integration tests covering various aspects of the blockchain.

## Contributing

Contributions are welcome! If you have suggestions, bug reports, or feature requests, feel free to open an issue or submit a pull request.

1. **Steps to Contribute**

- Fork the Repository
- Create a New Branch
   ```bash
   git checkout -b feature/YourFeature

- Make Changes and Commit
   ```bash
  git commit -m "Add your feature"

- Push to the Branch
   ```bash
  git push origin feature/YourFeature

- Open pull request

## License
This project is licensed under the MIT License.