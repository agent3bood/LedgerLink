use k256::ecdsa::{SigningKey, VerifyingKey};
use ledger::block::Block;
use ledger::{chain::Chain, transaction::Transaction};
use serde::{Deserialize, Serialize};
use utils::Utils;

pub struct Node {
    pub id: String,
    verifying_key: VerifyingKey,
    key: SigningKey,
    chain: Chain,
    rx: tokio::sync::mpsc::Receiver<String>,
    network_tx: tokio::sync::mpsc::Sender<String>,
}

impl Node {
    pub fn new(
        id: String,
        key: String,
        chain: Chain,
        rx: tokio::sync::mpsc::Receiver<String>,
        network_tx: tokio::sync::mpsc::Sender<String>,
    ) -> Result<Node, Box<dyn std::error::Error>> {
        let signing_key = Utils::get_signing_key(&key)?;
        let verifying_key = Utils::get_verifying_key(&id)?;

        Ok(Node {
            id,
            verifying_key: verifying_key,
            key: signing_key,
            chain,
            rx,
            network_tx,
        })
    }

    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if let Some(message) = self.rx.recv().await {
                self.handle_message(message)?;
            }
        }
    }

    fn handle_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        match serde_json::from_str::<Message>(&message) {
            Ok(message) => match message {
                Message::Transaction(transaction) => self.handle_transaction(*transaction),
                Message::Block(block) => self.handle_block(*block),
            },
            Err(_) => Err("Invalid message".into()),
        }
    }

    /// transactions comes from the senders of the transactions
    /// it should be signed by the sender
    /// nonce should be unique and increasing by 1 for each transaction from the same sender
    /// transactions will be processed in sequence of the nonce
    /// if the transaction is valid and not seen, it will be added to the mempool
    fn handle_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.chain.transaction_add(transaction);

        Ok(())
    }

    /// blocks comes from other nodes
    /// it should be signed by the sender
    /// it should be valid
    /// it should be the next block in the chain otherwise it will be added to the orphan blocks
    /// if the block is valid and not seen, it will be added to the chain
    pub fn handle_block(&mut self, block: Block) -> Result<(), Box<dyn std::error::Error>> {
        self.chain.block_add(block);
        // // signature
        // {
        //     if !block.verify() {
        //         return Err("Invalid signature".into());
        //     }
        // }
        // // seen
        // {
        //     let top_block = self.chain.block_get_top_index();
        //     if block.index <= top_block {
        //         return Err("Block already seen".into());
        //     } else if block.index > top_block + 1 {
        //         // orphan blocks
        //         return Ok(());
        //     }
        // }
        //
        // // verify block
        // {
        //     if !block.verify() {
        //         return Err("Invalid block".into());
        //     }
        // }
        //
        // // add to chain
        // {
        //     self.chain.block_add(block);
        // }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Transaction(Box<Transaction>),
    Block(Box<Block>),
}
