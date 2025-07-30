use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

use crate::match_events::*;
use shared_game_logic::game_state::Unit;

/// State machine for tracking match progression through Nostr events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchState {
    /// Match challenge posted, waiting for acceptance
    Challenged {
        challenge: MatchChallenge,
        expires_at: DateTime<Utc>,
    },
    /// Challenge accepted, waiting for token reveals
    Accepted {
        challenge: MatchChallenge,
        acceptance: MatchAcceptance,
        player1_revealed: bool,
        player2_revealed: bool,
    },
    /// Both tokens revealed, combat rounds in progress
    InCombat {
        match_data: MatchData,
        current_round: u32,
        completed_rounds: Vec<u32>,
        player1_committed: Vec<u32>, // rounds where player1 committed
        player2_committed: Vec<u32>, // rounds where player2 committed
        player1_revealed: Vec<u32>,  // rounds where player1 revealed
        player2_revealed: Vec<u32>,  // rounds where player2 revealed
    },
    /// Match completed, waiting for validation and loot distribution
    AwaitingValidation {
        match_data: MatchData,
        result: MatchResult,
        submitted_at: DateTime<Utc>,
    },
    /// Match validated, loot distributed
    Completed {
        match_data: MatchData,
        result: MatchResult,
        loot_distribution: LootDistribution,
        completed_at: DateTime<Utc>,
    },
    /// Match invalid due to cheating or errors
    Invalid {
        reason: String,
        failed_at: DateTime<Utc>,
    },
}

/// Core match data that persists across states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchData {
    pub match_event_id: String,
    pub player1_npub: String,
    pub player2_npub: String,
    pub league_id: u32,
    pub wager_amount: u64,

    // Commitment/reveal data
    pub player1_commitments: PlayerCommitments,
    pub player2_commitments: PlayerCommitments,
    pub player1_reveals: PlayerReveals,
    pub player2_reveals: PlayerReveals,

    // Generated armies (cached after token reveal)
    pub player1_army: Option<[Unit; 8]>,
    pub player2_army: Option<[Unit; 8]>,
}

/// State machine transitions for match events
#[derive(Debug, Clone)]
pub enum MatchEvent {
    ChallengePosted(MatchChallenge),
    ChallengeAccepted(MatchAcceptance),
    TokenRevealed(TokenReveal),
    MoveCommitted(MoveCommitment),
    MoveRevealed(MoveReveal),
    ResultSubmitted(MatchResult),
    LootDistributed(LootDistribution),
    InvalidationTriggered(String), // reason
    TimeoutExpired,
}

/// Result of a state transition
#[derive(Debug)]
pub struct TransitionResult {
    pub new_state: MatchState,
    pub actions: Vec<GameEngineAction>,
    pub errors: Vec<String>,
}

/// Actions the game engine should take after state transitions
#[derive(Debug, Clone)]
pub enum GameEngineAction {
    ValidateTokenCommitment {
        match_id: String,
        player_npub: String,
    },
    ValidateMoveCommitment {
        match_id: String,
        player_npub: String,
        round: u32,
    },
    GenerateArmies {
        match_id: String,
    },
    ExecuteCombatRound {
        match_id: String,
        round: u32,
    },
    ValidateMatchResult {
        match_id: String,
    },
    DistributeLoot {
        match_id: String,
        winner_npub: Option<String>,
    },
    PublishLootEvent {
        match_id: String,
        loot_distribution: LootDistribution,
    },
    ArchiveMatch {
        match_id: String,
    },
    InvalidateMatch {
        match_id: String,
        reason: String,
    },
}

impl MatchState {
    /// Create initial challenge state
    pub fn new_challenge(challenge: MatchChallenge) -> Self {
        let expires_at = DateTime::from_timestamp(challenge.expires_at as i64, 0)
            .unwrap_or_else(|| Utc::now() + chrono::Duration::minutes(30));

        MatchState::Challenged {
            challenge,
            expires_at,
        }
    }

