#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

static CSS: Asset = asset!("/assets/styles.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &js_sys::Function) -> JsValue;
}

// Data structures matching the Rust backend
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IntegrationData {
    pub services_running: bool,
    pub cdk_mint_status: ServiceStatus,
    pub nostr_relay_status: ServiceStatus,
    pub game_engine_status: ServiceStatus,
    pub cdk_mint_url: String,
    pub nostr_relay_url: String,
    pub game_engine_url: String,
    pub alice_balance: u64,
    pub bob_balance: u64,
    pub alice_tokens: Vec<String>,
    pub bob_tokens: Vec<String>,
    pub current_match_id: Option<String>,
    pub match_phase: String,
    pub pending_challenges: u32,
    pub completed_matches: u32,
    pub service_logs: HashMap<String, Vec<String>>,
    pub integration_log: Vec<String>,
    pub last_test_result: Option<String>,
    pub test_running: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize)]
struct MintTokensArgs {
    player: String,
    amount: u32,
}

pub fn App() -> Element {
    let mut dashboard_data = use_signal(|| IntegrationData::default());
    let mut status_message = use_signal(|| "Ready to start integration dashboard".to_string());
    let mut auto_refresh = use_signal(|| false);

    // Start services
    let start_services = move |_| async move {
        status_message.set("Starting services...".to_string());
        match invoke("start_integration_services", JsValue::NULL).await.as_string() {
            Some(msg) => {
                status_message.set(msg);
                refresh_data(&mut dashboard_data).await;
            }
            None => status_message.set("Failed to start services".to_string()),
        }
    };

    // Stop services
    let stop_services = move |_| async move {
        status_message.set("Stopping services...".to_string());
        match invoke("stop_integration_services", JsValue::NULL).await.as_string() {
            Some(msg) => {
                status_message.set(msg);
                refresh_data(&mut dashboard_data).await;
            }
            None => status_message.set("Failed to stop services".to_string()),
        }
    };

    // Run integration test
    let run_test = move |_| async move {
        status_message.set("Running integration test...".to_string());
        match invoke("run_integration_test", JsValue::NULL).await.as_string() {
            Some(result) => {
                status_message.set("Integration test completed".to_string());
                refresh_data(&mut dashboard_data).await;
            }
            None => status_message.set("Integration test failed".to_string()),
        }
    };

    // Mint tokens for Alice
    let mint_alice = move |_| async move {
        let args = serde_wasm_bindgen::to_value(&MintTokensArgs {
            player: "alice".to_string(),
            amount: 5,
        }).unwrap();
        
        match invoke("mint_tokens", args).await.as_string() {
            Some(msg) => {
                status_message.set(msg);
                refresh_data(&mut dashboard_data).await;
            }
            None => status_message.set("Failed to mint tokens for Alice".to_string()),
        }
    };

    // Mint tokens for Bob
    let mint_bob = move |_| async move {
        let args = serde_wasm_bindgen::to_value(&MintTokensArgs {
            player: "bob".to_string(),
            amount: 5,
        }).unwrap();
        
        match invoke("mint_tokens", args).await.as_string() {
            Some(msg) => {
                status_message.set(msg);
                refresh_data(&mut dashboard_data).await;
            }
            None => status_message.set("Failed to mint tokens for Bob".to_string()),
        }
    };

    // Refresh dashboard data
    let refresh = move |_| async move {
        refresh_data(&mut dashboard_data).await;
        status_message.set("Dashboard refreshed".to_string());
    };

    // Auto-refresh toggle
    let toggle_auto_refresh = move |_| {
        auto_refresh.set(!auto_refresh());
    };

    // Auto-refresh effect using WASM-compatible timing
    use_effect(move || {
        if auto_refresh() {
            let data_signal = dashboard_data.clone();
            spawn(async move {
                loop {
                    // Use gloo_timers for WASM compatibility
                    gloo_timers::future::TimeoutFuture::new(2000).await;
                    if auto_refresh() {
                        refresh_data_signal(data_signal.clone()).await;
                    } else {
                        break;
                    }
                }
            });
        }
    });

    let data = dashboard_data.read();

    rsx! {
        link { rel: "stylesheet", href: CSS }
        
        div { class: "dashboard-container",
            // Header
            header { class: "dashboard-header",
                h1 { "🏛️ MANASTR Integration Dashboard" }
                p { "Revolutionary Zero-Coordination Gaming - Real-time Service Monitor" }
                div { class: "status-bar",
                    span { class: "status-message", "{status_message}" }
                    button { 
                        class: "refresh-btn",
                        onclick: refresh,
                        "🔄 Refresh"
                    }
                    label { class: "auto-refresh-toggle",
                        input { 
                            r#type: "checkbox", 
                            checked: auto_refresh(),
                            onchange: toggle_auto_refresh
                        }
                        "Auto-refresh"
                    }
                }
            }

            // Main content
            main { class: "dashboard-main",
                
                // Service Control Panel
                section { class: "control-panel",
                    h2 { "🎮 Service Control" }
                    div { class: "button-grid",
                        button { 
                            class: "btn btn-primary",
                            disabled: data.services_running,
                            onclick: start_services,
                            "🚀 Start Services"
                        }
                        button { 
                            class: "btn btn-danger",
                            disabled: !data.services_running,
                            onclick: stop_services,
                            "🛑 Stop Services"
                        }
                        button { 
                            class: "btn btn-success",
                            disabled: !data.services_running || data.test_running,
                            onclick: run_test,
                            if data.test_running { "🧪 Running Test..." } else { "🧪 Run Integration Test" }
                        }
                    }
                }

                // Service Status
                section { class: "service-status",
                    h2 { "📊 Service Status" }
                    div { class: "service-grid",
                        ServiceCard {
                            name: "CDK Mint",
                            status: data.cdk_mint_status.clone(),
                            url: data.cdk_mint_url.clone(),
                            icon: "🏦"
                        }
                        ServiceCard {
                            name: "Nostr Relay",
                            status: data.nostr_relay_status.clone(),
                            url: data.nostr_relay_url.clone(),
                            icon: "📡"
                        }
                        ServiceCard {
                            name: "Game Engine",
                            status: data.game_engine_status.clone(),
                            url: data.game_engine_url.clone(),
                            icon: "🎮"
                        }
                    }
                }

                // Player Wallets
                section { class: "player-wallets",
                    h2 { "💰 Player Wallets" }
                    div { class: "wallet-grid",
                        WalletCard {
                            player: "Alice",
                            balance: data.alice_balance,
                            tokens: data.alice_tokens.len() as u32,
                            on_mint: mint_alice,
                            disabled: !data.services_running
                        }
                        WalletCard {
                            player: "Bob",
                            balance: data.bob_balance,
                            tokens: data.bob_tokens.len() as u32,
                            on_mint: mint_bob,
                            disabled: !data.services_running
                        }
                    }
                }

                // Match Information
                section { class: "match-info",
                    h2 { "⚔️ Match Information" }
                    div { class: "match-grid",
                        div { class: "match-card",
                            h4 { "Current Phase" }
                            p { class: "phase", "{data.match_phase}" }
                        }
                        div { class: "match-card",
                            h4 { "Active Match" }
                            p { 
                                if let Some(ref match_id) = data.current_match_id {
                                    "{match_id}"
                                } else {
                                    "No active match"
                                }
                            }
                        }
                        div { class: "match-card",
                            h4 { "Pending Challenges" }
                            p { class: "number", "{data.pending_challenges}" }
                        }
                        div { class: "match-card",
                            h4 { "Completed Matches" }
                            p { class: "number", "{data.completed_matches}" }
                        }
                    }
                }

                // Service Logs
                section { class: "service-logs",
                    h2 { "📋 Service Logs" }
                    div { class: "logs-container",
                        for (service_name, logs) in data.service_logs.iter() {
                            LogPanel {
                                service_name: service_name.clone(),
                                logs: logs.clone()
                            }
                        }
                    }
                }

                // Integration Log
                section { class: "integration-log",
                    h2 { "🔄 Integration Activity" }
                    div { class: "log-box",
                        for (i, log) in data.integration_log.iter().enumerate() {
                            div { key: "{i}", class: "log-entry", "{log}" }
                        }
                        if data.integration_log.is_empty() {
                            p { class: "no-logs", "No activity yet..." }
                        }
                    }
                }

                // Test Results
                if let Some(ref result) = data.last_test_result {
                    section { class: "test-results",
                        h2 { "🧪 Latest Test Results" }
                        pre { class: "test-output", "{result}" }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ServiceCardProps {
    name: String,
    status: ServiceStatus,
    url: String,
    icon: String,
}

fn ServiceCard(props: ServiceCardProps) -> Element {
    let status_class = match props.status {
        ServiceStatus::Running => "status-running",
        ServiceStatus::Starting => "status-starting",
        ServiceStatus::Failed(_) => "status-failed",
        ServiceStatus::Stopped => "status-stopped",
    };

    let status_text = match props.status {
        ServiceStatus::Running => "Running",
        ServiceStatus::Starting => "Starting...",
        ServiceStatus::Failed(ref msg) => msg,
        ServiceStatus::Stopped => "Stopped",
    };

    rsx! {
        div { class: "service-card {status_class}",
            div { class: "service-header",
                span { class: "service-icon", "{props.icon}" }
                h3 { "{props.name}" }
            }
            div { class: "service-body",
                p { class: "service-status", "{status_text}" }
                p { class: "service-url", "{props.url}" }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct WalletCardProps {
    player: String,
    balance: u64,
    tokens: u32,
    on_mint: EventHandler<MouseEvent>,
    disabled: bool,
}

fn WalletCard(props: WalletCardProps) -> Element {
    let player_class = match props.player.as_str() {
        "Alice" => "wallet-alice",
        "Bob" => "wallet-bob",
        _ => "wallet-default",
    };

    rsx! {
        div { class: "wallet-card {player_class}",
            div { class: "wallet-header",
                h3 { "{props.player}" }
                button { 
                    class: "mint-btn",
                    disabled: props.disabled,
                    onclick: move |evt| props.on_mint.call(evt),
                    "🪙 Mint Tokens"
                }
            }
            div { class: "wallet-body",
                div { class: "balance-row",
                    span { "Balance:" }
                    span { class: "balance", "{props.balance} MANA" }
                }
                div { class: "tokens-row",
                    span { "Tokens:" }
                    span { class: "tokens", "{props.tokens}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct LogPanelProps {
    service_name: String,
    logs: Vec<String>,
}

fn LogPanel(props: LogPanelProps) -> Element {
    rsx! {
        div { class: "log-panel",
            h4 { "{props.service_name}" }
            div { class: "log-content",
                for (i, log) in props.logs.iter().enumerate() {
                    div { key: "{i}", class: "log-line", "{log}" }
                }
                if props.logs.is_empty() {
                    p { class: "no-logs", "No logs available" }
                }
            }
        }
    }
}

// Helper function to refresh dashboard data
async fn refresh_data(dashboard_data: &mut Signal<IntegrationData>) {
    let data_js = invoke("get_dashboard_data", JsValue::NULL).await;
    if let Ok(data) = serde_wasm_bindgen::from_value::<IntegrationData>(data_js) {
        dashboard_data.set(data);
    }
}

// Clone-based version for async contexts
async fn refresh_data_signal(mut dashboard_data: Signal<IntegrationData>) {
    let data_js = invoke("get_dashboard_data", JsValue::NULL).await;
    if let Ok(data) = serde_wasm_bindgen::from_value::<IntegrationData>(data_js) {
        dashboard_data.set(data);
    }
}