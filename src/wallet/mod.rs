use base64::{Engine as _, engine::general_purpose};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use ring::signature::{ED25519, UnparsedPublicKey};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Wallet {
    pub keypair: Keypair,      // stays internal
    pub public_key: PublicKey, // exposed for faucet tracking / identity
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        Wallet {
            public_key: keypair.public,
            keypair,
        }
    }

    pub fn get_address(&self) -> String {
        // Convert public key bytes to hex string
        let pub_bytes = self.keypair.public.as_bytes();
        let hash = Sha256::digest(pub_bytes);
        let short_hash = &hash[..20]; // 20 bytes = 160 bits = 40 hex chars
        format!("KRDx{}", hex::encode(short_hash))
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
}

#[cfg(test)]
mod tests {
    use crate::wallet::Wallet;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_wallet_signature_and_address() {
        let wallet = Wallet::new();
        let message = b"Check-in at block 12345";

        let sig = wallet.sign(message);
        let valid = wallet.verify(message, &sig);
        assert!(valid, "Signature should verify");

        let hash = Sha256::digest(wallet.public_key.as_bytes());
        let short_hash = &hash[..20];
        let expected_addr = format!("KRDx{}", hex::encode(short_hash));

        assert_eq!(wallet.get_address(), expected_addr);
    }
}
