use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, debug};
use nostr::{Keys, EventBuilder, PublicKey, EventId};
use nostr_sdk::Client as NostrClient;
use shared_game_logic::commitment::*;
use shared_game_logic::generate_army_from_cashu_c_value;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use sha2::{Sha256, Digest};

use crate::players::TestPlayer;
use crate::matches::{MatchChallenge, MatchAcceptance, TokenReveal, MoveCommitment, MoveReveal, MatchResult};
use crate::validation::ValidationSummary;
use crate::utils::generate_nonce;

mod gaming_wallet;
use gaming_wallet::{GamingWallet, gaming_tokens_to_proofs, extract_c_value_bytes};

/// Main test suite for player-driven integration tests
/// 
/// This struct orchestrates the complete integration test flow,
/// managing player creation, match execution, and validation.
#[derive(Debug)]
pub struct PlayerDrivenTestSuite {
    http_client: Client,
    mint_url: String,
    relay_url: String,
    nostr_client: NostrClient,
}

impl PlayerDrivenTestSuite {
    /// Creates a new test suite instance with configured clients
    pub async fn new() -> Result<Self> {
        let http_client = Client::new();
        let mint_url = "http://localhost:3333".to_string();
        let relay_url = "ws://localhost:7777".to_string();
        
        let keys = Keys::generate();
        let nostr_client = NostrClient::new(&keys);
        nostr_client.add_relay(relay_url.clone()).await?;
        nostr_client.connect().await;
        
        Ok(Self {
            http_client,
            mint_url,
            relay_url,
            nostr_client,
        })
    }

    /// Runs the complete integration test suite
    /// 
    /// Executes all test scenarios in sequence:
    /// - Happy path match execution
    /// - Anti-cheat validation
    /// - Concurrent match processing
    /// - Edge case handling
    /// - Stress testing
    pub async fn run_comprehensive_tests(&self) -> Result<()> {
        info!("🚀 Starting Player-Driven Integration Test Suite");
        
        self.wait_for_services().await?;
        
        info!("📋 Test 1: Happy Path Player-Driven Match");
        self.test_happy_path_match().await?;
        
        info!("📋 Test 2: Anti-Cheat Commitment Verification");
        self.test_commitment_verification().await?;
        
        info!("📋 Test 3: Concurrent Player-Driven Matches");
        self.test_concurrent_matches().await?;
        
        info!("📋 Test 4: Edge Cases and Malicious Events");
        self.test_edge_cases().await?;
        
        info!("📋 Test 5: High-Volume Match Processing");
        self.test_stress_scenarios().await?;
        
        info!("✅ All Player-Driven Integration Tests Passed!");
        Ok(())
    }

    /// Waits for all required services to be ready
    async fn wait_for_services(&self) -> Result<()> {
        info!("⏳ Waiting for services to be ready...");
        
        // Wait for Cashu mint
        for attempt in 1..=30 {
            match self.http_client.get(&format!("{}/health", self.mint_url)).send().await {
                Ok(response) if response.status().is_success() => {
                    info!("✅ Cashu Mint ready");
                    break;
                }
                _ => {
                    if attempt == 30 {
                        return Err(anyhow::anyhow!("Cashu Mint not ready after 30 attempts"));
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
        
        info!("⏳ Waiting for Game Engine Bot to initialize (pure Nostr mode)...");
        sleep(Duration::from_secs(5)).await;
        info!("✅ Game Engine Bot assumed ready (state machine architecture)");
        
        self.nostr_client.connect().await;
        info!("✅ Nostr Relay ready");
        
        Ok(())
    }

    /// Creates deterministic 32-byte private key from seed string
    fn create_deterministic_key(&self, seed: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }

    /// Publishes a Nostr event for a player
    async fn publish_event<T: serde::Serialize>(&self, player: &TestPlayer, kind: u16, content: &T) -> Result<String> {
        let content_str = serde_json::to_string(content)?;
        let event = EventBuilder::new(
            nostr::Kind::Custom(kind),
            content_str,
            vec![]
        ).to_event(&player.keys)?;
        
        let event_id = event.id.to_hex();
        player.nostr_client.send_event(event).await?;
        
        Ok(event_id)
    }
}

// Include test modules
mod happy_path;
mod anti_cheat;
mod concurrent;
mod edge_cases;
mod stress;

impl PlayerDrivenTestSuite {
    /// Tests the complete happy path of a player-driven match
    pub async fn test_happy_path_match(&self) -> Result<()> {
        happy_path::test_happy_path_match(self).await
    }

    /// Tests anti-cheat commitment verification
    pub async fn test_commitment_verification(&self) -> Result<()> {
        anti_cheat::test_commitment_verification(self).await
    }

    /// Tests multiple concurrent player-driven matches
    pub async fn test_concurrent_matches(&self) -> Result<()> {
        concurrent::test_concurrent_matches(self).await
    }

    /// Tests edge cases and malicious events
    pub async fn test_edge_cases(&self) -> Result<()> {
        edge_cases::test_edge_cases(self).await
    }

    /// Tests high-volume match processing
    pub async fn test_stress_scenarios(&self) -> Result<()> {
        stress::test_stress_scenarios(self).await
    }
} 