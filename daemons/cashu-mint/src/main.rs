use anyhow::Result;
use tracing::{info, warn};

mod config;
mod errors;
mod stub_mint;

use config::MintConfig;
use stub_mint::create_stub_mint_router;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("cashu_mint=debug,tower_http=debug")
        .init();

    info!("🏪 Starting Cashu Stub Mint for Manastr Testing");

    // Load configuration
    let _config = MintConfig::load().unwrap_or_else(|_| {
        warn!("Using default configuration");
        MintConfig::default()
    });

    // Create the stub mint router
    let app = create_stub_mint_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3333").await?;
    info!("🚀 Cashu Stub Mint listening on http://127.0.0.1:3333");
    info!("💰 Supported currencies: Mana, Loot");
    info!("❤️ Health check: http://127.0.0.1:3333/health");
    info!("🔧 This is a STUB implementation for testing - Lightning operations are mocked");

    axum::serve(listener, app).await?;

    Ok(())
}