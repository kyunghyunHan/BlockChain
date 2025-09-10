use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone)]
struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    fn new(data: String, prev_block_hash: Vec<u8>) -> Self {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            data: data.into_bytes(),
            prev_block_hash,
            hash: Vec::new(),
        };
        
        block.calculate_hash();
        block
    }
    
    fn calculate_hash(&mut self) {
        let mut data = Vec::new();
        data.extend_from_slice(&self.prev_block_hash);
        data.extend_from_slice(&self.data);
        data.extend_from_slice(&self.timestamp.to_be_bytes());
        
        let hash = Sha256::digest(&data);
        self.hash = hash.to_vec();
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
        };
        blockchain.generate_genesis();
        blockchain
    }

    fn generate_genesis(&mut self) {
        let genesis_block = Block::new("Genesis Block".to_string(), Vec::new());
        self.blocks.push(genesis_block);
    }

    fn get_prev_hash(&self) -> Vec<u8> {
        if !self.blocks.is_empty() {
            self.blocks.last().unwrap().hash.clone()
        } else {
            Vec::new()
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_hash = self.get_prev_hash();
        let new_block = Block::new(data, prev_hash);
        self.blocks.push(new_block);
    }

    fn show_blocks(&self) {
        for block in &self.blocks {
            println!("Prev. hash: {}", hex::encode(&block.prev_block_hash));
            println!("Data: {}", String::from_utf8_lossy(&block.data));
            println!("Hash: {}", hex::encode(&block.hash));
            println!();
        }
    }
}

static BLOCKCHAIN: OnceLock<Mutex<Blockchain>> = OnceLock::new();

fn get_blockchain() -> &'static Mutex<Blockchain> {
    BLOCKCHAIN.get_or_init(|| Mutex::new(Blockchain::new()))
}

fn main() {
    let blockchain_mutex = get_blockchain();
    let mut chain = blockchain_mutex.lock().unwrap();
    
    chain.add_block("Send 1 BTC to Ivan".to_string());
    chain.add_block("Send 2 more BTC to Ivan".to_string());
    
    chain.show_blocks();
}

/*
Cargo.toml:
[dependencies]
sha2 = "0.10"
hex = "0.4"
*/