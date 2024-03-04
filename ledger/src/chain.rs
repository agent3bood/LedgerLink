use crate::block::Block;
use crate::transaction::Transaction;
use std::collections::HashMap;

pub struct Chain {
    pub blocks: Vec<Block>,
    depth: u64,
    balance: HashMap<String, u64>,
    nonce: HashMap<String, u64>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            blocks: vec![],
            depth: 0,
            balance: HashMap::new(),
            nonce: HashMap::new(),
        }
    }

    pub fn verify(&self) -> bool {
        if self.depth == 0 {
            return true;
        }

        if self.blocks.len() != self.depth as usize {
            return false;
        }

        let mut prev_block: Option<&Block> = None;
        for i in 0..self.depth {
            let i = i as usize;
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

    fn get_depth(&self) -> u64 {
        self.depth
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        block.verify();

        if self.blocks.is_empty() {
            if block.index != 0 {
                return false;
            }
        } else {
            let last_block = self.blocks.last().unwrap();
            if last_block.index + 1 != block.index {
                return false;
            }
            if last_block.hash != block.prev_hash {
                return false;
            }
        }

        for transaction in &block.transactions {
            if !transaction.verify() {
                return false;
            }
            let nonce = transaction.nonce;
            let last_known_nonce = self.nonce.get(&transaction.sender);
            match last_known_nonce {
                None => {
                    if nonce != 0 {
                        return false;
                    }
                }
                Some(last_known_nonce) => {
                    if nonce != last_known_nonce + 1 {
                        return false;
                    }
                }
            }

            let sender_balance = self.balance.get(&transaction.sender).unwrap_or(&0);
            if *sender_balance < transaction.amount {
                return false;
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
        self.depth += 1;
        true
    }

    pub fn transaction_seen(&self, sender: &str, nonce: u64) -> bool {
        let last_known_nonce = *self.nonce.get(sender).unwrap_or(&0);
        nonce > last_known_nonce
    }

    // pub fn transaction_seen(&self, transaction: &Transaction) -> bool {
    //     let last_known_nonce = *self.nonce.get(&transaction.sender).unwrap_or(&0);
    //     transaction.nonce > last_known_nonce
    // }

    fn get_block(&self, index: u64) -> Option<&Block> {
        self.blocks.get(index as usize)
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}
