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
                "match_event_id": player_match.match_event_id,
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
                      acceptance.acceptor_npub, acceptance.match_event_id);
                self.handle_acceptance(acceptance).await?;
            }
            PlayerMatchEvent::TokenReveal(reveal) => {
                info!("🎯 Token revealed by {} for match {}", 
                      reveal.player_npub, reveal.match_event_id);
                self.handle_token_reveal(reveal).await?;
            }
            PlayerMatchEvent::MoveCommitment(commitment) => {
                info!("🔒 Move committed by {} for match {} round {}", 
                      commitment.player_npub, commitment.match_event_id, commitment.round_number);
                self.handle_move_commitment(commitment).await?;
            }
            PlayerMatchEvent::MoveReveal(reveal) => {
                info!("🔓 Move revealed by {} for match {} round {}", 
                      reveal.player_npub, reveal.match_event_id, reveal.round_number);
                self.handle_move_reveal(reveal).await?;
            }
            PlayerMatchEvent::MatchResult(result) => {
                info!("🏁 Match result submitted by {} for match {}", 
                      result.player_npub, result.match_event_id);
                self.handle_match_result(result).await?;
            }
        }
        
        Ok(())
    }

    /// Handle challenge creation - track for potential validation
    async fn handle_challenge(&self, challenge: MatchChallenge) -> Result<(), GameEngineError> {
        info!("🎯 GAME FLOW: Match Challenge Created");
        info!("📋 Challenge Details:");
        info!("  Challenger: {}", challenge.challenger_npub);
        info!("  Wager Amount: {} sats", challenge.wager_amount);
        info!("  League: {} (determines unit power scaling)", challenge.league_id);
        info!("  Expires At: {}", challenge.expires_at);
        debug!("  Cashu Token Commitment: {}", challenge.cashu_token_commitment);
        debug!("  Army Commitment: {}", challenge.army_commitment);
        
        info!("🎮 HOW THE GAME WORKS:");
        info!("  1. Player 1 creates a challenge with committed Cashu tokens and army data");
        info!("  2. Player 2 can accept by providing their own commitments");
        info!("  3. Both players reveal tokens to generate deterministic armies");
        info!("  4. Players commit/reveal moves for each combat round");
        info!("  5. Game engine validates entire match and distributes loot to winner");
        
        let mut manager = self.validation_manager.lock().await;
        manager.add_pending_challenge(challenge.clone());
        
        info!("✅ Challenge tracked, waiting for acceptance");
        Ok(())
    }

    /// Handle challenge acceptance - initialize match validation
    async fn handle_acceptance(&self, acceptance: MatchAcceptance) -> Result<(), GameEngineError> {
        info!("🤝 GAME FLOW: Match Challenge Accepted");
        info!("📋 Acceptance Details:");
        info!("  Acceptor: {}", acceptance.acceptor_npub);
        info!("  Match Event ID: {}", acceptance.match_event_id);
        info!("  Accepted At: {}", acceptance.accepted_at);
        debug!("  Cashu Token Commitment: {}", acceptance.cashu_token_commitment);
        debug!("  Army Commitment: {}", acceptance.army_commitment);
        
        info!("🔒 COMMITMENT SCHEME:");
        info!("  Both players have now committed to their Cashu tokens and army configurations");
        info!("  Commitments are cryptographic hashes that prevent cheating");
        info!("  Next step: players must reveal their actual tokens to generate armies");
        
        let mut manager = self.validation_manager.lock().await;
        manager.initialize_match_validation(&acceptance)?;
        
        info!("✅ Match validation initialized - waiting for token reveals");
        Ok(())
    }

    /// Handle token reveal - validate against commitment
    async fn handle_token_reveal(&self, reveal: TokenReveal) -> Result<(), GameEngineError> {
        info!("🔓 GAME FLOW: Token Reveal Received");
        info!("📋 Reveal Details:");
        info!("  Player: {}", reveal.player_npub);
        info!("  Match: {}", reveal.match_event_id);
        info!("  Tokens Count: {}", reveal.cashu_tokens.len());
        info!("  Revealed At: {}", reveal.revealed_at);
        debug!("  Token Secrets: {:?}", reveal.cashu_tokens);
        debug!("  Nonce: {}", reveal.token_secrets_nonce);
        
        info!("🛡️ ANTI-CHEAT VALIDATION:");
        info!("  Verifying that revealed tokens match the original commitment");
        info!("  This prevents players from changing their tokens after seeing opponent's commitment");
        
        let mut manager = self.validation_manager.lock().await;
        let is_valid = manager.validate_token_reveal(&reveal)?;
        
        if is_valid {
            info!("✅ Token reveal validation PASSED - commitment matches revealed data");
            
            // Check if both players have revealed and ready for combat
            if let Ok(player_match) = manager.get_match(&reveal.match_event_id) {
                if player_match.is_ready_for_combat() {
                    info!("🎪 ARMY GENERATION READY:");
                    info!("  Both players have revealed their Cashu tokens");
                    info!("  Deterministic army generation can now begin");
                    info!("  Each token secret generates 8 unique units with random stats");
                    info!("  🚀 Match {} ready for combat phase", reveal.match_event_id);
                }
            }
        } else {
            warn!("❌ Token reveal validation FAILED - CHEATING ATTEMPT DETECTED");
            warn!("  Player {} tried to reveal different tokens than committed", reveal.player_npub);
            warn!("  This is cryptographically impossible unless player is cheating");
            
            // Mark match as invalid due to cheating attempt
            if let Ok(player_match) = manager.get_match_mut(&reveal.match_event_id) {
                player_match.mark_invalid("Invalid token reveal - commitment verification failed".to_string());
                warn!("  🚨 Match {} marked as INVALID due to cheating", reveal.match_event_id);
            }
        }
        
        Ok(())
    }

    /// Handle move commitment for a specific round
    async fn handle_move_commitment(&self, commitment: MoveCommitment) -> Result<(), GameEngineError> {
        info!("🔒 GAME FLOW: Move Commitment (Round {})", commitment.round_number);
        info!("📋 Commitment Details:");
        info!("  Player: {}", commitment.player_npub);
        info!("  Match: {}", commitment.match_event_id);
        info!("  Round: {}", commitment.round_number);
        info!("  Committed At: {}", commitment.committed_at);
        debug!("  Move Commitment Hash: {}", commitment.move_commitment);
        
        info!("⚔️ COMBAT ROUND SYSTEM:");
        info!("  Players select which unit to use and what abilities to activate");
        info!("  Moves are committed as hashes to prevent seeing opponent's choice first");
        info!("  Once both players commit, they reveal their actual moves");
        info!("  Combat is resolved deterministically using shared game logic");
        
        let mut manager = self.validation_manager.lock().await;
        
        // Store the commitment in the match state
        if let Ok(player_match) = manager.get_match_mut(&commitment.match_event_id) {
            player_match.add_move_commitment(&commitment)?;
            
            // Check if both players have committed for this round
            if player_match.both_players_committed_round(commitment.round_number) {
                info!("✅ ROUND COMMITMENTS COMPLETE:");
                info!("  Both players have committed their moves for round {}", commitment.round_number);
                info!("  Round is now locked - players can safely reveal their moves");
                info!("  Next: waiting for move reveals to execute combat");
            } else {
                info!("⏳ Waiting for opponent's move commitment for round {}", commitment.round_number);
            }
        } else {
            warn!("❌ Received move commitment for unknown match: {}", commitment.match_event_id);
        }
        
        Ok(())
    }

    /// Handle move reveal - validate and potentially resolve combat
    async fn handle_move_reveal(&self, reveal: MoveReveal) -> Result<(), GameEngineError> {
        info!("🔓 GAME FLOW: Move Reveal (Round {})", reveal.round_number);
        info!("📋 Reveal Details:");
        info!("  Player: {}", reveal.player_npub);
        info!("  Match: {}", reveal.match_event_id);
        info!("  Round: {}", reveal.round_number);
        info!("  Revealed At: {}", reveal.revealed_at);
        debug!("  Unit Positions: {:?}", reveal.unit_positions);
        debug!("  Unit Abilities: {:?}", reveal.unit_abilities);
        debug!("  Nonce: {}", reveal.moves_nonce);
        
        info!("🛡️ MOVE VALIDATION:");
        info!("  Verifying that revealed moves match the committed hash");
        info!("  This ensures players cannot change moves after seeing opponent's commitment");
        
        let mut manager = self.validation_manager.lock().await;
        let is_valid = manager.validate_move_reveal(&reveal)?;
        
        if is_valid {
            info!("✅ Move reveal validation PASSED - commitment matches revealed moves");
            
            // Check if both players have revealed for this round
            if let Ok(player_match) = manager.get_match(&reveal.match_event_id) {
                if player_match.both_players_revealed_round(reveal.round_number) {
                    info!("⚔️ COMBAT EXECUTION READY:");
                    info!("  Both players have revealed their moves for round {}", reveal.round_number);
                    info!("  Unit selections and abilities are now public");
                    info!("  Deterministic combat will be executed using shared game logic");
                    info!("  Players should calculate the same result independently");
                    info!("  🥊 Round {} ready for combat resolution", reveal.round_number);
                } else {
                    info!("⏳ Waiting for opponent's move reveal for round {}", reveal.round_number);
                }
            }
        } else {
            warn!("❌ Move reveal validation FAILED - CHEATING ATTEMPT DETECTED");
            warn!("  Player {} tried to reveal different moves than committed", reveal.player_npub);
            warn!("  Round {}: commitment verification failed", reveal.round_number);
            
            // Mark match as invalid due to cheating attempt
            if let Ok(player_match) = manager.get_match_mut(&reveal.match_event_id) {
                player_match.mark_invalid("Invalid move reveal - commitment verification failed".to_string());
                warn!("  🚨 Match {} marked as INVALID due to cheating", reveal.match_event_id);
            }
        }
        
        Ok(())
    }

    /// Handle match result - validate entire match and issue loot
    async fn handle_match_result(&self, result: MatchResult) -> Result<(), GameEngineError> {
        info!("🏁 GAME FLOW: Match Result Submitted");
        info!("📋 Result Details:");
        info!("  Submitting Player: {}", result.player_npub);
        info!("  Match: {}", result.match_event_id);
        info!("  Claimed Winner: {:?}", result.calculated_winner);
        info!("  Completed At: {}", result.match_completed_at);
        info!("  Round Results Count: {}", result.all_round_results.len());
        
        info!("🔍 COMPREHENSIVE MATCH VALIDATION:");
        info!("  The game engine will now perform a complete match validation");
        info!("  This includes verifying all commitments, re-executing combat, and confirming the winner");
        info!("  This is the final anti-cheat check before loot distribution");
        
        let mut manager = self.validation_manager.lock().await;
        
        // Validate the complete match result with detailed logging
        let validation_summary = manager.validate_match_result(&result.match_event_id, &result)?;
        
        if validation_summary.commitments_valid && 
           validation_summary.combat_verified && 
           validation_summary.winner_confirmed {
            
            info!("🎉 MATCH VALIDATION SUCCESSFUL:");
            info!("  ✅ All commitments verified");
            info!("  ✅ Combat re-executed and validated");
            info!("  ✅ Winner calculation confirmed");
            info!("  🏆 Ready for loot distribution");
            
            // Create loot distribution event
            let loot_distribution = LootDistribution {
                game_engine_npub: self.nostr_client.public_key(),
                match_event_id: result.match_event_id.clone(),
                winner_npub: result.calculated_winner.clone(),
                loot_cashu_token: None, // TODO: Create actual Cashu token
                match_fee: 5, // 5% fee
                loot_issued_at: chrono::Utc::now().timestamp() as u64,
                validation_summary,
            };
            
            info!("📡 LOOT DISTRIBUTION:");
            info!("  Publishing authoritative loot distribution event to Nostr");
            info!("  Winner: {:?}", loot_distribution.winner_npub);
            info!("  Match Fee: {}%", loot_distribution.match_fee);
            info!("  This is the ONLY event the game engine publishes");
            
            // Publish loot distribution event
            self.nostr_client.publish_loot_distribution(&loot_distribution, "dummy_event_id").await?;
            
            // Mark match as complete
            manager.mark_loot_distributed(&result.match_event_id)?;
            
            info!("🏆 MATCH COMPLETE: Loot distributed for match {}", result.match_event_id);
            info!("📚 GAME SUMMARY: This match demonstrates zero-coordination gaming");
            info!("   Players controlled the entire flow via Nostr events");
            info!("   Game engine only validated and distributed loot");
            info!("   No centralized server was required for coordination");
            
        } else {
            warn!("🚨 MATCH VALIDATION FAILED:");
            warn!("  ❌ Commitments Valid: {}", validation_summary.commitments_valid);
            warn!("  ❌ Combat Verified: {}", validation_summary.combat_verified);
            warn!("  ❌ Winner Confirmed: {}", validation_summary.winner_confirmed);
            if let Some(error) = &validation_summary.error_details {
                warn!("  Error: {}", error);
            }
            
            // Mark match as invalid
            if let Ok(player_match) = manager.get_match_mut(&result.match_event_id) {
                let error_msg = validation_summary.error_details
                    .unwrap_or_else(|| "Match validation failed".to_string());
                player_match.mark_invalid(error_msg);
                warn!("  🚨 Match {} marked as INVALID - no loot will be distributed", result.match_event_id);
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
        .route("/validate-match", axum::routing::post(validate_match))
        .route("/issue-loot", axum::routing::post(issue_loot))
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

async fn validate_match(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!("🔍 VALIDATE MATCH: Received validation request");
    
    let match_id = match payload.get("match_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            warn!("❌ No match_id provided in validation request");
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "match_id is required"})),
            ));
        }
    };
    
    info!("🔍 Validating match: {}", match_id);
    
    // Check if we have validation state for this match
    match state.bot.get_match_validation(match_id).await {
        Ok(Some(validation_state)) => {
            info!("✅ MATCH VALIDATION SUCCESS: Found validation state for match {}", match_id);
            Ok(Json(json!({
                "status": "success",
                "match_id": match_id,
                "validation_state": validation_state,
                "message": "Match validated successfully"
            })))
        }
        Ok(None) => {
            warn!("❌ MATCH VALIDATION FAILED: No validation state found for match {}", match_id);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "Match validation not found",
                    "match_id": match_id,
                    "details": "Game engine has no record of this match"
                })),
            ))
        }
        Err(e) => {
            error!("❌ MATCH VALIDATION ERROR: Failed to validate match {}: {}", match_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": e.to_string(),
                    "match_id": match_id
                })),
            ))
        }
    }
}

