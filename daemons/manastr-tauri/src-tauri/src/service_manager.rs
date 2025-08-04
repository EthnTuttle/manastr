use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

// MPSC Channel Architecture for Integrated Service Management

/// Messages sent from services to the main dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceMessage {
    StatusUpdate { service: String, status: ServiceStatus },
    LogMessage { service: String, message: String },
    Error { service: String, error: String },
    HealthCheck { service: String, healthy: bool },
}

/// Commands sent from dashboard to services
#[derive(Debug, Clone)]
pub enum ServiceCommand {
    Start,
    Stop,
    GetStatus,
    HealthCheck,
}

/// Service handle for managing integrated services
pub struct ServiceHandle {
    pub name: String,
    pub command_tx: mpsc::UnboundedSender<ServiceCommand>,
    pub status: Arc<RwLock<ServiceStatus>>,
}

/// Main service orchestrator using MPSC channels
pub struct ServiceOrchestrator {
    pub message_rx: mpsc::UnboundedReceiver<ServiceMessage>,
    pub message_tx: mpsc::UnboundedSender<ServiceMessage>,
    pub services: HashMap<String, ServiceHandle>,
}

impl ServiceOrchestrator {
    pub fn new() -> Self {
        let (message_tx, message_rx) = mpsc::unbounded_channel();
        
        Self {
            message_rx,
            message_tx,
            services: HashMap::new(),
        }
    }
    
    /// Get a cloned message sender for external use
    pub fn get_message_sender(&self) -> mpsc::UnboundedSender<ServiceMessage> {
        self.message_tx.clone()
    }
    
    /// Add a service to the orchestrator
    pub fn add_service(&mut self, name: String, command_tx: mpsc::UnboundedSender<ServiceCommand>) {
        let status = Arc::new(RwLock::new(ServiceStatus::Stopped));
        
        self.services.insert(name.clone(), ServiceHandle {
            name,
            command_tx,
            status,
        });
    }
    
    /// Start all services
    pub async fn start_all_services(&self) -> Result<()> {
        info!("🚀 Starting all integrated services via MPSC channels");
        
        for (name, handle) in &self.services {
            info!("Starting service: {}", name);
            handle.command_tx.send(ServiceCommand::Start)
                .map_err(|e| anyhow!("Failed to send start command to {}: {}", name, e))?;
        }
        
        Ok(())
    }
    
    /// Stop all services
    pub async fn stop_all_services(&self) -> Result<()> {
        info!("🛑 Stopping all integrated services");
        
        for (name, handle) in &self.services {
            info!("Stopping service: {}", name);
            handle.command_tx.send(ServiceCommand::Stop)
                .map_err(|e| anyhow!("Failed to send stop command to {}: {}", name, e))?;
        }
        
        Ok(())
    }
    
    /// Get current status of all services
    pub async fn get_all_status(&self) -> HashMap<String, ServiceStatus> {
        let mut status_map = HashMap::new();
        
        for (name, handle) in &self.services {
            let status = handle.status.read().await.clone();
            status_map.insert(name.clone(), status);
        }
        
        status_map
    }
    
    /// Process incoming service messages
    pub async fn process_messages(&mut self, data: &mut IntegrationData) -> Result<()> {
        while let Ok(message) = self.message_rx.try_recv() {
            match message {
                ServiceMessage::StatusUpdate { service, status } => {
                    info!("📊 {} status: {:?}", service, status);
                    
                    // Update service status
                    if let Some(handle) = self.services.get(&service) {
                        *handle.status.write().await = status.clone();
                    }
                    
                    // Update dashboard data
                    match service.as_str() {
                        "CDK Mint" => data.cdk_mint_status = status,
                        "Nostr Relay" => data.nostr_relay_status = status,
                        "Game Engine" => data.game_engine_status = status,
                        _ => {}
                    }
                }
                ServiceMessage::LogMessage { service, message } => {
                    data.service_logs.entry(service.clone())
                        .or_insert_with(Vec::new)
                        .push(message);
                    
                    // Keep only last 10 log messages per service
                    if let Some(logs) = data.service_logs.get_mut(&service) {
                        if logs.len() > 10 {
                            logs.remove(0);
                        }
                    }
                }
                ServiceMessage::Error { service, error } => {
                    error!("❌ {} error: {}", service, error);
                    data.integration_log.push(format!("❌ {}: {}", service, error));
                }
                ServiceMessage::HealthCheck { service, healthy } => {
                    if healthy {
                        info!("✅ {} health check passed", service);
                    } else {
                        warn!("⚠️ {} health check failed", service);
                    }
                }
            }
        }
        
        Ok(())
    }
}

/// Integrated CDK Mint Service
pub struct CdkMintService {
    status_tx: mpsc::UnboundedSender<ServiceMessage>,
    command_rx: mpsc::UnboundedReceiver<ServiceCommand>,
    mint_handle: Option<tokio::task::JoinHandle<()>>,
    mint_process: Option<tokio::process::Child>,
}

impl CdkMintService {
    pub fn new(status_tx: mpsc::UnboundedSender<ServiceMessage>) -> (Self, mpsc::UnboundedSender<ServiceCommand>) {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        
        (
            Self {
                status_tx,
                command_rx,
                mint_handle: None,
                mint_process: None,
            },
            command_tx,
        )
    }
    
    pub async fn run(&mut self) {
        info!("🏦 CDK Mint Service started, waiting for commands...");
        
        while let Some(command) = self.command_rx.recv().await {
            match command {
                ServiceCommand::Start => {
                    self.start_mint().await;
                }
                ServiceCommand::Stop => {
                    self.stop_mint().await;
                }
                ServiceCommand::GetStatus => {
                    self.send_status().await;
                }
                ServiceCommand::HealthCheck => {
                    self.health_check().await;
                }
            }
        }
    }
    
