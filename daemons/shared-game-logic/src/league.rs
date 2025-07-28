use crate::game_state::Unit;
use serde::{Deserialize, Serialize};

/// League modifiers that affect unit stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueModifier {
    pub id: u8,
    pub name: &'static str,
    pub attack_bonus: i8,
    pub defense_bonus: i8,
    pub health_bonus: i8,
}

/// Apply league-specific modifiers to a unit
pub fn apply_modifiers(unit: &mut Unit, league_id: u8) {
    let modifier = get_league_modifier(league_id);
    
    // Apply bonuses (ensuring minimums)
    unit.attack = apply_stat_modifier(unit.attack, modifier.attack_bonus);
    unit.defense = apply_stat_modifier(unit.defense, modifier.defense_bonus);
    
    let new_max_health = apply_stat_modifier(unit.max_health, modifier.health_bonus);
    let health_increase = new_max_health.saturating_sub(unit.max_health);
    
    unit.max_health = new_max_health;
    unit.health = unit.health.saturating_add(health_increase); // Current health scales with max
}

/// Get league modifier configuration
pub fn get_league_modifier(league_id: u8) -> LeagueModifier {
    // Simplified league system - in full game would have 16 leagues
    match league_id % 4 {
        0 => LeagueModifier {
            id: 0,
            name: "Fire League",
            attack_bonus: 10,
            defense_bonus: 0,
            health_bonus: 0,
        },
        1 => LeagueModifier {
            id: 1,
            name: "Ice League", 
            attack_bonus: 0,
            defense_bonus: 0,
            health_bonus: 20,
        },
        2 => LeagueModifier {
            id: 2,
            name: "Shadow League",
            attack_bonus: 5,
            defense_bonus: 5,
            health_bonus: 0,
        },
        3 => LeagueModifier {
            id: 3,
            name: "Nature League",
            attack_bonus: 0,
            defense_bonus: 5,
            health_bonus: 15,
        },
        _ => LeagueModifier {
            id: league_id,
            name: "Unknown League",
            attack_bonus: 0,
            defense_bonus: 0,
            health_bonus: 0,
        }
    }
}

/// Apply a stat modifier with minimum bounds
fn apply_stat_modifier(base: u8, modifier: i8) -> u8 {
    let result = base as i8 + modifier;
    if result < 1 { 1 } else { result as u8 }
}

/// Get all available league modifiers
pub fn get_all_league_modifiers() -> Vec<LeagueModifier> {
    (0..4).map(get_league_modifier).collect()
}

/// Calculate effective power rating for a unit with league modifiers
pub fn calculate_power_rating(base_unit: &Unit, league_id: u8) -> u32 {
    let mut unit = *base_unit;
    apply_modifiers(&mut unit, league_id);
    
    // Simple power calculation: attack + defense + (health * 2)
    unit.attack as u32 + unit.defense as u32 + (unit.health as u32 * 2)
}

