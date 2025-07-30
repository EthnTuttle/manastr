use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameEngineError {
    #[error("Nostr connection failed: {0}")]
    NostrConnectionError(String),

    #[error("Nostr error: {0}")]
    NostrError(String),

    #[error("Cashu mint communication failed: {0}")]
    CashuError(String),

    #[error("Invalid event format: {0}")]
    EventParsingError(String),

    #[error("Match not found: {0}")]
    MatchNotFound(String),

    #[error("Invalid game state transition")]
    InvalidStateTransition,

    #[error("Combat resolution failed: {0}")]
    CombatError(String),

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<String> for GameEngineError {
    fn from(err: String) -> Self {
        GameEngineError::Internal(err)
    }
}