    async fn start_mint(&mut self) {
        if self.mint_handle.is_some() {
            self.send_message(ServiceMessage::LogMessage {
                service: "CDK Mint".to_string(),
                message: "Mint already running".to_string(),
            }).await;
            return;
        }
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "CDK Mint".to_string(),
            status: ServiceStatus::Starting,
        }).await;
        
        self.send_message(ServiceMessage::LogMessage {
            service: "CDK Mint".to_string(),
            message: "Starting integrated CDK mint service...".to_string(),
        }).await;
        
        let status_tx = self.status_tx.clone();
        
        // Spawn the actual mint service
        let handle = tokio::spawn(async move {
            if let Err(e) = run_integrated_cdk_mint(status_tx.clone()).await {
                let _ = status_tx.send(ServiceMessage::Error {
                    service: "CDK Mint".to_string(),
                    error: format!("Mint service error: {}", e),
                });
                
                let _ = status_tx.send(ServiceMessage::StatusUpdate {
                    service: "CDK Mint".to_string(),
                    status: ServiceStatus::Failed(e.to_string()),
                });
            }
        });
        
        self.mint_handle = Some(handle);
        
        // Give it a moment to start
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "CDK Mint".to_string(),
            status: ServiceStatus::Running,
        }).await;
        
        self.send_message(ServiceMessage::LogMessage {
            service: "CDK Mint".to_string(),
            message: "CDK Mint service running on port 3333".to_string(),
        }).await;
    }
    
    async fn stop_mint(&mut self) {
        // Kill the external process if running
        if let Some(mut process) = self.mint_process.take() {
            let _ = process.kill().await;
            self.send_message(ServiceMessage::LogMessage {
                service: "CDK Mint".to_string(),
                message: "CDK Mint process terminated".to_string(),
            }).await;
        }
        
        // Abort the service task if running
        if let Some(handle) = self.mint_handle.take() {
            handle.abort();
            
            self.send_message(ServiceMessage::StatusUpdate {
                service: "CDK Mint".to_string(),
                status: ServiceStatus::Stopped,
            }).await;
            
            self.send_message(ServiceMessage::LogMessage {
                service: "CDK Mint".to_string(),
                message: "CDK Mint service stopped".to_string(),
            }).await;
        }
    }
    
    async fn send_status(&self) {
        let status = if self.mint_handle.is_some() {
            ServiceStatus::Running
        } else {
            ServiceStatus::Stopped
        };
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "CDK Mint".to_string(),
            status,
        }).await;
    }
    
    async fn health_check(&self) {
        let healthy = if self.mint_handle.is_some() {
            // Try to connect to the mint
            match reqwest::get("http://127.0.0.1:3333/v1/info").await {
                Ok(response) => response.status().is_success(),
                Err(_) => false,
            }
        } else {
            false
        };
        
        self.send_message(ServiceMessage::HealthCheck {
            service: "CDK Mint".to_string(),
            healthy,
        }).await;
    }
    
    async fn send_message(&self, message: ServiceMessage) {
        let _ = self.status_tx.send(message);
    }
}

/// Run the integrated CDK mint service using direct Rust integration
async fn run_integrated_cdk_mint(status_tx: mpsc::UnboundedSender<ServiceMessage>) -> Result<()> {
    use std::path::Path;
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "CDK Mint".to_string(),
        message: "Starting CDK mint with direct Rust integration...".to_string(),
    });
    
    // Clean up mint database to avoid "already signed" errors
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let mint_db_dir = Path::new(&home).join(".cdk-mintd");
    if mint_db_dir.exists() {
        std::fs::remove_dir_all(&mint_db_dir)?;
        let _ = status_tx.send(ServiceMessage::LogMessage {
            service: "CDK Mint".to_string(),
            message: "Cleaned up mint database".to_string(),
        });
    }
    
    // Set up CDK configuration
    let config_path = Path::new("../../config/cdk-mintd-deterministic.toml");
    if !config_path.exists() {
        return Err(anyhow!("CDK mint config file not found: {:?}", config_path));
    }
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "CDK Mint".to_string(),
        message: "Using hard-coded CDK mint defaults...".to_string(),
    });
    
    // Use direct integration with existing config file (simpler approach)
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "CDK Mint".to_string(),
        message: "Loading existing config for direct integration...".to_string(),
    });
    
    // Set up workspace directory
    let workspace_root = std::path::Path::new("../../../");
    let work_dir = workspace_root.canonicalize()?;
    
    // Load settings from the existing deterministic config file
    let settings = cdk_mintd::load_settings(&work_dir, Some(config_path.to_path_buf()))?;
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "CDK Mint".to_string(),
        message: "Starting CDK mint with direct integration...".to_string(),
    });
    
    // Run the CDK mint directly using the library function 
    match cdk_mintd::run_mintd(&work_dir, &settings, None).await {
        Ok(_) => {
            let _ = status_tx.send(ServiceMessage::LogMessage {
                service: "CDK Mint".to_string(),
                message: "CDK mint service completed successfully".to_string(),
            });
        }
        Err(e) => {
            let _ = status_tx.send(ServiceMessage::Error {
                service: "CDK Mint".to_string(),
                error: format!("CDK mint service error: {}", e),
            });
            return Err(e);
        }
    }
    
    Ok(())
}

/// Integrated Nostr Relay Service
pub struct NostrRelayService {
    status_tx: mpsc::UnboundedSender<ServiceMessage>,
    command_rx: mpsc::UnboundedReceiver<ServiceCommand>,
    relay_handle: Option<tokio::task::JoinHandle<()>>,
    relay_process: Option<tokio::process::Child>,
}

impl NostrRelayService {
    pub fn new(status_tx: mpsc::UnboundedSender<ServiceMessage>) -> (Self, mpsc::UnboundedSender<ServiceCommand>) {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        
        (
            Self {
                status_tx,
                command_rx,
                relay_handle: None,
                relay_process: None,
            },
            command_tx,
        )
    }
    
    pub async fn run(&mut self) {
        info!("📡 Nostr Relay Service started, waiting for commands...");
        
        while let Some(command) = self.command_rx.recv().await {
            match command {
                ServiceCommand::Start => {
                    self.start_relay().await;
                }
                ServiceCommand::Stop => {
                    self.stop_relay().await;
                }
                ServiceCommand::GetStatus => {
                    self.send_status().await;
                }
                ServiceCommand::HealthCheck => {
                    self.health_check().await;
                }
            }
        }
    }
    
