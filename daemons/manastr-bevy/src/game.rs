use bevy::prelude::*;
use crate::manastr_core::{GameState, ActiveMatch, MatchPhase, EconomicModel};
use crate::networking::{send_critical_action, CriticalMessage, RealtimeMessage};
use shared_game_logic::game_state::Unit;
use tracing::{info, warn};

// Type alias for army of 4 units
type Army = [Unit; 4];

/// Game Plugin - Professional game logic with ECS architecture
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, initialize_game_systems)
            .add_systems(Update, (
                handle_match_progression,
                update_game_animations,
                process_player_actions,
                validate_game_state,
            ))
            .add_event::<GameEvent>()
            .add_event::<PlayerAction>();
    }
}

/// Game events for ECS communication
#[derive(Event, Clone, Debug)]
pub enum GameEvent {
    MatchStarted { match_id: String },
    PhaseTransition { old_phase: MatchPhase, new_phase: MatchPhase },
    TokensMinted { player: String, amount: u64 },
    ArmyRevealed { player: String, army: Army },
    CombatRound { round: u32, results: CombatResults },
    MatchCompleted { winner: String, loot_awarded: u64 },
}

/// Player actions from UI input
#[derive(Event, Clone, Debug)]
pub enum PlayerAction {
    StartMatch,
    MintTokens { amount: u64 },
    CreateChallenge { wager: u64 },
    AcceptChallenge { match_id: String },
    RevealArmy { tokens: Vec<String> },
    SubmitMove { move_data: String },
    ClaimVictory { proof: String },
}

#[derive(Clone, Debug)]
pub struct CombatResults {
    pub alice_damage: u32,
    pub bob_damage: u32,
    pub round_winner: Option<String>,
    pub match_over: bool,
}

/// Game entities and components using Bevy ECS
#[derive(Component)]
pub struct Player {
    pub id: String,
    pub display_name: String,
    pub mana_balance: u64,
    pub loot_balance: u64,
}

#[derive(Component)]
pub struct Card {
    pub id: String,
    pub unit: Unit,
    pub position: Vec2,
    pub is_revealed: bool,
    pub owner: String,
}

#[derive(Component)]
pub struct AnimationState {
    pub current_animation: String,
    pub progress: f32,
    pub duration: f32,
}

#[derive(Component)]
pub struct Battlefield {
    pub alice_army: Option<Army>,
    pub bob_army: Option<Army>,
    pub current_round: u32,
    pub combat_state: CombatState,
}

#[derive(Clone, Debug)]
pub enum CombatState {
    Preparation,
    InProgress,
    RoundComplete,
    MatchComplete,
}

fn initialize_game_systems() {
    info!("🎮 Initializing professional game systems with Bevy ECS");
}

fn handle_match_progression(
    mut game_state: ResMut<GameState>,
    mut game_events: EventWriter<GameEvent>,
    economic_model: Res<EconomicModel>,
) {
    // Handle match state progression using revolutionary Manastr protocol
    
    if let Some(ref mut current_match) = game_state.current_match {
        match current_match.phase {
            MatchPhase::Minting => {
                // Players mint tokens with C value randomness
                info!("📍 Phase: Army Forging - Players minting tokens");
            }
            MatchPhase::Challenge => {
                // Alice creates challenge via Nostr event
                info!("📍 Phase: Challenge Creation");
                send_critical_action(CriticalMessage::ChallengeCreation {
                    challenge_id: current_match.match_id.clone(),
                });
            }
            MatchPhase::Acceptance => {
                // Bob accepts challenge via Nostr event
                info!("📍 Phase: Challenge Acceptance");
            }
            MatchPhase::ArmyReveal => {
                // Players reveal tokens for army generation
                info!("📍 Phase: Army Revelation");
            }
            MatchPhase::Combat(round) => {
                // Combat using shared deterministic logic
                info!("📍 Phase: Combat Round {}/3", round);
                execute_combat_round(current_match, round);
            }
            MatchPhase::Resolution => {
                // Calculate winner and loot distribution
                info!("📍 Phase: Match Resolution");
                let winner_loot = economic_model.calculate_winner_loot(current_match.total_wager);
                current_match.winner_loot = winner_loot;
            }
            MatchPhase::Complete => {
                // Match finished, distribute rewards
                info!("📍 Phase: Match Complete - Distributing {} LOOT", current_match.winner_loot);
                
                game_events.send(GameEvent::MatchCompleted {
                    winner: "Alice".to_string(), // Placeholder
                    loot_awarded: current_match.winner_loot,
                });
            }
        }
    }
}

