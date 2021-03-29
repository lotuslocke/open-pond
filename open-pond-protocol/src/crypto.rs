use rand::rngs::OsRng;
use schnorrkel::signing_context;
use schnorrkel::{Keypair, MiniSecretKey, PublicKey, Signature, SignatureError};
use std::fs;
use thiserror::Error;

/// Structure encapsulating authentication signature keys
#[derive(Debug)]
pub struct AuthKey {
    keypair: Keypair,
    public: PublicKey,
}

impl AuthKey {
    /// Function that loads saved keypair or generates a new one if prior keypair is inadequate
    pub fn load() -> CryptoResult<AuthKey> {
        let seed = match fs::read("private/auth-keypair") {
            Ok(bytes) => match MiniSecretKey::from_bytes(&bytes) {
                Ok(seed) => seed,
                _ => return Err(CryptoError::InvalidSeedFormat),
            },
            _ => AuthKey::generate()?,
        };
        let keypair = seed.expand_to_keypair(MiniSecretKey::UNIFORM_MODE);
        let public = seed.expand_to_public(MiniSecretKey::UNIFORM_MODE);
        Ok(AuthKey { keypair, public })
    }

    // Function used to generate and save new keypair
    pub fn generate() -> CryptoResult<MiniSecretKey> {
        let key = MiniSecretKey::generate_with(OsRng);
        if fs::write("private/auth-keypair", key.as_bytes()).is_err() {
            return Err(CryptoError::FailedSeedGeneration);
        };
        Ok(key)
    }

    /// Function that signs a message with authentication private key
    pub fn sign(&self, message: &[u8]) -> CryptoResult<Vec<u8>> {
        let context = signing_context(b"message authentication");
        Ok(self
            .keypair
            .sign(context.bytes(message))
            .to_bytes()
            .to_vec())
    }

    /// Function that produces a user's public key
    pub fn get_public(&self) -> Vec<u8> {
        self.public.to_bytes().to_vec()
    }

    /// Function that verifies a message against a provided public key
    pub fn verify(message: &[u8], token: Vec<u8>, pubkey: Vec<u8>) -> CryptoResult<bool> {
        let context = signing_context(b"message authentication");
        let signature = Signature::from_bytes(&token)?;
        let public_key = PublicKey::from_bytes(&pubkey)?;
        match public_key.verify(context.bytes(message), &signature) {
            Ok(_) => Ok(true),
            _ => Ok(false),
        }
    }
}

// Verify that one authentication keypair matches another
impl PartialEq for AuthKey {
    fn eq(&self, other: &Self) -> bool {
        self.keypair.to_bytes().to_vec() == other.keypair.to_bytes().to_vec()
    }
}

#[derive(Error, Debug)]
/// Errors generated from cryptographic operations
pub enum CryptoError {
    #[error("Failed to load stored seed - invalid format")]
    InvalidSeedFormat,
    #[error("Failed to generate new seed - failed save")]
    FailedSeedGeneration,
    #[error("Error deserializing crypto component from bytearray: {}", err)]
    Deserialization { err: String },
}

// Conversion from schnorrkel signature error to cryptography error
impl From<SignatureError> for CryptoError {
    fn from(error: SignatureError) -> Self {
        let err = format!("{}", error);
        CryptoError::Deserialization { err }
    }
}

// Convenience alias for cryptographic results
type CryptoResult<T> = Result<T, CryptoError>;
