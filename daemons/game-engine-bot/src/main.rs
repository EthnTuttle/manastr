use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error, debug};
use serde_json::json;

mod config;
mod errors;
mod game_state;
mod cashu_client;
mod nostr_client;
mod match_events;

// Use shared game logic instead of duplicated code
use shared_game_logic::{
    combat::{generate_units_from_token_secret, process_combat},
    game_state::{Unit, Ability, RoundResult},
};

use config::GameEngineConfig;
use errors::GameEngineError;
use game_state::{MatchValidationManager, MatchState};
use match_events::MatchPhase;
use cashu_client::CashuClient;
use nostr_client::{NostrClient, PlayerMatchEvent};
use match_events::*;

/// Game Engine Bot - Authoritative match resolution and loot distribution
pub struct GameEngineBot {
    config: GameEngineConfig,
    validation_manager: Arc<tokio::sync::Mutex<MatchValidationManager>>,
    cashu_client: Arc<CashuClient>,
    nostr_client: Arc<NostrClient>,
    match_event_receiver: Arc<tokio::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<PlayerMatchEvent>>>,
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

        // Initialize Nostr client
        let (match_event_sender, match_event_receiver) = tokio::sync::mpsc::unbounded_channel();
        let nostr_client = Arc::new(NostrClient::new(&config.nostr, match_event_sender).await?);
        
        info!("🎮 Initialized Game Engine Bot");
        info!("📊 Max concurrent matches: {}", config.game.max_concurrent_matches);
        info!("⏱️ Round timeout: {}s", config.game.round_timeout_seconds);
        info!("🏆 Loot reward per match: {}", config.game.loot_reward_per_match);
        info!("🔑 Bot pubkey: {}", nostr_client.public_key());

