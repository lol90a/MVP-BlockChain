# Blockchain MVP Project

This project is a simple blockchain application built with Rust for the backend and Yew for the frontend. The backend handles blockchain operations including smart contracts, decentralization, transparency, and security. The frontend displays the blockchain data fetched from the backend.

## Backend

The backend consists of several modules:

### Smart Contract
Handles the execution of smart contracts. It includes functionality to run WebAssembly (Wasm) smart contracts and interact with the blockchain.

### Decentralization
Manages the distribution of rewards and handling decentralized votes. This module ensures that the blockchain operations are decentralized and rewards are distributed fairly among participants.

### Transparency
Logs and provides access to transaction history. It ensures that all transactions are recorded and can be retrieved for transparency purposes.

### Security
Includes authentication and validation functions. This module ensures that all transactions and operations are secure and authenticated properly.

### Main
The entry point of the backend application. It sets up a TCP server, initializes the blockchain, and handles incoming client connections.

#### Running the Backend
To run the backend server, use the following command:
```sh
cargo run --bin blockchain_mvp_bin
