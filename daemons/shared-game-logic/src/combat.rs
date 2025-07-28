use crate::game_state::{Unit, Ability, RoundResult, GameLogicError};
use crate::abilities;
use crate::league;
use sha2::{Digest, Sha256};

/// Generate battle units from mana token secret (deterministic)
/// This logic is identical on both client and server for perfect synchronization
pub fn generate_units_from_token_secret(token_secret: &str, league_id: u8) -> [Unit; 8] {
    // Hash the token secret to get deterministic randomness
    let mut hasher = Sha256::new();
    hasher.update(token_secret.as_bytes());
    let hash = hasher.finalize();
    
    let mut units = [Unit::default(); 8];

    // Create 8 units from the 32-byte hash (4 bytes per unit)
    for (i, chunk) in hash.chunks(4).enumerate().take(8) {
        let base_attack = chunk[0] % 20 + 10; // 10-29 base attack
        let base_defense = chunk[1] % 15 + 5; // 5-19 base defense  
        let base_health = chunk[2] % 30 + 20; // 20-49 base health
        let ability_byte = chunk[3];
        
        // Create base unit
        let mut unit = Unit {
            attack: base_attack,
            defense: base_defense,
            health: base_health,
            max_health: base_health,
            ability: ability_from_byte(ability_byte),
        };
        
        // Apply league modifiers
        league::apply_modifiers(&mut unit, league_id);
        
        units[i] = unit;
    }
    
    units
}

/// Process combat between two units using identical server logic
pub fn process_combat(
    mut unit1: Unit,
    mut unit2: Unit,
    player1_npub: &str,
    player2_npub: &str,
) -> Result<RoundResult, GameLogicError> {
    // Store original units for result
    let _original_unit1 = unit1;
    let _original_unit2 = unit2;

    // Apply pre-combat abilities
    abilities::apply_pre_combat(&mut unit1, &mut unit2);

    // Calculate damage (attack - defense, minimum 0)
    let damage_to_unit2 = if unit2.ability == Ability::Shield {
        0 // Shield negates all damage
    } else {
        unit1.attack.saturating_sub(unit2.defense)
    };

    let damage_to_unit1 = if unit1.ability == Ability::Shield {
        0 // Shield negates all damage
    } else {
        unit2.attack.saturating_sub(unit1.defense)
    };

    // Apply damage
    unit1.take_damage(damage_to_unit1);
    unit2.take_damage(damage_to_unit2);

    // Apply post-combat abilities (healing)
    abilities::apply_post_combat(&mut unit1, &mut unit2);

    // Determine winner
    let winner = determine_round_winner(&unit1, &unit2, player1_npub, player2_npub);

    Ok(RoundResult {
        round: 0, // Will be set by caller
        player1_unit: unit1,
        player2_unit: unit2,
        damage_dealt: [damage_to_unit2, damage_to_unit1],
        winner,
    })
}

/// Determine the winner of a combat round
fn determine_round_winner(
    unit1: &Unit,
    unit2: &Unit,
    player1_npub: &str,
    player2_npub: &str,
) -> Option<String> {
    match (unit1.is_alive(), unit2.is_alive()) {
        (true, false) => Some(player1_npub.to_string()),
        (false, true) => Some(player2_npub.to_string()),
        (true, true) => {
            // Both alive, higher health wins
            if unit1.health > unit2.health {
                Some(player1_npub.to_string())
            } else if unit2.health > unit1.health {
                Some(player2_npub.to_string())
            } else {
                None // Tie
            }
        },
        (false, false) => None, // Both dead, tie
    }
}

/// Convert a byte to an ability (deterministic)
fn ability_from_byte(byte: u8) -> Ability {
    match byte % 4 {
        1 => Ability::Boost,
        2 => Ability::Shield,
        3 => Ability::Heal,
        _ => Ability::None,
    }
}

