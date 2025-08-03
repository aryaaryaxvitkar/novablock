use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        };
        bc.create_genesis_block();
        bc
    }

    fn create_genesis_block(&mut self) {
        let block = Block {
            index: 0,
            timestamp: Self::now(),
            transactions: vec![],
            previous_hash: "0".repeat(64),
            hash: String::new(),
            nonce: 0,
        };
        let hash = Self::calculate_hash(&block);
        self.chain.push(Block { hash, ..block });
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.current_transactions.push(transaction);
    }

    pub fn add_block(&mut self) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block {
            index: self.chain.len() as u32,
            timestamp: Self::now(),
            transactions: self.current_transactions.clone(),
            previous_hash: previous_block.hash.clone(),
            hash: String::new(),
            nonce: 0,
        };
        let hash = Self::calculate_hash(&new_block);
        self.chain.push(Block { hash, ..new_block });
        self.current_transactions.clear();
    }

    pub fn calculate_hash(block: &Block) -> String {
        let data = format!(
            "{}{:?}{}{}{}",
            block.index, block.transactions, block.timestamp, block.previous_hash, block.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    fn now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}
