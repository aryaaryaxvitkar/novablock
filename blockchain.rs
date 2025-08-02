use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: usize, transactions: Vec<Transaction>, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now().timestamp_millis();
        let mut nonce = 0;
        let mut hash;

        loop {
            let content = format!("{}{}{:?}{}{}", index, timestamp, transactions, previous_hash, nonce);
            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            let result = hasher.finalize();
            hash = hex::encode(&result);

            if hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }

            nonce += 1;
        }

        Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce,
            hash,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
    pub difficulty: usize,
    pub balances: std::collections::HashMap<String, u64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            current_transactions: vec![],
            difficulty: 4,
            balances: std::collections::HashMap::new(),
        };

        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_tx = Transaction {
            sender: "network".to_string(),
            recipient: "Arya".to_string(),
            amount: 100,
        };
        self.balances.insert("Arya".to_string(), 100);
        self.balances.insert("network".to_string(), 999_900);
        let block = Block::new(0, vec![genesis_tx], "0".to_string(), self.difficulty);
        self.chain.push(block);
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.current_transactions.push(tx);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_tx = Transaction {
            sender: "network".to_string(),
            recipient: miner_address.clone(),
            amount: 10,
        };

        self.current_transactions.push(reward_tx);

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(
            self.chain.len(),
            self.current_transactions.clone(),
            previous_hash,
            self.difficulty,
        );

        for tx in &self.current_transactions {
            *self.balances.entry(tx.recipient.clone()).or_insert(0) += tx.amount;
            *self.balances.entry(tx.sender.clone()).or_insert(0) -= tx.amount;
        }

        self.chain.push(new_block);
        self.current_transactions.clear();
    }
}
