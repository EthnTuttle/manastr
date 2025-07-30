use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a match challenge initiated by a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchChallenge {
    pub challenger_npub: String,
    pub wager_amount: u64,
    pub league_id: u8,
    pub cashu_token_commitment: String,
    pub army_commitment: String,
    pub expires_at: u64,
    pub created_at: u64,
    pub match_event_id: String,
}

/// Represents acceptance of a match challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchAcceptance {
    pub acceptor_npub: String,
    pub match_event_id: String,
    pub cashu_token_commitment: String,
    pub army_commitment: String,
    pub accepted_at: u64,
}

/// Represents revelation of Cashu tokens for army verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenReveal {
    pub player_npub: String,
    pub match_event_id: String,
    pub cashu_tokens: Vec<String>,
    pub token_secrets_nonce: String,
    pub revealed_at: u64,
}

/// Represents commitment to moves in a combat round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCommitment {
    pub player_npub: String,
    pub match_event_id: String,
    pub round_number: u32,
    pub move_commitment: String,
    pub committed_at: u64,
}

/// Represents revelation of moves in a combat round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveReveal {
    pub player_npub: String,
    pub match_event_id: String,
    pub round_number: u32,
    pub unit_positions: Vec<u8>,
    pub unit_abilities: Vec<String>,
    pub moves_nonce: String,
    pub revealed_at: u64,
}

/// Represents final match results submitted by players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub player_npub: String,
    pub match_event_id: String,
    pub final_army_state: Value,
    pub all_round_results: Vec<Value>,
    pub calculated_winner: Option<String>,
    pub match_completed_at: u64,
}

/// Represents loot distribution by the game engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootDistribution {
    pub game_engine_npub: String,
    pub match_event_id: String,
    pub winner_npub: String,
    pub loot_cashu_token: String,
    pub match_fee: u64,
    pub loot_issued_at: u64,
    pub validation_summary: super::validation::ValidationSummary,
}

/// Response from melt quote request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeltQuoteResponse {
    pub quote_id: String,
    pub amount: u64,
    pub fee_reserve: u64,
    pub paid: bool,
    pub expiry: Option<u64>,
}

/// Result from executing a melt operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeltResult {
    pub paid: bool,
    pub amount: u64,
    pub payment_preimage: Option<String>,
}
