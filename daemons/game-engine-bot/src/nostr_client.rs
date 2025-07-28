use anyhow::Result;
use nostr::{Event, EventBuilder, Keys, Kind, Tag, TagKind};
use nostr_sdk::{Client, RelayPoolNotification};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::config::NostrConfig;
use crate::errors::GameEngineError;
use crate::match_events::*;

/// Player-driven match event for the game engine to process
#[derive(Debug, Clone)]
pub enum PlayerMatchEvent {
    Challenge(MatchChallenge),
    Acceptance(MatchAcceptance),
    TokenReveal(TokenReveal),
    MoveCommitment(MoveCommitment),
    MoveReveal(MoveReveal),
    MatchResult(MatchResult),
}

/// Nostr client for the Game Engine Bot
pub struct NostrClient {
    client: Client,
    keys: Keys,
    match_event_sender: mpsc::UnboundedSender<PlayerMatchEvent>,
}

impl NostrClient {
    /// Create a new Nostr client for the game engine bot
    pub async fn new(
        config: &NostrConfig,
        match_event_sender: mpsc::UnboundedSender<PlayerMatchEvent>,
    ) -> Result<Self, GameEngineError> {
        // Parse private key
        let keys = Keys::parse(&config.private_key)
            .map_err(|e| GameEngineError::NostrError(format!("Invalid private key: {}", e)))?;

        let client = Client::new(&keys);

        // Connect to relay
        client
            .add_relay(&config.relay_url)
            .await
            .map_err(|e| GameEngineError::NostrError(format!("Failed to add relay: {}", e)))?;

        client.connect().await;

        info!("✅ Connected to Nostr relay: {}", config.relay_url);
        info!("🔑 Game Engine Bot pubkey: {}", keys.public_key());

        Ok(Self {
            client,
            keys,
            match_event_sender,
        })
    }

    /// Start listening for player-driven match events
    pub async fn start_event_listener(&self) -> Result<(), GameEngineError> {
        // Subscribe to all player-driven match event types
        let challenge_filter = nostr::Filter::new()
            .kind(KIND_MATCH_CHALLENGE)
            .since(nostr::Timestamp::now());

        let acceptance_filter = nostr::Filter::new()
            .kind(KIND_MATCH_ACCEPTANCE)
            .since(nostr::Timestamp::now());

        let token_reveal_filter = nostr::Filter::new()
            .kind(KIND_TOKEN_REVEAL)
            .since(nostr::Timestamp::now());

        let move_commitment_filter = nostr::Filter::new()
            .kind(KIND_MOVE_COMMITMENT)
            .since(nostr::Timestamp::now());

        let move_reveal_filter = nostr::Filter::new()
            .kind(KIND_MOVE_REVEAL)
            .since(nostr::Timestamp::now());

        let match_result_filter = nostr::Filter::new()
            .kind(KIND_MATCH_RESULT)
            .since(nostr::Timestamp::now());

        let subscription_id = self
            .client
            .subscribe(vec![
                challenge_filter,
                acceptance_filter,
                token_reveal_filter,
                move_commitment_filter,
                move_reveal_filter,
                match_result_filter,
            ], None)
            .await
            .map_err(|e| GameEngineError::NostrError(format!("Failed to subscribe: {}", e)))?;

        info!("📡 Subscribed to player-driven match events with ID: {:?}", subscription_id);

        // Start event processing loop
        self.process_notifications().await;

        Ok(())
    }

    /// Process incoming Nostr notifications
    async fn process_notifications(&self) {
        let mut notifications = self.client.notifications();

        while let Ok(notification) = notifications.recv().await {
            match notification {
                RelayPoolNotification::Event { event, .. } => {
                    if let Err(e) = self.handle_event(&event).await {
                        error!("Failed to handle event {}: {}", event.id, e);
                    }
                }
                RelayPoolNotification::Message { message, .. } => {
                    debug!("Relay message: {:?}", message);
                }
                RelayPoolNotification::Shutdown => {
                    warn!("Relay connection shutdown");
                    break;
                }
                _ => {}
            }
        }
    }

    /// Handle incoming player-driven match events
    async fn handle_event(&self, event: &Event) -> Result<(), GameEngineError> {
        debug!("Received event: {} from {}", event.kind, event.pubkey);

        // Parse event based on kind
        let player_event = match event.kind {
            kind if kind == KIND_MATCH_CHALLENGE => {
                let challenge: MatchChallenge = serde_json::from_str(&event.content)
                    .map_err(|e| GameEngineError::NostrError(format!("Failed to parse challenge: {}", e)))?;
                PlayerMatchEvent::Challenge(challenge)
            }
            kind if kind == KIND_MATCH_ACCEPTANCE => {
                let acceptance: MatchAcceptance = serde_json::from_str(&event.content)
                    .map_err(|e| GameEngineError::NostrError(format!("Failed to parse acceptance: {}", e)))?;
                PlayerMatchEvent::Acceptance(acceptance)
            }
            kind if kind == KIND_TOKEN_REVEAL => {
                let reveal: TokenReveal = serde_json::from_str(&event.content)
                    .map_err(|e| GameEngineError::NostrError(format!("Failed to parse token reveal: {}", e)))?;
                PlayerMatchEvent::TokenReveal(reveal)
            }
            kind if kind == KIND_MOVE_COMMITMENT => {
                let commitment: MoveCommitment = serde_json::from_str(&event.content)
                    .map_err(|e| GameEngineError::NostrError(format!("Failed to parse move commitment: {}", e)))?;
                PlayerMatchEvent::MoveCommitment(commitment)
            }
            kind if kind == KIND_MOVE_REVEAL => {
                let reveal: MoveReveal = serde_json::from_str(&event.content)
                    .map_err(|e| GameEngineError::NostrError(format!("Failed to parse move reveal: {}", e)))?;
                PlayerMatchEvent::MoveReveal(reveal)
            }
            kind if kind == KIND_MATCH_RESULT => {
                let result: MatchResult = serde_json::from_str(&event.content)
                    .map_err(|e| GameEngineError::NostrError(format!("Failed to parse match result: {}", e)))?;
                PlayerMatchEvent::MatchResult(result)
            }
            _ => {
                debug!("Ignoring unsupported event kind: {}", event.kind);
                return Ok(());
            }
        };

        // Send to game engine for processing
        self.match_event_sender
            .send(player_event)
            .map_err(|e| GameEngineError::NostrError(format!("Failed to send match event: {}", e)))?;

        Ok(())
    }

    /// Publish loot distribution event (ONLY event the game engine publishes)
    pub async fn publish_loot_distribution(
        &self,
        loot_distribution: &LootDistribution,
        match_event_id: &str,
    ) -> Result<(), GameEngineError> {
        let event = loot_distribution
            .to_nostr_event(&self.keys, match_event_id)
            .map_err(|e| GameEngineError::NostrError(format!("Failed to create loot event: {}", e)))?;

        self.client
            .send_event(event)
            .await
            .map_err(|e| GameEngineError::NostrError(format!("Failed to send loot event: {}", e)))?;

        info!("🏆 Published loot distribution for match {}", loot_distribution.match_id);

        Ok(())
    }


    /// Get the bot's public key
    pub fn public_key(&self) -> String {
        self.keys.public_key().to_string()
    }
}