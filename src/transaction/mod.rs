use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::wallet::Wallet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub signature: String,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(
        sender: &str,
        recipient: &str,
        amount: u64,
        signature: &str,
        timestamp: u64,
    ) -> Self {
        Transaction {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount,
            signature: signature.to_string(),
            timestamp,
        }
    }

    pub fn create(
        sender: &str,
        recipient: &str,
        amount: u64,
        sign_fn: impl Fn(&str) -> String,
    ) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        let message = format!("{}:{}:{}:{}", sender, recipient, amount, timestamp);
        let signature = sign_fn(&message);

        Transaction::new(sender, recipient, amount, &signature, timestamp)
    }

    pub fn is_valid(&self) -> bool {
        let message = format!(
            "{}:{}:{}:{}",
            self.sender, self.recipient, self.amount, self.timestamp
        );
        Wallet::verify_signature(&self.sender, &message, &self.signature)
    }
}
