use crate::transaction::Transaction;

pub struct Mempool {
    pool: Vec<Transaction>,
}

impl Mempool {
    pub(crate) fn len(&self) -> u64 {
        self.pool.len() as u64
    }
}

impl Mempool {
    pub fn new() -> Mempool {
        Mempool {
            pool: Vec::new(),
            // transactions: HashMap::new(),
        }
    }

    pub fn push(&mut self, transaction: Transaction) {
        self.pool.push(transaction);
    }

    pub fn drain(&mut self, n: usize) -> Vec<Transaction> {
        self.pool.drain(0..n).collect()
    }
}
