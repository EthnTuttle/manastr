use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State, Manager};
use tokio::time::Duration;
use tracing::{info, warn, error};

mod service_manager;
pub mod service_orchestrator;
pub mod integrated_nostr_relay;

use service_manager::{IntegrationData, IntegratedServiceManager};

// Global state for the dashboard
pub type DashboardState = Arc<Mutex<IntegrationData>>;
pub type ServiceManagerState = Arc<Mutex<Option<IntegratedServiceManager>>>;

#[tauri::command]
async fn start_integration_services(
    data_state: State<'_, DashboardState>,
    service_state: State<'_, ServiceManagerState>,
    app_handle: tauri::AppHandle
) -> Result<String, String> {
    info!("🚀 Starting Manastr integration services via MPSC channels...");
    
    // Initialize service manager if not already done (lazy initialization)
    let needs_init = {
        let service_manager_opt = service_state.lock().map_err(|e| format!("Service manager lock error: {}", e))?;
        service_manager_opt.is_none()
    };
    
    // Initialize if needed (outside of lock)
    if needs_init {
        info!("🔧 Initializing MPSC service architecture...");
        match IntegratedServiceManager::new().await {
            Ok(service_manager) => {
                let mut service_manager_opt = service_state.lock().map_err(|e| format!("Service manager lock error: {}", e))?;
                *service_manager_opt = Some(service_manager);
                info!("✅ MPSC service architecture initialized successfully");
            }
            Err(e) => {
                return Err(format!("Failed to initialize service manager: {}", e));
            }
        }
    }
    
    // Now start the services
    let start_result = {
        let service_manager_opt = service_state.lock().map_err(|e| format!("Service manager lock error: {}", e))?;
        if let Some(ref service_manager) = *service_manager_opt {
            service_manager.start_all_services_nonblocking()
        } else {
            Err(anyhow::anyhow!("Service manager not initialized"))
        }
    };
    
    match start_result {
        Ok(_) => {
            // Update dashboard data to reflect services are running
            if let Ok(mut data) = data_state.lock() {
                data.services_running = true;
                data.integration_log.push("🚀 All services starting via MPSC channels...".to_string());
            }
            
            // Start monitoring task if not already started
            if !MONITORING_STARTED.load(std::sync::atomic::Ordering::Relaxed) {
                MONITORING_STARTED.store(true, std::sync::atomic::Ordering::Relaxed);
                tokio::spawn(start_monitoring_task(app_handle, data_state.inner().clone(), service_state.inner().clone()));
                info!("🔄 MPSC message processing task started");
            }
            
            info!("✅ Start commands sent to all services via MPSC channels");
            Ok("Start commands sent to all services via MPSC channels".to_string())
        }
        Err(e) => {
            error!("❌ Failed to start services: {}", e);
            Err(format!("Failed to start services: {}", e))
        }
    }
}

#[tauri::command]
async fn stop_integration_services(
    data_state: State<'_, DashboardState>,
    service_state: State<'_, ServiceManagerState>
) -> Result<String, String> {
    info!("🛑 Stopping Manastr integration services...");
    
    // Send stop commands without holding the lock across await
    let stop_result = {
        let service_manager_opt = service_state.lock().map_err(|e| format!("Service manager lock error: {}", e))?;
        
        if let Some(ref service_manager) = *service_manager_opt {
            service_manager.stop_all_services_nonblocking()
        } else {
            Err(anyhow::anyhow!("Service manager not initialized"))
        }
    };
    
    match stop_result {
        Ok(_) => {
            // Update dashboard data to reflect services are stopped
            if let Ok(mut data) = data_state.lock() {
                data.services_running = false;
                data.integration_log.push("🛑 Stop commands sent to all services via MPSC channels".to_string());
            }
            
            info!("✅ Stop commands sent to all services via MPSC channels");
            Ok("Stop commands sent to all services via MPSC channels".to_string())
        }
        Err(e) => {
            error!("❌ Failed to stop services: {}", e);
            Err(format!("Failed to stop services: {}", e))
        }
    }
}

#[tauri::command]
async fn get_dashboard_data(state: State<'_, DashboardState>) -> Result<IntegrationData, String> {
    let data = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(data.clone())
}

#[tauri::command]
async fn run_integration_test(state: State<'_, DashboardState>) -> Result<String, String> {
    info!("🧪 Running full integration test...");
    
    let mut data = {
        let guard = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        guard.clone()
    };
    
    match service_manager::run_full_integration_test(&mut data).await {
        Ok(result) => {
            // Update the shared state
            if let Ok(mut guard) = state.lock() {
                *guard = data;
            }
            info!("✅ Integration test completed: {}", result);
            Ok(result)
        }
        Err(e) => {
            error!("❌ Integration test failed: {}", e);
            Err(format!("Integration test failed: {}", e))
        }
    }
}

