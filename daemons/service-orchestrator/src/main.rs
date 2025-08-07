use anyhow::{Context, Result};
use axum::Router;
use clap::Parser;
use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    net::TcpListener,
    signal,
    sync::Mutex,
    time::sleep,
};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{error, info, warn};

#[derive(Parser)]
#[command(name = "manastr-serve")]
#[command(about = "🚀 Manastr Service Orchestrator - Revolutionary Gaming System")]
struct Args {
    /// Port to serve the web client on
    #[arg(short, long, default_value = "8080")]
    port: u16,

    /// Skip building (useful for development)
    #[arg(long)]
    skip_build: bool,

    /// Run backend services only (no web server)
    #[arg(long)]
    backend_only: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug)]
struct ServiceConfig {
    name: String,
    command: String,
    args: Vec<String>,
    working_dir: PathBuf,
    health_check_url: Option<String>,
    health_check_timeout: Duration,
}

struct ServiceManager {
    services: HashMap<String, Child>,
    running: Arc<AtomicBool>,
}

impl ServiceManager {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    async fn start_service(&mut self, config: &ServiceConfig) -> Result<()> {
        info!("🚀 Starting service: {}", config.name);
        info!("   Command: {}", config.command);
        info!("   Args: {:?}", config.args);
        info!("   Working dir: {:?}", config.working_dir);
        
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args)
            .current_dir(&config.working_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = cmd
            .spawn()
            .with_context(|| format!("Failed to start service: {} (command: {})", config.name, config.command))?;

        self.services.insert(config.name.clone(), child);
        
        // Wait for service to be ready
        if let Some(health_url) = &config.health_check_url {
            self.wait_for_health_check(health_url, config.health_check_timeout)
                .await?;
        } else {
            // Just wait a bit for services without health checks
            sleep(Duration::from_secs(2)).await;
        }

        info!("✅ Service ready: {}", config.name);
        Ok(())
    }

