use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub lightning: LightningConfig,
    pub currencies: CurrenciesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningConfig {
    pub backend: String, // "stub", "lnd", "cln", etc.
    pub connection_string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrenciesConfig {
    pub mana: CurrencyConfig,
    pub loot: CurrencyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConfig {
    pub unit: String,
    pub precision: u8,
    pub min_amount: u64,
    pub max_amount: u64,
    pub fee_reserve: f64, // Percentage
}

impl Default for MintConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3333,
                cors_origins: vec!["*".to_string()],
            },
            database: DatabaseConfig {
                url: "sqlite:mint.db".to_string(),
                max_connections: 10,
            },
            lightning: LightningConfig {
                backend: "stub".to_string(),
                connection_string: None,
            },
            currencies: CurrenciesConfig {
                mana: CurrencyConfig {
                    unit: "mana".to_string(),
                    precision: 0,
                    min_amount: 1,
                    max_amount: 1_000_000,
                    fee_reserve: 1.0, // 1% reserve
                },
                loot: CurrencyConfig {
                    unit: "loot".to_string(),
                    precision: 0,
                    min_amount: 1,
                    max_amount: 1_000_000,
                    fee_reserve: 2.0, // 2% reserve for loot
                },
            },
        }
    }
}

impl MintConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "mint.toml";
        
        if fs::metadata(config_path).is_ok() {
            let config_str = fs::read_to_string(config_path)?;
            let config: MintConfig = toml::from_str(&config_str)?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = Self::default();
            let config_str = toml::to_string_pretty(&default_config)?;
            fs::write(config_path, config_str)?;
            Ok(default_config)
        }
    }
}