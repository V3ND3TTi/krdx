use crate::config::*;
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub merkle_root: String,
    pub hash: String,
    pub nonce: u64,
    pub data: Vec<String>, // Later this will be serialized structs, for now use strings
    pub miner_address: String,
}

impl Block {
    pub fn genesis() -> Self {
        let index = 0;
        let timestamp = Utc::now();
        let previous_hash = GENESIS_PREV_HASH.to_string();
        let data = vec![GENESIS_DATA.to_string()];
        let merkle_root = Self::calculate_merkle_root(&data);
        let nonce = 0;
        let miner_address = "GENESIS".to_string();

        let hash = Self::calculate_hash(
            index,
            timestamp,
            &previous_hash,
            &merkle_root,
            nonce,
            &miner_address,
        );

        Block {
            index,
            timestamp,
            previous_hash,
            merkle_root,
            hash,
            nonce,
            data,
            miner_address,
        }
    }

    pub fn calculate_merkle_root(data: &[String]) -> String {
        if data.is_empty() {
            return "0".repeat(64); // default empty root
        }

        let mut hashes: Vec<String> = data
            .iter()
            .map(|item| {
                let mut hasher = Sha256::new();
                hasher.update(item.as_bytes());
                format!("{:x}", hasher.finalize())
            })
            .collect();

        while hashes.len() > 1 {
            if hashes.len() % 2 != 0 {
                hashes.push(hashes.last().unwrap().clone()); // Duplicate last if odd count
            }

            let mut new_level = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let combined = format!("{}{}", hashes[i], hashes[i + 1]);
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                new_level.push(format!("{:x}", hasher.finalize()));
            }
            hashes = new_level;
        }

        hashes[0].clone()
    }

    pub fn calculate_hash(
        index: u64,
        timestamp: DateTime<Utc>,
        previous_hash: &str,
        merkle_root: &str,
        nonce: u64,
        miner_address: &str,
    ) -> String {
        let input = format!(
            "{}{}{}{}{}{}",
            index, timestamp, previous_hash, merkle_root, nonce, miner_address
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(
        index: u64,
        previous_hash: String,
        data: Vec<String>,
        miner_address: String,
    ) -> Self {
        let timestamp = Utc::now();
        let merkle_root = Self::calculate_merkle_root(&data);
        let mut nonce = 0;

        loop {
            let hash = Self::calculate_hash(
                index,
                timestamp,
                &previous_hash,
                &merkle_root,
                nonce,
                &miner_address,
            );

            if hash.starts_with(&"0".repeat(BLOCK_DIFFICULTY)) {
                return Block {
                    index,
                    timestamp,
                    previous_hash,
                    merkle_root,
                    hash,
                    nonce,
                    data,
                    miner_address,
                };
            }

            nonce += 1;
        }
    }
}
