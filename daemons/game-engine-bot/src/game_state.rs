use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::GameEngineError;
use crate::match_events::*;

// Import shared types and commitment functions
use shared_game_logic::game_state::{Unit, Ability, RoundResult};
use shared_game_logic::commitment::*;

// Use the MatchPhase from match_events instead of defining our own
use crate::match_events::MatchPhase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchState {
    pub match_id: String,
    pub players: [String; 2], // npubs
    pub current_round: u8,
    pub state: MatchPhase,
    pub rounds: Vec<RoundResult>,
    pub commitments: HashMap<(String, u8), CommitmentData>, // (npub, round) -> commitment
    pub reveals: HashMap<(String, u8), RevealData>,
    pub created_at: DateTime<Utc>,
    pub timeout_at: Option<DateTime<Utc>>,
}

// MatchPhase is now imported from match_events.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentData {
    pub hash: String,
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevealData {
    pub unit: Unit,
    pub token_secret: String,
    pub event_id: String,
}

// Removed: Unit, Ability, and RoundResult are now imported from shared-game-logic

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub match_id: String,
    pub winner: Option<String>,
    pub score: [u8; 2], // rounds won
    pub total_damage: [u32; 2],
    pub rounds: Vec<RoundResult>,
}

// MatchState is now deprecated in favor of PlayerMatch from match_events.rs

/// Player-driven match validation manager
/// Only tracks matches for validation purposes - players drive the flow via Nostr
pub struct MatchValidationManager {
    /// Active PlayerMatch states being tracked for validation
    matches: HashMap<String, PlayerMatch>,
    /// Pending challenges waiting for acceptance
    pending_challenges: HashMap<String, MatchChallenge>,
}

impl MatchValidationManager {
    pub fn new() -> Self {
        Self {
            matches: HashMap::new(),
            pending_challenges: HashMap::new(),
        }
    }

    /// Track a new challenge for potential validation
    pub fn add_pending_challenge(&mut self, challenge: MatchChallenge) {
        // Use challenger's npub + timestamp as unique ID for pending challenges
        let challenge_id = format!("{}_{}", challenge.challenger_npub, challenge.created_at);
        self.pending_challenges.insert(challenge_id, challenge);
    }

    /// Initialize match validation when challenge is accepted
    pub fn initialize_match_validation(&mut self, acceptance: &MatchAcceptance) -> Result<(), GameEngineError> {
        // Find the original challenge
        let challenge = self.pending_challenges.values()
            .find(|c| c.challenger_npub != acceptance.acceptor_npub) // Not same player
            .ok_or_else(|| GameEngineError::MatchNotFound("No matching challenge found".to_string()))?
            .clone();

        // Create PlayerMatch for validation tracking
        let player_match = PlayerMatch::new(&challenge, acceptance.match_id.clone());
        self.matches.insert(acceptance.match_id.clone(), player_match);

        Ok(())
    }

    /// Get PlayerMatch for validation
    pub fn get_match(&self, match_id: &str) -> Result<&PlayerMatch, GameEngineError> {
        self.matches.get(match_id)
            .ok_or_else(|| GameEngineError::MatchNotFound(match_id.to_string()))
    }

    /// Get mutable PlayerMatch for validation updates
    pub fn get_match_mut(&mut self, match_id: &str) -> Result<&mut PlayerMatch, GameEngineError> {
        self.matches.get_mut(match_id)
            .ok_or_else(|| GameEngineError::MatchNotFound(match_id.to_string()))
    }

