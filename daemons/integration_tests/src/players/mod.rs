use anyhow::Result;
use nostr::{Keys, PublicKey};
use nostr_sdk::Client as NostrClient;
use tracing::info;

use super::core::gaming_wallet::GamingWallet;

/// Represents a test player in the integration test environment
///
/// Contains all necessary components for a player to participate
/// in player-driven matches including Nostr keys, gaming wallet,
/// and session-specific data.
#[derive(Clone)]
pub struct TestPlayer {
    pub name: String,
    pub keys: Keys,
    pub public_key: PublicKey,
    pub nostr_client: NostrClient,
    pub gaming_wallet: GamingWallet,
    pub army_nonce: String,
    pub token_nonce: String,
}

/// Represents a player-driven match with all participants and state
#[derive(Clone)]
pub struct PlayerDrivenMatch {
    pub match_event_id: String,
    pub player1: TestPlayer,
    pub player2: TestPlayer,
    pub wager_amount: u64,
    pub league_id: u8,
    pub challenge_event_id: String,
    pub acceptance_event_id: Option<String>,
    pub phase: MatchPhase,
}

/// Tracks the current phase of a match
#[derive(Debug, Clone)]
pub enum MatchPhase {
    Created,
    Accepted,
    TokensRevealed,
    InProgress(u32),
    Completed,
    LootDistributed,
    Invalid(String),
}

impl TestPlayer {
    /// Creates a new test player with initialized components
    ///
    /// # Arguments
    /// * `name` - Player's display name
    /// * `mint_url` - URL of the Cashu mint service
    /// * `relay_url` - URL of the Nostr relay
    /// * `deterministic_seed` - Seed for deterministic key generation
    pub async fn new(
        name: &str,
        mint_url: String,
        relay_url: String,
        deterministic_seed: &str,
    ) -> Result<Self> {
        info!("Creating test player '{}'", name);

        let keys = Self::create_deterministic_keys(deterministic_seed)?;
        let public_key = keys.public_key();

        let nostr_client = NostrClient::new(&keys);
        nostr_client.add_relay(relay_url.clone()).await?;
        nostr_client.connect().await;

        let mut gaming_wallet = GamingWallet::new(mint_url).await?;
        let gaming_tokens = gaming_wallet.mint_gaming_tokens(100, "mana").await?;

        info!(
            "Player '{}' received {} gaming tokens",
            name,
            gaming_tokens.len()
        );

        Ok(Self {
            name: name.to_string(),
            keys,
            public_key,
            nostr_client,
            gaming_wallet,
            army_nonce: Self::generate_nonce(),
            token_nonce: Self::generate_nonce(),
        })
    }

    /// Creates deterministic Nostr keys from a seed string
    fn create_deterministic_keys(seed: &str) -> Result<Keys> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();
        let key_hex = format!("{hash:x}");

        Ok(Keys::parse(&key_hex)?)
    }

    /// Generates a random nonce for cryptographic operations
    fn generate_nonce() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let nonce: u64 = rng.gen();
        format!("{nonce:x}")
    }
}
