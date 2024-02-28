use crate::block::Block;
use crate::chain::Chain;
use std::collections::HashMap;

struct ChainMemory {
    blocks: Vec<Block>,
    depth: u64,
    balance: HashMap<String, u64>,
}

impl Chain for ChainMemory {
    fn new() -> ChainMemory {
        ChainMemory {
            blocks: vec![],
            depth: 0,
            balance: HashMap::new(),
        }
    }

    fn verify(&self) -> bool {
        if self.depth == 0 {
            return true;
        }

        for (i, block) in self.blocks.iter().enumerate() {
            if i != block.index as usize {
                return false;
            }
            if i != 0 {
                let prev_block = &self.get_block(i as u64 - 1).unwrap();
                if prev_block.hash != block.prev_hash {
                    return false;
                }
            }
            if !block.verify() {
                return false;
            }
        }

        true
    }

    fn get_depth(&self) -> u64 {
        self.depth
    }

    fn add_block(&mut self, block: Block) -> bool {
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

        for transaction in block.transactions.iter() {
            if !transaction.verify() {
                return false;
            }
            if self.balance.contains_key(&transaction.sender) {
                let sender_balance = self.balance.get(&transaction.sender).unwrap_or(&0);
                if *sender_balance < transaction.amount {
                    return false;
                }
            } else {
                return false;
            }
        }

        for transaction in block.transactions.iter() {
            let sender_balance = self.balance.get(&transaction.sender).unwrap_or(&0).clone();
            let receiver_balance = self
                .balance
                .get(&transaction.receiver)
                .unwrap_or(&0)
                .clone();
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

    fn get_block(&self, index: u64) -> Option<&Block> {
        self.blocks.get(index as usize)
    }

    fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}