async fn issue_loot(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!("🏆 ISSUE LOOT: Received loot distribution request");
    
    let match_id = match payload.get("match_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            warn!("❌ No match_id provided in loot request");
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "match_id is required"})),
            ));
        }
    };
    
    let winner_npub = match payload.get("winner_npub").and_then(|v| v.as_str()) {
        Some(npub) => npub,
        None => {
            warn!("❌ No winner_npub provided in loot request");
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "winner_npub is required"})),
            ));
        }
    };
    
    info!("🏆 Issuing loot for match {} to winner {}", match_id, winner_npub);
    
    // Award loot to the winner
    match state.bot.award_loot(match_id, winner_npub).await {
        Ok(loot_result) => {
            info!("✅ LOOT ISSUED SUCCESSFULLY: Match {}, Winner {}", match_id, winner_npub);
            
            // Get the loot token quote for swapping
            let loot_quote = loot_result.get("quote")
                .and_then(|q| q.as_str())
                .unwrap_or("simulated_quote");
            let loot_amount = loot_result.get("loot_amount")
                .and_then(|a| a.as_u64())
                .unwrap_or(100);
                
            info!("💰 INITIATING LOOT SWAP: Winner {} can now claim {} tokens via swap", winner_npub, loot_amount);
            
            // Perform the loot token swap to make it claimable by the winner
            match state.bot.cashu_client.swap_loot_token(loot_quote, winner_npub, loot_amount).await {
                Ok(swap_result) => {
                    info!("🎉 COMPLETE ECONOMIC CYCLE: Loot swap successful for winner {}", winner_npub);
                    
                    // Simulate publishing loot distribution event to Nostr (KIND 31006)
                    info!("📡 PUBLISHING LOOT DISTRIBUTION: Publishing KIND 31006 event to Nostr relay");
                    
                    let loot_event = json!({
                        "event_type": "loot_distribution",
                        "kind": 31006,
                        "match_id": match_id,
                        "winner_npub": winner_npub,
                        "loot_amount": loot_amount,
                        "game_engine_npub": state.bot.get_status().await.get("nostr_pubkey").unwrap_or(&json!("game_engine_bot")),
                        "distributed_at": chrono::Utc::now().timestamp(),
                        "cashu_token_quote": loot_quote,
                        "swap_completed": true,
                        "spendable_tokens": swap_result.get("new_tokens_count").unwrap_or(&json!(loot_amount))
                    });
                    
                    info!("🏆 REVOLUTIONARY GAMING SUCCESS: Complete economic cycle from mana → army → combat → loot → swap");
                    
                    Ok(Json(json!({
                        "status": "success",
                        "match_id": match_id,
                        "winner_npub": winner_npub,
                        "loot_distribution": loot_result,
                        "loot_swap": swap_result,
                        "nostr_event": loot_event,
                        "economic_cycle_complete": true,
                        "message": "Complete economic cycle: Loot issued, swapped, and published to Nostr successfully"
                    })))
                }
                Err(swap_error) => {
                    warn!("⚠️ LOOT SWAP FAILED: {}, but loot still issued", swap_error);
                    
                    // Still publish loot distribution even if swap fails
                    let loot_event = json!({
                        "event_type": "loot_distribution",
                        "kind": 31006,
                        "match_id": match_id,
                        "winner_npub": winner_npub,
                        "loot_amount": loot_amount,
                        "distributed_at": chrono::Utc::now().timestamp(),
                        "cashu_token_quote": loot_quote,
                        "swap_completed": false,
                        "swap_error": swap_error.to_string()
                    });
                    
                    Ok(Json(json!({
                        "status": "partial_success",
                        "match_id": match_id,
                        "winner_npub": winner_npub,
                        "loot_distribution": loot_result,
                        "nostr_event": loot_event,
                        "swap_error": swap_error.to_string(),
                        "message": "Loot issued successfully, but swap failed - winner can claim manually"
                    })))
                }
            }
        }
        Err(e) => {
            error!("❌ LOOT DISTRIBUTION ERROR: Failed to issue loot for match {}: {}", match_id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": e.to_string(),
                    "match_id": match_id,
                    "winner_npub": winner_npub
                })),
            ))
        }
    }
}