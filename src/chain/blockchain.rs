use crate::chain::block::Block;
use crate::chain::ledger::Ledger;
use crate::config::*;

#[derive(Debug)]
pub struct CoinbaseDistribution {
    pub faucet_address: String,
    pub founder_addresses: Vec<String>,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub ledger: Ledger,
    pub dist: CoinbaseDistribution,
}

impl Blockchain {
    pub fn new(dist: CoinbaseDistribution) -> Self {
        Self {
            chain: vec![Block::genesis()],
            ledger: Ledger::new(),
            dist,
        }
    }

    pub fn add_block(&mut self, data: Vec<String>, miner_address: String) {
        let last_block = self
            .chain
            .last()
            .expect("Blockchain should have at least the genesis block");

        let new_block = Block::mine_block(
            last_block.index + 1,
            last_block.hash.clone(),
            data,
            miner_address.clone(),
        );

        let faucet_cut = (BLOCK_REWARD_KOIN * 80) / 100;
        let leftover = BLOCK_REWARD_KOIN - faucet_cut;
        let founder_cut_each = leftover / self.dist.founder_addresses.len() as u64;

        self.ledger.credit(&self.dist.faucet_address, faucet_cut);

        for founder in &self.dist.founder_addresses {
            self.ledger.credit(founder, founder_cut_each);
        }

        self.chain.push(new_block);
    }

    pub fn is_valid_chain(chain: &[Block]) -> bool {
        for i in 1..chain.len() {
            let current = &chain[i];
            let previous = &chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.previous_hash,
                &current.merkle_root,
                current.nonce,
                &current.miner_address,
            );

            if current.hash != recalculated {
                return false;
            }
        }

        true
    }

    pub fn replace_chain(&mut self, new_chain: Vec<Block>) -> bool {
        if new_chain.len() <= self.chain.len() {
            println!("Received chain is not longer than the current chain. Ignoring.");
            return false;
        }

        if !Blockchain::is_valid_chain(&new_chain) {
            println!("Received chain is invalid. Ignoring.");
            return false;
        }

        println!("Replacing chain with the new, valid chain.");
        self.chain = new_chain;
        true
    }
}
