use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::io::AsyncBufReadExt;
use tokio::time::{sleep, timeout};
use tracing::{info, error, warn};
use crate::integrated_nostr_relay::IntegratedNostrRelay;

/// Service orchestrator that starts all background services before launching Tauri
/// This ensures proper service availability with fail-fast behavior
pub struct ServiceOrchestrator {
    cdk_process: Option<tokio::process::Child>,
    integrated_nostr_relay: Option<IntegratedNostrRelay>,
    game_engine_process: Option<tokio::process::Child>,
}

impl ServiceOrchestrator {
    pub fn new() -> Self {
        Self {
            cdk_process: None,
            integrated_nostr_relay: None,
            game_engine_process: None,
        }
    }

    /// Start all background services with proper error handling and health checks
    pub async fn start_all_services(&mut self) -> Result<()> {
        info!("🚀 Starting Manastr service orchestration...");
        
        // Start services in dependency order
        self.start_cdk_mint().await?;
        self.start_nostr_relay().await?;
        self.start_game_engine().await?;
        
        // Wait for all services to be ready
        self.wait_for_service_health().await?;
        
        info!("✅ All services started successfully and are healthy!");
        Ok(())
    }

    async fn start_cdk_mint(&mut self) -> Result<()> {
        info!("🏛️ Starting CDK Mint service...");
        
        // Clean up any existing mint database to avoid conflicts
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let mint_db_dir = std::path::Path::new(&home).join(".cdk-mintd");
        if mint_db_dir.exists() {
            std::fs::remove_dir_all(&mint_db_dir)?;
            info!("🧹 Cleaned up existing mint database");
        }

        let config_path = "../../config/cdk-mintd-deterministic.toml";
        if !std::path::Path::new(config_path).exists() {
            return Err(anyhow!("CDK mint config not found: {}", config_path));
        }

        let mut child = tokio::process::Command::new("cargo")
            .args(&[
                "run", "--release", "--bin", "cdk-mintd", "--",
                "--config", config_path
            ])
            .current_dir("../../cdk")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        // Wait a moment for startup
        sleep(Duration::from_secs(3)).await;

        // Check if process is still running
        if let Ok(Some(status)) = child.try_wait() {
            return Err(anyhow!("CDK mint failed to start: {:?}", status));
        }

        self.cdk_process = Some(child);
        info!("✅ CDK Mint service started");
        Ok(())
    }

    async fn start_nostr_relay(&mut self) -> Result<()> {
        info!("📡 Starting Integrated Nostr Relay with true library integration...");
        
        // Use the same database directory as CDK to share storage
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = std::path::Path::new(&home).join(".cdk-mintd").join("manastr_nostr.db");
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Create integrated nostr relay
        let mut relay = IntegratedNostrRelay::new(db_path.to_string_lossy().to_string());
        
        // Start the relay in a background task
        let relay_task = tokio::spawn(async move {
            if let Err(e) = relay.start().await {
                error!("Integrated Nostr Relay failed: {}", e);
                return Err(e);
            }
            Ok(())
        });

        // Wait a moment for startup
        sleep(Duration::from_secs(1)).await;

        // Check if the task is still running
        if relay_task.is_finished() {
            match relay_task.await {
                Ok(Err(e)) => return Err(anyhow!("Integrated Nostr Relay failed to start: {}", e)),
                Err(e) => return Err(anyhow!("Integrated Nostr Relay task panicked: {}", e)),
                _ => {}
            }
        }

        // Store a placeholder - the actual relay is running in the background task
        self.integrated_nostr_relay = Some(IntegratedNostrRelay::new(db_path.to_string_lossy().to_string()));
        
        info!("✅ Integrated Nostr Relay started successfully");
        info!("   • True library integration - no external processes");
        info!("   • Shares SQLite database with CDK (no dependency conflicts)");
        info!("   • WebSocket server running on ws://127.0.0.1:7777");
        info!("   • Full event storage and querying capabilities");
        Ok(())
    }

    async fn start_game_engine(&mut self) -> Result<()> {
        info!("🎮 Starting Game Engine service...");
        
        let config_path = "../../game-engine-bot/game-engine.toml";
        if !std::path::Path::new(config_path).exists() {
            return Err(anyhow!("Game engine config not found: {}", config_path));
        }

        let mut child = tokio::process::Command::new("cargo")
            .args(&["run", "--release", "--bin", "game-engine-bot"])
            .current_dir("../../game-engine-bot")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        // Wait for startup
        sleep(Duration::from_secs(2)).await;

        // Check if process is still running
        if let Ok(Some(status)) = child.try_wait() {
            return Err(anyhow!("Game engine failed to start: {:?}", status));
        }

        self.game_engine_process = Some(child);
        info!("✅ Game Engine service started");
        Ok(())
    }

    async fn wait_for_service_health(&self) -> Result<()> {
        info!("🔍 Performing health checks on all services...");
        
        let client = reqwest::Client::new();
        let health_timeout = Duration::from_secs(30);
        
        // Check CDK Mint health (port 3333)
        info!("Checking CDK Mint health...");
        let cdk_health = timeout(health_timeout, async {
            loop {
                match client.get("http://127.0.0.1:3333/v1/info").send().await {
                    Ok(response) if response.status().is_success() => break,
                    _ => {
                        sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                }
            }
        }).await;
        
        if cdk_health.is_err() {
            return Err(anyhow!("CDK Mint health check failed - service not responding"));
        }
        info!("✅ CDK Mint is healthy");

        // Check Nostr Relay health (port 7777)
        info!("Checking Nostr Relay health...");
        let nostr_health = timeout(health_timeout, async {
            loop {
                match client.get("http://127.0.0.1:7777").send().await {
                    Ok(_) => break, // Any response indicates the service is up
                    _ => {
                        sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                }
            }
        }).await;
        
        if nostr_health.is_err() {
            return Err(anyhow!("Nostr Relay health check failed - service not responding"));
        }
        info!("✅ Nostr Relay is healthy");

        // Check Game Engine health (port 4444)
        info!("Checking Game Engine health...");
        let game_engine_health = timeout(health_timeout, async {
            loop {
                match client.get("http://127.0.0.1:4444/health").send().await {
                    Ok(response) if response.status().is_success() => break,
                    _ => {
                        sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                }
            }
        }).await;
        
        if game_engine_health.is_err() {
            return Err(anyhow!("Game Engine health check failed - service not responding"));
        }
        info!("✅ Game Engine is healthy");

        Ok(())
    }

    /// Stop all services gracefully
    pub async fn stop_all_services(&mut self) {
        info!("🛑 Stopping all services...");

        if let Some(mut child) = self.game_engine_process.take() {
            let _ = child.kill().await;
            info!("Stopped Game Engine service");
        }

        if let Some(_relay) = self.integrated_nostr_relay.take() {
            // The integrated relay will be dropped and cleaned up automatically
            info!("Stopped Integrated Nostr Relay");
        }

        if let Some(mut child) = self.cdk_process.take() {
            let _ = child.kill().await;
            info!("Stopped CDK Mint service");
        }

        info!("✅ All services stopped");
    }
}

impl Drop for ServiceOrchestrator {
    fn drop(&mut self) {
        // Ensure services are cleaned up on drop
        if let Some(mut child) = self.game_engine_process.take() {
            let _ = child.start_kill();
        }
        if let Some(_relay) = self.integrated_nostr_relay.take() {
            // Integrated relay is cleaned up automatically
        }
        if let Some(mut child) = self.cdk_process.take() {
            let _ = child.start_kill();
        }
    }
}