/// Simulate multiple rounds of combat for testing
pub fn simulate_match(
    units1: &[Unit; 8],
    units2: &[Unit; 8],
    player1_npub: &str,
    player2_npub: &str,
) -> Result<Vec<RoundResult>, GameLogicError> {
    let mut results = Vec::new();
    
    // Best of 5 rounds (first to win 3 rounds)
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    
    for round in 0..5 {
        if player1_wins >= 3 || player2_wins >= 3 {
            break; // Match already decided
        }
        
        // Use units based on round (cycle through available units)
        let unit1 = units1[round % 8];
        let unit2 = units2[round % 8];
        
        let mut result = process_combat(unit1, unit2, player1_npub, player2_npub)?;
        result.round = round as u8 + 1;
        
        // Count wins
        if let Some(ref winner) = result.winner {
            if winner == player1_npub {
                player1_wins += 1;
            } else if winner == player2_npub {
                player2_wins += 1;
            }
        }
        
        results.push(result);
    }
    
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_unit_generation() {
        let secret = "test_token_secret_123";
        let league_id = 0;
        
        let units1 = generate_units_from_token_secret(secret, league_id);
        let units2 = generate_units_from_token_secret(secret, league_id);
        
        // Must be identical
        assert_eq!(units1, units2);
    }

    #[test]
    fn test_different_secrets_different_units() {
        let units1 = generate_units_from_token_secret("secret1", 0);
        let units2 = generate_units_from_token_secret("secret2", 0);
        
        // Should be different
        assert_ne!(units1, units2);
    }

    #[test]
    fn test_combat_basic() {
        let unit1 = Unit {
            attack: 20,
            defense: 10,
            health: 50,
            max_health: 50,
            ability: Ability::None,
        };
        
        let unit2 = Unit {
            attack: 15,
            defense: 5,
            health: 40,
            max_health: 40,
            ability: Ability::None,
        };

        let result = process_combat(unit1, unit2, "player1", "player2").unwrap();
        
        // Unit1 deals 20-5=15 damage to unit2 (40-15=25 health)
        // Unit2 deals 15-10=5 damage to unit1 (50-5=45 health)
        assert_eq!(result.player1_unit.health, 45);
        assert_eq!(result.player2_unit.health, 25);
        assert_eq!(result.winner, Some("player1".to_string()));
    }

    #[test]
    fn test_combat_shield_ability() {
        let unit1 = Unit {
            attack: 20,
            defense: 10,
            health: 50,
            max_health: 50,
            ability: Ability::None,
        };
        
        let unit2 = Unit {
            attack: 15,
            defense: 5,
            health: 40,
            max_health: 40,
            ability: Ability::Shield,
        };

        let result = process_combat(unit1, unit2, "player1", "player2").unwrap();
        
        // Unit2 has shield, takes no damage
        // Unit1 takes 15-10=5 damage
        assert_eq!(result.player1_unit.health, 45);
        assert_eq!(result.player2_unit.health, 40); // No damage due to shield
    }

    #[test]
    fn test_combat_boost_ability() {
        let unit1 = Unit {
            attack: 10,
            defense: 5,
            health: 30,
            max_health: 30,
            ability: Ability::Boost,
        };
        
        let unit2 = Unit {
            attack: 10,
            defense: 5,
            health: 30,
            max_health: 30,
            ability: Ability::None,
        };

        let result = process_combat(unit1, unit2, "player1", "player2").unwrap();
        
        // Unit1 has boost (double attack): 20-5=15 damage to unit2
        // Unit2 deals 10-5=5 damage to unit1
        assert_eq!(result.player1_unit.health, 25);
        assert_eq!(result.player2_unit.health, 15);
        assert_eq!(result.winner, Some("player1".to_string()));
    }

    #[test]
    fn test_combat_heal_ability() {
        let unit1 = Unit {
            attack: 5,
            defense: 0,
            health: 20,
            max_health: 40,
            ability: Ability::Heal,
        };
        
        let unit2 = Unit {
            attack: 5,
            defense: 0,
            health: 20,
            max_health: 40,
            ability: Ability::None,
        };

        let result = process_combat(unit1, unit2, "player1", "player2").unwrap();
        
        // Both take 5 damage (20-5=15 health)
        // Unit1 heals 50% of max_health = 20 (15+20=35, capped at 40)
        assert_eq!(result.player1_unit.health, 35);
        assert_eq!(result.player2_unit.health, 15);
        assert_eq!(result.winner, Some("player1".to_string()));
    }
}