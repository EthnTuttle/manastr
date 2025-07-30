use nostr::{Event, EventBuilder, Keys, Kind, Tag};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, error, info};

/// Player-driven match events for commitment/reveal scheme
/// These events are published by players, not the game engine
/// Game engine only validates and publishes loot distribution
///
// Custom Nostr event kinds for Manastr
pub const KIND_MATCH_CHALLENGE: Kind = Kind::Custom(31000);
pub const KIND_MATCH_ACCEPTANCE: Kind = Kind::Custom(31001);
pub const KIND_TOKEN_REVEAL: Kind = Kind::Custom(31002);
pub const KIND_MOVE_COMMITMENT: Kind = Kind::Custom(31003);
pub const KIND_MOVE_REVEAL: Kind = Kind::Custom(31004);
pub const KIND_MATCH_RESULT: Kind = Kind::Custom(31005);
pub const KIND_LOOT_DISTRIBUTION: Kind = Kind::Custom(31006);

/// Match challenge created by Player 1
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchChallenge {
    pub challenger_npub: String, // Serialized as string for JSON, but should be PublicKey
    pub wager_amount: u64,
    pub league_id: u8,
    pub cashu_token_commitment: String, // hash(cashu_token_secrets)
    pub army_commitment: String,        // hash(army_data + nonce)
    pub expires_at: u64,                // Unix timestamp
    pub created_at: u64,
    pub match_event_id: String, // EventId as hex string for JSON serialization
}

/// Match acceptance by Player 2
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchAcceptance {
    pub acceptor_npub: String,
    pub match_event_id: String,         // References the challenge EventId
    pub cashu_token_commitment: String, // Player 2's token commitment
    pub army_commitment: String,        // Player 2's army commitment
    pub accepted_at: u64,
}

/// Token revelation by both players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenReveal {
    pub player_npub: String,
    pub match_event_id: String,      // References the challenge EventId
    pub cashu_tokens: Vec<String>,   // Actual Cashu token secrets
    pub token_secrets_nonce: String, // Nonce used in commitment
    pub revealed_at: u64,
}

/// Move commitment for a specific round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCommitment {
    pub player_npub: String,
    pub match_event_id: String, // References the challenge EventId
    pub round_number: u32,
    pub move_commitment: String, // hash(unit_positions + unit_abilities + nonce)
    pub committed_at: u64,
}

/// Move revelation for a specific round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveReveal {
    pub player_npub: String,
    pub match_event_id: String, // References the challenge EventId
    pub round_number: u32,
    pub unit_positions: Vec<u8>,     // Positions of units for this round
    pub unit_abilities: Vec<String>, // Abilities used this round
    pub moves_nonce: String,         // Nonce from commitment
    pub revealed_at: u64,
}

/// Final match result published by both players
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchResult {
    pub player_npub: String,
    pub match_event_id: String,        // References the challenge EventId
    pub final_army_state: Value,       // Final state of all units
    pub all_round_results: Vec<Value>, // Results from all combat rounds
    pub calculated_winner: Option<String>, // Winner npub or None for draw
    pub match_completed_at: u64,
}

/// Loot distribution by Game Engine Bot (ONLY authoritative event from bot)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LootDistribution {
    pub game_engine_npub: String,
    pub match_event_id: String,      // References the challenge EventId
    pub winner_npub: Option<String>, // None for draw
    pub loot_cashu_token: Option<String>, // Loot token for winner (None for draw)
    pub match_fee: u64,              // Fee taken (5% of wager)
    pub loot_issued_at: u64,
    pub validation_summary: ValidationSummary,
}

/// Summary of game engine validation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub commitments_valid: bool,
    pub combat_verified: bool,
    pub signatures_valid: bool,
    pub winner_confirmed: bool,
    pub error_details: Option<String>,
}

/// Player-driven match state machine
#[derive(Debug, Clone)]
pub struct PlayerMatch {
    pub match_event_id: String, // EventId as hex string for internal tracking
    pub phase: MatchPhase,
    pub player1_npub: String,
    pub player2_npub: String,
    pub wager_amount: u64,
    pub league_id: u8,

    // Commitment tracking
    pub player1_commitments: PlayerCommitments,
    pub player2_commitments: PlayerCommitments,

    // Revealed data
    pub player1_reveals: PlayerReveals,
    pub player2_reveals: PlayerReveals,

