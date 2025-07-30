use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use axum::Router;

mod config;
mod errors;
mod stub_mint;
mod game_engine_api;

use config::MintConfig;
use stub_mint::StubMintState;
use game_engine_api::create_game_engine_router;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("cashu_mint=debug,tower_http=debug")
        .init();

    info!("🏪 Starting Cashu Stub Mint for Manastr Testing with Game Engine Authorization");

    // Load configuration with authorization settings
    let config = MintConfig::load().unwrap_or_else(|_| {
        warn!("Using default configuration");
        MintConfig::default()
    });

    // Log authorization status
    info!("🔐 Authorization: {} game engines configured", 
          config.authorization.authorized_game_engines.len());
    info!("🔄 Runtime config updates: {}", config.authorization.allow_runtime_updates);
    
    for engine in &config.authorization.authorized_game_engines {
        if engine.active {
            info!("   ✅ {} ({}...{})", 
                  engine.name, 
                  &engine.nostr_pubkey_hex[..8],
                  &engine.nostr_pubkey_hex[engine.nostr_pubkey_hex.len()-8..]);
        } else {
            info!("   ❌ {} (disabled)", engine.name);
        }
    }

    // Create shared state
    let shared_config = Arc::new(RwLock::new(config));
    let mint_state = Arc::new(RwLock::new(StubMintState::new()));

    // Create combined router with both standard Cashu and game engine endpoints
    let app = Router::new()
        .merge(create_stub_mint_router_with_state(mint_state.clone()))
        .merge(create_game_engine_router(shared_config.clone(), mint_state.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3333").await?;
    info!("🚀 Cashu Stub Mint listening on http://127.0.0.1:3333");
    info!("💰 Standard Cashu endpoints: /v1/* (minting, swapping)");
    info!("🎮 Game Engine endpoints: /game-engine/* (burn, query, mint-loot)");
    info!("❤️ Health check: http://127.0.0.1:3333/health");
    info!("🔧 This is a STUB implementation for testing - Lightning operations are mocked");
    info!("🔐 Game engine operations require valid Nostr signatures");

    // Start configuration hot-reload task if enabled
    if shared_config.read().await.authorization.allow_runtime_updates {
        let config_clone = shared_config.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                let mut config_guard = config_clone.write().await;
                match config_guard.reload_auth_config() {
                    Ok(true) => info!("🔄 Authorization configuration reloaded"),
                    Ok(false) => {}, // No update needed
                    Err(e) => warn!("⚠️ Failed to reload authorization config: {}", e),
                }
            }
        });
        info!("🔄 Started authorization config hot-reload task (10s interval)");
    }

    axum::serve(listener, app).await?;

    Ok(())
}

/// Create stub mint router that accepts external state (for integration with game engine API)
fn create_stub_mint_router_with_state(mint_state: Arc<RwLock<StubMintState>>) -> Router {
    stub_mint::create_stub_mint_router_with_state(mint_state)
}