use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MintConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub lightning: LightningConfig,
    pub currencies: CurrenciesConfig,
    pub authorization: AuthorizationConfig,
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

/// 🔐 CRITICAL: Game Engine Authorization Configuration
/// 
/// This config section controls which game engines are authorized to:
/// - Burn/spend mana tokens after match validation
/// - Query token spent status for anti-cheat validation
/// - Request loot token minting for winners
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// List of authorized game engine Nostr public keys (npub format)
    pub authorized_game_engines: Vec<GameEngineAuth>,
    /// Whether to allow runtime updates to authorization (via config reload)
    pub allow_runtime_updates: bool,
    /// File path to watch for authorization updates
    pub auth_config_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEngineAuth {
    /// Game engine identifier (human readable)
    pub name: String,
    /// Nostr public key in hex format (without npub prefix)
    pub nostr_pubkey_hex: String,
    /// Permissions granted to this game engine
    pub permissions: GameEnginePermissions,
    /// Whether this authorization is currently active
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEnginePermissions {
    /// Can burn mana tokens after match validation
    pub can_burn_mana: bool,
    /// Can query token spent status for anti-cheat
    pub can_query_spent_status: bool,
    /// Can request loot token minting
    pub can_mint_loot: bool,
    /// Maximum number of tokens that can be processed per request
    pub max_tokens_per_request: u32,
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
            authorization: AuthorizationConfig {
                authorized_game_engines: vec![
                    GameEngineAuth {
                        name: "Default Test Game Engine".to_string(),
                        // This is a test key - in production, use the actual game engine's pubkey
                        nostr_pubkey_hex: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".to_string(),
                        permissions: GameEnginePermissions {
                            can_burn_mana: true,
                            can_query_spent_status: true,
                            can_mint_loot: true,
                            max_tokens_per_request: 1000,
                        },
                        active: true,
                    }
                ],
                allow_runtime_updates: true,
                auth_config_file: Some("game-engine-auth.toml".to_string()),
            },
        }
    }
}

impl MintConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "mint.toml";
        
        if fs::metadata(config_path).is_ok() {
            let config_str = fs::read_to_string(config_path)?;
            let mut config: MintConfig = toml::from_str(&config_str)?;
            
            // Load additional authorization config if specified
            if let Some(auth_file) = &config.authorization.auth_config_file {
                if let Ok(auth_config) = Self::load_auth_config(auth_file) {
                    config.authorization.authorized_game_engines = auth_config.authorized_game_engines;
                }
            }
            
            Ok(config)
        } else {
            // Create default config file
            let default_config = Self::default();
            let config_str = toml::to_string_pretty(&default_config)?;
            fs::write(config_path, config_str)?;
            
            // Also create default auth config
            if let Some(auth_file) = &default_config.authorization.auth_config_file {
                Self::create_default_auth_config(auth_file)?;
            }
            
            Ok(default_config)
        }
    }
    
    /// Load game engine authorization configuration from separate file
    pub fn load_auth_config(auth_file: &str) -> Result<AuthorizationConfig, Box<dyn std::error::Error>> {
        let auth_str = fs::read_to_string(auth_file)?;
        let auth_config: AuthorizationConfig = toml::from_str(&auth_str)?;
        Ok(auth_config)
    }
    
    /// Create default authorization config file
    fn create_default_auth_config(auth_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let default_auth = AuthorizationConfig {
            authorized_game_engines: vec![
                GameEngineAuth {
                    name: "Test Game Engine".to_string(),
                    nostr_pubkey_hex: "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".to_string(),
                    permissions: GameEnginePermissions {
                        can_burn_mana: true,
                        can_query_spent_status: true,
                        can_mint_loot: true,
                        max_tokens_per_request: 1000,
                    },
                    active: true,
                }
            ],
            allow_runtime_updates: true,
            auth_config_file: Some(auth_file.to_string()),
        };
        
        let auth_str = toml::to_string_pretty(&default_auth)?;
        fs::write(auth_file, auth_str)?;
        Ok(())
    }
    
    /// Reload authorization configuration at runtime
    pub fn reload_auth_config(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if !self.authorization.allow_runtime_updates {
            return Ok(false);
        }
        
        if let Some(auth_file) = &self.authorization.auth_config_file {
            match Self::load_auth_config(auth_file) {
                Ok(new_auth_config) => {
                    self.authorization.authorized_game_engines = new_auth_config.authorized_game_engines;
                    Ok(true)
                }
                Err(e) => Err(e)
            }
        } else {
            Ok(false)
        }
    }
}

impl AuthorizationConfig {
    /// Check if a Nostr public key is authorized for a specific permission
    pub fn is_authorized(&self, nostr_pubkey_hex: &str, permission: &str) -> bool {
        self.authorized_game_engines
            .iter()
            .find(|engine| engine.active && engine.nostr_pubkey_hex == nostr_pubkey_hex)
            .map(|engine| match permission {
                "burn_mana" => engine.permissions.can_burn_mana,
                "query_spent" => engine.permissions.can_query_spent_status,
                "mint_loot" => engine.permissions.can_mint_loot,
                _ => false,
            })
            .unwrap_or(false)
    }
    
    /// Get the maximum tokens per request for an authorized game engine
    pub fn get_max_tokens_per_request(&self, nostr_pubkey_hex: &str) -> Option<u32> {
        self.authorized_game_engines
            .iter()
            .find(|engine| engine.active && engine.nostr_pubkey_hex == nostr_pubkey_hex)
            .map(|engine| engine.permissions.max_tokens_per_request)
    }
}