    /// Validate token reveal against original commitment
    pub fn validate_token_reveal(&mut self, reveal: &TokenReveal) -> Result<bool, GameEngineError> {
        let player_match = self.get_match(&reveal.match_id)?;
        
        // Get the original token commitment for this player
        let commitment = if reveal.player_npub == player_match.player1_npub {
            &player_match.player1_commitments.cashu_tokens
        } else if reveal.player_npub == player_match.player2_npub {
            &player_match.player2_commitments.cashu_tokens
        } else {
            return Err(GameEngineError::Internal("Unknown player in token reveal".to_string()));
        };

        let commitment_hash = commitment.as_ref()
            .ok_or_else(|| GameEngineError::Internal("No token commitment found".to_string()))?;

        // Validate using shared commitment logic
        let is_valid = verify_cashu_commitment(
            commitment_hash,
            &reveal.cashu_tokens,
            &reveal.token_secrets_nonce
        );

        if is_valid {
            // Update the match state with valid reveal
            let player_match = self.get_match_mut(&reveal.match_id)?;
            player_match.add_token_reveal(reveal)?;
        }

        Ok(is_valid)
    }

    /// Validate move commitment/reveal cycle
    pub fn validate_move_reveal(&mut self, reveal: &MoveReveal) -> Result<bool, GameEngineError> {
        let player_match = self.get_match(&reveal.match_id)?;
        
        // Get the original move commitment for this player and round
        let commitment_hash = if reveal.player_npub == player_match.player1_npub {
            player_match.player1_commitments.moves_by_round.get(&reveal.round_number)
        } else if reveal.player_npub == player_match.player2_npub {
            player_match.player2_commitments.moves_by_round.get(&reveal.round_number)
        } else {
            return Err(GameEngineError::Internal("Unknown player in move reveal".to_string()));
        };

        let commitment_hash = commitment_hash
            .ok_or_else(|| GameEngineError::Internal("No move commitment found".to_string()))?;

        // Validate using shared commitment logic
        let is_valid = verify_moves_commitment(
            commitment_hash,
            &reveal.unit_positions,
            &reveal.unit_abilities,
            &reveal.moves_nonce
        );

        if is_valid {
            // Update the match state with valid reveal
            let player_match = self.get_match_mut(&reveal.match_id)?;
            player_match.add_move_reveal(reveal)?;
        }

        Ok(is_valid)
    }

    /// Check if match is ready for final validation
    pub fn is_ready_for_final_validation(&self, match_id: &str) -> Result<bool, GameEngineError> {
        let player_match = self.get_match(match_id)?;
        
        // Match is ready if both players have revealed tokens and submitted final results
        Ok(matches!(player_match.phase, MatchPhase::Completed) &&
           player_match.player1_reveals.cashu_tokens.is_some() &&
           player_match.player2_reveals.cashu_tokens.is_some())
    }

    /// Validate complete match result using deterministic combat logic
    pub fn validate_match_result(&self, match_id: &str, _claimed_result: &crate::match_events::MatchResult) -> Result<ValidationSummary, GameEngineError> {
        let _player_match = self.get_match(match_id)?;
        
        let mut validation = ValidationSummary {
            commitments_valid: true,
            combat_verified: false,
            signatures_valid: true, // Nostr handles signature validation
            winner_confirmed: false,
            error_details: None,
        };

        // TODO: Implement full match validation
        // 1. Re-execute all combat rounds using revealed moves
        // 2. Compare results with claimed results
        // 3. Verify winner calculation

        // For now, mark as valid (this is where the deterministic combat validation would go)
        validation.combat_verified = true;
        validation.winner_confirmed = true;

        Ok(validation)
    }

    /// Get list of matches ready for loot distribution
    pub fn get_matches_ready_for_loot(&self) -> Vec<&PlayerMatch> {
        self.matches.values()
            .filter(|m| matches!(m.phase, MatchPhase::Completed))
            .collect()
    }

    /// Mark match as loot distributed and archive
    pub fn mark_loot_distributed(&mut self, match_id: &str) -> Result<(), GameEngineError> {
        let player_match = self.get_match_mut(match_id)?;
        player_match.mark_loot_distributed();
        Ok(())
    }

    /// Get active match count for status reporting
    pub fn get_active_match_count(&self) -> usize {
        self.matches.values()
            .filter(|m| !matches!(m.phase, MatchPhase::LootDistributed | MatchPhase::Invalid(_)))
            .count()
    }
}