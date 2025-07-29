// 🏛️ GAMING WALLET: Custom CDK Extension for Revolutionary Gaming
// =============================================================
//
// This wallet extends CDK functionality to expose the low-level cryptographic
// primitives (x, C values) needed for the revolutionary gaming architecture.
//
// 🚀 REVOLUTIONARY PURPOSE:
// - Access unblinded signature C values for army generation
// - Maintain full CDK compatibility while exposing gaming-specific data
// - Provide tamper-proof randomness from mint signatures
// - Enable 1 mana token = 1 army = 1 match economic model

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use cdk::{
    nuts::{Proof, Id, PublicKey},
    secret::Secret,
    Amount,
};
use cdk_fake_wallet::FakeWallet;
use sha2::{Digest, Sha256};
use nostr::Keys;

/// 🏛️ CANONICAL GAMING TOKEN: Complete Cashu token with gaming metadata
/// 
/// Extends CDK Proof with access to the cryptographic primitives needed
/// for tamper-proof army generation in the revolutionary gaming paradigm.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GamingToken {
    // CDK standard fields
    pub proof: Proof,           // Standard CDK proof structure
    pub amount: Amount,         // Token denomination
    pub keyset_id: Id,          // Mint keyset identifier
    
    // 🚀 REVOLUTIONARY GAMING FIELDS
    pub x_value: String,        // Blind factor (secret) - needed for reveals
    pub c_value: String,        // Unblinded signature hex string - SOURCE OF ARMY RANDOMNESS
    pub c_value_bytes: [u8; 32], // C value as 32-byte array for army generation
}

impl GamingToken {
    /// Get C value as 32-byte array for army generation
    /// This provides access to the full 256-bit unblinded signature
    pub fn get_c_value_bytes(&self) -> &[u8; 32] {
        &self.c_value_bytes
    }
    
    /// Generate army from this token's C value using shared combat logic
    /// This is the core of tamper-proof army generation
    pub fn generate_army(&self, league_id: u8) -> [shared_game_logic::game_state::Unit; 4] {
        use shared_game_logic::combat::generate_army_from_cashu_c_value;
        generate_army_from_cashu_c_value(&self.c_value_bytes, league_id)
    }
    
    /// Verify this token can generate the claimed army
    /// Ensures players cannot forge army compositions
    pub fn verify_army_generation(&self, claimed_army_hash: &str) -> bool {
        // Generate army from this token's C value using shared combat logic
        let army = self.generate_army(0); // Use default league for verification
        let army_json = serde_json::to_string(&army).unwrap();
        
        // Hash the generated army and compare
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(army_json.as_bytes());
        let computed_hash = format!("{:x}", hasher.finalize());
        
        computed_hash == claimed_army_hash
    }
}

/// 🏛️ GAMING WALLET: CDK Extension for Revolutionary Gaming
/// 
/// Extends FakeWallet to provide access to the cryptographic primitives
/// needed for the zero-coordination gaming architecture.
#[derive(Clone)]
pub struct GamingWallet {
    // Underlying CDK wallet for standard operations
    cdk_wallet: FakeWallet,
    
    // 🚀 REVOLUTIONARY GAMING EXTENSIONS
    gaming_tokens: HashMap<String, GamingToken>,  // Track tokens with C values
    mint_url: String,
    
    // Token generation state for deterministic testing
    token_counter: u64,
}

impl GamingWallet {
    /// Create new gaming wallet with CDK integration
    pub fn new(mint_url: String) -> Self {
        // Initialize with standard CDK FakeWallet configuration
        let fee_reserve = cdk::types::FeeReserve {
            min_fee_reserve: 1.into(),
            percent_fee_reserve: 0.01, // 1% fee
        };
        let payment_states = HashMap::new();
        let fail_payment_check = std::collections::HashSet::new();
        let payment_delay = 1; // 1 second delay for testing
        
        let cdk_wallet = FakeWallet::new(fee_reserve, payment_states, fail_payment_check, payment_delay);
        
        Self {
            cdk_wallet,
            gaming_tokens: HashMap::new(),
            mint_url,
            token_counter: 0,
        }
    }
    
