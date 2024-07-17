mod blockchain;
mod contract;
mod network;
mod smart_contract;

use blockchain::{generate_key_pair, Blockchain, Transaction};
use serde::Serialize;
use tokio::net::TcpListener;
use warp::Filter;

#[derive(Serialize)]
struct BlockchainResponse {
    chain: Vec<blockchain::Block>,
}

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

    println!("{:#?}", blockchain);

    // Clone the blockchain before moving it into the closure
    let blockchain_clone = blockchain.clone();
    let blockchain_filter = warp::path("blockchain")
        .map(move || {
            let response = BlockchainResponse {
                chain: blockchain_clone.chain.clone(),
            };
            warp::reply::json(&response)
        });

    let routes = warp::get().and(blockchain_filter);

    tokio::spawn(async move {
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let mut blockchain_clone = blockchain.clone();
        tokio::spawn(async move {
            network::handle_client(socket, &mut blockchain_clone).await;
        });
    }
}
