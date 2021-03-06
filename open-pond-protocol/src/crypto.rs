use rand::rngs::OsRng;
use schnorrkel::{Keypair, Signature, PublicKey, MiniSecretKey};
use schnorrkel::signing_context;
use std::fs;

/// Structure encapsulating authentication signature keys
pub struct AuthKeyPair {
    keypair: Keypair,
}

impl AuthKeyPair {
    /// Function that loads saved keypair or generates a new one if prior keypair is inadequate
    pub fn load() -> AuthKeyPair {
        let seed = match fs::read("private/auth-keypair") {
            Ok(bytes) => MiniSecretKey::from_bytes(&bytes).unwrap(),
            _ => AuthKeyPair::generate(),
        };
        let keypair = seed.expand_to_keypair(MiniSecretKey::UNIFORM_MODE);
        AuthKeyPair {
            keypair,
        }
    }

    /// Function that signs a message with authentication private key
    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        let context = signing_context(b"message authentication");
        self.keypair.sign(context.bytes(message)).to_bytes().to_vec()
    }

    /// Function that verifies a message against a provided public key
    pub fn verify(&self, message: &[u8], token: Vec<u8>, pubkey: Vec<u8>) {
        let context = signing_context(b"message authentication");
        let signature = Signature::from_bytes(&token).unwrap();
        let public_key = PublicKey::from_bytes(&pubkey).unwrap();
        public_key.verify(context.bytes(message), &signature).unwrap();
    }

    // Private function used to generate and save new keypair
    fn generate() -> MiniSecretKey {
        let key = MiniSecretKey::generate_with(OsRng);
        fs::write("private/auth-keypair", key.as_bytes()).unwrap();
        return key;
    }
}