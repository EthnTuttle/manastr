use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MintError {
    #[error("CDK error: {0}")]
    Cdk(#[from] cdk::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Lightning error: {0}")]
    Lightning(String),
    
    #[error("Unsupported currency: {0}")]
    UnsupportedCurrency(String),
    
    #[error("Invalid quote: {0}")]
    InvalidQuote(String),
    
    #[error("Invalid proof: {0}")]
    InvalidProof(String),
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Quote not found: {0}")]
    QuoteNotFound(String),
    
    #[error("Quote expired: {0}")]
    QuoteExpired(String),
    
    #[error("Payment failed: {0}")]
    PaymentFailed(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

impl IntoResponse for MintError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            MintError::Cdk(ref err) => {
                (StatusCode::BAD_REQUEST, "CDK_ERROR", err.to_string())
            }
            MintError::Database(ref err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", err.to_string())
            }
            MintError::Lightning(ref msg) => {
                (StatusCode::BAD_GATEWAY, "LIGHTNING_ERROR", msg.clone())
            }
            MintError::UnsupportedCurrency(ref currency) => {
                (StatusCode::BAD_REQUEST, "UNSUPPORTED_CURRENCY", format!("Currency '{}' is not supported", currency))
            }
            MintError::InvalidQuote(ref msg) => {
                (StatusCode::BAD_REQUEST, "INVALID_QUOTE", msg.clone())
            }
            MintError::InvalidProof(ref msg) => {
                (StatusCode::BAD_REQUEST, "INVALID_PROOF", msg.clone())
            }
            MintError::InsufficientBalance => {
                (StatusCode::BAD_REQUEST, "INSUFFICIENT_BALANCE", "Insufficient balance for operation".to_string())
            }
            MintError::QuoteNotFound(ref id) => {
                (StatusCode::NOT_FOUND, "QUOTE_NOT_FOUND", format!("Quote '{}' not found", id))
            }
            MintError::QuoteExpired(ref id) => {
                (StatusCode::BAD_REQUEST, "QUOTE_EXPIRED", format!("Quote '{}' has expired", id))
            }
            MintError::PaymentFailed(ref msg) => {
                (StatusCode::BAD_REQUEST, "PAYMENT_FAILED", msg.clone())
            }
            MintError::Internal(ref msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg.clone())
            }
            MintError::Serialization(ref err) => {
                (StatusCode::BAD_REQUEST, "SERIALIZATION_ERROR", err.to_string())
            }
            MintError::Config(ref msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "CONFIG_ERROR", msg.clone())
            }
        };

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": message,
            }
        }));

        (status, body).into_response()
    }
}