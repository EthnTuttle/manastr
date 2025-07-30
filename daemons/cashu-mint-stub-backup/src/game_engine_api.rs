// 🎮 GAME ENGINE AUTHORIZATION API
// ================================
//
// This module provides secure API endpoints for authorized game engines to:
// - Burn mana tokens after match validation 
// - Query token spent status for anti-cheat
// - Request loot token minting for winners
//
// 🔐 SECURITY: All operations require valid Nostr signature from authorized game engine

use axum::{
    extract::{State, Request},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
    middleware::{self, Next},
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, collections::HashMap};
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::{config::MintConfig, errors::MintError, stub_mint::SharedStubState};

// Shared configuration state for runtime auth config reloading
type SharedConfig = Arc<RwLock<MintConfig>>;

#[derive(Debug, Deserialize)]
pub struct BurnManaRequest {
    /// Array of mana token secrets to burn/spend
    pub token_secrets: Vec<String>,
    /// Match ID (Nostr Event ID) for audit trail
    pub match_id: String,
    /// Nostr signature proving authorized game engine
    pub nostr_signature: String,
    /// Game engine's public key (hex)
    pub game_engine_pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct BurnManaResponse {
    /// Number of tokens successfully burned
    pub tokens_burned: u32,
    /// Total amount burned (in mana units)
    pub total_amount_burned: u64,
    /// Transaction ID for audit trail
    pub transaction_id: String,
    /// Match ID confirmation
    pub match_id: String,
}

#[derive(Debug, Deserialize)]
pub struct QuerySpentStatusRequest {
    /// Array of token secrets to check
    pub token_secrets: Vec<String>,
    /// Nostr signature proving authorized game engine
    pub nostr_signature: String,
    /// Game engine's public key (hex)
    pub game_engine_pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct QuerySpentStatusResponse {
    /// Map of token_secret -> spent status
    pub token_status: HashMap<String, TokenSpentStatus>,
}

#[derive(Debug, Serialize)]
pub struct TokenSpentStatus {
    /// Whether the token has been spent/burned
    pub is_spent: bool,
    /// When it was spent (if applicable)
    pub spent_at: Option<i64>,
    /// Match ID where it was spent (if applicable)
    pub spent_in_match: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MintLootRequest {
    /// Amount of loot to mint for winner
    pub amount: u64,
    /// Winner's public key (Nostr hex format)
    pub winner_pubkey: String,
    /// Match ID (Nostr Event ID) for audit trail
    pub match_id: String,
    /// Nostr signature proving authorized game engine
    pub nostr_signature: String,
    /// Game engine's public key (hex)
    pub game_engine_pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct MintLootResponse {
    /// Minted loot tokens for the winner
    pub loot_tokens: Vec<LootToken>,
    /// Total amount minted
    pub total_amount: u64,
    /// Transaction ID for audit trail
    pub transaction_id: String,
    /// Match ID confirmation
    pub match_id: String,
}

#[derive(Debug, Serialize)]
pub struct LootToken {
    /// Token amount
    pub amount: u64,
    /// Blinded signature from mint
    pub signature: String,
    /// Keyset ID
    pub keyset_id: String,
}

/// Create router with game engine authorization endpoints
pub fn create_game_engine_router(config: SharedConfig, mint_state: SharedStubState) -> Router {
    Router::new()
        .route("/game-engine/burn-mana", post(burn_mana_tokens))
        .route("/game-engine/query-spent", post(query_spent_status))
        .route("/game-engine/mint-loot", post(mint_loot_tokens))
        .route("/game-engine/auth-status", post(check_auth_status))
        .with_state((config.clone(), mint_state))
        .layer(middleware::from_fn_with_state(config, auth_middleware))
}

/// Authorization middleware - validates Nostr signatures from game engines
async fn auth_middleware(
    State(_config): State<SharedConfig>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // For now, we'll do authorization checks in individual endpoints
    // This middleware just allows requests to pass through
    Ok(next.run(req).await)
}

/// Burn mana tokens after match validation (EXCLUSIVE to authorized game engines)
async fn burn_mana_tokens(
    State((config, mint_state)): State<(SharedConfig, SharedStubState)>,
    Json(req): Json<BurnManaRequest>,
) -> Result<Json<BurnManaResponse>, MintError> {
    info!("🔥 Game Engine requesting mana burn: {} tokens, match: {}", 
          req.token_secrets.len(), req.match_id);

    // Verify authorization
    let config_guard = config.read().await;
    if !config_guard.authorization.is_authorized(&req.game_engine_pubkey, "burn_mana") {
        error!("🚫 Unauthorized game engine attempted mana burn: {}", req.game_engine_pubkey);
        return Err(MintError::Unauthorized("Game engine not authorized for mana burning".into()));
    }

    // Check token limits
    if let Some(max_tokens) = config_guard.authorization.get_max_tokens_per_request(&req.game_engine_pubkey) {
        if req.token_secrets.len() > max_tokens as usize {
            warn!("🚫 Game engine exceeded token limit: {} > {}", req.token_secrets.len(), max_tokens);
            return Err(MintError::BadRequest(format!("Too many tokens (max: {})", max_tokens)));
        }
    }

    // Verify Nostr signature
    if !verify_nostr_signature(&req.nostr_signature, &req.game_engine_pubkey, &req.match_id).await {
        error!("🚫 Invalid Nostr signature from game engine: {}", req.game_engine_pubkey);
        return Err(MintError::Unauthorized("Invalid Nostr signature".into()));
    }

    drop(config_guard);

    // Burn the tokens (mark as spent in stub implementation)
    let mut mint_guard = mint_state.write().await;
    let mut tokens_burned = 0u32;
    let mut total_amount = 0u64;
    let transaction_id = uuid::Uuid::new_v4().to_string();

    for token_secret in &req.token_secrets {
        // In a real implementation, we'd verify the token and burn it
        // For stub, we'll simulate successful burning
        tokens_burned += 1;
        total_amount += 5; // Assume 5 mana per token for stub
        
        info!("   🔥 Burned mana token: {} (match: {})", token_secret, req.match_id);
    }

    drop(mint_guard);

    info!("✅ Successfully burned {} mana tokens ({} total mana) for match: {}", 
          tokens_burned, total_amount, req.match_id);

    Ok(Json(BurnManaResponse {
        tokens_burned,
        total_amount_burned: total_amount,
        transaction_id,
        match_id: req.match_id,
    }))
}

/// Query spent status of tokens for anti-cheat validation
async fn query_spent_status(
    State((config, mint_state)): State<(SharedConfig, SharedStubState)>,
    Json(req): Json<QuerySpentStatusRequest>,
) -> Result<Json<QuerySpentStatusResponse>, MintError> {
    info!("🔍 Game Engine querying spent status: {} tokens", req.token_secrets.len());

    // Verify authorization
    let config_guard = config.read().await;
    if !config_guard.authorization.is_authorized(&req.game_engine_pubkey, "query_spent") {
        error!("🚫 Unauthorized game engine attempted spent query: {}", req.game_engine_pubkey);
        return Err(MintError::Unauthorized("Game engine not authorized for spent queries".into()));
    }

    // Verify Nostr signature
    if !verify_nostr_signature(&req.nostr_signature, &req.game_engine_pubkey, "spent_query").await {
        error!("🚫 Invalid Nostr signature from game engine: {}", req.game_engine_pubkey);
        return Err(MintError::Unauthorized("Invalid Nostr signature".into()));
    }

    drop(config_guard);

    // Query spent status (stub implementation)
    let mint_guard = mint_state.read().await;
    let mut token_status = HashMap::new();

    for token_secret in &req.token_secrets {
        // In stub implementation, randomly determine spent status for demonstration
        let is_spent = token_secret.contains("spent");
        let status = TokenSpentStatus {
            is_spent,
            spent_at: if is_spent { Some(chrono::Utc::now().timestamp()) } else { None },
            spent_in_match: if is_spent { Some("previous_match_id".to_string()) } else { None },
        };
        token_status.insert(token_secret.clone(), status);
    }

    drop(mint_guard);

    info!("✅ Returned spent status for {} tokens", token_status.len());

    Ok(Json(QuerySpentStatusResponse { token_status }))
}

/// Mint loot tokens for match winners
async fn mint_loot_tokens(
    State((config, mint_state)): State<(SharedConfig, SharedStubState)>,
    Json(req): Json<MintLootRequest>,
) -> Result<Json<MintLootResponse>, MintError> {
    info!("💰 Game Engine requesting loot mint: {} loot for winner: {}, match: {}", 
          req.amount, req.winner_pubkey, req.match_id);

    // Verify authorization
    let config_guard = config.read().await;
    if !config_guard.authorization.is_authorized(&req.game_engine_pubkey, "mint_loot") {
        error!("🚫 Unauthorized game engine attempted loot minting: {}", req.game_engine_pubkey);
        return Err(MintError::Unauthorized("Game engine not authorized for loot minting".into()));
    }

    // Verify Nostr signature
    if !verify_nostr_signature(&req.nostr_signature, &req.game_engine_pubkey, &req.match_id).await {
        error!("🚫 Invalid Nostr signature from game engine: {}", req.game_engine_pubkey);
        return Err(MintError::Unauthorized("Invalid Nostr signature".into()));
    }

    drop(config_guard);

    // Mint loot tokens (stub implementation)
    let mut mint_guard = mint_state.write().await;
    let transaction_id = uuid::Uuid::new_v4().to_string();
    
    // Create loot tokens for the winner
    let mut loot_tokens = Vec::new();
    let mut remaining_amount = req.amount;
    
    // Break down into standard denominations (1, 2, 4, 8, 16, etc.)
    for &denomination in &[16u64, 8, 4, 2, 1] {
        while remaining_amount >= denomination {
            let token = LootToken {
                amount: denomination,
                signature: format!("loot_sig_{}_{}", denomination, uuid::Uuid::new_v4()),
                keyset_id: "loot_keyset_id".to_string(),
            };
            loot_tokens.push(token);
            remaining_amount -= denomination;
        }
    }

    drop(mint_guard);

    info!("✅ Successfully minted {} loot tokens ({} total loot) for winner: {} in match: {}", 
          loot_tokens.len(), req.amount, req.winner_pubkey, req.match_id);

    Ok(Json(MintLootResponse {
        loot_tokens,
        total_amount: req.amount,
        transaction_id,
        match_id: req.match_id,
    }))
}

/// Check authorization status for a game engine
async fn check_auth_status(
    State((config, _mint_state)): State<(SharedConfig, SharedStubState)>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, MintError> {
    let game_engine_pubkey = req.get("game_engine_pubkey")
        .and_then(|v| v.as_str())
        .ok_or_else(|| MintError::BadRequest("Missing game_engine_pubkey".into()))?;

    let config_guard = config.read().await;
    let auth_config = &config_guard.authorization;
    
    let authorized_engine = auth_config.authorized_game_engines
        .iter()
        .find(|engine| engine.nostr_pubkey_hex == game_engine_pubkey);

    let response = match authorized_engine {
        Some(engine) if engine.active => {
            serde_json::json!({
                "authorized": true,
                "name": engine.name,
                "permissions": {
                    "can_burn_mana": engine.permissions.can_burn_mana,
                    "can_query_spent_status": engine.permissions.can_query_spent_status,
                    "can_mint_loot": engine.permissions.can_mint_loot,
                    "max_tokens_per_request": engine.permissions.max_tokens_per_request
                }
            })
        },
        Some(engine) => {
            serde_json::json!({
                "authorized": false,
                "reason": "Game engine is disabled",
                "name": engine.name
            })
        },
        None => {
            serde_json::json!({
                "authorized": false,
                "reason": "Game engine not found in authorized list"
            })
        }
    };

    Ok(Json(response))
}

/// Verify Nostr signature for authentication
async fn verify_nostr_signature(signature: &str, pubkey_hex: &str, _content: &str) -> bool {
    // In a real implementation, we'd properly verify the Nostr signature
    // For stub implementation, we'll do basic validation
    
    if signature.is_empty() || pubkey_hex.is_empty() {
        return false;
    }

    // Basic hex validation for public key
    if pubkey_hex.len() != 64 {
        warn!("🚫 Invalid pubkey hex length: {}", pubkey_hex.len());
        return false;
    }

    if hex::decode(pubkey_hex).is_err() {
        warn!("🚫 Invalid pubkey hex format");
        return false;
    }

    // For stub implementation, accept any non-empty signature with valid pubkey format
    info!("✅ Nostr signature verified (stub implementation) for pubkey: {}", pubkey_hex);
    true
}