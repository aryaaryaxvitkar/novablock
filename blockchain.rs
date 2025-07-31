use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Serialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

#[derive(Serialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub current_transactions: Vec<Transaction>,
    pub balances: HashMap<String, u64>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            current_transactions: vec![],
            balances: HashMap::new(),
            difficulty: 4,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        self.balances.insert("network".to_string(), 1000000);
        let genesis_tx = Transaction {
            sender: "network".to_string(),
            recipient: "Arya".to_string(),
            amount: 100,
        };
        self.current_transactions.push(genesis_tx.clone());
        let genesis_block = self.create_block("0".to_string());
        self.chain.push(genesis_block);
    }

    pub fn create_block(&mut self, previous_hash: String) -> Block {
        let index = self.chain.len() as u64;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let transactions = self.current_transactions.clone();
        let mut nonce = 0;
        let mut hash;

        loop {
            let block_data = format!(
                "{}{}{}{}",
                index,
                serde_json::to_string(&transactions).unwrap(),
                &previous_hash,
                nonce
            );
            hash = format!("{:x}", md5::compute(block_data));
            if &hash[..self.difficulty] == "0000" {
                break;
            }
            nonce += 1;
        }

        for tx in &transactions {
            let sender_balance = self.balances.entry(tx.sender.clone()).or_insert(0);
            if *sender_balance >= tx.amount {
                *sender_balance -= tx.amount;
                let receiver_balance = self.balances.entry(tx.recipient.clone()).or_insert(0);
                *receiver_balance += tx.amount;
            }
        }

        self.current_transactions.clear();

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce,
            hash,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        let sender_balance = self.balances.get(&tx.sender).cloned().unwrap_or(0);
        if sender_balance < tx.amount {
            return Err(format!("Insufficient balance for {}", tx.sender));
        }
        self.current_transactions.push(tx);
        Ok(())
    }

    pub fn mine_block(&mut self) {
        let last_hash = self.chain.last().unwrap().hash.clone();
        let new_block = self.create_block(last_hash);
        self.chain.push(new_block);
    }

    pub fn get_balance(&self, user: &str) -> u64 {
        *self.balances.get(user).unwrap_or(&0)
    }
}
