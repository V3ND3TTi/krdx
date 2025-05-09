use super::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block =
            Block::new(0, String::from("0"), vec![String::from("Genesis Block")], 0);
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: Vec<String>) {
        let previous_block = self.chain.last().expect("Chain should never be empty");
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            data,
            0, // nonce = 0 for now — PoW/PoP logic comes later
        );
        self.chain.push(new_block);
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.previous_hash,
                &current.merkle_root,
                current.nonce,
            );

            if current.hash != recalculated {
                return false;
            }
        }
        true
    }
}