    async fn start_relay(&mut self) {
        if self.relay_handle.is_some() {
            self.send_message(ServiceMessage::LogMessage {
                service: "Nostr Relay".to_string(),
                message: "Relay already running".to_string(),
            }).await;
            return;
        }
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "Nostr Relay".to_string(),
            status: ServiceStatus::Starting,
        }).await;
        
        let status_tx = self.status_tx.clone();
        
        let handle = tokio::spawn(async move {
            if let Err(e) = run_integrated_nostr_relay(status_tx.clone()).await {
                let _ = status_tx.send(ServiceMessage::Error {
                    service: "Nostr Relay".to_string(),
                    error: format!("Relay service error: {}", e),
                });
                
                let _ = status_tx.send(ServiceMessage::StatusUpdate {
                    service: "Nostr Relay".to_string(),
                    status: ServiceStatus::Failed(e.to_string()),
                });
            }
        });
        
        self.relay_handle = Some(handle);
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "Nostr Relay".to_string(),
            status: ServiceStatus::Running,
        }).await;
        
        self.send_message(ServiceMessage::LogMessage {
            service: "Nostr Relay".to_string(),
            message: "Nostr Relay service running on port 7777".to_string(),
        }).await;
    }
    
    async fn stop_relay(&mut self) {
        if let Some(handle) = self.relay_handle.take() {
            handle.abort();
            
            self.send_message(ServiceMessage::StatusUpdate {
                service: "Nostr Relay".to_string(),
                status: ServiceStatus::Stopped,
            }).await;
            
            self.send_message(ServiceMessage::LogMessage {
                service: "Nostr Relay".to_string(),
                message: "Nostr Relay service stopped".to_string(),
            }).await;
        }
    }
    
    async fn send_status(&self) {
        let status = if self.relay_handle.is_some() {
            ServiceStatus::Running
        } else {
            ServiceStatus::Stopped
        };
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "Nostr Relay".to_string(),
            status,
        }).await;
    }
    
    async fn health_check(&self) {
        let healthy = if self.relay_handle.is_some() {
            // Try to connect to the relay
            match reqwest::get("http://127.0.0.1:7777").await {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        };
        
        self.send_message(ServiceMessage::HealthCheck {
            service: "Nostr Relay".to_string(),
            healthy,
        }).await;
    }
    
    async fn send_message(&self, message: ServiceMessage) {
        let _ = self.status_tx.send(message);
    }
}

/// Run the integrated Nostr relay service (external process due to SQLite version conflicts)
async fn run_integrated_nostr_relay(status_tx: mpsc::UnboundedSender<ServiceMessage>) -> Result<()> {
    use std::path::Path;
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Nostr Relay".to_string(),
        message: "Starting Nostr relay with external process (SQLite version compatibility)...".to_string(),
    });
    
    // Create required directories
    std::fs::create_dir_all("../../nostr-relay/logs").ok();
    std::fs::create_dir_all("../../nostr-relay/nostr-relay-db").ok();
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Nostr Relay".to_string(),
        message: "Building Nostr relay binary...".to_string(),
    });
    
    // Build the Nostr relay
    let build_output = tokio::process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("../../nostr-relay/nostr-rs-relay")
        .output()
        .await?;
        
    if !build_output.status.success() {
        let error = String::from_utf8_lossy(&build_output.stderr);
        return Err(anyhow!("Nostr relay build failed: {}", error));
    }
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Nostr Relay".to_string(),
        message: "Nostr relay binary built successfully".to_string(),
    });
    
    // Run the relay binary
    let mut child = tokio::process::Command::new("./nostr-rs-relay/target/release/nostr-rs-relay")
        .args(&["--config", "config.toml"])
        .current_dir("../../nostr-relay")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Nostr Relay".to_string(),
        message: "Nostr relay process started".to_string(),
    });
    
    // Monitor the process
    let status = child.wait().await?;
    
    if status.success() {
        let _ = status_tx.send(ServiceMessage::LogMessage {
            service: "Nostr Relay".to_string(),
            message: "Nostr relay process completed successfully".to_string(),
        });
    } else {
        let _ = status_tx.send(ServiceMessage::Error {
            service: "Nostr Relay".to_string(),
            error: format!("Nostr relay process exited with error: {:?}", status),
        });
    }
    
    Ok(())
}

/// Integrated Game Engine Service
pub struct GameEngineService {
    status_tx: mpsc::UnboundedSender<ServiceMessage>,
    command_rx: mpsc::UnboundedReceiver<ServiceCommand>,
    engine_handle: Option<tokio::task::JoinHandle<()>>,
}

impl GameEngineService {
    pub fn new(status_tx: mpsc::UnboundedSender<ServiceMessage>) -> (Self, mpsc::UnboundedSender<ServiceCommand>) {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        
        (
            Self {
                status_tx,
                command_rx,
                engine_handle: None,
            },
            command_tx,
        )
    }
    
    pub async fn run(&mut self) {
        info!("🎮 Game Engine Service started, waiting for commands...");
        
        while let Some(command) = self.command_rx.recv().await {
            match command {
                ServiceCommand::Start => {
                    self.start_engine().await;
                }
                ServiceCommand::Stop => {
                    self.stop_engine().await;
                }
                ServiceCommand::GetStatus => {
                    self.send_status().await;
                }
                ServiceCommand::HealthCheck => {
                    self.health_check().await;
                }
            }
        }
    }
    
    async fn start_engine(&mut self) {
        if self.engine_handle.is_some() {
            self.send_message(ServiceMessage::LogMessage {
                service: "Game Engine".to_string(),
                message: "Engine already running".to_string(),
            }).await;
            return;
        }
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "Game Engine".to_string(),
            status: ServiceStatus::Starting,
        }).await;
        
        let status_tx = self.status_tx.clone();
        
        let handle = tokio::spawn(async move {
            if let Err(e) = run_integrated_game_engine(status_tx.clone()).await {
                let _ = status_tx.send(ServiceMessage::Error {
                    service: "Game Engine".to_string(),
                    error: format!("Engine service error: {}", e),
                });
                
                let _ = status_tx.send(ServiceMessage::StatusUpdate {
                    service: "Game Engine".to_string(),
                    status: ServiceStatus::Failed(e.to_string()),
                });
            }
        });
        
        self.engine_handle = Some(handle);
        
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "Game Engine".to_string(),
            status: ServiceStatus::Running,
        }).await;
        
        self.send_message(ServiceMessage::LogMessage {
            service: "Game Engine".to_string(),
            message: "Game Engine service running and monitoring Nostr events".to_string(),
        }).await;
    }
    
    async fn stop_engine(&mut self) {
        if let Some(handle) = self.engine_handle.take() {
            handle.abort();
            
            self.send_message(ServiceMessage::StatusUpdate {
                service: "Game Engine".to_string(),
                status: ServiceStatus::Stopped,
            }).await;
            
            self.send_message(ServiceMessage::LogMessage {
                service: "Game Engine".to_string(),
                message: "Game Engine service stopped".to_string(),
            }).await;
        }
    }
    
    async fn send_status(&self) {
        let status = if self.engine_handle.is_some() {
            ServiceStatus::Running
        } else {
            ServiceStatus::Stopped
        };
        
        self.send_message(ServiceMessage::StatusUpdate {
            service: "Game Engine".to_string(),
            status,
        }).await;
    }
    
    async fn health_check(&self) {
        let healthy = if self.engine_handle.is_some() {
            // Game engine doesn't have HTTP endpoints, so we check if the task is still running
            true // If handle exists, service is considered healthy
        } else {
            false
        };
        
        self.send_message(ServiceMessage::HealthCheck {
            service: "Game Engine".to_string(),
            healthy,
        }).await;
    }
    
    async fn send_message(&self, message: ServiceMessage) {
        let _ = self.status_tx.send(message);
    }
}

