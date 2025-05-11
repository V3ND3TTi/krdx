use serde::{Deserialize, Serialize};

use crate::wallet::Wallet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,           // base64 public key
    pub recipient_alias: String,  // KRDx... address for UI
    pub recipient_pubkey: String, // base64 public key of recipient
    pub amount: u64,              // in Koins
    pub signature: String,        // base64 signature
    pub timestamp: u64,           // Unix time
}

impl Transaction {
    pub fn new(
        sender: &str,
        recipient_alias: &str,
        recipient_pubkey: &str,
        amount: u64,
        signature: &str,
        timestamp: u64,
    ) -> Self {
        Transaction {
            sender: sender.to_string(),
            recipient_alias: recipient_alias.to_string(),
            recipient_pubkey: recipient_pubkey.to_string(),
            amount,
            signature: signature.to_string(),
            timestamp,
        }
    }

    pub fn is_valid(&self) -> bool {
        let message = format!(
            "{}:{}:{}:{}",
            self.sender, self.recipient_alias, self.amount, self.timestamp
        );
        Wallet::verify_signature(&self.sender, &message, &self.signature)
    }
}
