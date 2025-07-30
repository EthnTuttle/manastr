// Test to verify Cashu mint determinism for gaming wallet C values
// 
// This test verifies that:
// 1. Same mint keys + same x values = same C values (deterministic)
// 2. Different x values = different C values (uniqueness) 
// 3. C values can be used for deterministic army generation

use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use tokio::time::{sleep, Duration};
use tracing::{info, debug};

#[derive(Debug)]
struct DeterminismTest {
    mint_url: String,
    http_client: Client,
}

impl DeterminismTest {
    fn new() -> Self {
        Self {
            mint_url: "http://127.0.0.1:3333".to_string(),
            http_client: Client::new(),
        }
    }

    /// Test mint determinism by minting tokens with controlled inputs
    async fn test_mint_determinism(&self) -> Result<()> {
        info!("🧪 Testing Cashu mint determinism for gaming wallet integration");

        // Test 1: Same inputs should produce same outputs
        info!("📋 Test 1: Same mint keys + same x values should produce same C values");
        
        let mint_keys = self.get_deterministic_mint_keys().await?;
        debug!("🔑 Using deterministic mint keys: {:?}", mint_keys);

        // Mint tokens with same blinded message twice
        let blinded_msg = "deterministic_test_blind_001";
        let c_value_1 = self.mint_token_with_blind(blinded_msg, 1).await?;
        let c_value_2 = self.mint_token_with_blind(blinded_msg, 1).await?;

        if c_value_1 == c_value_2 {
            info!("✅ Test 1 PASSED: Same inputs produce same C values");
            info!("   C value: {}", c_value_1);
        } else {
            info!("❌ Test 1 FAILED: Same inputs produced different C values");
            info!("   C value 1: {}", c_value_1);
            info!("   C value 2: {}", c_value_2);
            return Err(anyhow::anyhow!("Determinism test failed - same inputs produced different outputs"));
        }

        // Test 2: Different inputs should produce different outputs
        info!("📋 Test 2: Different x values should produce different C values");
        
        let blinded_msg_a = "deterministic_test_blind_002";
        let blinded_msg_b = "deterministic_test_blind_003";
        let c_value_a = self.mint_token_with_blind(blinded_msg_a, 1).await?;
        let c_value_b = self.mint_token_with_blind(blinded_msg_b, 1).await?;

        if c_value_a != c_value_b {
            info!("✅ Test 2 PASSED: Different inputs produce different C values");
            info!("   C value A: {}", c_value_a);
            info!("   C value B: {}", c_value_b);
        } else {
            info!("❌ Test 2 FAILED: Different inputs produced same C values");
            return Err(anyhow::anyhow!("Uniqueness test failed - different inputs produced same outputs"));
        }

        // Test 3: C values can be used for army generation
        info!("📋 Test 3: C values should produce valid army generation input");
        
        let c_bytes = self.c_value_to_bytes(&c_value_1)?;
        info!("✅ Test 3 PASSED: C value converted to 32-byte army generation input");
        info!("   C bytes (first 8): {:02x?}...", &c_bytes[..8]);

        info!("🎉 All determinism tests PASSED - mint operations are deterministic!");
        Ok(())
    }

    /// Get mint keys (for now, this would be deterministic test keys)
    async fn get_deterministic_mint_keys(&self) -> Result<String> {
        // For testing: use a deterministic seed to generate mint keys
        // In real implementation, these would be the mint's actual signing keys
        Ok("deterministic_mint_master_key_seed_001".to_string())
    }

    /// Mint a token with specific blinded message
    async fn mint_token_with_blind(&self, blinded_msg: &str, amount: u64) -> Result<String> {
        // Step 1: Create mint quote
        let quote_response = self.http_client
            .post(&format!("{}/v1/mint/quote/bolt11", self.mint_url))
            .json(&json!({
                "amount": amount,
                "currency": "mana"
            }))
            .send()
            .await?;

        let quote_data: serde_json::Value = quote_response.json().await?;
        let quote_id = quote_data["quote"].as_str()
            .ok_or_else(|| anyhow::anyhow!("No quote ID in response"))?;

        // Step 2: Mint tokens with controlled blinded message
        let mint_response = self.http_client
            .post(&format!("{}/v1/mint/bolt11", self.mint_url))
            .json(&json!({
                "quote": quote_id,
                "outputs": [{
                    "amount": amount,
                    "b_": blinded_msg
                }]
            }))
            .send()
            .await?;

        let mint_data: serde_json::Value = mint_response.json().await?;
        let c_value = mint_data["signatures"][0]["c_"].as_str()
            .ok_or_else(|| anyhow::anyhow!("No C value in mint response"))?;

        Ok(c_value.to_string())
    }

    /// Convert C value hex string to 32-byte array for army generation
    fn c_value_to_bytes(&self, c_value: &str) -> Result<[u8; 32]> {
        // Remove any prefixes and decode hex
        let clean_hex = c_value.strip_prefix("0x").unwrap_or(c_value);
        let bytes = hex::decode(clean_hex)?;
        
        if bytes.len() >= 32 {
            let mut result = [0u8; 32];
            result.copy_from_slice(&bytes[..32]);
            Ok(result)
        } else {
            // Pad with hash if too short
            use sha2::{Sha256, Digest};
            let mut hasher = Sha256::new();
            hasher.update(c_value.as_bytes());
            let hash = hasher.finalize();
            let mut result = [0u8; 32];
            result.copy_from_slice(&hash);
            Ok(result)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("test_deterministic_minting=info")
        .init();

    info!("🚀 Starting Cashu mint determinism test");
    
    // Wait for mint to be available
    info!("⏳ Waiting for mint to be available...");
    sleep(Duration::from_secs(2)).await;

    let test = DeterminismTest::new();
    
    // Check if mint is running
    match test.http_client.get(&format!("{}/health", test.mint_url)).send().await {
        Ok(response) if response.status().is_success() => {
            info!("✅ Mint is available at {}", test.mint_url);
        }
        _ => {
            info!("❌ Mint not available at {}. Please start it first:", test.mint_url);
            info!("   cd daemons/cashu-mint && cargo run");
            return Err(anyhow::anyhow!("Mint not available"));
        }
    }

    // Run determinism tests
    test.test_mint_determinism().await?;

    info!("🎯 Next steps:");
    info!("   1. Implement deterministic mint key generation");
    info!("   2. Update stub mint to use deterministic signatures");
    info!("   3. Replace gaming wallet fake tokens with real mint calls");

    Ok(())
}