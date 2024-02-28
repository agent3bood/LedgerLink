use sha256::digest;

pub struct Transaction {
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) amount: u64,
    pub(crate) timestamp: u64,
    pub(crate) hash: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, timestamp: u64) -> Transaction {
        let hash = Transaction::calculate_hash(&sender, &receiver, amount, timestamp);
        Transaction {
            sender,
            receiver,
            amount,
            timestamp,
            hash,
        }
    }

    pub fn verify(&self) -> bool {
        self.hash
            == Transaction::calculate_hash(
                &self.sender,
                &self.receiver,
                self.amount,
                self.timestamp,
            )
    }

    fn calculate_hash(sender: &str, receiver: &str, amount: u64, timestamp: u64) -> String {
        let data = format!("{}{}{}{}", sender, receiver, amount, timestamp);
        hex::encode(digest(data.as_bytes()))
    }
}