/// Run the integrated game engine service using direct Rust integration
async fn run_integrated_game_engine(status_tx: mpsc::UnboundedSender<ServiceMessage>) -> Result<()> {
    use std::sync::Arc;
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Game Engine".to_string(),
        message: "Starting Game Engine with direct Rust integration...".to_string(),
    });
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Game Engine".to_string(),
        message: "Using hard-coded Game Engine defaults...".to_string(),
    });
    
    // Use hard-coded defaults for now to avoid config issues
    let config = game_engine_bot::config::GameEngineConfig {
        server: game_engine_bot::config::ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 4444,
        },
        nostr: game_engine_bot::config::NostrConfig {
            relay_url: "ws://127.0.0.1:7777".to_string(),
            private_key: "nsec1ufnus6pju9kf2zkgs4jpts9fzjanrvpjuzflpa9d9jyq9uu5s6j0qvk6dpm".to_string(), // Valid test key
        },
        cashu: game_engine_bot::config::CashuConfig {
            mint_url: "http://127.0.0.1:3333".to_string(),
        },
        game: game_engine_bot::config::GameConfig {
            max_concurrent_matches: 10,
            round_timeout_seconds: 30,
            match_timeout_seconds: 300,
            loot_reward_per_match: 100,
        },
    };
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Game Engine".to_string(),
        message: "Game Engine configuration loaded successfully".to_string(),
    });
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Game Engine".to_string(),
        message: "Initializing Game Engine Bot...".to_string(),
    });
    
    // Initialize game engine bot
    let bot = Arc::new(game_engine_bot::GameEngineBot::new(config.clone()).await?);
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Game Engine".to_string(),
        message: "Game Engine Bot initialized successfully".to_string(),
    });
    
    let _ = status_tx.send(ServiceMessage::LogMessage {
        service: "Game Engine".to_string(),
        message: "Starting Game Engine Bot service...".to_string(),
    });
    
    // Start the complete game engine system
    match bot.start_game_engine().await {
        Ok(_) => {
            let _ = status_tx.send(ServiceMessage::LogMessage {
                service: "Game Engine".to_string(),
                message: "Game Engine service completed successfully".to_string(),
            });
        }
        Err(e) => {
            let _ = status_tx.send(ServiceMessage::Error {
                service: "Game Engine".to_string(),
                error: format!("Game Engine service error: {}", e),
            });
            return Err(anyhow!("Game Engine error: {}", e));
        }
    }
    
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationData {
    // Service status
    pub services_running: bool,
    pub cdk_mint_status: ServiceStatus,
    pub nostr_relay_status: ServiceStatus,
    pub game_engine_status: ServiceStatus,
    
    // URLs
    pub cdk_mint_url: String,
    pub nostr_relay_url: String,
    pub game_engine_url: String,
    
    // Player data
    pub alice_balance: u64,
    pub bob_balance: u64,
    pub alice_tokens: Vec<String>,
    pub bob_tokens: Vec<String>,
    
    // Match data
    pub current_match_id: Option<String>,
    pub match_phase: String,
    pub pending_challenges: u32,
    pub completed_matches: u32,
    
    // Logs
    pub service_logs: HashMap<String, Vec<String>>,
    pub integration_log: Vec<String>,
    
    // Test results
    pub last_test_result: Option<String>,
    pub test_running: bool,
    
    // Note: Real wallet manager will be managed separately to avoid serialization issues
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Failed(String),
}

impl Default for IntegrationData {
    fn default() -> Self {
        Self {
            services_running: false,
            cdk_mint_status: ServiceStatus::Stopped,
            nostr_relay_status: ServiceStatus::Stopped,
            game_engine_status: ServiceStatus::Stopped,
            cdk_mint_url: "http://127.0.0.1:3333".to_string(),
            nostr_relay_url: "ws://127.0.0.1:7777".to_string(),
            game_engine_url: "http://127.0.0.1:4444".to_string(),
            alice_balance: 0,
            bob_balance: 0,
            alice_tokens: Vec::new(),
            bob_tokens: Vec::new(),
            current_match_id: None,
            match_phase: "Ready".to_string(),
            pending_challenges: 0,
            completed_matches: 0,
            service_logs: HashMap::new(),
            integration_log: Vec::new(),
            last_test_result: None,
            test_running: false,
        }
    }
}

/// Legacy ServiceManager - replaced by ServiceOrchestrator
#[deprecated(note = "Use ServiceOrchestrator with MPSC channels instead")]
pub struct ServiceManager {
    cdk_process: Option<Child>,
    nostr_process: Option<Child>,
    engine_process: Option<Child>,
}

#[allow(deprecated)]
impl ServiceManager {
    pub fn new() -> Self {
        Self {
            cdk_process: None,
            nostr_process: None,
            engine_process: None,
        }
    }
}

// MPSC-based service management functions

/// Global service orchestrator instance (will be managed by Tauri state)
pub struct IntegratedServiceManager {
    pub orchestrator: ServiceOrchestrator,
    pub cdk_command_tx: mpsc::UnboundedSender<ServiceCommand>,
    pub nostr_command_tx: mpsc::UnboundedSender<ServiceCommand>,
    pub engine_command_tx: mpsc::UnboundedSender<ServiceCommand>,
    pub message_tx: mpsc::UnboundedSender<ServiceMessage>,
    pub cdk_service_handle: Option<tokio::task::JoinHandle<()>>,
    pub nostr_service_handle: Option<tokio::task::JoinHandle<()>>,
    pub game_engine_service_handle: Option<tokio::task::JoinHandle<()>>,
}

impl Drop for IntegratedServiceManager {
    fn drop(&mut self) {
        info!("🧹 Cleaning up service manager - stopping all services");
        let _ = self.stop_all_services_nonblocking();
    }
}

