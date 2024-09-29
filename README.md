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
