use k256::ecdsa::{SigningKey, VerifyingKey};
use ledger::{chain::Chain, transaction::Transaction};
use std::sync::{Arc, Mutex};
use utils::Utils;

pub struct Node {
    id: VerifyingKey,
    key: SigningKey,
    chain: Arc<Mutex<Chain>>,
    transactions_pool: Vec<Transaction>,
}

impl Node {
    pub fn new(
        id: String,
        key: String,
        chain: Arc<Mutex<Chain>>,
    ) -> Result<Node, Box<dyn std::error::Error>> {
        let signing_key = Utils::get_signing_key(&key)?;
        let verifying_key = Utils::get_verifying_key(&id)?;

        Ok(Node {
            id: verifying_key,
            key: signing_key,
            chain,
            transactions_pool: Vec::new(),
        })
    }

    /// transactions comes from the senders or other nodes
    /// it should be signed by the sender
    /// nonce should be unique and increasing by 1 for each transaction from the same sender
    /// transactions will be processed in sequence of the nonce
    /// if the transaction is valid and not seen, it will be added to the mempool
    ///
    ///
    pub fn handle_transaction(
        &mut self,
        nonce: u64,
        amount: u64,
        sender: String,
        receiver: String,
        signature: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let transaction = Transaction::new(nonce, amount, &sender, &receiver, Some(&signature))?;
        // seen
        {
            if self.chain.lock().unwrap().transaction_seen(&sender, nonce) {
                return Err("Transaction already seen".into());
            }
        }
        // signature
        {
            if !transaction.verify() {
                return Err("Invalid signature".into());
            }
        }
        // add to mempool
        {
            // TODO mempool size limit
            self.transactions_pool.push(transaction);
        }

        Ok(())
    }

    /// blocks comes from other nodes
    pub fn handle_block(
        &mut self,
        index: u64,
        timestamp: u64,
        transactions: Vec<()>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
