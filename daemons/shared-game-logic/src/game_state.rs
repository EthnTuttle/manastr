use serde::{Deserialize, Serialize};

/// A battle unit with stats and special ability
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub attack: u8,
    pub defense: u8,
    pub health: u8,
    pub max_health: u8,
    pub ability: Ability,
}

/// Special abilities that units can have
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Ability {
    None,
    Boost,  // Double attack this round
    Shield, // Negate damage this round
    Heal,   // Restore 50% max health post-combat
}

/// Result of a combat round between two units
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub round: u8,
    pub player1_unit: Unit,
    pub player2_unit: Unit,
    pub damage_dealt: [u8; 2], // [damage to unit2, damage to unit1]
    pub winner: Option<String>,
}

/// Error type for game logic operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameLogicError {
    InvalidInput(String),
    CombatError(String),
    SerializationError(String),
}

impl std::fmt::Display for GameLogicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameLogicError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            GameLogicError::CombatError(msg) => write!(f, "Combat error: {msg}"),
            GameLogicError::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
        }
    }
}

impl std::error::Error for GameLogicError {}

// Rust methods for Unit (not WASM exported)
impl Unit {
    pub fn new(attack: u8, defense: u8, health: u8, max_health: u8, ability: Ability) -> Unit {
        Unit {
            attack,
            defense,
            health,
            max_health,
            ability,
        }
    }

    /// Check if unit is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Apply damage to the unit
    pub fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }

    /// Heal the unit by specified amount (capped at max_health)
    pub fn heal(&mut self, amount: u8) {
        self.health = (self.health + amount).min(self.max_health);
    }
}

// WASM-specific methods for RoundResult
impl RoundResult {
    pub fn new(
        round: u8,
        player1_unit: Unit,
        player2_unit: Unit,
        damage_dealt: [u8; 2],
        winner: Option<String>,
    ) -> RoundResult {
        RoundResult {
            round,
            player1_unit,
            player2_unit,
            damage_dealt,
            winner,
        }
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            attack: 10,
            defense: 5,
            health: 25,
            max_health: 25,
            ability: Ability::None,
        }
    }
}
