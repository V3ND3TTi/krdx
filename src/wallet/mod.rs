use crate::{transaction::Transaction, utils};
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use ring::signature::{ED25519, UnparsedPublicKey};

#[derive(Debug)]
pub struct Wallet {
    pub keypair: Keypair,
    pub public_key: PublicKey,
    pub address: String,
    pub btc_address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public;
        let pubkey_bytes = public_key.as_bytes();
        let address = general_purpose::STANDARD.encode(pubkey_bytes);
        let btc_address = utils::btc_style_address(pubkey_bytes);

        Wallet {
            keypair,
            public_key,
            address,
            btc_address,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.keypair.public.verify(message, signature).is_ok()
    }

    pub fn verify_signature(sender_address: &str, message: &str, signature: &str) -> bool {
        // Decode the base64 address (public key)
        let public_key_bytes = match general_purpose::STANDARD.decode(sender_address) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Decode the base64 signature
        let signature_bytes = match general_purpose::STANDARD.decode(signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Verify the signature using the public key and message
        let public_key = UnparsedPublicKey::new(&ED25519, public_key_bytes);
        public_key
            .verify(message.as_bytes(), &signature_bytes)
            .is_ok()
    }

    pub fn create_transaction(&self, recipient: &Wallet, amount: u64) -> Transaction {
        let timestamp = Utc::now().timestamp() as u64;
        let message = format!(
            "{}:{}:{}:{}",
            self.address, recipient.btc_address, amount, timestamp
        );
        let signature = self.sign(message.as_bytes());
        let signature_b64 = general_purpose::STANDARD.encode(signature.as_ref());

        Transaction::new(
            &self.address,
            &recipient.btc_address,
            &recipient.address,
            amount,
            &signature_b64,
            timestamp,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::wallet::Wallet;

    #[test]
    fn test_wallet_signature_and_address() {
        let wallet = Wallet::new();
        let message = b"Check-in at block 12345";

        let sig = wallet.sign(message);
        let valid = wallet.verify(message, &sig);
        assert!(valid, "Signature should verify");
    }

    #[test]
    fn test_wallet_btc_address_generation() {
        let wallet = Wallet::new();

        println!("Base64 Address: {}", wallet.address);
        println!("BTC Address: {}", wallet.btc_address);

        assert!(
            wallet.btc_address.starts_with("KRDx"),
            "Bech32 address must start with KRDx"
        );
        assert!(
            wallet.btc_address.len() > 10,
            "Bech32 address length looks too short"
        );
    }
}