    /// 🏛️ CANONICAL MINTING: Create gaming tokens with C values
    /// 
    /// This is the AUTHORITATIVE method for creating mana tokens with
    /// the cryptographic primitives needed for army generation.
    pub async fn mint_gaming_tokens(&mut self, amount: u64, currency: &str) -> Result<Vec<GamingToken>> {
        tracing::info!("🪙 GAMING WALLET: Minting {} {} tokens with C values for army generation", amount, currency);
        
        let mut gaming_tokens = Vec::new();
        
        for i in 0..amount {
            // Generate deterministic but unique values for testing
            // In production: these would come from actual CDK minting process
            let x_value = format!("x_blind_factor_{}_{}_test", self.token_counter, i);
            let c_value = self.generate_deterministic_c_value(self.token_counter, i);
            
            // Create CDK proof structure (simplified for testing)
            let proof = Proof {
                amount: Amount::from(1u64), // 1 mana token
                secret: Secret::new(x_value.clone()),
                c: PublicKey::from_hex(&c_value).map_err(|e| {
                    tracing::error!("Failed to create PublicKey from hex '{}': {}", c_value, e);
                    e
                }).unwrap(),
                witness: None,
                dleq: None,
                keyset_id: Id::from_bytes(&[0u8; 8]).unwrap(),
            };
            
            // Convert C value hex string to 32-byte array
            let c_value_bytes = self.hex_to_32_bytes(&c_value);
            
            // Create gaming token with both CDK data and gaming extensions
            let gaming_token = GamingToken {
                proof: proof.clone(),
                amount: Amount::from(1u64),
                keyset_id: Id::from_bytes(&[0u8; 8]).unwrap(),
                x_value: x_value.clone(),
                c_value: c_value.clone(),
                c_value_bytes,
            };
            
            // Store token for future reference
            let token_id = format!("token_{}_{}", self.token_counter, i);
            self.gaming_tokens.insert(token_id, gaming_token.clone());
            
            gaming_tokens.push(gaming_token);
        }
        
        self.token_counter += 1;
        
        tracing::info!("✅ GAMING WALLET: Created {} gaming tokens with unique C values for army generation", gaming_tokens.len());
        Ok(gaming_tokens)
    }
    
    /// Generate deterministic C value for testing (simulates mint signature)
    /// In production: C values come from mint's cryptographic signatures
    fn generate_deterministic_c_value(&self, batch: u64, index: u64) -> String {
        // Create deterministic seed for key generation
        let seed = format!("mint_c_value_{}_{}_{}", self.mint_url, batch, index);
        
        // Generate deterministic Nostr keys from seed (both are secp256k1)
        // This simulates what a Cashu mint would provide as unblinded signature
        let keys = Keys::parse(&format!("{:0>64}", hex::encode(
            Sha256::digest(seed.as_bytes())
        ))).unwrap_or_else(|_| Keys::generate());
        
        // Get the compressed public key in hex format (33 bytes: prefix + 32 bytes)
        // This is the format that CDK expects for PublicKey
        let pubkey_32_bytes = keys.public_key().to_hex();
        // Add compressed public key prefix (02 or 03) - we'll use 02 for simplicity
        format!("02{}", pubkey_32_bytes)
    }
    
    /// Convert hex string to 32-byte array for army generation
    /// Extracts the 32 bytes from compressed public key (skipping 02/03 prefix)
    fn hex_to_32_bytes(&self, hex_string: &str) -> [u8; 32] {
        // Remove 0x prefix if present
        let clean_hex = hex_string.strip_prefix("0x").unwrap_or(hex_string);
        
        // Decode hex string to bytes
        let bytes = hex::decode(clean_hex).unwrap_or_else(|_| {
            // If hex decode fails, use hash of the string as fallback
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(hex_string.as_bytes());
            hasher.finalize().to_vec()
        });
        
        // Extract 32 bytes for army generation (skip 02/03 prefix if it's a compressed pubkey)
        let start_idx = if bytes.len() == 33 && (bytes[0] == 0x02 || bytes[0] == 0x03) { 1 } else { 0 };
        let mut result = [0u8; 32];
        
        // Copy available bytes, pad with zeros if needed
        let copy_len = std::cmp::min(32, bytes.len() - start_idx);
        if copy_len > 0 {
            result[..copy_len].copy_from_slice(&bytes[start_idx..start_idx + copy_len]);
        }
        
        result
    }
    