#[tauri::command]
async fn mint_tokens(
    state: State<'_, DashboardState>, 
    player: String, 
    amount: u32
) -> Result<String, String> {
    info!("🪙 Minting {} tokens for {}", amount, player);
    
    let mut data = {
        let guard = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        guard.clone()
    };
    
    match service_manager::mint_tokens_for_player(&mut data, &player, amount).await {
        Ok(tokens) => {
            // Update the shared state
            if let Ok(mut guard) = state.lock() {
                *guard = data;
            }
            info!("✅ Minted {} tokens for {}", tokens.len(), player);
            Ok(format!("Minted {} tokens for {}", tokens.len(), player))
        }
        Err(e) => {
            error!("❌ Failed to mint tokens: {}", e);
            Err(format!("Failed to mint tokens: {}", e))
        }
    }
}

// Background monitoring task - processes MPSC messages from services
async fn start_monitoring_task(
    app_handle: AppHandle, 
    data_state: DashboardState, 
    service_state: ServiceManagerState
) {
    let mut interval = tokio::time::interval(Duration::from_secs(1)); // Process messages more frequently
    
    loop {
        interval.tick().await;
        
        // Process MPSC messages from services without holding locks across await
        let services_running = {
            // First, try to process messages
            let process_result = {
                let mut service_manager_opt = match service_state.try_lock() {
                    Ok(manager) => manager,
                    Err(_) => continue, // Skip if locked
                };
                
                let mut data = match data_state.try_lock() {
                    Ok(data) => data,
                    Err(_) => continue, // Skip if locked
                };
                
                // Process incoming service messages - this is the problematic await
                // Instead, let's use a non-async version
                let services_running = data.services_running;
                
                // Check if service manager is initialized
                if let Some(ref mut service_manager) = *service_manager_opt {
                    // For now, process without the async call - we'll need to refactor this
                    match service_manager.orchestrator.message_rx.try_recv() {
                    Ok(message) => {
                        // Process the message synchronously
                        match message {
                            service_manager::ServiceMessage::StatusUpdate { service, status } => {
                                info!("📊 {} status: {:?}", service, status);
                                match service.as_str() {
                                    "CDK Mint" => data.cdk_mint_status = status,
                                    "Nostr Relay" => data.nostr_relay_status = status,
                                    "Game Engine" => data.game_engine_status = status,
                                    _ => {}
                                }
                            }
                            service_manager::ServiceMessage::LogMessage { service, message } => {
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
                            service_manager::ServiceMessage::Error { service, error } => {
                                error!("❌ {} error: {}", service, error);
                                data.integration_log.push(format!("❌ {}: {}", service, error));
                            }
                            service_manager::ServiceMessage::HealthCheck { service, healthy } => {
                                if healthy {
                                    info!("✅ {} health check passed", service);
                                } else {
                                    warn!("⚠️ {} health check failed", service);
                                }
                            }
                        }
                    }
                    Err(_) => {} // No messages available
                    }
                }
                
                services_running
            };
            
            process_result
        };
        
        // Only emit updates if services are running
        if services_running {
            // Get a copy of data for emitting
            let data_copy = match data_state.try_lock() {
                Ok(data) => data.clone(),
                Err(_) => continue,
            };
            
            // Emit updated data to frontend
            if let Err(e) = app_handle.emit("dashboard-update", &data_copy) {
                warn!("Failed to emit dashboard update: {}", e);
            }
        }
    }
}

// Global monitoring task handle
static MONITORING_STARTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🏛️ Starting Manastr Integration Dashboard with MPSC Architecture");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_integration_services,
            stop_integration_services,
            get_dashboard_data,
            run_integration_test,
            mint_tokens
        ])
        .setup(move |app| {
            // Initialize dashboard state
            let dashboard_state: DashboardState = Arc::new(Mutex::new(IntegrationData::default()));
            app.manage(dashboard_state);
            
            // Create a placeholder service manager state that will be initialized lazily
            // We can't run async code in the setup function, so we'll initialize on first use
            let empty_service_manager = Arc::new(Mutex::new(None::<IntegratedServiceManager>));
            app.manage(empty_service_manager);
            
            info!("🏛️ Manastr Integration Dashboard initialized successfully");
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                // Get the service manager and stop all services
                let app_handle = window.app_handle();
                if let Some(service_state) = app_handle.try_state::<ServiceManagerState>() {
                    if let Ok(service_manager_opt) = service_state.try_lock() {
                        if let Some(ref service_manager) = *service_manager_opt {
                            info!("🛑 Application closing - stopping all services...");
                            let _ = service_manager.stop_all_services_nonblocking();
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