/// Get league display information for UI
pub fn get_league_display_info(league_id: u8) -> String {
    let modifier = get_league_modifier(league_id);
    let mut info = modifier.name.to_string();
    
    let mut bonuses = Vec::new();
    if modifier.attack_bonus > 0 {
        bonuses.push(format!("+{} ATK", modifier.attack_bonus));
    }
    if modifier.defense_bonus > 0 {
        bonuses.push(format!("+{} DEF", modifier.defense_bonus));
    }
    if modifier.health_bonus > 0 {
        bonuses.push(format!("+{} HP", modifier.health_bonus));
    }
    
    if !bonuses.is_empty() {
        info.push_str(&format!(" ({})", bonuses.join(", ")));
    }
    
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_league_modifiers() {
        let mut unit = Unit {
            attack: 15,
            defense: 10,
            health: 30,
            max_health: 30,
            ability: crate::game_state::Ability::None,
        };
        
        apply_modifiers(&mut unit, 0); // Fire League
        
        assert_eq!(unit.attack, 25); // +10 attack
        assert_eq!(unit.defense, 10); // No change
        assert_eq!(unit.health, 30); // No change
        assert_eq!(unit.max_health, 30); // No change
    }

    #[test]
    fn test_ice_league_modifiers() {
        let mut unit = Unit {
            attack: 15,
            defense: 10,
            health: 30,
            max_health: 30,
            ability: crate::game_state::Ability::None,
        };
        
        apply_modifiers(&mut unit, 1); // Ice League
        
        assert_eq!(unit.attack, 15); // No change
        assert_eq!(unit.defense, 10); // No change
        assert_eq!(unit.health, 50); // +20 health (scales current)
        assert_eq!(unit.max_health, 50); // +20 max health
    }

    #[test]
    fn test_shadow_league_modifiers() {
        let mut unit = Unit {
            attack: 15,
            defense: 10,
            health: 30,
            max_health: 30,
            ability: crate::game_state::Ability::None,
        };
        
        apply_modifiers(&mut unit, 2); // Shadow League
        
        assert_eq!(unit.attack, 20); // +5 attack
        assert_eq!(unit.defense, 15); // +5 defense
        assert_eq!(unit.health, 30); // No change
        assert_eq!(unit.max_health, 30); // No change
    }

    #[test]
    fn test_nature_league_modifiers() {
        let mut unit = Unit {
            attack: 15,
            defense: 10,
            health: 30,
            max_health: 30,
            ability: crate::game_state::Ability::None,
        };
        
        apply_modifiers(&mut unit, 3); // Nature League
        
        assert_eq!(unit.attack, 15); // No change
        assert_eq!(unit.defense, 15); // +5 defense
        assert_eq!(unit.health, 45); // +15 health (scales current)
        assert_eq!(unit.max_health, 45); // +15 max health
    }

    #[test]
    fn test_minimum_stat_bounds() {
        let mut unit = Unit {
            attack: 1,
            defense: 1,
            health: 1,
            max_health: 1,
            ability: crate::game_state::Ability::None,
        };
        
        // Apply negative modifiers (shouldn't happen in practice, but test bounds)
        unit.attack = apply_stat_modifier(unit.attack, -10);
        unit.defense = apply_stat_modifier(unit.defense, -10);
        
        assert_eq!(unit.attack, 1); // Minimum 1
        assert_eq!(unit.defense, 1); // Minimum 1
    }

    #[test]
    fn test_power_rating_calculation() {
        let base_unit = Unit {
            attack: 10,
            defense: 5,
            health: 20,
            max_health: 20,
            ability: crate::game_state::Ability::None,
        };
        
        // Fire League: +10 attack
        let fire_power = calculate_power_rating(&base_unit, 0);
        // 20 attack + 5 defense + (20 health * 2) = 65
        assert_eq!(fire_power, 65);
        
        // Ice League: +20 health
        let ice_power = calculate_power_rating(&base_unit, 1);
        // 10 attack + 5 defense + (40 health * 2) = 95
        assert_eq!(ice_power, 95);
    }

    #[test]
    fn test_league_display_info() {
        assert_eq!(get_league_display_info(0), "Fire League (+10 ATK)");
        assert_eq!(get_league_display_info(1), "Ice League (+20 HP)");
        assert_eq!(get_league_display_info(2), "Shadow League (+5 ATK, +5 DEF)");
        assert_eq!(get_league_display_info(3), "Nature League (+5 DEF, +15 HP)");
    }

    #[test]
    fn test_all_league_modifiers() {
        let modifiers = get_all_league_modifiers();
        assert_eq!(modifiers.len(), 4);
        
        assert_eq!(modifiers[0].name, "Fire League");
        assert_eq!(modifiers[1].name, "Ice League");
        assert_eq!(modifiers[2].name, "Shadow League");
        assert_eq!(modifiers[3].name, "Nature League");
    }
}