        Ok(Self {
            config,
            validation_manager: Arc::new(tokio::sync::Mutex::new(MatchValidationManager::new())),
            cashu_client,
            nostr_client,
            match_event_receiver: Arc::new(tokio::sync::Mutex::new(match_event_receiver)),
        })
    }

    /// Get bot status and active validation count
    pub async fn get_status(&self) -> serde_json::Value {
        let manager = self.validation_manager.lock().await;
        let active_validations = manager.get_active_match_count();
        
        json!({
            "status": "healthy",
            "service": "game-engine-bot",
            "version": env!("CARGO_PKG_VERSION"),
            "role": "validator_and_loot_distributor",
            "active_validations": active_validations,
            "cashu_mint": self.config.cashu.mint_url,
            "nostr_relay": self.config.nostr.relay_url
        })
    }

    /// Get details of a specific match validation state
    pub async fn get_match_validation(&self, match_id: &str) -> Result<Option<serde_json::Value>, GameEngineError> {
        let manager = self.validation_manager.lock().await;
        match manager.get_match(match_id) {
            Ok(player_match) => Ok(Some(json!({
                "match_id": player_match.match_id,
                "phase": format!("{:?}", player_match.phase),
                "player1": player_match.player1_npub,
                "player2": player_match.player2_npub,
                "wager_amount": player_match.wager_amount,
                "league_id": player_match.league_id
            }))),
            Err(GameEngineError::MatchNotFound(_)) => Ok(None),
            Err(e) => Err(e),
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

    /// Start the Nostr event processing loop (must be called from within an Arc<GameEngineBot>)
    pub async fn start_nostr_integration(self: Arc<Self>) -> Result<(), GameEngineError> {
        // Start listening for Nostr events
        self.nostr_client.start_event_listener().await?;
        
        // Start processing game events
        let bot_clone = Arc::clone(&self);
        
        tokio::spawn(async move {
            bot_clone.process_match_events().await;
        });
        
        info!("🚀 Nostr integration started");
        Ok(())
    }

    /// Process incoming player-driven match events from Nostr
    async fn process_match_events(&self) {
        let mut receiver = self.match_event_receiver.lock().await;
        
        while let Some(event) = receiver.recv().await {
            if let Err(e) = self.handle_match_event(event).await {
                error!("Failed to handle match event: {}", e);
            }
        }
    }

    /// Handle a specific player-driven match event
    async fn handle_match_event(&self, event: PlayerMatchEvent) -> Result<(), GameEngineError> {
        match event {
            PlayerMatchEvent::Challenge(challenge) => {
                info!("⚔️ Challenge received from {} for {} sats", 
                      challenge.challenger_npub, challenge.wager_amount);
                self.handle_challenge(challenge).await?;
            }
            PlayerMatchEvent::Acceptance(acceptance) => {
                info!("✅ Challenge accepted by {} for match {}", 
                      acceptance.acceptor_npub, acceptance.match_id);
                self.handle_acceptance(acceptance).await?;
            }
            PlayerMatchEvent::TokenReveal(reveal) => {
                info!("🎯 Token revealed by {} for match {}", 
                      reveal.player_npub, reveal.match_id);
                self.handle_token_reveal(reveal).await?;
            }
            PlayerMatchEvent::MoveCommitment(commitment) => {
                info!("🔒 Move committed by {} for match {} round {}", 
                      commitment.player_npub, commitment.match_id, commitment.round_number);
                self.handle_move_commitment(commitment).await?;
            }
            PlayerMatchEvent::MoveReveal(reveal) => {
                info!("🔓 Move revealed by {} for match {} round {}", 
                      reveal.player_npub, reveal.match_id, reveal.round_number);
                self.handle_move_reveal(reveal).await?;
            }
            PlayerMatchEvent::MatchResult(result) => {
                info!("🏁 Match result submitted by {} for match {}", 
                      result.player_npub, result.match_id);
                self.handle_match_result(result).await?;
            }
        }
        
        Ok(())
    }

    /// Handle challenge creation - track for potential validation
    async fn handle_challenge(&self, challenge: MatchChallenge) -> Result<(), GameEngineError> {
        let mut manager = self.validation_manager.lock().await;
        manager.add_pending_challenge(challenge.clone());
        
        debug!("Tracking challenge from {} for {} sats in league {}", 
               challenge.challenger_npub, challenge.wager_amount, challenge.league_id);
        Ok(())
    }

    /// Handle challenge acceptance - initialize match validation
    async fn handle_acceptance(&self, acceptance: MatchAcceptance) -> Result<(), GameEngineError> {
        let mut manager = self.validation_manager.lock().await;
        manager.initialize_match_validation(&acceptance)?;
        
        info!(
            "Initialized validation for match {} between challenger and {}", 
            acceptance.match_id, acceptance.acceptor_npub
        );
        
        Ok(())
    }

    /// Handle token reveal - validate against commitment
    async fn handle_token_reveal(&self, reveal: TokenReveal) -> Result<(), GameEngineError> {
        let mut manager = self.validation_manager.lock().await;
        let is_valid = manager.validate_token_reveal(&reveal)?;
        
        if is_valid {
            info!(
                "✅ Valid token reveal from {} for match {}", 
                reveal.player_npub, reveal.match_id
            );
            
            // Check if both players have revealed and ready for combat
            if let Ok(player_match) = manager.get_match(&reveal.match_id) {
                if player_match.is_ready_for_combat() {
                    info!("🚀 Match {} ready for combat - both players revealed tokens", reveal.match_id);
                }
            }
        } else {
            warn!(
                "❌ Invalid token reveal from {} for match {} - commitment verification failed", 
                reveal.player_npub, reveal.match_id
            );
            
            // Mark match as invalid due to cheating attempt
            if let Ok(player_match) = manager.get_match_mut(&reveal.match_id) {
                player_match.mark_invalid("Invalid token reveal - commitment verification failed".to_string());
            }
        }
        
        Ok(())
    }

    /// Handle move commitment for a specific round
    async fn handle_move_commitment(&self, commitment: MoveCommitment) -> Result<(), GameEngineError> {
        let mut manager = self.validation_manager.lock().await;
        
        // Store the commitment in the match state
        if let Ok(player_match) = manager.get_match_mut(&commitment.match_id) {
            player_match.add_move_commitment(&commitment)?;
            
            // Check if both players have committed for this round
            if player_match.both_players_committed_round(commitment.round_number) {
                info!(
                    "📝 Both players committed for match {} round {} - waiting for reveals", 
                    commitment.match_id, commitment.round_number
                );
            } else {
                debug!(
                    "📝 Move commitment stored from {} for match {} round {}", 
                    commitment.player_npub, commitment.match_id, commitment.round_number
                );
            }
        } else {
            warn!("❌ Received move commitment for unknown match: {}", commitment.match_id);
        }
        
        Ok(())
    }

    /// Handle move reveal - validate and potentially resolve combat
    async fn handle_move_reveal(&self, reveal: MoveReveal) -> Result<(), GameEngineError> {
        let mut manager = self.validation_manager.lock().await;
        let is_valid = manager.validate_move_reveal(&reveal)?;
        
        if is_valid {
            info!(
                "✅ Valid move reveal from {} for match {} round {}", 
                reveal.player_npub, reveal.match_id, reveal.round_number
            );
            
            // Check if both players have revealed for this round
            if let Ok(player_match) = manager.get_match(&reveal.match_id) {
                if player_match.both_players_revealed_round(reveal.round_number) {
                    info!(
                        "⚔️ Both players revealed for match {} round {} - combat can be validated", 
                        reveal.match_id, reveal.round_number
                    );
                    
                    // TODO: Here we could optionally validate the combat round
                    // using shared_game_logic, but for now we trust players
                    // to do honest combat and validate at the end
                }
            }
        } else {
            warn!(
                "❌ Invalid move reveal from {} for match {} round {} - commitment verification failed", 
                reveal.player_npub, reveal.match_id, reveal.round_number
            );
            
            // Mark match as invalid due to cheating attempt
            if let Ok(player_match) = manager.get_match_mut(&reveal.match_id) {
                player_match.mark_invalid("Invalid move reveal - commitment verification failed".to_string());
            }
        }
        
        Ok(())
    }

    /// Handle match result - validate entire match and issue loot
    async fn handle_match_result(&self, result: MatchResult) -> Result<(), GameEngineError> {
        let mut manager = self.validation_manager.lock().await;
        
        info!(
            "🏁 Validating complete match result from {} for match {}", 
            result.player_npub, result.match_id
        );
        
        // Validate the complete match result
        let validation_summary = manager.validate_match_result(&result.match_id, &result)?;
        
        if validation_summary.commitments_valid && 
           validation_summary.combat_verified && 
           validation_summary.winner_confirmed {
            
            info!("✅ Match {} validation successful - issuing loot distribution", result.match_id);
            
            // Create loot distribution event
            let loot_distribution = LootDistribution {
                game_engine_npub: self.nostr_client.public_key(),
                match_id: result.match_id.clone(),
                winner_npub: result.calculated_winner.clone(),
                loot_cashu_token: None, // TODO: Create actual Cashu token
                match_fee: 5, // 5% fee
                loot_issued_at: chrono::Utc::now().timestamp() as u64,
                validation_summary,
            };
            
            // Publish loot distribution event
            self.nostr_client.publish_loot_distribution(&loot_distribution, "dummy_event_id").await?;
            
            // Mark match as complete
            manager.mark_loot_distributed(&result.match_id)?;
            
            info!("🏆 Loot distributed for match {}", result.match_id);
            
        } else {
            warn!(
                "❌ Match {} validation failed: {:?}", 
                result.match_id, validation_summary.error_details
            );
            
            // Mark match as invalid
            if let Ok(player_match) = manager.get_match_mut(&result.match_id) {
                let error_msg = validation_summary.error_details
                    .unwrap_or_else(|| "Match validation failed".to_string());
                player_match.mark_invalid(error_msg);
            }
        }
        
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub bot: Arc<GameEngineBot>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("game_engine_bot=debug")
        .init();

    info!("🎮 Starting Game Engine Bot...");

    // Load configuration
    let config = GameEngineConfig::load()?;
    info!("📋 Configuration loaded: {}:{}", config.server.host, config.server.port);

    // Initialize game engine bot
    let bot = Arc::new(GameEngineBot::new(config.clone()).await?);
    info!("✅ Game Engine Bot initialized");

    // Start Nostr integration
    let bot_clone = Arc::clone(&bot);
    tokio::spawn(async move {
        if let Err(e) = bot_clone.start_nostr_integration().await {
            error!("Failed to start Nostr integration: {}", e);
        }
    });

    // Create application state
    let app_state = AppState { bot };

    // Build application router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/status", get(bot_status))
        .route("/match/:match_id", get(get_match))
        .route("/test/create_match", get(create_test_match))
        .route("/test/award_loot", get(test_award_loot))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("🚀 Game Engine Bot listening on http://{}", addr);
    info!("📊 Status: http://{}/status", addr);
    info!("🎯 Test endpoints available for demonstration");
    info!("🎮 Ready for authoritative match resolution!");

    // In a full implementation, here we would:
    // 1. Connect to Nostr relay and subscribe to game events
    // 2. Start the main event processing loop
    // 3. Handle match state transitions and combat resolution

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "game-engine-bot",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    }))
}

async fn bot_status(State(state): State<AppState>) -> Json<serde_json::Value> {
    let status = state.bot.get_status().await;
    Json(status)
}

async fn get_match(
    axum::extract::Path(match_id): axum::extract::Path<String>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.bot.get_match_validation(&match_id).await {
        Ok(Some(validation_state)) => Ok(Json(validation_state)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Match validation not found"})),
        )),
        Err(e) => {
            error!("Failed to get match validation {}: {}", match_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            ))
        }
    }
}

async fn create_test_match(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match state.bot.create_test_match("npub1test1", "npub1test2").await {
        Ok(match_id) => Ok(Json(json!({
            "match_id": match_id,
            "message": "Test match created",
            "players": ["npub1test1", "npub1test2"]
        }))),
        Err(e) => {
            error!("Failed to create test match: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            ))
        }
    }
}

async fn test_award_loot(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let test_match_id = "test_match_123";
    let winner = "npub1winner";
    
    match state.bot.award_loot(test_match_id, winner).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            error!("Failed to award loot: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            ))
        }
    }
}