    /// Get gaming token by ID for army generation
    pub fn get_gaming_token(&self, token_id: &str) -> Option<&GamingToken> {
        self.gaming_tokens.get(token_id)
    }
    
    /// Get all gaming tokens (for match wagering)
    pub fn get_all_gaming_tokens(&self) -> Vec<GamingToken> {
        self.gaming_tokens.values().cloned().collect()
    }
    
    /// Verify token and return C value bytes for army generation
    /// This is how the game validates that armies come from real tokens
    pub fn verify_and_get_c_value_bytes(&self, token_proof: &Proof) -> Option<[u8; 32]> {
        // Find gaming token by proof
        for gaming_token in self.gaming_tokens.values() {
            if gaming_token.proof.secret.to_string() == token_proof.secret.to_string()
                && gaming_token.proof.c.to_hex() == token_proof.c.to_hex() {
                return Some(gaming_token.c_value_bytes);
            }
        }
        None
    }
}

/// Helper function to convert gaming tokens to CDK proofs for reveals
pub fn gaming_tokens_to_proofs(gaming_tokens: &[GamingToken]) -> Vec<Proof> {
    gaming_tokens.iter().map(|token| token.proof.clone()).collect()
}

/// Helper function to extract C value bytes for army generation
pub fn extract_c_value_bytes(gaming_tokens: &[GamingToken]) -> Vec<[u8; 32]> {
    gaming_tokens.iter().map(|token| token.c_value_bytes).collect()
}

/// Main function for gaming wallet testing/demonstration
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();

    tracing::info!("🏛️ GAMING WALLET: Demonstrating revolutionary CDK extension");
    
    let mut wallet = GamingWallet::new("http://localhost:3333".to_string());
    
    // Demonstrate minting gaming tokens with C values
    let tokens = wallet.mint_gaming_tokens(5, "mana").await?;
    
    tracing::info!("🚀 Successfully created {} gaming tokens with unique army seeds", tokens.len());
    
    for (i, token) in tokens.iter().enumerate() {
        tracing::info!("Token {}: C value = {}, C bytes = {:?}", 
                      i + 1, &token.c_value[..16], &token.c_value_bytes[..4]);
        
        // Demonstrate army generation from Cashu C values
        let army = token.generate_army(0); // League 0 for testing
        tracing::info!("  Generated Army: {:?}", army.iter().map(|u| (u.attack, u.defense, u.health, u.ability)).collect::<Vec<_>>());
    }
    
    tracing::info!("✅ Gaming wallet demonstration complete - Army generation working!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_gaming_wallet_creation() {
        let wallet = GamingWallet::new("http://localhost:3333".to_string());
        assert_eq!(wallet.mint_url, "http://localhost:3333");
        assert_eq!(wallet.gaming_tokens.len(), 0);
    }
    
    #[tokio::test] 
    async fn test_mint_gaming_tokens() {
        let mut wallet = GamingWallet::new("http://localhost:3333".to_string());
        let tokens = wallet.mint_gaming_tokens(5, "mana").await.unwrap();
        
        assert_eq!(tokens.len(), 5);
        
        // Verify each token has unique C values
        let mut c_values = std::collections::HashSet::new();
        
        for token in &tokens {
            assert!(!token.c_value.is_empty());
            assert!(!token.x_value.is_empty());
            
            // Ensure uniqueness
            assert!(c_values.insert(token.c_value.clone()));
        }
    }
    
    #[tokio::test]
    async fn test_army_generation_deterministic() {
        let mut wallet = GamingWallet::new("http://localhost:3333".to_string());
        let tokens = wallet.mint_gaming_tokens(3, "mana").await.unwrap();
        
        // Same C value should always generate same army
        for token in &tokens {
            let army1 = token.generate_army(0);
            let army2 = token.generate_army(0);
            
            // Compare armies by serializing to JSON
            let army1_json = serde_json::to_string(&army1).unwrap();
            let army2_json = serde_json::to_string(&army2).unwrap();
            assert_eq!(army1_json, army2_json);
        }
    }
}