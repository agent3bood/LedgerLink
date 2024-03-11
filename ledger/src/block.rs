use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use utils::Utils;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub hash: String,
    pub prev_hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        timestamp: u64,
        prev_hash: String,
        transactions: Vec<Transaction>,
    ) -> Block {
        let hash = Block::calculate_hash(index, timestamp, &prev_hash, &transactions);
        Block {
            index,
            transactions,
            timestamp,
            hash,
            prev_hash,
        }
    }

    pub fn genesis() -> Block {
        Block::new(0, 0, "0".to_string(), vec![])
    }

    pub fn verify(&self) -> bool {
        self.verify_hash() && self.verify_transactions()
    }

    fn verify_hash(&self) -> bool {
        self.hash
            == Block::calculate_hash(
                self.index,
                self.timestamp,
                &self.prev_hash,
                &self.transactions,
            )
    }

    fn verify_transactions(&self) -> bool {
        for transaction in &self.transactions {
            if !transaction.verify() {
                return false;
            }
        }
        true
    }

    fn calculate_hash(
        index: u64,
        timestamp: u64,
        prev_hash: &str,
        transactions: &Vec<Transaction>,
    ) -> String {
        let mut data = format!("{}{}{}", index, timestamp, prev_hash);
        for transaction in transactions {
            data.push_str(&transaction.hash);
        }
        Utils::hash_data(&data)
    }
}
