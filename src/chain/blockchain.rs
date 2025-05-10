use crate::chain::block::Block;
use crate::chain::ledger::Ledger;

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

const BLOCK_REWARD_KOIN: u64 = 4_200_000_000; // 42 KRD in Koin

impl Blockchain {
    pub fn new(dist: CoinbaseDistribution) -> Self {
        Self {
            chain: vec![Block::genesis_block()],
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
                &current.miner_address,
            );

            if current.hash != recalculated {
                return false;
            }
        }
        true
    }
}