    // Match results
    pub round_results: Vec<Value>,
    pub final_winner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchPhase {
    Created,         // Challenge published
    Accepted,        // Challenge accepted
    TokensRevealed,  // Both players revealed tokens
    InProgress(u32), // Active combat (current round number)
    Completed,       // Match finished, waiting for loot
    LootDistributed, // Loot issued, match archived
    Invalid(String), // Match invalid (cheating detected, etc.)
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerCommitments {
    pub cashu_tokens: Option<String>,
    pub army: Option<String>,
    pub moves_by_round: HashMap<u32, String>, // round -> commitment
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayerReveals {
    pub cashu_tokens: Option<Vec<String>>,
    pub token_nonce: Option<String>,
    pub moves_by_round: HashMap<u32, (Vec<u8>, Vec<String>, String)>, // round -> (positions, abilities, nonce)
}

impl PlayerMatch {
    pub fn new(challenge: &MatchChallenge, match_event_id: String) -> Self {
        Self {
            match_event_id,
            phase: MatchPhase::Created,
            player1_npub: challenge.challenger_npub.clone(),
            player2_npub: String::new(), // Set when accepted
            wager_amount: challenge.wager_amount,
            league_id: challenge.league_id,
            player1_commitments: PlayerCommitments {
                cashu_tokens: Some(challenge.cashu_token_commitment.clone()),
                army: Some(challenge.army_commitment.clone()),
                moves_by_round: HashMap::new(),
            },
            player2_commitments: PlayerCommitments::default(),
            player1_reveals: PlayerReveals::default(),
            player2_reveals: PlayerReveals::default(),
            round_results: Vec::new(),
            final_winner: None,
        }
    }

    pub fn accept(&mut self, acceptance: &MatchAcceptance) -> Result<(), String> {
        if !matches!(self.phase, MatchPhase::Created) {
            return Err("Match not in created state".to_string());
        }

        self.player2_npub = acceptance.acceptor_npub.clone();
        self.player2_commitments.cashu_tokens = Some(acceptance.cashu_token_commitment.clone());
        self.player2_commitments.army = Some(acceptance.army_commitment.clone());
        self.phase = MatchPhase::Accepted;

        info!(
            "Match {} accepted by {}",
            self.match_event_id, self.player2_npub
        );
        Ok(())
    }

    pub fn add_token_reveal(&mut self, reveal: &TokenReveal) -> Result<(), String> {
        if !matches!(
            self.phase,
            MatchPhase::Accepted | MatchPhase::TokensRevealed
        ) {
            return Err("Match not ready for token reveals".to_string());
        }

        if reveal.player_npub == self.player1_npub {
            self.player1_reveals.cashu_tokens = Some(reveal.cashu_tokens.clone());
            self.player1_reveals.token_nonce = Some(reveal.token_secrets_nonce.clone());
        } else if reveal.player_npub == self.player2_npub {
            self.player2_reveals.cashu_tokens = Some(reveal.cashu_tokens.clone());
            self.player2_reveals.token_nonce = Some(reveal.token_secrets_nonce.clone());
        } else {
            return Err("Token reveal from unknown player".to_string());
        }

        // Check if both players have revealed
        if self.player1_reveals.cashu_tokens.is_some()
            && self.player2_reveals.cashu_tokens.is_some()
        {
            self.phase = MatchPhase::TokensRevealed;
            info!(
                "Match {} - both players revealed tokens",
                self.match_event_id
            );
        }

        Ok(())
    }

    pub fn add_move_commitment(&mut self, commitment: &MoveCommitment) -> Result<(), String> {
        let round = commitment.round_number;

        if commitment.player_npub == self.player1_npub {
            self.player1_commitments
                .moves_by_round
                .insert(round, commitment.move_commitment.clone());
        } else if commitment.player_npub == self.player2_npub {
            self.player2_commitments
                .moves_by_round
                .insert(round, commitment.move_commitment.clone());
        } else {
            return Err("Move commitment from unknown player".to_string());
        }

        debug!(
            "Added move commitment for round {} from {}",
            round, commitment.player_npub
        );
        Ok(())
    }

    pub fn add_move_reveal(&mut self, reveal: &MoveReveal) -> Result<(), String> {
        let round = reveal.round_number;

        if reveal.player_npub == self.player1_npub {
            self.player1_reveals.moves_by_round.insert(
                round,
                (
                    reveal.unit_positions.clone(),
                    reveal.unit_abilities.clone(),
                    reveal.moves_nonce.clone(),
                ),
            );
        } else if reveal.player_npub == self.player2_npub {
            self.player2_reveals.moves_by_round.insert(
                round,
                (
                    reveal.unit_positions.clone(),
                    reveal.unit_abilities.clone(),
                    reveal.moves_nonce.clone(),
                ),
            );
        } else {
            return Err("Move reveal from unknown player".to_string());
        }

        debug!(
            "Added move reveal for round {} from {}",
            round, reveal.player_npub
        );
        Ok(())
    }

    pub fn both_players_committed_round(&self, round: u32) -> bool {
        self.player1_commitments.moves_by_round.contains_key(&round)
            && self.player2_commitments.moves_by_round.contains_key(&round)
    }

    pub fn both_players_revealed_round(&self, round: u32) -> bool {
        self.player1_reveals.moves_by_round.contains_key(&round)
            && self.player2_reveals.moves_by_round.contains_key(&round)
    }

    pub fn is_ready_for_combat(&self) -> bool {
        matches!(self.phase, MatchPhase::TokensRevealed)
            && self.player1_reveals.cashu_tokens.is_some()
            && self.player2_reveals.cashu_tokens.is_some()
    }

    pub fn set_final_result(&mut self, winner: Option<String>) {
        self.final_winner = winner;
        self.phase = MatchPhase::Completed;
        info!(
            "Match {} completed - winner: {:?}",
            self.match_event_id, self.final_winner
        );
    }

    pub fn mark_loot_distributed(&mut self) {
        self.phase = MatchPhase::LootDistributed;
        info!(
            "Match {} - loot distributed, match archived",
            self.match_event_id
        );
    }

    pub fn mark_invalid(&mut self, reason: String) {
        self.phase = MatchPhase::Invalid(reason.clone());
        error!("Match {} marked invalid: {}", self.match_event_id, reason);
    }
}

/// Helper functions for creating Nostr events
impl MatchChallenge {
    pub fn to_nostr_event(&self, keys: &Keys) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let tags = vec![
            Tag::custom(
                nostr::TagKind::Custom("d".into()),
                vec![self.challenger_npub.clone()],
            ),
            Tag::custom(
                nostr::TagKind::Custom("wager".into()),
                vec![self.wager_amount.to_string()],
            ),
            Tag::custom(
                nostr::TagKind::Custom("league".into()),
                vec![self.league_id.to_string()],
            ),
            Tag::custom(
                nostr::TagKind::Custom("expires".into()),
                vec![self.expires_at.to_string()],
            ),
        ];

        let event = EventBuilder::new(KIND_MATCH_CHALLENGE, content, tags).to_event(keys)?;
        Ok(event)
    }
}

impl MatchAcceptance {
    pub fn to_nostr_event(
        &self,
        keys: &Keys,
        challenge_event_id: &str,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let tags = vec![
            Tag::event(nostr::EventId::from_hex(challenge_event_id)?),
            Tag::custom(
                nostr::TagKind::Custom("wager".into()),
                vec!["100".to_string()],
            ), // TODO: Use actual wager
        ];

        let event = EventBuilder::new(KIND_MATCH_ACCEPTANCE, content, tags).to_event(keys)?;
        Ok(event)
    }
}

impl TokenReveal {
    pub fn to_nostr_event(
        &self,
        keys: &Keys,
        match_event_id: &str,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let tags = vec![
            Tag::event(nostr::EventId::from_hex(match_event_id)?),
            Tag::custom(
                nostr::TagKind::Custom("phase".into()),
                vec!["token_reveal".to_string()],
            ),
        ];

        let event = EventBuilder::new(KIND_TOKEN_REVEAL, content, tags).to_event(keys)?;
        Ok(event)
    }
}

impl MoveCommitment {
    pub fn to_nostr_event(
        &self,
        keys: &Keys,
        match_event_id: &str,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let tags = vec![
            Tag::event(nostr::EventId::from_hex(match_event_id)?),
            Tag::custom(
                nostr::TagKind::Custom("round".into()),
                vec![self.round_number.to_string()],
            ),
            Tag::custom(
                nostr::TagKind::Custom("phase".into()),
                vec!["move_commit".to_string()],
            ),
        ];

        let event = EventBuilder::new(KIND_MOVE_COMMITMENT, content, tags).to_event(keys)?;
        Ok(event)
    }
}

impl MoveReveal {
    pub fn to_nostr_event(
        &self,
        keys: &Keys,
        match_event_id: &str,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let tags = vec![
            Tag::event(nostr::EventId::from_hex(match_event_id)?),
            Tag::custom(
                nostr::TagKind::Custom("round".into()),
                vec![self.round_number.to_string()],
            ),
            Tag::custom(
                nostr::TagKind::Custom("phase".into()),
                vec!["move_reveal".to_string()],
            ),
        ];

        let event = EventBuilder::new(KIND_MOVE_REVEAL, content, tags).to_event(keys)?;
        Ok(event)
    }
}

impl MatchResult {
    pub fn to_nostr_event(
        &self,
        keys: &Keys,
        match_event_id: &str,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let winner_tag = self
            .calculated_winner
            .as_ref()
            .unwrap_or(&"draw".to_string())
            .clone();

        let tags = vec![
            Tag::event(nostr::EventId::from_hex(match_event_id)?),
            Tag::custom(nostr::TagKind::Custom("winner".into()), vec![winner_tag]),
            Tag::custom(
                nostr::TagKind::Custom("phase".into()),
                vec!["match_complete".to_string()],
            ),
        ];

        let event = EventBuilder::new(KIND_MATCH_RESULT, content, tags).to_event(keys)?;
        Ok(event)
    }
}

impl LootDistribution {
    /// Calculate optimized loot amount (95% of total wager, 5% system fee)
    pub fn calculate_optimized_loot_amount(&self) -> u64 {
        // Get total mana wagered from both players
        let total_wager = self.total_mana_wagered();

        // Return 95% to winner as loot tokens
        (total_wager * 95) / 100
    }

    /// Get total mana wagered by both players  
    pub fn total_mana_wagered(&self) -> u64 {
        // For current implementation, wager_amount represents per-player wager
        // So total is wager_amount * 2 (both players)
        // TODO: Get actual wager amounts from match data
        200 // Placeholder - should be calculated from actual match data
    }

    pub fn to_nostr_event(
        &self,
        keys: &Keys,
        match_event_id: &str,
    ) -> Result<Event, Box<dyn std::error::Error>> {
        let content = serde_json::to_string(self)?;
        let winner_tag = self
            .winner_npub
            .as_ref()
            .unwrap_or(&"draw".to_string())
            .clone();

        let tags = vec![
            Tag::event(nostr::EventId::from_hex(match_event_id)?),
            Tag::custom(nostr::TagKind::Custom("winner".into()), vec![winner_tag]),
            Tag::custom(
                nostr::TagKind::Custom("loot_amount".into()),
                vec![self.calculate_optimized_loot_amount().to_string()],
            ),
            Tag::custom(
                nostr::TagKind::Custom("match_event_id".into()),
                vec![self.match_event_id.clone()],
            ),
        ];

        let event = EventBuilder::new(KIND_LOOT_DISTRIBUTION, content, tags).to_event(keys)?;
        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_match_creation_and_acceptance() {
        let challenge = MatchChallenge {
            challenger_npub: "npub1alice".to_string(),
            wager_amount: 100,
            league_id: 0,
            cashu_token_commitment: "commitment_hash_123".to_string(),
            army_commitment: "army_hash_456".to_string(),
            expires_at: 1690000000,
            created_at: 1689900000,
            match_event_id: "match_event_123".to_string(),
        };

        let match_id = "match_123".to_string();
        let mut player_match = PlayerMatch::new(&challenge, match_id.clone());

        assert_eq!(player_match.player1_npub, "npub1alice");
        assert!(matches!(player_match.phase, MatchPhase::Created));

        let acceptance = MatchAcceptance {
            acceptor_npub: "npub1bob".to_string(),
            match_event_id: match_id.clone(),
            cashu_token_commitment: "bob_token_commitment".to_string(),
            army_commitment: "bob_army_commitment".to_string(),
            accepted_at: 1689910000,
        };

        player_match.accept(&acceptance).unwrap();
        assert_eq!(player_match.player2_npub, "npub1bob");
        assert!(matches!(player_match.phase, MatchPhase::Accepted));
    }

    #[test]
    fn test_token_reveal_flow() {
        let challenge = MatchChallenge {
            challenger_npub: "npub1alice".to_string(),
            wager_amount: 100,
            league_id: 0,
            cashu_token_commitment: "alice_commitment".to_string(),
            army_commitment: "alice_army".to_string(),
            expires_at: 1690000000,
            created_at: 1689900000,
            match_event_id: "match_event_123".to_string(),
        };

        let mut player_match = PlayerMatch::new(&challenge, "match_123".to_string());

        let acceptance = MatchAcceptance {
            acceptor_npub: "npub1bob".to_string(),
            match_event_id: "match_123".to_string(),
            cashu_token_commitment: "bob_commitment".to_string(),
            army_commitment: "bob_army".to_string(),
            accepted_at: 1689910000,
        };
        player_match.accept(&acceptance).unwrap();

        // Alice reveals tokens
        let alice_reveal = TokenReveal {
            player_npub: "npub1alice".to_string(),
            match_event_id: "match_123".to_string(),
            cashu_tokens: vec!["token1".to_string(), "token2".to_string()],
            token_secrets_nonce: "alice_nonce".to_string(),
            revealed_at: 1689920000,
        };
        player_match.add_token_reveal(&alice_reveal).unwrap();

        // Still in Accepted phase (only one player revealed)
        assert!(matches!(player_match.phase, MatchPhase::Accepted));

        // Bob reveals tokens
        let bob_reveal = TokenReveal {
            player_npub: "npub1bob".to_string(),
            match_event_id: "match_123".to_string(),
            cashu_tokens: vec!["token3".to_string(), "token4".to_string()],
            token_secrets_nonce: "bob_nonce".to_string(),
            revealed_at: 1689930000,
        };
        player_match.add_token_reveal(&bob_reveal).unwrap();

        // Now both revealed - should be ready for combat
        assert!(matches!(player_match.phase, MatchPhase::TokensRevealed));
        assert!(player_match.is_ready_for_combat());
    }

    #[test]
    fn test_move_commitment_reveal_cycle() {
        let challenge = MatchChallenge {
            challenger_npub: "npub1alice".to_string(),
            wager_amount: 100,
            league_id: 0,
            cashu_token_commitment: "alice_commitment".to_string(),
            army_commitment: "alice_army".to_string(),
            expires_at: 1690000000,
            created_at: 1689900000,
            match_event_id: "match_event_123".to_string(),
        };

        let mut player_match = PlayerMatch::new(&challenge, "match_123".to_string());

        // Add move commitments for round 1
        let alice_commitment = MoveCommitment {
            player_npub: "npub1alice".to_string(),
            match_event_id: "match_123".to_string(),
            round_number: 1,
            move_commitment: "alice_move_commitment_r1".to_string(),
            committed_at: 1689940000,
        };
        player_match.add_move_commitment(&alice_commitment).unwrap();

        let bob_commitment = MoveCommitment {
            player_npub: "npub1bob".to_string(),
            match_event_id: "match_123".to_string(),
            round_number: 1,
            move_commitment: "bob_move_commitment_r1".to_string(),
            committed_at: 1689940001,
        };
        player_match.add_move_commitment(&bob_commitment).unwrap();

        assert!(player_match.both_players_committed_round(1));
        assert!(!player_match.both_players_revealed_round(1));

        // Add move reveals for round 1
        let alice_reveal = MoveReveal {
            player_npub: "npub1alice".to_string(),
            match_event_id: "match_123".to_string(),
            round_number: 1,
            unit_positions: vec![1, 2, 3],
            unit_abilities: vec!["boost".to_string()],
            moves_nonce: "alice_r1_nonce".to_string(),
            revealed_at: 1689950000,
        };
        player_match.add_move_reveal(&alice_reveal).unwrap();

        let bob_reveal = MoveReveal {
            player_npub: "npub1bob".to_string(),
            match_event_id: "match_123".to_string(),
            round_number: 1,
            unit_positions: vec![4, 5, 6],
            unit_abilities: vec!["shield".to_string()],
            moves_nonce: "bob_r1_nonce".to_string(),
            revealed_at: 1689950001,
        };
        player_match.add_move_reveal(&bob_reveal).unwrap();

        assert!(player_match.both_players_committed_round(1));
        assert!(player_match.both_players_revealed_round(1));
    }
}
