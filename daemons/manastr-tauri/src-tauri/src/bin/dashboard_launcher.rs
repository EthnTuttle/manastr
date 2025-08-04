use anyhow::Result;
use std::process::Command;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber;

// Import the service orchestrator
use manastr_tauri_lib::service_orchestrator::ServiceOrchestrator;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 MANASTR DASHBOARD LAUNCHER");
    info!("============================");
    info!("");
    info!("Starting all background services before launching Tauri dashboard...");

    // Create service orchestrator
    let mut orchestrator = ServiceOrchestrator::new();

    // Start all services - PANIC if any fail to start
    match orchestrator.start_all_services().await {
        Ok(_) => {
            info!("✅ All services started successfully!");
            info!("🖥️  Launching Tauri dashboard...");
        }
        Err(e) => {
            error!("❌ CRITICAL: Failed to start services: {}", e);
            error!("💥 PANICKING - Cannot continue without all services running");
            panic!("SERVICE STARTUP FAILURE: {}", e);
        }
    }

    // Services are ready - now start Tauri in background
    let mut tauri_process = match Command::new("cargo")
        .args(&["tauri", "dev"])
        .current_dir(".")
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            error!("❌ Failed to start Tauri dashboard: {}", e);
            orchestrator.stop_all_services().await;
            panic!("TAURI STARTUP FAILURE: {}", e);
        }
    };

    info!("🎉 Dashboard launched successfully!");
    info!("📋 Services running:");
    info!("   • CDK Mint:     http://127.0.0.1:3333");
    info!("   • Nostr Relay:  ws://127.0.0.1:7777");
    info!("   • Game Engine:  http://127.0.0.1:4444");
    info!("");
    info!("Press Ctrl+C to stop all services and dashboard");

    // Wait for shutdown signal
    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    info!("🛑 Shutdown signal received");
    info!("Stopping Tauri dashboard...");
    
    // Kill Tauri process
    let _ = tauri_process.kill();
    let _ = tauri_process.wait();

    // Stop all services
    orchestrator.stop_all_services().await;

    info!("✅ Clean shutdown complete");
    Ok(())
}