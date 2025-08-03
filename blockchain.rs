use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use ed25519_dalek::{PublicKey, Signature};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: PublicKey,
    pub recipient: String,
    pub amount: u64,
    pub signature: Signature,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![],
            mempool: vec![],
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.chain.iter().flat_map(|block| &block.transactions)
            .filter(|tx| tx.recipient == address)
            .map(|tx| tx.amount)
            .sum()
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        self.mempool.push(tx);
    }

    pub fn get_chain(&self) -> &Vec<Block> {
        &self.chain
    }
}