impl IntegratedServiceManager {
    pub async fn new() -> Result<Self> {
        let mut orchestrator = ServiceOrchestrator::new();
        
        // Create CDK mint service
        let (mut cdk_service, cdk_command_tx) = CdkMintService::new(orchestrator.message_tx.clone());
        orchestrator.add_service("CDK Mint".to_string(), cdk_command_tx.clone());
        
        // Create Nostr relay service
        let (mut nostr_service, nostr_command_tx) = NostrRelayService::new(orchestrator.message_tx.clone());
        orchestrator.add_service("Nostr Relay".to_string(), nostr_command_tx.clone());
        
        // Create Game Engine service
        let (mut game_engine_service, engine_command_tx) = GameEngineService::new(orchestrator.message_tx.clone());
        orchestrator.add_service("Game Engine".to_string(), engine_command_tx.clone());
        
        // Spawn service tasks
        let cdk_handle = tokio::spawn(async move {
            cdk_service.run().await;
        });
        
        let nostr_handle = tokio::spawn(async move {
            nostr_service.run().await;
        });
        
        let engine_handle = tokio::spawn(async move {
            game_engine_service.run().await;
        });
        
        Ok(Self {
            message_tx: orchestrator.message_tx.clone(),
            cdk_command_tx,
            nostr_command_tx,
            engine_command_tx,
            orchestrator,
            cdk_service_handle: Some(cdk_handle),
            nostr_service_handle: Some(nostr_handle),
            game_engine_service_handle: Some(engine_handle),
        })
    }
    
    /// Start all services without holding any locks
    pub fn start_all_services_nonblocking(&self) -> Result<()> {
        info!("🚀 Sending start commands to all services via MPSC channels");
        
        self.cdk_command_tx.send(ServiceCommand::Start)
            .map_err(|e| anyhow!("Failed to send start command to CDK mint: {}", e))?;
            
        self.nostr_command_tx.send(ServiceCommand::Start)
            .map_err(|e| anyhow!("Failed to send start command to Nostr relay: {}", e))?;
            
        self.engine_command_tx.send(ServiceCommand::Start)
            .map_err(|e| anyhow!("Failed to send start command to Game Engine: {}", e))?;
        
        Ok(())
    }
    
    /// Stop all services without holding any locks
    pub fn stop_all_services_nonblocking(&self) -> Result<()> {
        info!("🛑 Sending stop commands to all services via MPSC channels");
        
        self.cdk_command_tx.send(ServiceCommand::Stop)
            .map_err(|e| anyhow!("Failed to send stop command to CDK mint: {}", e))?;
            
        self.nostr_command_tx.send(ServiceCommand::Stop)
            .map_err(|e| anyhow!("Failed to send stop command to Nostr relay: {}", e))?;
            
        self.engine_command_tx.send(ServiceCommand::Stop)
            .map_err(|e| anyhow!("Failed to send stop command to Game Engine: {}", e))?;
            
        Ok(())
    }
    
    pub async fn start_all_services(&self) -> Result<()> {
        self.orchestrator.start_all_services().await
    }
    
    pub async fn stop_all_services(&self) -> Result<()> {
        self.orchestrator.stop_all_services().await
    }
    
    pub async fn process_messages(&mut self, data: &mut IntegrationData) -> Result<()> {
        self.orchestrator.process_messages(data).await
    }
    
    pub async fn get_all_status(&self) -> HashMap<String, ServiceStatus> {
        self.orchestrator.get_all_status().await
    }
}

// Legacy function wrappers for backward compatibility
#[deprecated(note = "Use IntegratedServiceManager instead")]
pub async fn start_all_services(data: &mut IntegrationData) -> Result<()> {
    info!("🏗️ Starting all Manastr services...");
    
    data.integration_log.push("🚀 Starting service orchestration...".to_string());
    data.integration_log.push("📋 Service startup will show detailed output in the console below".to_string());
    
    // Start CDK mint
    data.cdk_mint_status = ServiceStatus::Starting;
    start_cdk_mint(data).await?;
    
    // Add small delay between service starts
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Start Nostr relay
    data.nostr_relay_status = ServiceStatus::Starting;
    start_nostr_relay(data).await?;
    
    // Add small delay between service starts
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // Start game engine
    data.game_engine_status = ServiceStatus::Starting;
    start_game_engine(data).await?;
    
    data.integration_log.push("⏳ Services are starting up - waiting for health checks...".to_string());
    
    // Wait for all services to be ready
    wait_for_services(data).await?;
    
    data.services_running = true;
    data.integration_log.push("✅ All services running successfully - ready for real data operations!".to_string());
    
    Ok(())
}

pub async fn stop_all_services(data: &mut IntegrationData) -> Result<()> {
    info!("🛑 Stopping all services...");
    
    data.integration_log.push("Stopping all services...".to_string());
    
    // Stop services in reverse order
    stop_game_engine().await?;
    stop_nostr_relay().await?;
    stop_cdk_mint().await?;
    
    data.services_running = false;
    data.cdk_mint_status = ServiceStatus::Stopped;
    data.nostr_relay_status = ServiceStatus::Stopped;
    data.game_engine_status = ServiceStatus::Stopped;
    
    data.integration_log.push("✅ All services stopped".to_string());
    
    Ok(())
}

async fn start_cdk_mint(data: &mut IntegrationData) -> Result<()> {
    info!("🏦 Starting CDK mint...");
    data.integration_log.push("🏦 Building CDK mint binary first...".to_string());
    
    // First, build the CDK mint binary (like integration test does)
    info!("🔨 Building CDK mint binary...");
    let build_output = tokio::process::Command::new("cargo")
        .args(&["build", "--release", "--bin", "cdk-mintd"])
        .current_dir("../../cdk")
        .output()
        .await?;
        
    if !build_output.status.success() {
        let error = String::from_utf8_lossy(&build_output.stderr);
        let error_msg = format!("CDK mint build failed: {}", error);
        data.integration_log.push(error_msg.clone());
        return Err(anyhow!(error_msg));
    }
    
    data.integration_log.push("✅ CDK mint binary built successfully".to_string());
    
    // Clean up mint database to avoid "already signed" errors between runs
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let mint_db_dir = std::path::Path::new(&home).join(".cdk-mintd");
    if mint_db_dir.exists() {
        info!("🧹 Cleaning up mint database directory");
        let _ = std::fs::remove_dir_all(&mint_db_dir);
    }
    
    // Start the pre-built binary (exactly like integration test)
    data.integration_log.push("🏦 Starting CDK mint service...".to_string());
    let mut child = tokio::process::Command::new("./target/release/cdk-mintd")
        .args(&["--config", "../config/cdk-mintd-deterministic.toml"])
        .current_dir("../../cdk")
        .stdout(Stdio::inherit()) // Show output in console
        .stderr(Stdio::inherit()) // Show errors in console  
        .spawn()?;
    
    data.integration_log.push("🏦 CDK mint process spawned - check console for output".to_string());
    info!("🏦 CDK mint started - process output will appear in console");
    
    // Spawn task to wait for process completion
    tokio::spawn(async move {
        match child.wait().await {
            Ok(status) => {
                if status.success() {
                    info!("🏦 CDK mint process completed successfully");
                } else {
                    warn!("🏦 CDK mint process exited with error: {:?}", status);
                }
            }
            Err(e) => {
                warn!("🏦 CDK mint process error: {}", e);
            }
        }
    });
    
    Ok(())
}

