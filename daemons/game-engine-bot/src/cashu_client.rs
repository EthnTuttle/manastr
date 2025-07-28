use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::errors::GameEngineError;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct CashuClient {
    client: Client,
    mint_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintQuoteRequest {
    pub amount: u64,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintQuoteResponse {
    pub quote: String,
    pub request: String, // Lightning payment request
    pub amount: u64,
    pub currency: String,
    pub state: String,
    pub expiry: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LootTokenResult {
    pub quote: String,
    pub amount: u64,
    pub winner_npub: String,
    pub match_id: String,
}

impl CashuClient {
    pub fn new(mint_url: String) -> Self {
        Self {
            client: Client::new(),
            mint_url,
        }
    }

    /// Verify that the mint is accessible
    pub async fn health_check(&self) -> Result<bool, GameEngineError> {
        let url = format!("{}/health", self.mint_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                warn!("Cashu mint health check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Get mint information
    pub async fn get_mint_info(&self) -> Result<serde_json::Value, GameEngineError> {
        let url = format!("{}/v1/info", self.mint_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
            
        Ok(response)
    }

    /// Request a mint quote for loot tokens
    /// This simulates the process - in a real implementation, the game engine
    /// would have special authority to mint loot tokens directly
    pub async fn create_loot_token(
        &self,
        winner_npub: &str,
        amount: u64,
        match_id: &str,
    ) -> Result<LootTokenResult, GameEngineError> {
        info!("🏆 Creating loot token: {} for winner {} (match {})", 
              amount, winner_npub, match_id);

        // In a real implementation, this would be a special authenticated endpoint
        // For now, we simulate the loot token creation
        let quote_request = MintQuoteRequest {
            amount,
            currency: Some("loot".to_string()),
        };

        let url = format!("{}/v1/mint/quote/bolt11", self.mint_url);
        
        let response = self.client
            .post(&url)
            .json(&quote_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(GameEngineError::CashuError(
                format!("Failed to create loot quote: {}", response.status())
            ));
        }

        let quote_response: MintQuoteResponse = response.json().await?;

        // In a real implementation, the game engine would have authority to mint
        // the loot token directly without requiring Lightning payment
        info!("🎯 Loot token quote created: {} (amount: {})", quote_response.quote, amount);

        Ok(LootTokenResult {
            quote: quote_response.quote,
            amount,
            winner_npub: winner_npub.to_string(),
            match_id: match_id.to_string(),
        })
    }

    /// Verify a mana token (not implemented in pure CDK mint)
    /// This would validate token signatures and check spent status
    pub async fn verify_mana_token(
        &self,
        _token_secret: &str,
        _token_signature: &str,
    ) -> Result<bool, GameEngineError> {
        // In a pure CDK mint, token verification is handled client-side
        // The game engine trusts that clients provide valid tokens
        // In production, this would use proper CDK token verification
        
        info!("🔍 Mana token verification (client-side logic)");
        Ok(true)
    }

    /// Get keysets from the mint
    pub async fn get_keysets(&self) -> Result<serde_json::Value, GameEngineError> {
        let url = format!("{}/v1/keysets", self.mint_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
            
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cashu_client_creation() {
        let client = CashuClient::new("http://localhost:3333".to_string());
        assert_eq!(client.mint_url, "http://localhost:3333");
    }

    // Note: Integration tests would require a running mint
    // These are unit tests for the client structure
}