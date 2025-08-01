use anyhow::Result;
use nostr::{Event, Keys};
use nostr_sdk::{Client, RelayPoolNotification};
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
    CombatMove(CombatMove),
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
            .map_err(|e| GameEngineError::NostrError(format!("Invalid private key: {e}")))?;

        let client = Client::new(&keys);

        // Connect to relay
        client
            .add_relay(&config.relay_url)
            .await
            .map_err(|e| GameEngineError::NostrError(format!("Failed to add relay: {e}")))?;

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
        // OPTIMIZED FILTERING: Only process game-related Nostr events (KIND 31000-31005)
        // This prevents wasting computational resources on non-game events
        let since_timestamp = nostr::Timestamp::now() - 3600; // 1 hour ago for integration testing

        // Single efficient filter for all game event types
        let game_events_filter = nostr::Filter::new()
            .kinds(vec![
                KIND_MATCH_CHALLENGE,  // 21000 - Player creates match
                KIND_MATCH_ACCEPTANCE, // 21001 - Player accepts challenge
                KIND_TOKEN_REVEAL,     // 21002 - Player reveals Cashu tokens
                KIND_COMBAT_MOVE,      // 21003 - Player submits combat move
                KIND_MATCH_RESULT,     // 21004 - Player submits final match state
                                       // NOTE: KIND_LOOT_DISTRIBUTION (21005) excluded - game engine publishes this
            ])
            .since(since_timestamp);

        let _subscription_id = self
            .client
            .subscribe(vec![game_events_filter], None)
            .await
            .map_err(|e| GameEngineError::NostrError(format!("Failed to subscribe: {e}")))?;

        info!("📡 🎯 OPTIMIZED FILTERING: Subscribed to game events only (KIND 31000-31005)");

        // Start event processing loop in background task
        let client_clone = self.client.clone();
        let sender_clone = self.match_event_sender.clone();
        tokio::spawn(async move {
            let temp_client = NostrClient {
                client: client_clone,
                keys: Keys::generate(), // Dummy keys for processing
                match_event_sender: sender_clone,
            };
            temp_client.process_notifications().await;
        });

        info!("🚀 Nostr event processing task started");
        Ok(())
    }

    /// Process incoming Nostr notifications
    async fn process_notifications(&self) {
        let mut notifications = self.client.notifications();
        let mut processed_events = 0u64;
        info!("🔍 Starting Nostr notification processing loop with optimized game event filtering");

        while let Ok(notification) = notifications.recv().await {
            match notification {
                RelayPoolNotification::Event { event, .. } => {
                    processed_events += 1;

                    // Only game events (KIND 31000-31005) should reach here due to subscription filter
                    debug!(
                        "📥 Game event received: kind={}, id={}, from={}",
                        event.kind, event.id, event.pubkey
                    );

                    if let Err(e) = self.handle_event(&event).await {
                        error!("Failed to handle event {}: {}", event.id, e);
                    }

                    // Periodic efficiency logging
                    if processed_events % 100 == 0 {
                        info!("📊 Processed {} game events (filtered subscription working efficiently)", processed_events);
                    }
                }
                RelayPoolNotification::Message { message, .. } => {
                    debug!("Relay message: {:?}", message);
                }
                RelayPoolNotification::Shutdown => {
                    warn!(
                        "Relay connection shutdown after processing {} game events",
                        processed_events
                    );
                    break;
                }
                _ => {
                    debug!("Other notification: {:?}", notification);
                }
            }
        }
        warn!(
            "🔍 Exited Nostr notification processing loop after {} game events",
            processed_events
        );
    }

    /// Handle incoming player-driven match events
    async fn handle_event(&self, event: &Event) -> Result<(), GameEngineError> {
        // OPTIMIZED: Game engine only processes game events (31000-31005)
        // All other events are filtered out at subscription level for efficiency
        debug!(
            "🎮 Processing game event: {} from {}",
            event.kind, event.pubkey
        );

        // Parse event based on kind - only game events should reach here due to subscription filter
        let player_event = match event.kind {
            kind if kind == KIND_MATCH_CHALLENGE => {
                let challenge: MatchChallenge =
                    serde_json::from_str(&event.content).map_err(|e| {
                        GameEngineError::NostrError(format!("Failed to parse challenge: {e}"))
                    })?;
                PlayerMatchEvent::Challenge(challenge)
            }
            kind if kind == KIND_MATCH_ACCEPTANCE => {
                let acceptance: MatchAcceptance =
                    serde_json::from_str(&event.content).map_err(|e| {
                        GameEngineError::NostrError(format!("Failed to parse acceptance: {e}"))
                    })?;
                PlayerMatchEvent::Acceptance(acceptance)
            }
            kind if kind == KIND_TOKEN_REVEAL => {
                let reveal: TokenReveal = serde_json::from_str(&event.content).map_err(|e| {
                    GameEngineError::NostrError(format!("Failed to parse token reveal: {e}"))
                })?;
                PlayerMatchEvent::TokenReveal(reveal)
            }
            kind if kind == KIND_COMBAT_MOVE => {
                let combat_move: CombatMove =
                    serde_json::from_str(&event.content).map_err(|e| {
                        GameEngineError::NostrError(format!(
                            "Failed to parse combat move: {e}"
                        ))
                    })?;
                PlayerMatchEvent::CombatMove(combat_move)
            }
            kind if kind == KIND_MATCH_RESULT => {
                let result: MatchResult = serde_json::from_str(&event.content).map_err(|e| {
                    GameEngineError::NostrError(format!("Failed to parse match result: {e}"))
                })?;
                PlayerMatchEvent::MatchResult(result)
            }
            _ => {
                // This should never happen due to subscription filtering, but log for debugging
                warn!(
                    "⚠️ Unexpected event kind received: {} (subscription filter may need update)",
                    event.kind
                );
                return Ok(());
            }
        };

        // Send to game engine for processing
        self.match_event_sender.send(player_event).map_err(|e| {
            GameEngineError::NostrError(format!("Failed to send match event: {e}"))
        })?;

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
            .map_err(|e| {
                GameEngineError::NostrError(format!("Failed to create loot event: {e}"))
            })?;

        self.client.send_event(event).await.map_err(|e| {
            GameEngineError::NostrError(format!("Failed to send loot event: {e}"))
        })?;

        info!(
            "🏆 Published loot distribution for match {}",
            loot_distribution.match_event_id
        );

        Ok(())
    }

    /// Get the bot's public key
    pub fn public_key(&self) -> String {
        self.keys.public_key().to_string()
    }
}
