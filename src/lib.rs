use chrono::Utc;
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DIFFICULTY_PREFIX: &str = "00";

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c))
    }
    res
}

pub struct App {
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub timestamp: i64,
    pub nonce: u64,
    pub hash: String,
    pub previous_hash: String,
    pub data: String,
}

fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id,
        "timestamp": timestamp,
        "previous_hash": previous_hash,
        "data": data,
        "nonce": nonce,
    });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("Mining block..");
    let mut rng = rand::thread_rng();
    let mut nonce = 0;
    let mut iteration = 0;

    loop {
        if iteration % 100000 == 0 {
            info!("iteration {}", iteration);
        }
        iteration += 1;

        let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = hash_to_binary_representation(&hash);
        if binary_hash.starts_with(DIFFICULTY_PREFIX) {
            info!("mined! nonce: {}, hash: {}", nonce, hex::encode(&hash));
            return (nonce, hex::encode(hash));
        }
        nonce = rng.gen();
    }
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let (nonce, hash) = mine_block(id, timestamp, &previous_hash, &data);
        Block {
            id,
            timestamp,
            nonce,
            hash,
            previous_hash,
            data,
        }
    }

    fn calculate_hash(&self) -> Vec<u8> {
        return calculate_hash(self.id, self.timestamp, &self.previous_hash, &self.data, self.nonce);
    }

    fn can_extend_to(&self, next_block: &Block) -> bool {
        if next_block.id != self.id + 1 {
            return false;
        }
        if next_block.previous_hash != self.hash {
            return false;
        }

        if let Ok(decoded_hash) = hex::decode(&next_block.hash) {
            if !hash_to_binary_representation(&decoded_hash).starts_with(DIFFICULTY_PREFIX) {
                return false;
            }
        } else {
            return false;
        }

        hex::encode(next_block.calculate_hash()) == next_block.hash
    }

    pub fn mine_next_block(&self, data: String) -> Block {
        Block::new(self.id + 1, self.hash.clone(), data)
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self { blocks: vec![] };
        app.genesis();
        app
    }

    pub fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
        };
        self.blocks.push(genesis_block);
    }

    pub fn try_add_block(&mut self, block: Block) -> Result<(), String> {
        if self.blocks.last().unwrap().can_extend_to(&block) {
            self.blocks.push(block);
        } else {
            return Err("could not add invalid block".to_string());
        }
        Ok(())
    }

    pub fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
        for i in 1..chain.len() {
            if !chain[i - 1].can_extend_to(&chain[i]) {
                return false;
            }
        }

        true
    }

    pub fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);
        if is_local_valid && is_remote_valid {
            if local.len() >= remote.len() {
                return local;
            }
            return remote;
        }

        if !is_local_valid && !is_remote_valid {
            panic!("local and remote chains are both invalid");
        }

        if is_local_valid {
            return local;
        }
        return remote;
    }

    pub fn get_last_block(&self) -> &Block {
        self.blocks.last().unwrap()
    }
}

pub mod p2p;
