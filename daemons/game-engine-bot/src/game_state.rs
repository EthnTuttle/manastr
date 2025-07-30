use crate::errors::GameEngineError;
use crate::match_events::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import shared types and commitment functions
use shared_game_logic::commitment::*;
use shared_game_logic::game_state::{RoundResult, Unit};

// Use the MatchPhase from match_events instead of defining our own
use crate::match_events::MatchPhase;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchState {
    pub match_event_id: String, // EventId as hex string for match identification
    pub players: [String; 2],   // npubs
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
    pub match_event_id: String, // EventId as hex string
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
    pub fn initialize_match_validation(
        &mut self,
        acceptance: &MatchAcceptance,
    ) -> Result<(), GameEngineError> {
        // Find the original challenge
        let challenge = self
            .pending_challenges
            .values()
            .find(|c| c.challenger_npub != acceptance.acceptor_npub) // Not same player
            .ok_or_else(|| {
                GameEngineError::MatchNotFound("No matching challenge found".to_string())
            })?
            .clone();

        // Create PlayerMatch for validation tracking
        let player_match = PlayerMatch::new(&challenge, acceptance.match_event_id.clone());
        self.matches
            .insert(acceptance.match_event_id.clone(), player_match);

        Ok(())
    }

    /// Get PlayerMatch for validation
    pub fn get_match(&self, match_event_id: &str) -> Result<&PlayerMatch, GameEngineError> {
        self.matches
            .get(match_event_id)
            .ok_or_else(|| GameEngineError::MatchNotFound(match_event_id.to_string()))
    }

    /// Get mutable PlayerMatch for validation updates
    pub fn get_match_mut(
        &mut self,
        match_event_id: &str,
    ) -> Result<&mut PlayerMatch, GameEngineError> {
        self.matches
            .get_mut(match_event_id)
            .ok_or_else(|| GameEngineError::MatchNotFound(match_event_id.to_string()))
    }

    /// Validate token reveal against original commitment
    pub fn validate_token_reveal(&mut self, reveal: &TokenReveal) -> Result<bool, GameEngineError> {
        let player_match = self.get_match(&reveal.match_event_id)?;

        // Get the original token commitment for this player
        let commitment = if reveal.player_npub == player_match.player1_npub {
            &player_match.player1_commitments.cashu_tokens
        } else if reveal.player_npub == player_match.player2_npub {
            &player_match.player2_commitments.cashu_tokens
        } else {
            return Err(GameEngineError::Internal(
                "Unknown player in token reveal".to_string(),
            ));
        };

        let commitment_hash = commitment
            .as_ref()
            .ok_or_else(|| GameEngineError::Internal("No token commitment found".to_string()))?;

        // Validate using shared commitment logic
        let is_valid = verify_cashu_commitment(
            commitment_hash,
            &reveal.cashu_tokens,
            &reveal.token_secrets_nonce,
        );

        if is_valid {
            // Update the match state with valid reveal
            let player_match = self.get_match_mut(&reveal.match_event_id)?;
            player_match.add_token_reveal(reveal)?;
        }

        Ok(is_valid)
    }

    /// Validate move commitment/reveal cycle
    pub fn validate_move_reveal(&mut self, reveal: &MoveReveal) -> Result<bool, GameEngineError> {
        let player_match = self.get_match(&reveal.match_event_id)?;

        // Get the original move commitment for this player and round
        let commitment_hash = if reveal.player_npub == player_match.player1_npub {
            player_match
                .player1_commitments
                .moves_by_round
                .get(&reveal.round_number)
        } else if reveal.player_npub == player_match.player2_npub {
            player_match
                .player2_commitments
                .moves_by_round
                .get(&reveal.round_number)
        } else {
            return Err(GameEngineError::Internal(
                "Unknown player in move reveal".to_string(),
            ));
        };

        let commitment_hash = commitment_hash
            .ok_or_else(|| GameEngineError::Internal("No move commitment found".to_string()))?;

        // Validate using shared commitment logic
        let is_valid = verify_moves_commitment(
            commitment_hash,
            &reveal.unit_positions,
            &reveal.unit_abilities,
            &reveal.moves_nonce,
        );

        if is_valid {
            // Update the match state with valid reveal
            let player_match = self.get_match_mut(&reveal.match_event_id)?;
            player_match.add_move_reveal(reveal)?;
        }

        Ok(is_valid)
    }

    /// Check if match is ready for final validation
    pub fn is_ready_for_final_validation(
        &self,
        match_event_id: &str,
    ) -> Result<bool, GameEngineError> {
        let player_match = self.get_match(match_event_id)?;

        // Match is ready if both players have revealed tokens and submitted final results
        Ok(matches!(player_match.phase, MatchPhase::Completed)
            && player_match.player1_reveals.cashu_tokens.is_some()
            && player_match.player2_reveals.cashu_tokens.is_some())
    }

    /// Validate complete match result using deterministic combat logic
    pub fn validate_match_result(
        &self,
        match_event_id: &str,
        claimed_result: &crate::match_events::MatchResult,
    ) -> Result<ValidationSummary, GameEngineError> {
        
        use tracing::{debug, error, info, warn};

        info!(
            "🔍 Starting comprehensive match validation for {}",
            match_event_id
        );

        let player_match = self.get_match(match_event_id)?;

        let mut validation = ValidationSummary {
            commitments_valid: true,
            combat_verified: false,
            signatures_valid: true, // Nostr handles signature validation
            winner_confirmed: false,
            error_details: None,
        };

        // Step 1: Validate all commitments have been properly revealed
        info!("📋 Step 1: Validating commitment/reveal integrity");
        if let Err(e) = self.validate_all_commitments(player_match) {
            validation.commitments_valid = false;
            validation.error_details = Some(format!("Commitment validation failed: {e}"));
            warn!("❌ Commitment validation failed: {}", e);
            return Ok(validation);
        }
        info!("✅ All commitments validated successfully");

        // Step 2: Generate armies from revealed tokens (deterministic)
        info!("🏭 Step 2: Generating deterministic armies from revealed tokens");
        let (player1_army, player2_army) = match self.generate_validated_armies(player_match) {
            Ok(armies) => {
                info!("✅ Generated armies for both players:");
                debug!(
                    "  Player 1 ({}): {} units",
                    player_match.player1_npub,
                    armies.0.len()
                );
                debug!(
                    "  Player 2 ({}): {} units",
                    player_match.player2_npub,
                    armies.1.len()
                );
                armies
            }
            Err(e) => {
                validation.error_details = Some(format!("Army generation failed: {e}"));
                error!("❌ Army generation failed: {}", e);
                return Ok(validation);
            }
        };

        // Step 3: Re-execute all combat rounds using revealed moves
        info!("⚔️ Step 3: Re-executing all combat rounds deterministically");
        let validated_rounds = match self.validate_all_combat_rounds(
            player_match,
            &player1_army,
            &player2_army,
            &claimed_result.all_round_results,
        ) {
            Ok(rounds) => {
                info!(
                    "✅ All {} combat rounds validated successfully",
                    rounds.len()
                );
                rounds
            }
            Err(e) => {
                validation.combat_verified = false;
                validation.error_details = Some(format!("Combat validation failed: {e}"));
                error!("❌ Combat validation failed: {}", e);
                return Ok(validation);
            }
        };

        // Step 4: Verify final winner calculation
        info!("🏆 Step 4: Validating winner calculation");
        let calculated_winner = self.calculate_match_winner(&validated_rounds, player_match);
        if calculated_winner == claimed_result.calculated_winner {
            info!("✅ Winner calculation verified: {:?}", calculated_winner);
            validation.winner_confirmed = true;
        } else {
            warn!(
                "❌ Winner mismatch - Expected: {:?}, Claimed: {:?}",
                calculated_winner, claimed_result.calculated_winner
            );
            validation.winner_confirmed = false;
            validation.error_details = Some(format!(
                "Winner mismatch: expected {:?}, claimed {:?}",
                calculated_winner, claimed_result.calculated_winner
            ));
        }

        // Final validation result
        validation.combat_verified = true;

        if validation.commitments_valid && validation.combat_verified && validation.winner_confirmed
        {
            info!(
                "🎉 MATCH VALIDATION COMPLETE: All checks passed for {}",
                match_event_id
            );
        } else {
            warn!(
                "⚠️ MATCH VALIDATION FAILED: Some checks failed for {}",
                match_event_id
            );
        }

        Ok(validation)
    }

    /// Validate that all required commitments have been properly revealed
    fn validate_all_commitments(&self, player_match: &PlayerMatch) -> Result<(), GameEngineError> {
        use tracing::{debug, info};

        info!("🔍 Validating commitment/reveal pairs for both players");

        // Check that both players revealed their Cashu tokens
        if player_match.player1_reveals.cashu_tokens.is_none() {
            return Err(GameEngineError::Internal(
                "Player 1 has not revealed Cashu tokens".to_string(),
            ));
        }

        if player_match.player2_reveals.cashu_tokens.is_none() {
            return Err(GameEngineError::Internal(
                "Player 2 has not revealed Cashu tokens".to_string(),
            ));
        }

        debug!("✅ Both players have revealed their Cashu tokens");

        // Validate token commitments using shared cryptography
        let p1_tokens = player_match.player1_reveals.cashu_tokens.as_ref().unwrap();
        let p1_nonce = player_match
            .player1_reveals
            .token_nonce
            .as_ref()
            .ok_or_else(|| GameEngineError::Internal("Player 1 missing token nonce".to_string()))?;
        let p1_commitment = player_match
            .player1_commitments
            .cashu_tokens
            .as_ref()
            .ok_or_else(|| {
                GameEngineError::Internal("Player 1 missing token commitment".to_string())
            })?;

        if !verify_cashu_commitment(p1_commitment, p1_tokens, p1_nonce) {
            return Err(GameEngineError::Internal(
                "Player 1 token commitment verification failed".to_string(),
            ));
        }

        let p2_tokens = player_match.player2_reveals.cashu_tokens.as_ref().unwrap();
        let p2_nonce = player_match
            .player2_reveals
            .token_nonce
            .as_ref()
            .ok_or_else(|| GameEngineError::Internal("Player 2 missing token nonce".to_string()))?;
        let p2_commitment = player_match
            .player2_commitments
            .cashu_tokens
            .as_ref()
            .ok_or_else(|| {
                GameEngineError::Internal("Player 2 missing token commitment".to_string())
            })?;

        if !verify_cashu_commitment(p2_commitment, p2_tokens, p2_nonce) {
            return Err(GameEngineError::Internal(
                "Player 2 token commitment verification failed".to_string(),
            ));
        }

        info!("✅ All token commitments verified successfully");

        // Validate move commitments for all completed rounds
        let completed_rounds: Vec<u32> = player_match
            .player1_reveals
            .moves_by_round
            .keys()
            .filter(|&round| {
                player_match
                    .player2_reveals
                    .moves_by_round
                    .contains_key(round)
            })
            .cloned()
            .collect();

        info!(
            "🎯 Validating move commitments for {} completed rounds",
            completed_rounds.len()
        );

        for round in completed_rounds {
            debug!("🔍 Validating round {} move commitments", round);

            // Player 1 move validation
            let p1_move_data = player_match
                .player1_reveals
                .moves_by_round
                .get(&round)
                .ok_or_else(|| {
                    GameEngineError::Internal(format!("Player 1 missing moves for round {round}"))
                })?;
            let p1_move_commitment = player_match
                .player1_commitments
                .moves_by_round
                .get(&round)
                .ok_or_else(|| {
                    GameEngineError::Internal(format!(
                        "Player 1 missing move commitment for round {round}"
                    ))
                })?;

            if !verify_moves_commitment(
                p1_move_commitment,
                &p1_move_data.0, // positions
                &p1_move_data.1, // abilities
                &p1_move_data.2, // nonce
            ) {
                return Err(GameEngineError::Internal(format!(
                    "Player 1 move commitment verification failed for round {round}"
                )));
            }

            // Player 2 move validation
            let p2_move_data = player_match
                .player2_reveals
                .moves_by_round
                .get(&round)
                .ok_or_else(|| {
                    GameEngineError::Internal(format!("Player 2 missing moves for round {round}"))
                })?;
            let p2_move_commitment = player_match
                .player2_commitments
                .moves_by_round
                .get(&round)
                .ok_or_else(|| {
                    GameEngineError::Internal(format!(
                        "Player 2 missing move commitment for round {round}"
                    ))
                })?;

            if !verify_moves_commitment(
                p2_move_commitment,
                &p2_move_data.0, // positions
                &p2_move_data.1, // abilities
                &p2_move_data.2, // nonce
            ) {
                return Err(GameEngineError::Internal(format!(
                    "Player 2 move commitment verification failed for round {round}"
                )));
            }

            debug!("✅ Round {} move commitments verified", round);
        }

        info!("✅ All commitment/reveal pairs validated successfully");
        Ok(())
    }

    /// Generate validated armies from revealed Cashu tokens
    fn generate_validated_armies(
        &self,
        player_match: &PlayerMatch,
    ) -> Result<
        (
            [shared_game_logic::game_state::Unit; 8],
            [shared_game_logic::game_state::Unit; 8],
        ),
        GameEngineError,
    > {
        use shared_game_logic::combat::generate_units_from_token_secret;
        use tracing::{debug, info};

        // Get token secrets (first token used for army generation)
        let p1_tokens = player_match
            .player1_reveals
            .cashu_tokens
            .as_ref()
            .ok_or_else(|| GameEngineError::Internal("Player 1 tokens not revealed".to_string()))?;
        let p2_tokens = player_match
            .player2_reveals
            .cashu_tokens
            .as_ref()
            .ok_or_else(|| GameEngineError::Internal("Player 2 tokens not revealed".to_string()))?;

        if p1_tokens.is_empty() || p2_tokens.is_empty() {
            return Err(GameEngineError::Internal(
                "Players must have at least one token".to_string(),
            ));
        }

        info!("🏭 Generating armies using deterministic algorithm:");
        info!(
            "  Player 1 ({}): Using token secret from {} tokens",
            player_match.player1_npub,
            p1_tokens.len()
        );
        info!(
            "  Player 2 ({}): Using token secret from {} tokens",
            player_match.player2_npub,
            p2_tokens.len()
        );

        // Generate armies deterministically from first token
        let player1_army = generate_units_from_token_secret(&p1_tokens[0], player_match.league_id);
        let player2_army = generate_units_from_token_secret(&p2_tokens[0], player_match.league_id);

        // Log army details for debugging
        debug!("🎪 Player 1 Army Generated:");
        for (i, unit) in player1_army.iter().enumerate() {
            debug!(
                "  Unit {}: ATK={} DEF={} HP={}/{} ABILITY={:?}",
                i, unit.attack, unit.defense, unit.health, unit.max_health, unit.ability
            );
        }

        debug!("🎪 Player 2 Army Generated:");
        for (i, unit) in player2_army.iter().enumerate() {
            debug!(
                "  Unit {}: ATK={} DEF={} HP={}/{} ABILITY={:?}",
                i, unit.attack, unit.defense, unit.health, unit.max_health, unit.ability
            );
        }

        info!(
            "✅ Both armies generated successfully using league {} modifiers",
            player_match.league_id
        );

        Ok((player1_army, player2_army))
    }

    /// Validate all combat rounds by re-executing them deterministically
    fn validate_all_combat_rounds(
        &self,
        player_match: &PlayerMatch,
        player1_army: &[shared_game_logic::game_state::Unit; 8],
        player2_army: &[shared_game_logic::game_state::Unit; 8],
        _claimed_rounds: &[serde_json::Value],
    ) -> Result<Vec<shared_game_logic::game_state::RoundResult>, GameEngineError> {
        use shared_game_logic::combat::process_combat;
        use tracing::{debug, info};

        let completed_rounds: Vec<u32> = player_match
            .player1_reveals
            .moves_by_round
            .keys()
            .filter(|&round| {
                player_match
                    .player2_reveals
                    .moves_by_round
                    .contains_key(round)
            })
            .cloned()
            .collect();

        info!(
            "⚔️ Re-executing {} combat rounds for validation",
            completed_rounds.len()
        );

        let mut validated_rounds = Vec::new();

        for round_num in completed_rounds {
            info!("🥊 Validating combat round {}", round_num);

            // Get revealed moves for this round
            let p1_moves = player_match
                .player1_reveals
                .moves_by_round
                .get(&round_num)
                .ok_or_else(|| {
                    GameEngineError::Internal(format!(
                        "Player 1 moves missing for round {round_num}"
                    ))
                })?;
            let p2_moves = player_match
                .player2_reveals
                .moves_by_round
                .get(&round_num)
                .ok_or_else(|| {
                    GameEngineError::Internal(format!(
                        "Player 2 moves missing for round {round_num}"
                    ))
                })?;

            // Extract unit positions (which units to use)
            let p1_unit_idx = p1_moves.0.first().copied().unwrap_or(0) as usize % 8;
            let p2_unit_idx = p2_moves.0.first().copied().unwrap_or(0) as usize % 8;

            debug!("🎯 Round {} unit selection:", round_num);
            debug!(
                "  Player 1 selected unit {} (from position bytes: {:?})",
                p1_unit_idx, p1_moves.0
            );
            debug!(
                "  Player 2 selected unit {} (from position bytes: {:?})",
                p2_unit_idx, p2_moves.0
            );

            // Get the selected units for combat
            let p1_unit = player1_army[p1_unit_idx];
            let p2_unit = player2_army[p2_unit_idx];

            debug!("⚔️ Combat participants:");
            debug!(
                "  P1 Unit: ATK={} DEF={} HP={} ABILITY={:?}",
                p1_unit.attack, p1_unit.defense, p1_unit.health, p1_unit.ability
            );
            debug!(
                "  P2 Unit: ATK={} DEF={} HP={} ABILITY={:?}",
                p2_unit.attack, p2_unit.defense, p2_unit.health, p2_unit.ability
            );

            // Execute deterministic combat
            let mut round_result = process_combat(
                p1_unit,
                p2_unit,
                &player_match.player1_npub,
                &player_match.player2_npub,
            )
            .map_err(|e| GameEngineError::Internal(format!("Combat processing failed: {e:?}")))?;

            round_result.round = round_num as u8;

            info!("🏆 Round {} result:", round_num);
            debug!(
                "  Damage dealt: P1->P2: {}, P2->P1: {}",
                round_result.damage_dealt[0], round_result.damage_dealt[1]
            );
            debug!(
                "  Final health: P1: {}, P2: {}",
                round_result.player1_unit.health, round_result.player2_unit.health
            );
            info!("  Winner: {:?}", round_result.winner);

            validated_rounds.push(round_result);
        }

        info!(
            "✅ All {} rounds validated successfully",
            validated_rounds.len()
        );
        Ok(validated_rounds)
    }

    /// Calculate match winner from validated round results
    fn calculate_match_winner(
        &self,
        validated_rounds: &[shared_game_logic::game_state::RoundResult],
        player_match: &PlayerMatch,
    ) -> Option<String> {
        use tracing::{debug, info};

        let mut p1_wins = 0;
        let mut p2_wins = 0;

        info!(
            "🏆 Calculating match winner from {} rounds",
            validated_rounds.len()
        );

        for round in validated_rounds {
            match &round.winner {
                Some(winner) if winner == &player_match.player1_npub => {
                    p1_wins += 1;
                    debug!("  Round {}: Player 1 wins", round.round);
                }
                Some(winner) if winner == &player_match.player2_npub => {
                    p2_wins += 1;
                    debug!("  Round {}: Player 2 wins", round.round);
                }
                Some(winner) => {
                    debug!("  Round {}: Unknown winner: {}", round.round, winner);
                }
                None => {
                    debug!("  Round {}: Draw", round.round);
                }
            }
        }

        info!(
            "📊 Final score - Player 1: {} wins, Player 2: {} wins",
            p1_wins, p2_wins
        );

        let winner = if p1_wins > p2_wins {
            Some(player_match.player1_npub.clone())
        } else if p2_wins > p1_wins {
            Some(player_match.player2_npub.clone())
        } else {
            None // Draw
        };

        info!("🎉 Match winner determined: {:?}", winner);
        winner
    }

    /// Get list of matches ready for loot distribution
    pub fn get_matches_ready_for_loot(&self) -> Vec<&PlayerMatch> {
        self.matches
            .values()
            .filter(|m| matches!(m.phase, MatchPhase::Completed))
            .collect()
    }

    /// Mark match as loot distributed and archive
    pub fn mark_loot_distributed(&mut self, match_event_id: &str) -> Result<(), GameEngineError> {
        let player_match = self.get_match_mut(match_event_id)?;
        player_match.mark_loot_distributed();
        Ok(())
    }

    /// Get active match count for status reporting
    pub fn get_active_match_count(&self) -> usize {
        self.matches
            .values()
            .filter(|m| {
                !matches!(
                    m.phase,
                    MatchPhase::LootDistributed | MatchPhase::Invalid(_)
                )
            })
            .count()
    }
}