    /// Process a match event and return new state with actions
    pub fn transition(self, event: MatchEvent) -> TransitionResult {
        match (self, event) {
            // Challenge accepted - move to token reveal phase
            (
                MatchState::Challenged { challenge, .. },
                MatchEvent::ChallengeAccepted(acceptance),
            ) => {
                info!("🤝 Challenge accepted, waiting for token reveals");

                let _match_data = MatchData::new(&challenge, &acceptance);
                let new_state = MatchState::Accepted {
                    challenge,
                    acceptance,
                    player1_revealed: false,
                    player2_revealed: false,
                };

                TransitionResult {
                    new_state,
                    actions: vec![],
                    errors: vec![],
                }
            }

            // Token revealed in accepted state
            (
                MatchState::Accepted {
                    challenge,
                    acceptance,
                    mut player1_revealed,
                    mut player2_revealed,
                },
                MatchEvent::TokenRevealed(reveal),
            ) => {
                let mut actions = vec![GameEngineAction::ValidateTokenCommitment {
                    match_id: reveal.match_event_id.clone(),
                    player_npub: reveal.player_npub.clone(),
                }];

                // Update reveal status
                if reveal.player_npub == challenge.challenger_npub {
                    player1_revealed = true;
                } else if reveal.player_npub == acceptance.acceptor_npub {
                    player2_revealed = true;
                }

                // If both revealed, transition to combat
                if player1_revealed && player2_revealed {
                    info!("🎪 Both players revealed tokens, transitioning to combat");

                    let match_data = MatchData::new(&challenge, &acceptance);
                    let new_state = MatchState::InCombat {
                        match_data,
                        current_round: 1,
                        completed_rounds: vec![],
                        player1_committed: vec![],
                        player2_committed: vec![],
                        player1_revealed: vec![],
                        player2_revealed: vec![],
                    };

                    actions.push(GameEngineAction::GenerateArmies {
                        match_id: reveal.match_event_id.clone(),
                    });

                    TransitionResult {
                        new_state,
                        actions,
                        errors: vec![],
                    }
                } else {
                    let new_state = MatchState::Accepted {
                        challenge,
                        acceptance,
                        player1_revealed,
                        player2_revealed,
                    };

                    TransitionResult {
                        new_state,
                        actions,
                        errors: vec![],
                    }
                }
            }

            // Move committed during combat
            (
                MatchState::InCombat {
                    match_data,
                    current_round,
                    completed_rounds,
                    mut player1_committed,
                    mut player2_committed,
                    player1_revealed,
                    player2_revealed,
                },
                MatchEvent::MoveCommitted(commitment),
            ) => {
                let round = commitment.round_number;
                let actions = vec![GameEngineAction::ValidateMoveCommitment {
                    match_id: commitment.match_event_id.clone(),
                    player_npub: commitment.player_npub.clone(),
                    round,
                }];

                // Track commitment
                if commitment.player_npub == match_data.player1_npub {
                    if !player1_committed.contains(&round) {
                        player1_committed.push(round);
                    }
                } else if commitment.player_npub == match_data.player2_npub
                    && !player2_committed.contains(&round) {
                        player2_committed.push(round);
                    }

                let new_state = MatchState::InCombat {
                    match_data,
                    current_round,
                    completed_rounds,
                    player1_committed,
                    player2_committed,
                    player1_revealed,
                    player2_revealed,
                };

                TransitionResult {
                    new_state,
                    actions,
                    errors: vec![],
                }
            }

            // Move revealed during combat
            (
                MatchState::InCombat {
                    match_data,
                    current_round,
                    completed_rounds,
                    player1_committed,
                    player2_committed,
                    mut player1_revealed,
                    mut player2_revealed,
                },
                MatchEvent::MoveRevealed(reveal),
            ) => {
                let round = reveal.round_number;
                let mut actions = vec![];

                // Track reveal
                if reveal.player_npub == match_data.player1_npub {
                    if !player1_revealed.contains(&round) {
                        player1_revealed.push(round);
                    }
                } else if reveal.player_npub == match_data.player2_npub
                    && !player2_revealed.contains(&round) {
                        player2_revealed.push(round);
                    }

                // Check if round is complete (both players revealed)
                if player1_revealed.contains(&round) && player2_revealed.contains(&round) {
                    actions.push(GameEngineAction::ExecuteCombatRound {
                        match_id: reveal.match_event_id.clone(),
                        round,
                    });
                }

                let new_state = MatchState::InCombat {
                    match_data,
                    current_round,
                    completed_rounds,
                    player1_committed,
                    player2_committed,
                    player1_revealed,
                    player2_revealed,
                };

                TransitionResult {
                    new_state,
                    actions,
                    errors: vec![],
                }
            }

            // Match result submitted
            (MatchState::InCombat { match_data, .. }, MatchEvent::ResultSubmitted(result)) => {
                info!("🏁 Match result submitted, transitioning to validation");

                let new_state = MatchState::AwaitingValidation {
                    match_data,
                    result: result.clone(),
                    submitted_at: Utc::now(),
                };

                let actions = vec![GameEngineAction::ValidateMatchResult {
                    match_id: result.match_event_id.clone(),
                }];

                TransitionResult {
                    new_state,
                    actions,
                    errors: vec![],
                }
            }

            // Loot distributed - final state
            (
                MatchState::AwaitingValidation {
                    match_data, result, ..
                },
                MatchEvent::LootDistributed(loot_distribution),
            ) => {
                info!("🏆 Loot distributed, match completed");

                let match_id = loot_distribution.match_event_id.clone();
                let loot_distribution_clone = loot_distribution.clone();

                let new_state = MatchState::Completed {
                    match_data,
                    result,
                    loot_distribution: loot_distribution_clone,
                    completed_at: Utc::now(),
                };

                let actions = vec![
                    GameEngineAction::PublishLootEvent {
                        match_id: match_id.clone(),
                        loot_distribution,
                    },
                    GameEngineAction::ArchiveMatch { match_id },
                ];

                TransitionResult {
                    new_state,
                    actions,
                    errors: vec![],
                }
            }

            // Invalidation at any point
            (state, MatchEvent::InvalidationTriggered(reason)) => {
                warn!("🚨 Match invalidated: {}", reason);

                let new_state = MatchState::Invalid {
                    reason: reason.clone(),
                    failed_at: Utc::now(),
                };

                let match_id = match state {
                    MatchState::Challenged { challenge, .. } => challenge.challenger_npub.clone(),
                    MatchState::Accepted { acceptance, .. } => acceptance.match_event_id.clone(),
                    MatchState::InCombat { match_data, .. } => match_data.match_event_id.clone(),
                    MatchState::AwaitingValidation { match_data, .. } => {
                        match_data.match_event_id.clone()
                    }
                    _ => "unknown".to_string(),
                };

                let actions = vec![GameEngineAction::InvalidateMatch { match_id, reason }];

                TransitionResult {
                    new_state,
                    actions,
                    errors: vec![],
                }
            }

            // Invalid transitions
            (state, event) => {
                let error_msg = format!("Invalid transition: {state:?} -> {event:?}");
                warn!("{}", error_msg);

                TransitionResult {
                    new_state: state,
                    actions: vec![],
                    errors: vec![error_msg],
                }
            }
        }
    }

