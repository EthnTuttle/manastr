use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEngineConfig {
    pub server: ServerConfig,
    pub nostr: NostrConfig,
    pub cashu: CashuConfig,
    pub game: GameConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrConfig {
    pub relay_url: String,
    pub private_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashuConfig {
    pub mint_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub max_concurrent_matches: u32,
    pub round_timeout_seconds: u64,
    pub match_timeout_seconds: u64,
    pub loot_reward_per_match: u64,
}

impl Default for GameEngineConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 4444,
            },
            nostr: NostrConfig {
                relay_url: "ws://localhost:7777".to_string(),
                private_key: "game_engine_bot_private_key_hex".to_string(),
            },
            cashu: CashuConfig {
                mint_url: "http://localhost:3333".to_string(),
            },
            game: GameConfig {
                max_concurrent_matches: 100,
                round_timeout_seconds: 300,  // 5 minutes
                match_timeout_seconds: 1800, // 30 minutes
                loot_reward_per_match: 1000,
            },
        }
    }
}

impl GameEngineConfig {
    pub fn load() -> Result<Self> {
        let config_path = "game-engine.toml";
        
        if !std::path::Path::new(config_path).exists() {
            // Create default config file
            let default_config = Self::default();
            let toml_string = toml::to_string_pretty(&default_config)?;
            fs::write(config_path, toml_string)?;
            tracing::info!("📋 Created default {} configuration file", config_path);
        }
        
        let config_str = fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&config_str)?;
        
        Ok(config)
    }
}