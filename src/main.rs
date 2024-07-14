mod blockchain;
mod network;
mod transaction;
mod block;
mod smart_contract;
mod utils;

use blockchain::Blockchain;
use transaction::{Transaction, generate_key_pair};
use tokio::net::TcpListener;
use wasmer::Value;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    println!("Server running on 127.0.0.1:8082");
    let mut blockchain = Blockchain::new(2);

    // Generate key pair for signing transactions
    let (key_pair, public_key) = generate_key_pair();
    
    // Create a genesis transaction
    let mut genesis_transaction = Transaction {
        sender: String::from("genesis"),
        receiver: String::from("network"),
        amount: 50,
        signature: vec![],
    };

    // Sign the transaction using the private key
    genesis_transaction.sign(&key_pair);
    
    // Add the transaction to the blockchain
    blockchain.add_block(vec![genesis_transaction.clone()]);

    // Verify the transaction using the public key
    let is_valid = genesis_transaction.verify(&public_key);
    println!("Transaction valid: {}", is_valid);

    // Example usage of execute_smart_contract
    let wasm_bytes = include_bytes!("test_contract.wasm");
    let function = "contract_function";
    let params = vec![Value::I32(42)];
    match smart_contract::execute_smart_contract(wasm_bytes, function, &params) {
        Ok(result) => println!("Smart contract executed successfully: {:?}", result),
        Err(e) => println!("Smart contract execution failed: {}", e),
    }

    println!("{:#?}", blockchain);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let mut blockchain_clone = blockchain.clone();
        tokio::spawn(async move {
            network::handle_client(socket, &mut blockchain_clone).await;
        });
    }
}