    /// Check if match is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            MatchState::Completed { .. } | MatchState::Invalid { .. }
        )
    }

    /// Get match ID if available
    pub fn get_match_id(&self) -> Option<String> {
        match self {
            MatchState::Challenged { challenge, .. } => {
                Some(format!("challenge_{}", challenge.challenger_npub))
            }
            MatchState::Accepted { acceptance, .. } => Some(acceptance.match_event_id.clone()),
            MatchState::InCombat { match_data, .. } => Some(match_data.match_event_id.clone()),
            MatchState::AwaitingValidation { match_data, .. } => {
                Some(match_data.match_event_id.clone())
            }
            MatchState::Completed { match_data, .. } => Some(match_data.match_event_id.clone()),
            MatchState::Invalid { .. } => None,
        }
    }

    /// Get current phase as string for logging
    pub fn phase_name(&self) -> &str {
        match self {
            MatchState::Challenged { .. } => "Challenged",
            MatchState::Accepted { .. } => "Accepted",
            MatchState::InCombat { .. } => "InCombat",
            MatchState::AwaitingValidation { .. } => "AwaitingValidation",
            MatchState::Completed { .. } => "Completed",
            MatchState::Invalid { .. } => "Invalid",
        }
    }
}

impl MatchData {
    /// Create new match data from challenge and acceptance
    pub fn new(challenge: &MatchChallenge, acceptance: &MatchAcceptance) -> Self {
        Self {
            match_event_id: acceptance.match_event_id.clone(),
            player1_npub: challenge.challenger_npub.clone(),
            player2_npub: acceptance.acceptor_npub.clone(),
            league_id: challenge.league_id as u32,
            wager_amount: challenge.wager_amount,

            player1_commitments: PlayerCommitments {
                cashu_tokens: Some(challenge.cashu_token_commitment.clone()),
                army: Some(challenge.army_commitment.clone()),
                moves_by_round: HashMap::new(),
            },
            player2_commitments: PlayerCommitments {
                cashu_tokens: Some(acceptance.cashu_token_commitment.clone()),
                army: Some(acceptance.army_commitment.clone()),
                moves_by_round: HashMap::new(),
            },
            player1_reveals: PlayerReveals::default(),
            player2_reveals: PlayerReveals::default(),

            player1_army: None,
            player2_army: None,
        }
    }
}