    async fn wait_for_health_check(&self, url: &str, timeout_duration: Duration) -> Result<()> {
        let client = reqwest::Client::new();
        let start_time = std::time::Instant::now();

        while start_time.elapsed() < timeout_duration && self.running.load(Ordering::Relaxed) {
            match client.get(url).send().await {
                Ok(response) if response.status().is_success() => {
                    return Ok(());
                }
                Ok(_) => {
                    sleep(Duration::from_millis(500)).await;
                }
                Err(_) => {
                    sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(anyhow::anyhow!("Health check failed for URL: {}", url))
    }

    async fn stop_all_services(&mut self) -> Result<()> {
        info!("🛑 Stopping all services...");
        self.running.store(false, Ordering::Relaxed);

        for (name, mut child) in self.services.drain() {
            info!("🛑 Stopping service: {}", name);
            
            // Try graceful shutdown first
            if let Err(e) = child.kill() {
                warn!("Failed to kill service {}: {}", name, e);
            }
            
            // Wait for process to exit
            match child.wait() {
                Ok(status) => info!("✅ Service {} exited with status: {}", name, status),
                Err(e) => warn!("Error waiting for service {} to exit: {}", name, e),
            }
        }

        info!("✅ All services stopped");
        Ok(())
    }
}

struct ManastrOrchestrator {
    project_root: PathBuf,
    service_manager: Arc<Mutex<ServiceManager>>,
}

impl ManastrOrchestrator {
    fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()
            .context("Failed to get current directory")?;
            
        // Check if we're already in the project root (contains daemons/ directory)
        let project_root = if current_dir.join("daemons").exists() {
            current_dir
        } else {
            // We're running from inside service-orchestrator, go up two levels
            current_dir
                .parent()
                .context("No parent directory")?
                .parent()
                .context("Invalid project structure")?
                .to_path_buf()
        };

        Ok(Self {
            project_root,
            service_manager: Arc::new(Mutex::new(ServiceManager::new())),
        })
    }

    async fn build_all(&self) -> Result<()> {
        info!("🏗️ Building all Manastr components...");
        
        // Build Rust components
        info!("⚙️ Building Rust workspace...");
        let rust_build = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(&self.project_root)
            .status()
            .context("Failed to build Rust workspace")?;

        if !rust_build.success() {
            return Err(anyhow::anyhow!("Rust workspace build failed"));
        }

        // Build CDK separately
        info!("💰 Building CDK mint...");
        let cdk_build = Command::new("cargo")
            .args(&["build", "--release", "--bin", "cdk-mintd"])
            .current_dir(&self.project_root.join("daemons/cdk"))
            .status()
            .context("Failed to build CDK mint")?;

        if !cdk_build.success() {
            return Err(anyhow::anyhow!("CDK mint build failed"));
        }

        // Build Nostr relay
        info!("📡 Building Nostr relay...");
        let relay_build = Command::new("cargo")
            .args(&["build", "--release"])
            .current_dir(&self.project_root.join("daemons/nostr-relay/nostr-rs-relay"))
            .status()
            .context("Failed to build Nostr relay")?;

        if !relay_build.success() {
            return Err(anyhow::anyhow!("Nostr relay build failed"));
        }

        // Build WASM
        info!("🌐 Building WASM components...");
        let wasm_dir = self.project_root.join("daemons/shared-game-logic");
        let wasm_build = Command::new("wasm-pack")
            .args(&["build", "--target", "web", "--out-dir", "pkg"])
            .current_dir(&wasm_dir)
            .status()
            .context("Failed to build WASM components")?;

        if !wasm_build.success() {
            return Err(anyhow::anyhow!("WASM build failed"));
        }

        // Build web client
        info!("🚀 Building quantum web client...");
        let web_dir = self.project_root.join("daemons/manastr-web");
        
        // Check if node_modules exists, install if not
        if !web_dir.join("node_modules").exists() {
            info!("📦 Installing web dependencies...");
            let npm_install = Command::new("bash")
                .args(&["-c", "npm install"])
                .current_dir(&web_dir)
                .status()
                .context("Failed to install npm dependencies")?;

            if !npm_install.success() {
                return Err(anyhow::anyhow!("npm install failed"));
            }
        }

        let web_build = Command::new("bash")
            .args(&["-c", "npm run build"])
            .current_dir(&web_dir)
            .status()
            .context("Failed to build web client")?;

        if !web_build.success() {
            return Err(anyhow::anyhow!("Web client build failed"));
        }

        info!("✅ All components built successfully!");
        Ok(())
    }

    fn get_service_configs(&self) -> Vec<ServiceConfig> {
        vec![
            // Nostr Relay
            ServiceConfig {
                name: "nostr-relay".to_string(),
                command: self.project_root
                    .join("daemons/nostr-relay/nostr-rs-relay/target/release/nostr-rs-relay")
                    .to_string_lossy()
                    .to_string(),
                args: vec![
                    "--config".to_string(),
                    self.project_root
                        .join("daemons/nostr-relay/config.toml")
                        .to_string_lossy()
                        .to_string(),
                ],
                working_dir: self.project_root.join("daemons/nostr-relay"),
                health_check_url: None, // Nostr relay doesn't have HTTP endpoint
                health_check_timeout: Duration::from_secs(5),
            },
            // CDK Mint
            ServiceConfig {
                name: "cdk-mint".to_string(),
                command: self.project_root
                    .join("daemons/cdk/target/release/cdk-mintd")
                    .to_string_lossy()
                    .to_string(),
                args: vec![
                    "--config".to_string(),
                    self.project_root
                        .join("daemons/config/cdk-mintd-deterministic.toml")
                        .to_string_lossy()
                        .to_string(),
                ],
                working_dir: self.project_root.join("daemons/cdk"),
                health_check_url: Some("http://localhost:3333/v1/info".to_string()),
                health_check_timeout: Duration::from_secs(30),
            },
            // Game Engine (No HTTP endpoints - Pure Nostr communication)
            ServiceConfig {
                name: "game-engine".to_string(),
                command: self.project_root
                    .join("target/release/game-engine-bot")
                    .to_string_lossy()
                    .to_string(),
                args: vec![
                    "--config".to_string(),
                    self.project_root
                        .join("daemons/game-engine-bot/game-engine.toml")
                        .to_string_lossy()
                        .to_string(),
                ],
                working_dir: self.project_root.join("daemons/game-engine-bot"),
                health_check_url: None, // No HTTP endpoints - communicates via Nostr only
                health_check_timeout: Duration::from_secs(5),
            },
        ]
    }

    async fn start_all_services(&self) -> Result<()> {
        let configs = self.get_service_configs();
        let mut manager = self.service_manager.lock().await;

        for config in &configs {
            manager.start_service(config).await
                .with_context(|| format!("Failed to start service: {}", config.name))?;
        }

        info!("🚀 All backend services are running!");
        Ok(())
    }

    async fn serve_web(&self, port: u16) -> Result<()> {
        let web_dist_path = self.project_root.join("daemons/manastr-web/dist");
        
        if !web_dist_path.exists() {
            return Err(anyhow::anyhow!(
                "Web client not built. Run without --skip-build or build manually with 'just build-web'"
            ));
        }

        info!("🌐 Starting quantum web server on port {}...", port);
        
        // Create the web service
        let serve_dir = ServeDir::new(&web_dist_path);

        let app = Router::new()
            .nest_service("/", serve_dir)
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http());

        let listener = TcpListener::bind(&format!("0.0.0.0:{}", port)).await
            .context("Failed to bind to address")?;

        info!("✅ Quantum web server ready at http://localhost:{}", port);
        info!("🚀 MANASTR SYSTEM FULLY OPERATIONAL!");
        info!("");
        info!("🌍 Web Interface: http://localhost:{}", port);
        info!("📡 Nostr Relay: ws://localhost:7777");
        info!("💰 Cashu Mint: http://localhost:3333");
        info!("🎮 Game Engine: http://localhost:4444");
        info!("");
        info!("Press Ctrl+C to shutdown all services");

        axum::serve(listener, app).await
            .context("Web server error")?;

        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        info!("🛑 Shutting down Manastr system...");
        let mut manager = self.service_manager.lock().await;
        manager.stop_all_services().await?;
        info!("👋 Manastr system shutdown complete");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("manastr_serve={},service_orchestrator={}", log_level, log_level))
        .init();

    info!("🏛️ MANASTR SERVICE ORCHESTRATOR");
    info!("===============================");
    info!("Revolutionary Zero-Coordination Gaming System");
    info!("");

    let orchestrator = ManastrOrchestrator::new()
        .context("Failed to initialize orchestrator")?;

    // Build everything (unless skipped)
    if !args.skip_build {
        orchestrator.build_all().await
            .context("Build failed")?;
    } else {
        info!("⏭️ Skipping build (--skip-build specified)");
    }

    // Start all backend services
    orchestrator.start_all_services().await
        .context("Failed to start services")?;

    // Set up signal handling for graceful shutdown
    let orchestrator_clone = Arc::new(orchestrator);
    let shutdown_orchestrator = orchestrator_clone.clone();
    
    let shutdown_signal = async {
        let _ = signal::ctrl_c().await;
        info!("🛑 Received shutdown signal");
    };

    if args.backend_only {
        // Backend services only - just wait for shutdown signal
        info!("🚀 Backend services operational! All services ready for connections:");
        info!("📡 Nostr Relay: ws://localhost:7777");
        info!("💰 Cashu Mint: http://localhost:3333");
        info!("🎮 Game Engine: Nostr communication only");
        info!("");
        info!("Press Ctrl+C to shutdown all services");
        
        // Just wait for shutdown signal
        shutdown_signal.await;
    } else {
        // Start web server and wait for shutdown signal
        let web_server = orchestrator_clone.serve_web(args.port);
        
        tokio::select! {
            result = web_server => {
                if let Err(e) = result {
                    error!("Web server error: {}", e);
                }
            }
            _ = shutdown_signal => {
                info!("🛑 Shutdown initiated");
            }
        }
    }

    // Graceful shutdown
    shutdown_orchestrator.shutdown().await
        .context("Shutdown failed")?;

    Ok(())
}