fn execute_combat_round(active_match: &mut ActiveMatch, round: u32) {
    // Use shared game logic for deterministic combat
    // This ensures same results on all clients
    
    info!("⚔️ Executing combat round {} with shared deterministic logic", round);
    
    // Placeholder for actual combat execution
    // let results = shared_game_logic::combat::execute_round(alice_army, bob_army, round);
    
    // Send results via critical Nostr event (not WebRTC)
    send_critical_action(CriticalMessage::MatchResult {
        result_signature: format!("round_{}_results", round),
    });
}

fn update_game_animations(
    time: Res<Time>,
    mut query: Query<&mut AnimationState>,
) {
    // Update all game animations using Bevy's built-in animation system
    for mut animation in query.iter_mut() {
        animation.progress += time.delta_seconds() / animation.duration;
        
        if animation.progress >= 1.0 {
            animation.progress = 0.0;
            // Animation complete - could trigger events
        }
    }
}

fn process_player_actions(
    mut player_actions: EventReader<PlayerAction>,
    mut game_state: ResMut<GameState>,
    mut game_events: EventWriter<GameEvent>,
) {
    for action in player_actions.read() {
        match action {
            PlayerAction::StartMatch => {
                info!("🎯 Player action: Starting new match");
                
                // Create new match with proper economics
                let new_match = ActiveMatch {
                    match_id: generate_match_id(),
                    alice: "Alice".to_string(),
                    bob: "Bob".to_string(),
                    phase: MatchPhase::Minting,
                    total_wager: 200, // 100 MANA per player
                    winner_loot: 1000, // Will be calculated properly
                };
                
                game_state.current_match = Some(new_match.clone());
                
                game_events.send(GameEvent::MatchStarted {
                    match_id: new_match.match_id,
                });
            }
            
            PlayerAction::MintTokens { amount } => {
                info!("🪙 Player action: Minting {} tokens", amount);
                
                // This would integrate with gaming wallet to mint real tokens
                // let tokens = gaming_wallet.mint_gaming_tokens(*amount, "mana").await?;
                
                game_events.send(GameEvent::TokensMinted {
                    player: "Player".to_string(),
                    amount: *amount,
                });
            }
            
            PlayerAction::RevealArmy { tokens } => {
                info!("🔮 Player action: Revealing army with {} tokens", tokens.len());
                
                // Generate army from token C values using shared logic
                // let army = shared_game_logic::combat::generate_army_from_tokens(tokens);
                
                // Send via critical Nostr event
                send_critical_action(CriticalMessage::TokenReveal {
                    token_data: serde_json::to_string(tokens).unwrap_or_default(),
                });
            }
            
            _ => {
                // Handle other player actions
            }
        }
    }
}

fn validate_game_state(game_state: Res<GameState>) {
    // Continuous validation of game state integrity
    // Ensure all game rules are enforced
    // Validate economic constraints
    
    if let Some(ref current_match) = game_state.current_match {
        // Validate match state
        if current_match.total_wager < 200 {
            warn!("⚠️ Invalid match: Total wager below minimum (200 MANA)");
        }
        
        // Validate economic model: 100 MANA = 1000 LOOT
        let expected_loot = (current_match.total_wager * 10 * 95) / 100; // 95% to winner
        if current_match.winner_loot != expected_loot && current_match.phase == MatchPhase::Resolution {
            warn!("⚠️ Invalid economics: Expected {} LOOT, got {}", expected_loot, current_match.winner_loot);
        }
    }
}

fn generate_match_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("match_{}", timestamp)
}