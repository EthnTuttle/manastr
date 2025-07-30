use crate::game_state::{Ability, Unit};

/// Apply pre-combat abilities (like Boost)
pub fn apply_pre_combat(unit1: &mut Unit, unit2: &mut Unit) {
    // Apply Boost ability - double attack for this round
    if unit1.ability == Ability::Boost {
        unit1.attack = unit1.attack.saturating_mul(2);
    }

    if unit2.ability == Ability::Boost {
        unit2.attack = unit2.attack.saturating_mul(2);
    }
}

/// Apply post-combat abilities (like Heal)
pub fn apply_post_combat(unit1: &mut Unit, unit2: &mut Unit) {
    // Apply Heal ability - restore 50% max health if still alive
    if unit1.ability == Ability::Heal && unit1.is_alive() {
        let heal_amount = (unit1.max_health / 2).max(1);
        unit1.heal(heal_amount);
    }

    if unit2.ability == Ability::Heal && unit2.is_alive() {
        let heal_amount = (unit2.max_health / 2).max(1);
        unit2.heal(heal_amount);
    }
}

/// Get ability description for UI display
pub fn get_ability_description(ability: Ability) -> &'static str {
    match ability {
        Ability::None => "No special ability",
        Ability::Boost => "Double attack damage this round",
        Ability::Shield => "Negate all damage this round",
        Ability::Heal => "Restore 50% max health after combat",
    }
}

/// Get ability display name
pub fn get_ability_name(ability: Ability) -> &'static str {
    match ability {
        Ability::None => "None",
        Ability::Boost => "Boost",
        Ability::Shield => "Shield",
        Ability::Heal => "Heal",
    }
}

/// Check if ability affects combat damage calculation
pub fn affects_damage_calculation(ability: Ability) -> bool {
    matches!(ability, Ability::Boost | Ability::Shield)
}

/// Check if ability provides post-combat effects
pub fn has_post_combat_effect(ability: Ability) -> bool {
    matches!(ability, Ability::Heal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boost_doubles_attack() {
        let mut unit1 = Unit {
            attack: 10,
            defense: 5,
            health: 20,
            max_health: 20,
            ability: Ability::Boost,
        };

        let mut unit2 = Unit {
            attack: 8,
            defense: 3,
            health: 15,
            max_health: 15,
            ability: Ability::None,
        };

        apply_pre_combat(&mut unit1, &mut unit2);

        assert_eq!(unit1.attack, 20); // Doubled
        assert_eq!(unit2.attack, 8); // Unchanged
    }

    #[test]
    fn test_heal_restores_health() {
        let mut unit1 = Unit {
            attack: 10,
            defense: 5,
            health: 10, // Damaged
            max_health: 40,
            ability: Ability::Heal,
        };

        let mut unit2 = Unit {
            attack: 8,
            defense: 3,
            health: 5, // Damaged
            max_health: 20,
            ability: Ability::None,
        };

        apply_post_combat(&mut unit1, &mut unit2);

        // Unit1 heals 50% of max_health = 20, so 10+20=30
        assert_eq!(unit1.health, 30);
        // Unit2 doesn't heal
        assert_eq!(unit2.health, 5);
    }

    #[test]
    fn test_heal_caps_at_max_health() {
        let mut unit = Unit {
            attack: 10,
            defense: 5,
            health: 35, // Close to max
            max_health: 40,
            ability: Ability::Heal,
        };

        let mut dummy = Unit::default();
        apply_post_combat(&mut unit, &mut dummy);

        // Should be capped at max_health
        assert_eq!(unit.health, 40);
    }

    #[test]
    fn test_dead_units_dont_heal() {
        let mut unit = Unit {
            attack: 10,
            defense: 5,
            health: 0, // Dead
            max_health: 40,
            ability: Ability::Heal,
        };

        let mut dummy = Unit::default();
        apply_post_combat(&mut unit, &mut dummy);

        // Dead units don't heal
        assert_eq!(unit.health, 0);
    }

    #[test]
    fn test_ability_descriptions() {
        assert_eq!(get_ability_name(Ability::None), "None");
        assert_eq!(get_ability_name(Ability::Boost), "Boost");
        assert_eq!(get_ability_name(Ability::Shield), "Shield");
        assert_eq!(get_ability_name(Ability::Heal), "Heal");

        assert!(get_ability_description(Ability::Boost).contains("Double attack"));
        assert!(get_ability_description(Ability::Shield).contains("Negate"));
        assert!(get_ability_description(Ability::Heal).contains("Restore"));
    }
}
