use crate::blockchain::block::Block;
use crate::blockchain::transaction::Transaction;
use crate::blockchain::utils::proof_of_work;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        Self {
            chain: vec![Self::genesis_block()],
            difficulty,
        }
    }

    pub fn genesis_block() -> Block {
        Block {
            index: 0,
            previous_hash: String::from("0"),
            timestamp: 0,
            transactions: vec![],
            hash: String::from("0"),
            nonce: 0,
        }
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.latest_block();
        let index = previous_block.index + 1;
        let timestamp = Utc::now().timestamp_millis() as u128;
        let previous_hash = &previous_block.hash;
        let (hash, nonce) = proof_of_work(
            &format!("{}{}{}{:?}", index, previous_hash, timestamp, transactions),
            self.difficulty,
        );

        let block = Block {
            index,
            previous_hash: previous_hash.clone(),
            timestamp,
            transactions,
            hash,
            nonce,
        };

        self.chain.push(block);
    }
}
