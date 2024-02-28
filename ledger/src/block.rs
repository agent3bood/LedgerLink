use crate::transaction::Transaction;

pub struct Block {
    pub(crate) index: u64,
    pub(crate) transactions: Vec<Transaction>,
    pub(crate) timestamp: u64,
    pub(crate) hash: String,
    pub(crate) prev_hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        timestamp: u64,
        prev_hash: String,
    ) -> Block {
        let hash = Block::calculate_hash(index, &transactions, timestamp, &prev_hash);
        Block {
            index,
            transactions,
            timestamp,
            hash,
            prev_hash,
        }
    }

    pub fn verify(&self) -> bool {
        self.hash
            == Block::calculate_hash(
                self.index,
                &self.transactions,
                self.timestamp,
                &self.prev_hash,
            )
    }

    fn calculate_hash(
        index: u64,
        transactions: &Vec<Transaction>,
        timestamp: u64,
        prev_hash: &str,
    ) -> String {
        let mut data = format!("{}{}{}", index, timestamp, prev_hash);
        for transaction in transactions {
            data.push_str(&transaction.hash);
        }
        hex::encode(sha256::digest(data.as_bytes()))
    }
}
