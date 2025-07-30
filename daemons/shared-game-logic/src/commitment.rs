use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

/// Commitment/Reveal cryptographic functions for player-driven matches
/// This ensures fair play by preventing players from changing moves after seeing opponent's actions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
    pub hash: String,
    pub data_type: CommitmentType,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitmentType {
    CashuTokens, // Commitment to Cashu token secrets
    Army,        // Commitment to generated army
    Moves,       // Commitment to round moves (positions + abilities)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitReveal {
    pub commitment: String,
    pub data: String,
    pub nonce: String,
}

/// Create a cryptographic commitment to data with a random nonce
pub fn create_commitment(data: &str, nonce: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hasher.update(nonce.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify that revealed data matches the original commitment
pub fn verify_commitment(commitment: &str, revealed_data: &str, nonce: &str) -> bool {
    let computed_commitment = create_commitment(revealed_data, nonce);
    commitment == computed_commitment
}

/// Generate a secure random nonce for commitment schemes
pub fn generate_nonce() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect()
}

/// Create commitment to Cashu token secrets
pub fn commit_to_cashu_tokens(token_secrets: &[String], nonce: &str) -> String {
    let data = serde_json::to_string(token_secrets).unwrap();
    create_commitment(&data, nonce)
}

/// Create commitment to army data (generated units)
pub fn commit_to_army(army_data: &str, nonce: &str) -> String {
    create_commitment(army_data, nonce)
}

/// Create commitment to round moves (unit positions and abilities)
pub fn commit_to_moves(positions: &[u8], abilities: &[String], nonce: &str) -> String {
    let moves_data = serde_json::to_string(&(positions, abilities)).unwrap();
    create_commitment(&moves_data, nonce)
}

/// Verify Cashu token commitment
pub fn verify_cashu_commitment(commitment: &str, revealed_tokens: &[String], nonce: &str) -> bool {
    let revealed_data = serde_json::to_string(revealed_tokens).unwrap();
    verify_commitment(commitment, &revealed_data, nonce)
}

/// Verify army commitment
pub fn verify_army_commitment(commitment: &str, revealed_army: &str, nonce: &str) -> bool {
    verify_commitment(commitment, revealed_army, nonce)
}

/// Verify moves commitment
pub fn verify_moves_commitment(
    commitment: &str,
    revealed_positions: &[u8],
    revealed_abilities: &[String],
    nonce: &str,
) -> bool {
    let revealed_data = serde_json::to_string(&(revealed_positions, revealed_abilities)).unwrap();
    verify_commitment(commitment, &revealed_data, nonce)
}

/// Hash function for Nostr event IDs and other data integrity
pub fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

// WASM exports for web client usage
#[wasm_bindgen]
pub fn wasm_create_commitment(data: &str, nonce: &str) -> String {
    create_commitment(data, nonce)
}

#[wasm_bindgen]
pub fn wasm_verify_commitment(commitment: &str, revealed_data: &str, nonce: &str) -> bool {
    verify_commitment(commitment, revealed_data, nonce)
}

#[wasm_bindgen]
pub fn wasm_generate_nonce() -> String {
    generate_nonce()
}

#[wasm_bindgen]
pub fn wasm_commit_to_cashu_tokens(token_secrets: JsValue, nonce: &str) -> String {
    let tokens: Vec<String> = serde_wasm_bindgen::from_value(token_secrets).unwrap();
    commit_to_cashu_tokens(&tokens, nonce)
}

#[wasm_bindgen]
pub fn wasm_verify_cashu_commitment(
    commitment: &str,
    revealed_tokens: JsValue,
    nonce: &str,
) -> bool {
    let tokens: Vec<String> = serde_wasm_bindgen::from_value(revealed_tokens).unwrap();
    verify_cashu_commitment(commitment, &tokens, nonce)
}

#[wasm_bindgen]
pub fn wasm_commit_to_moves(positions: &[u8], abilities: JsValue, nonce: &str) -> String {
    let abilities_vec: Vec<String> = serde_wasm_bindgen::from_value(abilities).unwrap();
    commit_to_moves(positions, &abilities_vec, nonce)
}

#[wasm_bindgen]
pub fn wasm_verify_moves_commitment(
    commitment: &str,
    positions: &[u8],
    abilities: JsValue,
    nonce: &str,
) -> bool {
    let abilities_vec: Vec<String> = serde_wasm_bindgen::from_value(abilities).unwrap();
    verify_moves_commitment(commitment, positions, &abilities_vec, nonce)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_verify_cycle() {
        let data = "test_data_12345";
        let nonce = "random_nonce_67890";

        let commitment = create_commitment(data, nonce);
        assert!(verify_commitment(&commitment, data, nonce));

        // Verify fails with wrong data
        assert!(!verify_commitment(&commitment, "wrong_data", nonce));

        // Verify fails with wrong nonce
        assert!(!verify_commitment(&commitment, data, "wrong_nonce"));
    }

    #[test]
    fn test_cashu_token_commitment() {
        let tokens = vec![
            "token_secret_1".to_string(),
            "token_secret_2".to_string(),
            "token_secret_3".to_string(),
        ];
        let nonce = "test_nonce";

        let commitment = commit_to_cashu_tokens(&tokens, nonce);
        assert!(verify_cashu_commitment(&commitment, &tokens, nonce));

        // Verify fails with different tokens
        let different_tokens = vec!["different_token".to_string()];
        assert!(!verify_cashu_commitment(
            &commitment,
            &different_tokens,
            nonce
        ));
    }

    #[test]
    fn test_moves_commitment() {
        let positions = vec![1, 2, 3, 4];
        let abilities = vec!["boost".to_string(), "shield".to_string()];
        let nonce = "moves_nonce";

        let commitment = commit_to_moves(&positions, &abilities, nonce);
        assert!(verify_moves_commitment(
            &commitment,
            &positions,
            &abilities,
            nonce
        ));

        // Verify fails with different moves
        let different_positions = vec![5, 6, 7, 8];
        assert!(!verify_moves_commitment(
            &commitment,
            &different_positions,
            &abilities,
            nonce
        ));
    }

    #[test]
    fn test_deterministic_hashing() {
        let data = "deterministic_test";
        let nonce = "fixed_nonce";

        let commitment1 = create_commitment(data, nonce);
        let commitment2 = create_commitment(data, nonce);

        // Same inputs should produce same commitment
        assert_eq!(commitment1, commitment2);
    }

    #[test]
    fn test_nonce_generation() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();

        // Different nonces should be generated
        assert_ne!(nonce1, nonce2);

        // Nonces should be hex strings of expected length (64 chars = 32 bytes * 2)
        assert_eq!(nonce1.len(), 64);
        assert_eq!(nonce2.len(), 64);

        // Should be valid hex
        assert!(nonce1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(nonce2.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