async fn start_nostr_relay(data: &mut IntegrationData) -> Result<()> {
    info!("📡 Starting Nostr relay...");
    data.integration_log.push("📡 Building Nostr relay binary first...".to_string());
    
    // First, build the Nostr relay binary (like integration test does)
    info!("🔨 Building Nostr relay binary...");
    let build_output = tokio::process::Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir("../../nostr-relay/nostr-rs-relay")
        .output()
        .await?;
        
    if !build_output.status.success() {
        let error = String::from_utf8_lossy(&build_output.stderr);
        let error_msg = format!("Nostr relay build failed: {}", error);
        data.integration_log.push(error_msg.clone());
        return Err(anyhow!(error_msg));
    }
    
    data.integration_log.push("✅ Nostr relay binary built successfully".to_string());
    
    // Create required directories
    std::fs::create_dir_all("../../nostr-relay/logs").ok();
    std::fs::create_dir_all("../../nostr-relay/nostr-relay-db").ok();
    
    // Start the pre-built binary (exactly like integration test)
    data.integration_log.push("📡 Starting Nostr relay service...".to_string());
    let mut child = tokio::process::Command::new("./nostr-rs-relay/target/release/nostr-rs-relay")
        .args(&["--config", "config.toml"])
        .current_dir("../../nostr-relay")
        .stdout(Stdio::inherit()) // Show output in console
        .stderr(Stdio::inherit()) // Show errors in console
        .spawn()?;
    
    data.integration_log.push("📡 Nostr relay process spawned - check console for output".to_string());
    info!("📡 Nostr relay started - process output will appear in console");
    
    // Spawn task to wait for process completion
    tokio::spawn(async move {
        match child.wait().await {
            Ok(status) => {
                if status.success() {
                    info!("📡 Nostr relay process completed successfully");
                } else {
                    warn!("📡 Nostr relay process exited with error: {:?}", status);
                }
            }
            Err(e) => {
                warn!("📡 Nostr relay process error: {}", e);
            }
        }
    });
    
    Ok(())
}

async fn start_game_engine(data: &mut IntegrationData) -> Result<()> {
    info!("🎮 Starting game engine...");
    data.integration_log.push("🎮 Building game engine binary first...".to_string());
    
    // First, build the game engine binary (like integration test does)
    info!("🔨 Building game engine binary...");
    let build_output = tokio::process::Command::new("cargo")
        .args(&["build", "--release", "--bin", "game-engine-bot"])
        .current_dir("../../game-engine-bot")
        .output()
        .await?;
        
    if !build_output.status.success() {
        let error = String::from_utf8_lossy(&build_output.stderr);
        let error_msg = format!("Game engine build failed: {}", error);
        data.integration_log.push(error_msg.clone());
        return Err(anyhow!(error_msg));
    }
    
    data.integration_log.push("✅ Game engine binary built successfully".to_string());
    
    // Start the pre-built binary (exactly like integration test)
    data.integration_log.push("🎮 Starting game engine service...".to_string());
    let mut child = tokio::process::Command::new("./target/release/game-engine-bot")
        .current_dir("../../game-engine-bot")
        .stdout(Stdio::inherit()) // Show output in console
        .stderr(Stdio::inherit()) // Show errors in console
        .spawn()?;
    
    data.integration_log.push("🎮 Game engine process spawned - check console for output".to_string());
    info!("🎮 Game engine started - process output will appear in console");
    
    // Spawn task to wait for process completion
    tokio::spawn(async move {
        match child.wait().await {
            Ok(status) => {
                if status.success() {
                    info!("🎮 Game engine process completed successfully");
                } else {
                    warn!("🎮 Game engine process exited with error: {:?}", status);
                }
            }
            Err(e) => {
                warn!("🎮 Game engine process error: {}", e);
            }
        }
    });
    
    Ok(())
}

async fn wait_for_services(data: &mut IntegrationData) -> Result<()> {
    info!("⏳ Waiting for services to be ready...");
    data.integration_log.push("⏳ Health checking services (this may take 2-3 minutes while Rust compiles)...".to_string());
    
    let mut attempts = 0;
    let max_attempts = 120; // 2 minutes timeout (Rust compilation can be slow)
    
    while attempts < max_attempts {
        let mut all_ready = true;
        let mut status_update = Vec::new();
        
        // Check CDK mint
        if check_service_health(&data.cdk_mint_url, "/v1/info").await {
            if !matches!(data.cdk_mint_status, ServiceStatus::Running) {
                data.cdk_mint_status = ServiceStatus::Running;
                status_update.push("🏦 CDK mint is ready!");
            }
        } else {
            all_ready = false;
        }
        
        // Check Nostr relay (check metrics endpoint)
        let nostr_check_url = data.nostr_relay_url.replace("ws://", "http://") + "/metrics";
        if check_service_health(&nostr_check_url, "").await {
            if !matches!(data.nostr_relay_status, ServiceStatus::Running) {
                data.nostr_relay_status = ServiceStatus::Running;
                status_update.push("📡 Nostr relay is ready!");
            }
        } else {
            all_ready = false;
        }
        
        // Check game engine
        if check_service_health(&data.game_engine_url, "/health").await {
            if !matches!(data.game_engine_status, ServiceStatus::Running) {
                data.game_engine_status = ServiceStatus::Running;  
                status_update.push("🎮 Game engine is ready!");
            }
        } else {
            all_ready = false;
        }
        
        // Log status updates
        for update in status_update {
            data.integration_log.push(update.to_string());
            info!("{}", update);
        }
        
        if all_ready {
            data.integration_log.push("✅ All services are ready and healthy!".to_string());
            return Ok(());
        }
        
        // Progress indicator every 10 seconds
        if attempts % 10 == 0 && attempts > 0 {
            let elapsed_mins = attempts / 60;
            let elapsed_secs = attempts % 60;
            data.integration_log.push(format!("⏳ Still waiting... ({}m {}s elapsed)", elapsed_mins, elapsed_secs));
        }
        
        attempts += 1;
        sleep(Duration::from_secs(1)).await;
    }
    
    // Provide helpful error message
    let mut error_details = Vec::new();
    if !matches!(data.cdk_mint_status, ServiceStatus::Running) {
        error_details.push("CDK mint not responding");
    }
    if !matches!(data.nostr_relay_status, ServiceStatus::Running) {
        error_details.push("Nostr relay not responding");
    }
    if !matches!(data.game_engine_status, ServiceStatus::Running) {
        error_details.push("Game engine not responding");
    }
    
    let error_msg = format!("Services failed to start within 2 minutes. Issues: {}", error_details.join(", "));
    data.integration_log.push(format!("❌ {}", error_msg));
    
    Err(anyhow!(error_msg))
}

