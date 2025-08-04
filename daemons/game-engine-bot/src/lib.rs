//! Game Engine Bot Library
//! 
//! This library provides the main GameEngineBot struct and related functionality
//! for the Manastr decentralized gaming engine.

// Re-export all the modules for external use
pub mod cashu_client;
pub mod config;
pub mod errors;
pub mod game_state;
pub mod match_events;
pub mod match_state_machine;
pub mod match_tracker;
pub mod nostr_client;

// Re-export the main types for easy access
pub use cashu_client::CashuClient;
pub use config::GameEngineConfig;
pub use errors::GameEngineError;
pub use match_state_machine::{GameEngineAction, MatchState};
pub use match_tracker::{run_cleanup_task, MatchTracker, TrackedAction};
pub use nostr_client::{NostrClient, PlayerMatchEvent};

// Copy the GameEngineBot struct and its implementation from main.rs
use anyhow::Result;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Game Engine Bot - Authoritative match resolution and loot distribution via Nostr
/// Now operates purely through state machine transitions
pub struct GameEngineBot {
    config: GameEngineConfig,
    match_tracker: Arc<MatchTracker>,
    cashu_client: Arc<CashuClient>,
    nostr_client: Arc<NostrClient>,
    match_event_receiver:
        Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<PlayerMatchEvent>>>,
    action_receiver: Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<TrackedAction>>>,
}

impl GameEngineBot {
    pub async fn new(config: GameEngineConfig) -> Result<Self, GameEngineError> {
        // Initialize Cashu client
        let cashu_client = Arc::new(CashuClient::new(config.cashu.mint_url.clone()));

        // Test connection to mint
        if !cashu_client.health_check().await? {
            warn!("⚠️ Cashu mint not available at {}", config.cashu.mint_url);
        } else {
            info!("✅ Connected to Cashu mint at {}", config.cashu.mint_url);
        }

        // Initialize match tracker with state machine
        let (match_tracker, action_receiver) = MatchTracker::new(
            config.game.max_concurrent_matches as usize,
            config.game.round_timeout_seconds / 60, // convert to minutes
        );
        let match_tracker = Arc::new(match_tracker);

        // Initialize Nostr client
        let (match_event_sender, match_event_receiver) = tokio::sync::mpsc::unbounded_channel();
        let nostr_client = Arc::new(NostrClient::new(&config.nostr, match_event_sender).await?);

        info!("🎮 Initialized Game Engine Bot with State Machine Architecture");
        info!(
            "📊 Max concurrent matches: {}",
            config.game.max_concurrent_matches
        );
        info!(
            "⏱️ Match timeout: {} minutes",
            config.game.round_timeout_seconds / 60
        );
        info!(
            "🏆 Loot reward per match: {}",
            config.game.loot_reward_per_match
        );
        info!("🔑 Bot pubkey: {}", nostr_client.public_key());
        info!("🤖 Operating purely via Nostr events (no HTTP endpoints)");

        Ok(Self {
            config,
            match_tracker,
            cashu_client,
            nostr_client,
            match_event_receiver: Arc::new(tokio::sync::Mutex::new(match_event_receiver)),
            action_receiver: Arc::new(tokio::sync::Mutex::new(action_receiver)),
        })
    }

    /// Start the complete game engine system
    pub async fn start_game_engine(self: Arc<Self>) -> Result<(), GameEngineError> {
        info!("🚀 Starting Game Engine Bot with State Machine Architecture");

        // Start listening for Nostr events
        self.nostr_client.start_event_listener().await?;

        // Start match event processing loop
        let bot_clone = Arc::clone(&self);
        tokio::spawn(async move {
            bot_clone.process_match_events().await;
        });

        // Start state machine action processing loop
        let bot_clone = Arc::clone(&self);
        tokio::spawn(async move {
            bot_clone.process_state_actions().await;
        });

        // Start periodic cleanup task
        let tracker_clone = Arc::clone(&self.match_tracker);
        tokio::spawn(async move {
            run_cleanup_task(tracker_clone).await;
        });

        info!("🎮 Game Engine Bot fully operational");
        info!(
            "📡 Listening for Nostr events on: {}",
            self.config.nostr.relay_url
        );
        info!("🤖 Operating in pure state machine mode (no HTTP endpoints)");

        Ok(())
    }

    /// Process incoming player-driven match events from Nostr via state machine
    async fn process_match_events(&self) {
        let mut receiver = self.match_event_receiver.lock().await;

        info!("🎮 Started Nostr match event processing loop");

        while let Some(event) = receiver.recv().await {
            debug!("📨 Received Nostr match event: {:?}", event);

            if let Err(e) = self.match_tracker.process_event(event).await {
                error!(
                    "❌ Failed to process match event through state machine: {}",
                    e
                );
            }
        }
    }

    /// Process state machine actions (loot distribution, match invalidation)
    async fn process_state_actions(&self) {
        let mut receiver = self.action_receiver.lock().await;

        info!("🎰 Started state machine action processing loop");

        while let Some(action) = receiver.recv().await {
            debug!("⚡ Processing state machine action: {:?}", action);

            if let Err(e) = self.handle_action(action).await {
                error!("❌ Failed to process state machine action: {}", e);
            }
        }
    }

    /// Handle actions generated by the state machine (like loot distribution)
    async fn handle_action(&self, action: TrackedAction) -> Result<(), GameEngineError> {
        match action.action {
            GameEngineAction::DistributeLoot { match_id, winner_npub } => {
                if let Some(winner) = winner_npub {
                    info!(
                        "🏆 Distributing loot for match {} to winner {}",
                        match_id, winner
                    );

                    // Create loot token for the winner
                    let loot_result = self
                        .cashu_client
                        .create_loot_token(
                            &winner,
                            self.config.game.loot_reward_per_match,
                            &match_id,
                        )
                        .await?;

                    info!(
                        "💰 Loot token created for {}: {}",
                        winner, loot_result.quote
                    );
                } else {
                    warn!("🤷 No winner determined for match {}", match_id);
                }
            }
            GameEngineAction::InvalidateMatch { match_id, reason } => {
                warn!(
                    "❌ Invalidating match {}: {}",
                    match_id, reason
                );
                // TODO: Publish match invalidation event to Nostr when needed
            }
            _ => {
                debug!("🔧 Handling other game engine action: {:?}", action.action);
                // Handle other action types as needed
            }
        }

        Ok(())
    }
}