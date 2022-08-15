use chrono::Utc;
use hex;
use serde_json;
use sha2::{Digest, Sha256};

const DIFFICULTY_PREFIX: &str = "00";

struct Chain {
    blocks: Vec<Block>,
    transactions: Vec<&'static str>,
}

impl Chain {
    fn new() -> Self {
        Self {
            blocks: vec![],
            transactions: vec![],
        }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            hash: "0".repeat(64),
            previous_hash: String::from("GENESIS"),
            data: String::from("GENESIS"),
            nonce: 0,
        };
        self.blocks.push(genesis_block);
    }

    fn view(&self) {
        for block in self.blocks.iter() {
            block.display();
        }
    }

    fn add_transaction(&mut self, tx: &'static str) {
        self.transactions.push(tx);
    }

    fn mine_block(&mut self) {
        let last_block = self.blocks.last().expect("Chain needs at lest one block!");
        let data = self.transactions.join("\n").to_string();
        let mut nonce: u64 = 0;

        let block = loop {
            // if nonce % 10000 == 0 { println!("nonce: {}", nonce); }
            let block_data = serde_json::json!({
                "id": last_block.id + 1,
                "timestamp": Utc::now().timestamp(),
                "previous_hash": last_block.hash,
                "data": data,
                "nonce": nonce,
            });
            let (hash, hash_bits) = hash_json(block_data.to_string());
            if hash_bits.starts_with(DIFFICULTY_PREFIX) {
                break Block {
                    id: last_block.id + 1,
                    timestamp: Utc::now().timestamp(),
                    previous_hash: last_block.hash.clone(),
                    data: data,
                    nonce,
                    hash,
                };
            }
            nonce += 1;
        };

        self.try_add_block(block);
        self.transactions = vec![];
    }

    fn try_add_block(&mut self, block: Block) {
        let last_block = self.blocks.last().expect("Chain needs at lest one block!");
        if self.is_block_valid(&block, last_block) {
            self.blocks.push(block);
        } else {
            panic!("Invalid block!");
        }
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            return false;
        } else if block.id != previous_block.id + 1 {
            return false;
        } else if block.calculate_hash() != block.hash {
            return false;
        } else {
            return true;
        }
    }
}

struct Block {
    id: u64,
    hash: String,
    previous_hash: String,
    timestamp: i64,
    data: String,
    nonce: u64,
}

impl Block {
    fn calculate_hash(&self) -> String {
        let json = serde_json::json!({
            "id": self.id,
            "previous_hash": self.previous_hash,
            "timestamp": self.timestamp,
            "data": self.data,
            "nonce": self.nonce,
        });

        let mut hasher = Sha256::new();
        hasher.update(json.to_string().as_bytes());
        let hash_bytes = hasher.finalize().as_slice().to_owned();
        let hash = hex::encode(&hash_bytes);
        hash
    }

    fn display(&self) {
        let prev = if self.previous_hash.len() < 24 {
            &self.previous_hash
        } else {
            &self.previous_hash[0..24]
        };
        println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
        println!("┃ {} ┃", format!("{:<24 }", prev));
        println!("┠──────────────────────────┨");
        println!("┃ id = {} ┃", format!("{:<19 }", self.id));
        println!("┃ timestamp = {} ┃", format!("{:<12 }", self.timestamp));
        println!("┃ nonce = {} ┃", format!("{:<16 }", self.nonce));
        println!("┃ data =                   ┃");
        for tx in self.data.split("\n") {
            println!("┃   {} ┃", format!("{:<22 }", tx));
        }
        println!("┠──────────────────────────┨");
        println!("┃ {} ┃", &self.hash[0..24]);
        println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    }
}

fn hash_json(json: String) -> (String, String) {
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    let hash_bytes = hasher.finalize().as_slice().to_owned();
    let mut hash_bits = String::new();
    for c in &hash_bytes {
        hash_bits.push_str(&format!("{:b}", c));
    }
    let hash = hex::encode(&hash_bytes);
    (hash, hash_bits)
}

fn main() {
    let mut chain = Chain::new();
    chain.genesis();
    
    chain.add_transaction("001: 10.23 -> 002");
    chain.mine_block();
    
    chain.add_transaction("003: 9.00 -> 001");
    chain.add_transaction("002: 2.19 -> 004");
    chain.add_transaction("005: 3.12 -> 004");
    chain.mine_block();

    chain.add_transaction("011: 34.12 -> 005");
    chain.add_transaction("009: 6.12 -> 007");
    chain.mine_block();

    chain.view();
}
