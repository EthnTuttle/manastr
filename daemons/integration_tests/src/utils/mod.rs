use rand::Rng;

/// Generates a random nonce for cryptographic operations
pub fn generate_nonce() -> String {
    let mut rng = rand::thread_rng();
    let nonce: u64 = rng.gen();
    format!("{:x}", nonce)
}

/// Creates a deterministic key from a seed string
pub fn create_deterministic_key(seed: &str) -> String {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
} 