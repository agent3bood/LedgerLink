use crate::block::Block;
use crate::mempool::Mempool;
use crate::transaction::Transaction;
use std::collections::HashMap;

pub struct Chain {
    pub blocks: Vec<Block>,
    block_orphan: Vec<Block>,
    balance: HashMap<String, u64>,
    nonce: HashMap<String, u64>,
    mempool: Mempool,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            blocks: vec![],
            block_orphan: vec![],
            balance: HashMap::new(),
            nonce: HashMap::new(),
            mempool: Mempool::new(),
        }
    }

    /// verify the whole chain, every block and every transaction
    pub fn verify(&self) -> bool {
        let depth = self.blocks.len();

        let mut prev_block: Option<&Block> = None;
        for i in 0..depth {
            let block = &self.blocks[i];
            if i != block.index as usize {
                return false;
            }
            if !block.verify() {
                return false;
            }
            if let Some(prev_block) = prev_block {
                if prev_block.hash != block.prev_hash {
                    return false;
                }
            }
            prev_block = Some(block);
        }

        true
    }

    pub fn transaction_add(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.transaction_seen(&transaction.sender, transaction.nonce) {
            return Err("Transaction already seen".into());
        }

        if !transaction.verify() {
            return Err("Invalid transaction".into());
        }

        self.mempool.push(transaction);

        Ok(())
    }

    fn transaction_balance_verify(&self, transaction: &Transaction) -> bool {
        let sender_balance = self.balance.get(&transaction.sender).unwrap_or(&0);
        if *sender_balance < transaction.amount {
            return false;
        }
        true
    }

    fn transaction_nonce_verify(&self, transaction: &Transaction) -> bool {
        let nonce = transaction.nonce;
        let last_known_nonce = self.nonce.get(&transaction.sender).unwrap_or(&0);
        if nonce != last_known_nonce + 1 {
            return false;
        }
        true
    }

    fn transaction_seen(&self, sender: &str, nonce: u64) -> bool {
        let last_known_nonce = *self.nonce.get(sender).unwrap_or(&0);
        nonce < last_known_nonce
    }

    pub fn block_add(&mut self, block: Block) -> Result<(), Box<dyn std::error::Error>> {
        block.verify();

        if block.index == 0 {
            self.blocks.push(Block::genesis());
            return Ok(());
        }

        let last_block = self.blocks.last().unwrap();
        if last_block.index + 1 != block.index {
            self.block_orphan.push(block);
            return Ok(());
        }
        if last_block.hash != block.prev_hash {
            return Err("Invalid prev_hash".into());
        }

        for transaction in &block.transactions {
            if !transaction.verify() {
                return Err(format!(
                    "Invalid transaction\nsender:{} nonce:{}",
                    transaction.sender, transaction.nonce
                )
                .into());
            }

            let nonce = transaction.nonce;
            let last_known_nonce = self.nonce.get(&transaction.sender).unwrap_or(&0);
            if nonce != last_known_nonce + 1 {
                return Err(format!(
                    "Invalid nonce\nsender:{} nonce:{}",
                    transaction.sender, nonce
                )
                .into());
            }

            let sender_balance = self.balance.get(&transaction.sender).unwrap_or(&0);
            if *sender_balance < transaction.amount {
                return Err(format!(
                    "Insufficient balance\nsender:{} nonce:{}",
                    transaction.sender, nonce
                )
                .into());
            }
        }

        for transaction in &block.transactions {
            let sender_balance = *self.balance.get(&transaction.sender).unwrap_or(&0);
            let receiver_balance = *self.balance.get(&transaction.receiver).unwrap_or(&0);
            self.balance.insert(
                transaction.sender.clone(),
                sender_balance - transaction.amount,
            );
            self.balance.insert(
                transaction.receiver.clone(),
                receiver_balance + transaction.amount,
            );
        }

        self.blocks.push(block);

        Ok(())
    }

    pub fn block_mint(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut transactions = self.mempool.drain(10);
        transactions.retain(|transaction| {
            if self.transaction_seen(&transaction.sender, transaction.nonce) {
                return false;
            }
            if !transaction.verify() {
                return false;
            }
            if !self.transaction_balance_verify(transaction) {
                return false;
            }
            if !self.transaction_nonce_verify(transaction) {
                return false;
            }

            true
        });
        let last_block = self.blocks.last().unwrap();
        let index = last_block.index + 1;
        let prev_hash = last_block.hash.clone();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let block = Block::new(index, timestamp, prev_hash, transactions);
        match self.block_add(block) {
            Ok(_) => {
                assert_eq!(self.blocks.last().unwrap().index, index);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // pub fn last_seen_nonce(&self, sender: &str) -> Option<u64> {
    //     self.nonce.get(sender).copied()
    // }

    // pub fn transaction_seen(&self, transaction: &Transaction) -> bool {
    //     let last_known_nonce = *self.nonce.get(&transaction.sender).unwrap_or(&0);
    //     transaction.nonce > last_known_nonce
    // }

    // fn get_block(&self, index: u64) -> Option<&Block> {
    //     self.blocks.get(index as usize)
    // }

    // pub fn get_last_block(&self) -> Option<&Block> {
    //     self.blocks.last()
    // }

    // pub fn block_get_top_index(&self) -> u64 {
    //     self.depth - 1
    // }

    // fn get_blocks(&self) -> &Vec<Block> {
    //     &self.blocks
    // }
}
