use anyhow::Result;
use tracing::info;

use integration_tests::PlayerDrivenTestSuite;

/// Main entry point for player-driven integration tests
/// 
/// Runs the complete test suite to validate the zero-coordination
/// gaming architecture.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("🚀 Starting Player-Driven Integration Test Suite");
    
    let test_suite = PlayerDrivenTestSuite::new().await?;
    test_suite.run_comprehensive_tests().await?;
    
    info!("🎉 All Player-Driven Integration Tests Completed Successfully!");
    Ok(())
} 