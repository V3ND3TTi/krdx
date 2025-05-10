use std::collections::HashMap;

#[derive(Debug)]
pub struct Ledger {
    pub balances: HashMap<String, u64>, // Address -> Balance in KRD
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn credit(&mut self, address: &str, amount: u64) {
        let entry = self.balances.entry(address.to_string()).or_insert(0);
        *entry += amount;
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}
