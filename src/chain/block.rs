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
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: Vec<String>, nonce: u64) -> Self {
        let timestamp = Utc::now();
        let merkle_root = Self::calculate_merkle_root(&data);
        let hash = Self::calculate_hash(index, timestamp, &previous_hash, &merkle_root, nonce);
        Block {
            index,
            timestamp,
            previous_hash,
            merkle_root,
            hash,
            nonce,
            data,
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
    ) -> String {
        let input = format!(
            "{}{}{}{}{}",
            index, timestamp, previous_hash, merkle_root, nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
