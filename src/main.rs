use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Mutex, OnceLock};
use num_bigint::BigUint;
use num_traits::{Zero, One};

const TARGET_BITS: u32 = 20;  // 앞에 0이 5개 (20비트)
const MAX_NONCE: u64 = u64::MAX;

#[derive(Debug, Clone)]
struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
    nonce: u64,  // 🆕 nonce 필드 추가
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
            nonce: 0,
        };

        // 🆕 작업 증명을 통해 블록 생성
        let pow = ProofOfWork::new(&mut block);
        let (nonce, hash) = pow.run();
        
        block.hash = hash;
        block.nonce = nonce;
        block
    }
}

// 🆕 작업 증명 구조체
struct ProofOfWork<'a> {
    block: &'a mut Block,
    target: BigUint,
}

impl<'a> ProofOfWork<'a> {
    fn new(block: &'a mut Block) -> Self {
        let mut target = BigUint::one();
        target <<= 256 - TARGET_BITS;  // 목표값 계산
        
        ProofOfWork { block, target }
    }

    fn prepare_data(&self, nonce: u64) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.block.prev_block_hash);
        data.extend_from_slice(&self.block.data);
        data.extend_from_slice(&self.block.timestamp.to_be_bytes());
        data.extend_from_slice(&(TARGET_BITS as i64).to_be_bytes());
        data.extend_from_slice(&(nonce as i64).to_be_bytes());
        data
    }

    // 🆕 채굴 과정 (nonce 찾기)
    fn run(self) -> (u64, Vec<u8>) {
        let mut nonce = 0u64;
        let mut hash = [0u8; 32];
        
        println!("Mining the block containing \"{}\"", 
                 String::from_utf8_lossy(&self.block.data));

        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = Sha256::digest(&data).into();
            
            print!("\r{}", hex::encode(&hash));
            
            let hash_int = BigUint::from_bytes_be(&hash);
            if hash_int < self.target {
                break;  // 조건을 만족하는 해시 발견!
            } else {
                nonce += 1;  // nonce 증가하여 다시 시도
            }
        }
        
        println!("\n");
        (nonce, hash.to_vec())
    }

    // 🆕 작업 증명 검증
    fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.nonce);
        let hash = Sha256::digest(&data);
        let hash_int = BigUint::from_bytes_be(&hash);
        
        hash_int < self.target
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
            println!("Nonce: {}", block.nonce);  // 🆕 nonce 출력
            
            // 🆕 작업 증명 검증
            let mut temp_block = block.clone();
            let pow = ProofOfWork::new(&mut temp_block);
            println!("PoW: {}", pow.validate());
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
num-bigint = "0.4" 
num-traits = "0.2"
hex = "0.4"
*/