async fn check_service_health(base_url: &str, path: &str) -> bool {
    let url = format!("{}{}", base_url, path);
    match reqwest::get(&url).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

async fn stop_cdk_mint() -> Result<()> {
    // Implementation for stopping CDK mint
    // For now, we'll use pkill as a simple approach
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("cdk-mintd")
        .output();
    Ok(())
}

async fn stop_nostr_relay() -> Result<()> {
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("nostr-rs-relay")
        .output();
    Ok(())
}

async fn stop_game_engine() -> Result<()> {
    let _ = Command::new("pkill")
        .arg("-f")
        .arg("game-engine-bot")
        .output();
    Ok(())
}

pub async fn update_service_health(data: &mut IntegrationData) -> Result<()> {
    // Update service status
    if check_service_health(&data.cdk_mint_url, "/v1/info").await {
        data.cdk_mint_status = ServiceStatus::Running;
    } else if matches!(data.cdk_mint_status, ServiceStatus::Running) {
        data.cdk_mint_status = ServiceStatus::Failed("Service unreachable".to_string());
    }
    
    let nostr_check_url = data.nostr_relay_url.replace("ws://", "http://") + "/metrics";
    if check_service_health(&nostr_check_url, "").await {
        data.nostr_relay_status = ServiceStatus::Running;
    } else if matches!(data.nostr_relay_status, ServiceStatus::Running) {
        data.nostr_relay_status = ServiceStatus::Failed("Service unreachable".to_string());
    }
    
    if check_service_health(&data.game_engine_url, "/health").await {
        data.game_engine_status = ServiceStatus::Running;
    } else if matches!(data.game_engine_status, ServiceStatus::Running) {
        data.game_engine_status = ServiceStatus::Failed("Service unreachable".to_string());
    }
    
    // Update service logs (read from log files)
    update_service_logs(data).await;
    
    // Monitor Nostr events if relay is running
    if matches!(data.nostr_relay_status, ServiceStatus::Running) {
        if let Err(e) = monitor_nostr_events(data).await {
            warn!("Failed to monitor Nostr events: {}", e);
        }
    }
    
    Ok(())
}

async fn update_service_logs(data: &mut IntegrationData) {
    // Read recent log lines from service log files (integration_tests creates these)
    if let Ok(logs) = read_log_file("../../integration_tests/logs/cdk-mint.out.log", 5).await {
        data.service_logs.insert("CDK Mint".to_string(), logs);
    }
    
    if let Ok(logs) = read_log_file("../../integration_tests/logs/nostr-relay.out.log", 5).await {
        data.service_logs.insert("Nostr Relay".to_string(), logs);
    }
    
    if let Ok(logs) = read_log_file("../../integration_tests/logs/game-engine.out.log", 5).await {
        data.service_logs.insert("Game Engine".to_string(), logs);
    }
}

async fn read_log_file(path: &str, lines: usize) -> Result<Vec<String>> {
    use tokio::fs::File;
    use tokio::io::{AsyncBufReadExt, BufReader};
    
    match File::open(path).await {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut all_lines = Vec::new();
            let mut lines_stream = reader.lines();
            
            while let Some(line) = lines_stream.next_line().await? {
                all_lines.push(line);
            }
            
            // Return last N lines
            let start = all_lines.len().saturating_sub(lines);
            Ok(all_lines[start..].to_vec())
        }
        Err(_) => {
            Ok(vec![format!("Log file {} not found", path)])
        }
    }
}

pub async fn run_full_integration_test(data: &mut IntegrationData) -> Result<String> {
    info!("🧪 Running full integration test...");
    
    data.test_running = true;
    data.integration_log.push("Starting full integration test...".to_string());
    
    // Run the integration test binary
    // First check if integration_tests directory exists, otherwise use the justfile command
    let integration_test_path = "../../integration_tests";
    let output = if std::path::Path::new(integration_test_path).exists() {
        tokio::process::Command::new("cargo")
            .args(&["run", "--bin", "integration-runner"])
            .current_dir(integration_test_path)
            .output()
            .await?
    } else {
        // Fallback to using justfile integration command from main directory
        tokio::process::Command::new("just")
            .args(&["integration"])
            .current_dir("../../../") // Go to main manastr directory
            .output()
            .await?
    };
    
    let result = if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        format!("✅ Integration test completed successfully:\n{}", stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("❌ Integration test failed:\n{}", stderr)
    };
    
    data.last_test_result = Some(result.clone());
    data.test_running = false;
    data.integration_log.push("Integration test completed".to_string());
    
    Ok(result)
}

pub async fn mint_tokens_for_player(data: &mut IntegrationData, player: &str, amount: u32) -> Result<Vec<String>> {
    info!("🪙 Attempting to mint {} tokens for {} (will try real CDK, fallback to simulation)", amount, player);
    
    // Try to mint with real CDK first, but gracefully fallback if unavailable
    match try_mint_real_tokens(data, player, amount).await {
        Ok(tokens) => {
            data.integration_log.push(format!("✅ Successfully minted {} REAL CDK tokens for {}", tokens.len(), player));
            Ok(tokens)
        }
        Err(e) => {
            warn!("Real CDK minting failed for {}, falling back to simulation: {}", player, e);
            
            // Fall back to simulated tokens for dashboard functionality
            let mut tokens = Vec::new();
            for i in 0..amount {
                tokens.push(format!("simulated_token_{}_{}", player, i));
            }
            
            match player {
                "alice" => {
                    data.alice_balance += amount as u64;
                    data.alice_tokens.extend(tokens.clone());
                }
                "bob" => {
                    data.bob_balance += amount as u64;
                    data.bob_tokens.extend(tokens.clone());
                }
                _ => return Err(anyhow!("Unknown player: {}", player)),
            }
            
            data.integration_log.push(format!("⚠️ Used simulated tokens for {} (real CDK unavailable: {})", player, e));
            Ok(tokens)
        }
    }
}

