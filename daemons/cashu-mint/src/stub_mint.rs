use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use uuid::Uuid;

use crate::errors::MintError;

pub type SharedStubState = Arc<RwLock<StubMintState>>;

#[derive(Clone)]
pub struct StubMintState {
    pub tokens: HashMap<String, StubToken>,
    pub quotes: HashMap<String, StubQuote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubToken {
    pub id: String,
    pub amount: u64,
    pub currency: String,
    pub secret: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubQuote {
    pub id: String,
    pub amount: u64,
    pub currency: String,
    pub state: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub currencies: Vec<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintInfoResponse {
    pub name: String,
    pub version: String,
    pub description: String,
    pub currencies: HashMap<String, CurrencyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyInfo {
    pub unit: String,
    pub precision: u8,
    pub min_amount: u64,
    pub max_amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysResponse {
    pub keysets: Vec<KeysetInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeysetInfo {
    pub id: String,
    pub currency: String,
    pub keys: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct MintQuoteRequest {
    pub amount: u64,
    pub currency: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MintQuoteResponse {
    pub quote: String,
    pub request: String, // Lightning payment request (stub)
    pub amount: u64,
    pub currency: String,
    pub state: String,
    pub expiry: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct MintRequest {
    pub quote: String,
    pub outputs: Vec<MintOutput>,
}

#[derive(Debug, Serialize, Deserialize)]  
pub struct MintOutput {
    pub amount: u64,
    pub b_: String, // Blinded message
}

#[derive(Debug, Serialize)]
pub struct MintResponse {
    pub signatures: Vec<MintSignature>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MintSignature {
    pub amount: u64,
    pub c_: String, // Blind signature
    pub id: String, // Keyset ID
}

#[derive(Debug, Deserialize)]
pub struct SwapRequest {
    pub inputs: Vec<SwapInput>,
    pub outputs: Vec<MintOutput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapInput {
    pub amount: u64,
    pub secret: String,
    pub c: String, // Signature
}

#[derive(Debug, Serialize)]
pub struct SwapResponse {
    pub signatures: Vec<MintSignature>,
}

impl StubMintState {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            quotes: HashMap::new(),
        }
    }
}

pub fn create_stub_mint_router() -> Router {
    let state = Arc::new(RwLock::new(StubMintState::new()));
    create_stub_mint_router_with_state(state)
}

pub fn create_stub_mint_router_with_state(state: SharedStubState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/info", get(mint_info))
        .route("/v1/keys", get(keys))
        .route("/v1/keys/:currency", get(keys_by_currency))
        .route("/v1/keysets", get(keysets))
        .route("/v1/mint/quote/bolt11", post(mint_quote_bolt11))
        .route("/v1/mint/bolt11", post(mint_bolt11))
        .route("/v1/swap", post(swap))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        currencies: vec!["mana".to_string(), "loot".to_string()],
        version: "0.1.0-stub".to_string(),
    })
}

async fn mint_info() -> Json<MintInfoResponse> {
    let mut currencies = HashMap::new();
    currencies.insert(
        "mana".to_string(),
        CurrencyInfo {
            unit: "mana".to_string(),
            precision: 0,
            min_amount: 1,
            max_amount: 1_000_000,
        },
    );
    currencies.insert(
        "loot".to_string(),
        CurrencyInfo {
            unit: "loot".to_string(),
            precision: 0,
            min_amount: 1,
            max_amount: 1_000_000,
        },
    );

    Json(MintInfoResponse {
        name: "Manastr Stub Mint".to_string(),
        version: "0.1.0-stub".to_string(),
        description: "Stub mint for testing Mana Strategy Game".to_string(),
        currencies,
    })
}

async fn keys(State(_state): State<SharedStubState>) -> Result<Json<KeysResponse>, MintError> {
    // Return deterministic test keys
    let mut keys_map = HashMap::new();
    keys_map.insert("1".to_string(), "test_key_1".to_string());
    keys_map.insert("2".to_string(), "test_key_2".to_string());
    keys_map.insert("4".to_string(), "test_key_4".to_string());
    keys_map.insert("8".to_string(), "test_key_8".to_string());

    let keyset = KeysetInfo {
        id: "test_keyset_id".to_string(),
        currency: "mana".to_string(),
        keys: keys_map,
    };

    Ok(Json(KeysResponse {
        keysets: vec![keyset],
    }))
}

async fn keys_by_currency(
    Path(currency): Path<String>,
    State(_state): State<SharedStubState>,
) -> Result<Json<KeysResponse>, MintError> {
    let mut keys_map = HashMap::new();
    keys_map.insert("1".to_string(), format!("test_key_1_{}", currency));
    keys_map.insert("2".to_string(), format!("test_key_2_{}", currency));
    keys_map.insert("4".to_string(), format!("test_key_4_{}", currency));
    keys_map.insert("8".to_string(), format!("test_key_8_{}", currency));

    let keyset = KeysetInfo {
        id: format!("test_keyset_{}", currency),
        currency: currency.clone(),
        keys: keys_map,
    };

    Ok(Json(KeysResponse {
        keysets: vec![keyset],
    }))
}

async fn keysets(State(_state): State<SharedStubState>) -> Result<Json<Vec<String>>, MintError> {
    Ok(Json(vec![
        "test_keyset_mana".to_string(),
        "test_keyset_loot".to_string(),
    ]))
}

async fn mint_quote_bolt11(
    State(state): State<SharedStubState>,
    Json(req): Json<MintQuoteRequest>,
) -> Result<Json<MintQuoteResponse>, MintError> {
    let currency = req.currency.unwrap_or_else(|| "mana".to_string());
    let quote_id = Uuid::new_v4().to_string();
    
    let quote = StubQuote {
        id: quote_id.clone(),
        amount: req.amount,
        currency: currency.clone(),
        state: "unpaid".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    {
        let mut state_guard = state.write().await;
        state_guard.quotes.insert(quote_id.clone(), quote);
    }

    info!("💰 Created mint quote {} for {} {}", quote_id, req.amount, currency);

    Ok(Json(MintQuoteResponse {
        quote: quote_id,
        request: format!("lnbc{}u1...stub_invoice", req.amount), // Stub Lightning invoice
        amount: req.amount,
        currency,
        state: "unpaid".to_string(),
        expiry: Some((chrono::Utc::now().timestamp() + 3600) as u64),
    }))
}

async fn mint_bolt11(
    State(state): State<SharedStubState>,
    Json(req): Json<MintRequest>,
) -> Result<Json<MintResponse>, MintError> {
    info!("🔨 Minting tokens for quote: {}", req.quote);
    
    // Simulate minting process
    let mut signatures = Vec::new();
    for output in req.outputs {
        let signature = MintSignature {
            amount: output.amount,
            c_: format!("stub_signature_{}", Uuid::new_v4()),
            id: "test_keyset_id".to_string(),
        };
        signatures.push(signature);
    }

    // Update quote state
    {
        let mut state_guard = state.write().await;
        if let Some(quote) = state_guard.quotes.get_mut(&req.quote) {
            quote.state = "paid".to_string();
        }
    }

    info!("✅ Minted {} signatures", signatures.len());

    Ok(Json(MintResponse { signatures }))
}

async fn swap(
    State(_state): State<SharedStubState>,
    Json(req): Json<SwapRequest>,
) -> Result<Json<SwapResponse>, MintError> {
    info!("🔄 Processing swap request with {} inputs, {} outputs", 
          req.inputs.len(), req.outputs.len());
    
    // Simulate swap process
    let mut signatures = Vec::new();
    for output in req.outputs {
        let signature = MintSignature {
            amount: output.amount,
            c_: format!("swap_signature_{}", Uuid::new_v4()),
            id: "test_keyset_id".to_string(),
        };
        signatures.push(signature);
    }

    info!("✅ Swapped tokens: {} signatures", signatures.len());

    Ok(Json(SwapResponse { signatures }))
}