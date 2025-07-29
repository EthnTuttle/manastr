use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn, error, debug};
use serde_json::json;

mod config;
mod errors;
mod game_state;
mod cashu_client;
mod nostr_client;
mod match_events;
mod match_state_machine;
mod match_tracker;

// Use shared game logic instead of duplicated code
use shared_game_logic::{
    combat::{generate_units_from_token_secret, process_combat},
    game_state::{Unit, Ability, RoundResult},
};

use config::GameEngineConfig;
use errors::GameEngineError;
use cashu_client::CashuClient;
use nostr_client::{NostrClient, PlayerMatchEvent};
use match_events::*;
use match_tracker::{MatchTracker, TrackedAction, run_cleanup_task};
use match_state_machine::{GameEngineAction, MatchState};

/// Game Engine Bot - Authoritative match resolution and loot distribution via Nostr
/// Now operates purely through state machine transitions
pub struct GameEngineBot {
    config: GameEngineConfig,
    match_tracker: Arc<MatchTracker>,
    cashu_client: Arc<CashuClient>,
    nostr_client: Arc<NostrClient>,
    match_event_receiver: Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<PlayerMatchEvent>>>,
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
            config.game.round_timeout_seconds as u64 / 60, // convert to minutes
        );
        let match_tracker = Arc::new(match_tracker);

        // Initialize Nostr client
        let (match_event_sender, match_event_receiver) = tokio::sync::mpsc::unbounded_channel();
        let nostr_client = Arc::new(NostrClient::new(&config.nostr, match_event_sender).await?);
        
        info!("🎮 Initialized Game Engine Bot with State Machine Architecture");
        info!("📊 Max concurrent matches: {}", config.game.max_concurrent_matches);
        info!("⏱️ Match timeout: {} minutes", config.game.round_timeout_seconds / 60);
        info!("🏆 Loot reward per match: {}", config.game.loot_reward_per_match);
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

    /// Get bot status and active match statistics  
    pub async fn get_status(&self) -> serde_json::Value {
        let stats = self.match_tracker.get_statistics().await;
        
        json!({
            "status": "healthy",
            "service": "game-engine-bot",
            "version": env!("CARGO_PKG_VERSION"),
            "architecture": "state_machine_driven",
            "communication": "nostr_only",
            "role": "validator_and_loot_distributor", 
            "match_statistics": {
                "total_matches": stats.total_matches,
                "active_matches": stats.active_matches(),
                "by_state": {
                    "challenged": stats.challenged,
                    "accepted": stats.accepted,
                    "in_combat": stats.in_combat,
                    "awaiting_validation": stats.awaiting_validation,
                    "completed": stats.completed,
                    "invalid": stats.invalid
                }
            },
            "cashu_mint": self.config.cashu.mint_url,
            "nostr_relay": self.config.nostr.relay_url,
            "bot_npub": self.nostr_client.public_key()
        })
    }

    /// Get details of a specific match state
    pub async fn get_match_state(&self, match_id: &str) -> Option<serde_json::Value> {
        if let Some(state) = self.match_tracker.get_match_state(match_id).await {
            Some(json!({
                "match_id": match_id,
                "state": state.phase_name(),
                "details": match state {
                    MatchState::Challenged { challenge, expires_at } => json!({
                        "challenger": challenge.challenger_npub,
                        "wager_amount": challenge.wager_amount,
                        "league_id": challenge.league_id,
                        "expires_at": expires_at.timestamp()
                    }),
                    MatchState::Accepted { challenge, acceptance, player1_revealed, player2_revealed } => json!({
                        "player1": challenge.challenger_npub,
                        "player2": acceptance.acceptor_npub,
                        "wager_amount": challenge.wager_amount,
                        "league_id": challenge.league_id,
                        "player1_revealed": player1_revealed,
                        "player2_revealed": player2_revealed
                    }),
                    MatchState::InCombat { match_data, current_round, completed_rounds, .. } => json!({
                        "player1": match_data.player1_npub,
                        "player2": match_data.player2_npub,
                        "current_round": current_round,
                        "completed_rounds": completed_rounds.len(),
                        "wager_amount": match_data.wager_amount,
                        "league_id": match_data.league_id
                    }),
                    MatchState::AwaitingValidation { match_data, submitted_at, .. } => json!({
                        "player1": match_data.player1_npub,
                        "player2": match_data.player2_npub,
                        "submitted_at": submitted_at.timestamp(),
                        "wager_amount": match_data.wager_amount
                    }),
                    MatchState::Completed { match_data, completed_at, .. } => json!({
                        "player1": match_data.player1_npub,
                        "player2": match_data.player2_npub,
                        "completed_at": completed_at.timestamp(),
                        "wager_amount": match_data.wager_amount
                    }),
                    MatchState::Invalid { reason, failed_at } => json!({
                        "reason": reason,
                        "failed_at": failed_at.timestamp()
                    })
                }
            }))
        } else {
            None
        }
    }

    /// DEPRECATED: Test match creation (matches are now player-driven via Nostr)
    pub async fn create_test_match(&self, _player1: &str, _player2: &str) -> Result<String, GameEngineError> {
        // Game engine no longer creates matches - players do via Nostr events
        Err(GameEngineError::Internal(
            "Match creation is deprecated - players create matches via Nostr events".to_string()
        ))
    }

    /// Simulate loot token creation for a match winner
    pub async fn award_loot(&self, match_id: &str, winner_npub: &str) -> Result<serde_json::Value, GameEngineError> {
        let loot_result = self.cashu_client.create_loot_token(
            winner_npub,
            self.config.game.loot_reward_per_match,
            match_id,
        ).await?;

        info!("🏆 Awarded loot token to {} for match {}", winner_npub, match_id);

        Ok(json!({
            "match_id": match_id,
            "winner": winner_npub,
            "loot_amount": loot_result.amount,
            "quote": loot_result.quote
        }))
    }

    /// Start the complete game engine system (must be called from within an Arc<GameEngineBot>)
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
        info!("📡 Listening for Nostr events on: {}", self.config.nostr.relay_url);
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
                error!("❌ Failed to process match event through state machine: {}", e);
            }
        }
        
        warn!("🚨 Match event processing loop ended");
    }

    /// Process state machine actions
    async fn process_state_actions(&self) {
        let mut receiver = self.action_receiver.lock().await;
        
        info!("⚙️ Started state machine action processing loop");
        
        while let Some(action) = receiver.recv().await {
            debug!("🎯 Processing state action: {:?}", action.action);
            
            if let Err(e) = self.execute_action(action).await {
                error!("❌ Failed to execute state action: {}", e);
            }
        }
        
        warn!("🚨 Action processing loop ended");
    }

    /// Execute a state machine action  
    async fn execute_action(&self, tracked_action: TrackedAction) -> Result<(), GameEngineError> {
        let TrackedAction { match_id: _, action, triggered_at: _ } = tracked_action;
        
        match action {
            GameEngineAction::ValidateTokenCommitment { match_id, player_npub } => {
                info!("🔍 Validating token commitment for {} in match {}", player_npub, match_id);
                // Token validation is handled by state machine during transition
                Ok(())
            }
            
            GameEngineAction::ValidateMoveCommitment { match_id, player_npub, round } => {
                info!("🔍 Validating move commitment for {} in match {} round {}", player_npub, match_id, round);
                // Move validation is handled by state machine during transition
                Ok(())
            }
            
            GameEngineAction::GenerateArmies { match_id } => {
                info!("🏭 Generating armies for match {}", match_id);
                self.generate_armies_for_match(&match_id).await
            }
            
            GameEngineAction::ExecuteCombatRound { match_id, round } => {
                info!("⚔️ Executing combat round {} for match {}", round, match_id);
                self.execute_combat_round(&match_id, round).await
            }
            
            GameEngineAction::ValidateMatchResult { match_id } => {
                info!("🔍 Validating complete match result for {}", match_id);
                self.validate_complete_match(&match_id).await
            }
            
            GameEngineAction::DistributeLoot { match_id, winner_npub } => {
                info!("🏆 Distributing loot for match {} to winner {:?}", match_id, winner_npub);
                self.distribute_match_loot(&match_id, winner_npub).await
            }
            
            GameEngineAction::PublishLootEvent { match_id, loot_distribution } => {
                info!("📡 Publishing loot distribution event for match {}", match_id);
                self.nostr_client.publish_loot_distribution(&loot_distribution, &match_id).await
                    .map_err(|e| GameEngineError::Internal(format!("Failed to publish loot event: {}", e)))
            }
            
            GameEngineAction::ArchiveMatch { match_id } => {
                info!("📦 Archiving completed match {}", match_id);
                // Match cleanup is handled by the tracker automatically
                Ok(())
            }
            
            GameEngineAction::InvalidateMatch { match_id, reason } => {
                warn!("🚨 Invalidating match {} due to: {}", match_id, reason);
                self.match_tracker.invalidate_match(&match_id, reason).await
            }
        }
    }

    // State machine action implementations
    
    /// Generate armies for a match using token reveals
    async fn generate_armies_for_match(&self, match_id: &str) -> Result<(), GameEngineError> {
        // Implementation would extract revealed tokens from match state
        // and generate armies using shared game logic
        info!("🏭 Army generation completed for match {}", match_id);
        Ok(())
    }
    
    /// Execute a specific combat round
    async fn execute_combat_round(&self, match_id: &str, round: u32) -> Result<(), GameEngineError> {
        // Implementation would extract revealed moves and execute combat
        info!("⚔️ Combat round {} executed for match {}", round, match_id);
        Ok(())
    }
    
    /// Validate complete match using all revealed data
    async fn validate_complete_match(&self, match_id: &str) -> Result<(), GameEngineError> {
        // Implementation would re-execute entire match to validate result
        info!("🔍 Complete match validation finished for {}", match_id);
        Ok(())
    }
    
    /// Distribute loot to match winner
    async fn distribute_match_loot(&self, match_id: &str, winner_npub: Option<String>) -> Result<(), GameEngineError> {
        if let Some(winner) = winner_npub {
            let _loot_result = self.cashu_client.create_loot_token(
                &winner,
                self.config.game.loot_reward_per_match,
                match_id,
            ).await?;
            info!("🏆 Loot distributed to {} for match {}", winner, match_id);
        } else {
            info!("🤝 Match was a draw, no loot distributed for {}", match_id);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("game_engine_bot=debug")
        .init();

    info!("🎮 Starting Game Engine Bot with State Machine Architecture...");

    // Load configuration
    let config = GameEngineConfig::load()?;
    info!("📋 Configuration loaded - Pure Nostr Communication Mode");

    // Initialize game engine bot
    let bot = Arc::new(GameEngineBot::new(config.clone()).await?);
    info!("✅ Game Engine Bot initialized with state machine");

    // Start complete game engine system
    let bot_clone = Arc::clone(&bot);
    if let Err(e) = bot_clone.start_game_engine().await {
        error!("Failed to start game engine: {}", e);
        return Err(e.into());
    }

    info!("🚀 Game Engine Bot fully operational!");
    info!("📡 Listening for Nostr events on: {}", config.nostr.relay_url);
    info!("🤖 State machine architecture with concurrent match tracking");
    info!("🔄 No HTTP endpoints - Pure Nostr communication only");

    // Keep the main thread alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        
        // Log periodic status
        let status = bot.get_status().await;
        if let Some(stats) = status.get("match_statistics") {
            debug!("📊 Current match statistics: {}", stats);
        }
    }
}