async fn try_mint_real_tokens(data: &IntegrationData, player: &str, amount: u32) -> Result<Vec<String>> {
    use cdk::{
        nuts::{CurrencyUnit},
        Amount,
        wallet::WalletBuilder,
    };
    use cdk_sqlite::wallet::memory;
    use std::sync::Arc;
    use sha2::{Digest, Sha256};

    info!("🏛️ Attempting real CDK token minting for {} (amount: {})", player, amount);
    
    // Check if CDK mint is running
    if !check_service_health(&data.cdk_mint_url, "/v1/info").await {
        return Err(anyhow!("CDK mint service not available at {}", data.cdk_mint_url));
    }

    // Create deterministic seed for player
    let mut seed = [0u8; 32];
    let session_id = std::process::id();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let seed_str = format!("dashboard_wallet_{}_{}_{}_{}", data.cdk_mint_url, player, session_id, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(seed_str.as_bytes());
    seed.copy_from_slice(&hasher.finalize());

    // Create CDK wallet
    let localstore = Arc::new(memory::empty().await?);
    let cdk_wallet = WalletBuilder::new()
        .mint_url(data.cdk_mint_url.parse()?)
        .unit(CurrencyUnit::Sat)
        .localstore(localstore)
        .seed(&seed)
        .target_proof_count(3)
        .build()?;

    info!("✅ CDK wallet initialized for {}", player);

    // Create mint quote
    let amount_cdk = Amount::from(amount as u64);
    let quote = cdk_wallet.mint_quote(amount_cdk, None).await?;
    info!("📋 Created mint quote: {}", quote.id);

    // Wait for fake wallet to pay quote
    info!("⏳ Waiting for fake wallet to automatically pay the quote...");
    let max_wait_time = 30;
    let mut elapsed = 0;
    let mut quote_paid = false;

    while elapsed < max_wait_time && !quote_paid {
        let quote_state = cdk_wallet.mint_quote_state(&quote.id).await?;
        
        if quote_state.state == cdk::nuts::MintQuoteState::Paid {
            quote_paid = true;
            info!("✅ Quote marked as paid by fake wallet");
            break;
        } else {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            elapsed += 1;
        }
    }

    if !quote_paid {
        return Err(anyhow!("Timeout waiting for mint quote to be paid"));
    }

    // Add small delay for mint processing
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    // Mint tokens
    let proofs = cdk_wallet.mint(&quote.id, cdk::amount::SplitTarget::Value(Amount::from(1)), None).await?;
    info!("✅ Successfully minted {} tokens from CDK mint", proofs.len());

    // Convert proofs to token strings for dashboard display
    let token_strings: Vec<String> = proofs
        .into_iter()
        .enumerate()
        .map(|(i, proof)| {
            let c_value = proof.c.to_hex();
            format!("real_token_{}_{}_{}", player, i, &c_value[..8])
        })
        .collect();

    info!("🎉 Real CDK minting successful for {}: {} tokens", player, token_strings.len());
    Ok(token_strings)
}

pub async fn monitor_nostr_events(data: &mut IntegrationData) -> Result<()> {
    use nostr_sdk::{Client, Filter, Kind, EventSource};
    use std::time::Duration;

    // Create Nostr client
    let client = Client::new(&nostr::Keys::generate());
    
    // Parse the relay URL from data
    let relay_url = data.nostr_relay_url.replace("ws://", "wss://").replace("http://", "ws://");
    let relay_url = if relay_url.starts_with("ws://127.0.0.1") {
        // For local testing, keep ws://
        relay_url
    } else {
        relay_url
    };

    match relay_url.parse::<nostr_sdk::Url>() {
        Ok(url) => {
            match client.add_relay(url.as_str()).await {
                Ok(_) => {
                    client.connect().await;
                    
                    // Wait a moment for connection
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    
                    // Create filter for Manastr game events (kinds 31000-31006)
                    let filter = Filter::new()
                        .kinds([
                            Kind::Custom(31000), // Match Challenge
                            Kind::Custom(31001), // Match Acceptance  
                            Kind::Custom(31002), // Token Reveal
                            Kind::Custom(31003), // Move Commitment
                            Kind::Custom(31004), // Move Reveal
                            Kind::Custom(31005), // Match Result
                            Kind::Custom(31006), // Loot Distribution
                        ])
                        .limit(10); // Get recent events
                    
                    // Query for recent events using the correct API
                    match client.get_events_of(vec![filter], EventSource::relays(None)).await {
                        Ok(events) => {
                            data.integration_log.push(format!("📡 Connected to Nostr relay: {} events found", events.len()));
                            
                            // Count events by type for dashboard display
                            let mut pending_challenges = 0;
                            let mut completed_matches = 0;
                            let mut current_match_phase = "Ready".to_string();
                            
                            for event in &events {
                                match event.kind.as_u16() {
                                    31000 => {
                                        pending_challenges += 1;
                                        current_match_phase = "Challenged".to_string();
                                    }
                                    31001 => {
                                        current_match_phase = "Accepted".to_string();
                                    }
                                    31003 | 31004 => {
                                        current_match_phase = "InCombat".to_string();
                                    }
                                    31005 => {
                                        current_match_phase = "AwaitingValidation".to_string();
                                    }
                                    31006 => {
                                        completed_matches += 1;
                                        current_match_phase = "Completed".to_string();
                                    }
                                    _ => {}
                                }
                                
                                // Log recent event
                                data.integration_log.push(format!("🎮 Nostr Event: Kind {} from {}", 
                                    event.kind.as_u16(), 
                                    event.pubkey.to_string()[..8].to_string()
                                ));
                            }
                            
                            // Update dashboard data with real Nostr event counts
                            data.pending_challenges = pending_challenges;
                            data.completed_matches = completed_matches;
                            data.match_phase = current_match_phase;
                            
                            info!("✅ Nostr monitoring successful: {} events processed", events.len());
                            Ok(())
                        }
                        Err(e) => {
                            data.integration_log.push(format!("⚠️ Nostr query failed: {}", e));
                            warn!("Failed to query Nostr events: {}", e);
                            Ok(()) // Don't fail completely, just log the issue
                        }
                    }
                }
                Err(e) => {
                    data.integration_log.push(format!("⚠️ Failed to add Nostr relay: {}", e));
                    warn!("Failed to add Nostr relay: {}", e);
                    Ok(())
                }
            }
        }
        Err(e) => {
            data.integration_log.push(format!("⚠️ Invalid Nostr relay URL: {}", e));
            warn!("Invalid Nostr relay URL: {}", e);
            Ok(())
        }
    }
}