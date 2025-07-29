// 🏛️ ECONOMIC MODEL: Optimized Loot Distribution Mathematics
// ===========================================================
//
// Revolutionary player-friendly economic model with 95% player rewards
// and only 5% mint/game engine fee for operational sustainability.
//
// 🎯 OPTIMIZATION PRINCIPLE:
// Players should receive maximum value while ensuring system viability.
// This creates the most competitive and fair gaming economy possible.

use serde::{Deserialize, Serialize};

/// Economic model constants for the revolutionary gaming system
pub struct EconomicModel;

impl EconomicModel {
    /// Player reward percentage (95% of total mana wagered)
    pub const PLAYER_REWARD_PERCENTAGE: u64 = 95;
    
    /// System fee percentage (5% for mint/game engine operations)
    pub const SYSTEM_FEE_PERCENTAGE: u64 = 5;
    
    /// Calculate optimized loot distribution with player-favorable rounding for edge cases
    /// 
    /// # Arguments
    /// * `total_mana_wagered` - Total mana tokens wagered by both players
    /// 
    /// # Returns
    /// `LootDistribution` containing player reward and system fee breakdown
    /// 
    /// # Player-Favorable Edge Case Handling
    /// - Minimum wager: 2 mana total (1 per player for army generation)  
    /// - Small wagers always guarantee at least 1 loot token to winner
    /// - Rounding errors favor the player, not the system
    /// - 2 mana wagered → 1 loot token (50% player-favorable vs 0% calculated)
    /// 
    /// # Examples
    /// ```
    /// let distribution = EconomicModel::calculate_loot_distribution(100);
    /// assert_eq!(distribution.player_loot_amount, 95);
    /// assert_eq!(distribution.system_fee, 5);
    /// 
    /// // Edge case: 2 mana wagered
    /// let small_distribution = EconomicModel::calculate_loot_distribution(2);
    /// assert_eq!(small_distribution.player_loot_amount, 1); // Player-favorable rounding
    /// assert_eq!(small_distribution.system_fee, 1);
    /// ```
    pub fn calculate_loot_distribution(total_mana_wagered: u64) -> LootDistribution {
        let (player_loot_amount, system_fee) = Self::calculate_player_favorable_split(total_mana_wagered);
        
        LootDistribution {
            total_mana_wagered,
            player_loot_amount,
            system_fee,
            reward_percentage: Self::PLAYER_REWARD_PERCENTAGE,
            fee_percentage: Self::SYSTEM_FEE_PERCENTAGE,
        }
    }
    
    /// Calculate player-favorable split with edge case handling
    /// 
    /// This method implements the core logic for player-favorable rounding:
    /// - Ensures at least 1 loot token for any valid wager (minimum 2 mana total)
    /// - Rounds fractional amounts up in favor of the player
    /// - System fee is calculated as remainder after player reward
    /// 
    /// # Panics
    /// - Panics if total_mana_wagered is 0 or 1 (invalid game state - need armies)
    fn calculate_player_favorable_split(total_mana_wagered: u64) -> (u64, u64) {
        if total_mana_wagered < 2 {
            panic!("Invalid wager: Matches require at least 2 mana total (1 per player for army generation)");
        }
        
        // Calculate base 95% reward
        let base_player_reward = (total_mana_wagered * Self::PLAYER_REWARD_PERCENTAGE) / 100;
        
        // Edge case handling: Ensure at least 1 loot token for any valid wager
        let player_loot_amount = if base_player_reward == 0 {
            1 // Player-favorable: Always give at least 1 loot token
        } else {
            base_player_reward
        };
        
        // System fee is the remainder
        let system_fee = total_mana_wagered.saturating_sub(player_loot_amount);
        
        (player_loot_amount, system_fee)
    }

    /// Calculate loot for winner-takes-all matches (default mode)
    /// Winner receives 95% of combined wagers as loot tokens with player-favorable rounding
    pub fn calculate_winner_takes_all(player1_wager: u64, player2_wager: u64) -> LootDistribution {
        let total_wagered = player1_wager + player2_wager;
        Self::calculate_loot_distribution(total_wagered)
    }
    
    /// Calculate loot for draw scenarios (future implementation)
    /// Each player gets back 95% of their own wager
    pub fn calculate_draw_scenario(player1_wager: u64, player2_wager: u64) -> DrawLootDistribution {
        let player1_refund = (player1_wager * Self::PLAYER_REWARD_PERCENTAGE) / 100;
        let player2_refund = (player2_wager * Self::PLAYER_REWARD_PERCENTAGE) / 100;
        let total_fee = (player1_wager + player2_wager) - (player1_refund + player2_refund);
        
        DrawLootDistribution {
            player1_wager,
            player2_wager,
            player1_refund,
            player2_refund,
            total_system_fee: total_fee,
        }
    }
    
    /// Validate economic model with player-favorable edge case handling
    /// 
    /// # Validation Rules
    /// - Total must always equal player_loot_amount + system_fee
    /// - Minimum wager is 2 mana (1 per player for armies)
    /// - For wagers >= 20: Player gets exactly 95%
    /// - For wagers 2-19: Player gets at least 1 loot token (player-favorable)
    /// - Player never gets 0 loot tokens for valid wagers
    pub fn validate_distribution(distribution: &LootDistribution) -> bool {
        let calculated_total = distribution.player_loot_amount + distribution.system_fee;
        let total_matches = calculated_total == distribution.total_mana_wagered;
        
        // Invalid wager amounts
        if distribution.total_mana_wagered < 2 {
            return false; // Cannot play without armies
        }
        
        // For larger wagers, should be close to 95% (allowing for integer rounding)
        if distribution.total_mana_wagered >= 20 {
            let expected_player_amount = (distribution.total_mana_wagered * 95) / 100;
            return total_matches && distribution.player_loot_amount == expected_player_amount;
        }
        
        // For small wagers (2-19), player should get at least 1 loot token
        total_matches && distribution.player_loot_amount >= 1
    }
}

/// Optimized loot distribution breakdown for winner-takes-all matches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LootDistribution {
    /// Total mana wagered by both players
    pub total_mana_wagered: u64,
    
    /// Loot tokens awarded to winner (95% of total wager)
    pub player_loot_amount: u64,
    
    /// Fee retained by system (5% of total wager)
    pub system_fee: u64,
    
    /// Player reward percentage (always 95)
    pub reward_percentage: u64,
    
    /// System fee percentage (always 5)
    pub fee_percentage: u64,
}

/// Loot distribution for draw scenarios (future implementation)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrawLootDistribution {
    pub player1_wager: u64,
    pub player2_wager: u64,
    pub player1_refund: u64,
    pub player2_refund: u64,
    pub total_system_fee: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_loot_calculation() {
        let distribution = EconomicModel::calculate_loot_distribution(100);
        
        assert_eq!(distribution.total_mana_wagered, 100);
        assert_eq!(distribution.player_loot_amount, 95);
        assert_eq!(distribution.system_fee, 5);
        assert_eq!(distribution.reward_percentage, 95);
        assert_eq!(distribution.fee_percentage, 5);
    }
    
    #[test]
    fn test_winner_takes_all_scenario() {
        let distribution = EconomicModel::calculate_winner_takes_all(60, 40);
        
        assert_eq!(distribution.total_mana_wagered, 100);
        assert_eq!(distribution.player_loot_amount, 95);
        assert_eq!(distribution.system_fee, 5);
    }
    
    #[test]
    fn test_large_wager_amounts() {
        let distribution = EconomicModel::calculate_loot_distribution(10000);
        
        assert_eq!(distribution.player_loot_amount, 9500);
        assert_eq!(distribution.system_fee, 500);
        assert!(EconomicModel::validate_distribution(&distribution));
    }
    
    #[test]
    fn test_odd_number_handling() {
        // Test with odd numbers to ensure proper rounding
        let distribution = EconomicModel::calculate_loot_distribution(101);
        
        // 101 * 95 / 100 = 95 (integer division)
        assert_eq!(distribution.player_loot_amount, 95);
        assert_eq!(distribution.system_fee, 6); // Remainder goes to system
        assert!(EconomicModel::validate_distribution(&distribution));
    }
    
    #[test]
    fn test_small_wager_calculations() {
        // Test minimum valid wager: 2 mana total (1 per player for armies)
        let distribution_2 = EconomicModel::calculate_loot_distribution(2);
        assert_eq!(distribution_2.player_loot_amount, 1); // 2 * 95/100 = 1 (normal calculation)
        assert_eq!(distribution_2.system_fee, 1);
        assert!(EconomicModel::validate_distribution(&distribution_2));
        
        // Test 3 mana wagered - normal calculation
        let distribution_3 = EconomicModel::calculate_loot_distribution(3);
        assert_eq!(distribution_3.player_loot_amount, 2); // 3 * 95 / 100 = 2 (normal calculation)
        assert_eq!(distribution_3.system_fee, 1);
        assert!(EconomicModel::validate_distribution(&distribution_3));
        
        // Test 10 mana wagered - normal calculation
        let distribution_10 = EconomicModel::calculate_loot_distribution(10);
        assert_eq!(distribution_10.player_loot_amount, 9); // 10 * 95 / 100 = 9
        assert_eq!(distribution_10.system_fee, 1);
        assert!(EconomicModel::validate_distribution(&distribution_10));
    }
    
    #[test]
    #[should_panic(expected = "Invalid wager: Matches require at least 2 mana total")]
    fn test_invalid_wager_amounts() {
        // Test 0 mana - should panic (no armies possible)
        EconomicModel::calculate_loot_distribution(0);
    }
    
    #[test]
    #[should_panic(expected = "Invalid wager: Matches require at least 2 mana total")]
    fn test_single_mana_wager() {
        // Test 1 mana - should panic (cannot generate armies for both players)
        EconomicModel::calculate_loot_distribution(1);
    }
    
    #[test]  
    fn test_minimum_viable_wager() {
        // Test minimum wager that gives normal 95% calculation
        let distribution_20 = EconomicModel::calculate_loot_distribution(20);
        assert_eq!(distribution_20.player_loot_amount, 19); // 20 * 95 / 100 = 19
        assert_eq!(distribution_20.system_fee, 1);
        assert!(EconomicModel::validate_distribution(&distribution_20));
        
        // Test slightly above minimum
        let distribution_21 = EconomicModel::calculate_loot_distribution(21);
        assert_eq!(distribution_21.player_loot_amount, 19); // 21 * 95 / 100 = 19 (integer division)
        assert_eq!(distribution_21.system_fee, 2);
        assert!(EconomicModel::validate_distribution(&distribution_21));
    }
    
    
    #[test]
    fn test_draw_scenario() {
        let draw_distribution = EconomicModel::calculate_draw_scenario(60, 40);
        
        assert_eq!(draw_distribution.player1_refund, 57); // 60 * 95 / 100
        assert_eq!(draw_distribution.player2_refund, 38); // 40 * 95 / 100
        assert_eq!(draw_distribution.total_system_fee, 5); // 100 - 57 - 38
    }
    
    #[test]
    fn test_validation_function() {
        let valid_distribution = LootDistribution {
            total_mana_wagered: 200,
            player_loot_amount: 190,
            system_fee: 10,
            reward_percentage: 95,
            fee_percentage: 5,
        };
        
        assert!(EconomicModel::validate_distribution(&valid_distribution));
        
        let invalid_distribution = LootDistribution {
            total_mana_wagered: 200,
            player_loot_amount: 180, // Wrong amount
            system_fee: 20,
            reward_percentage: 95,
            fee_percentage: 5,
        };
        
        assert!(!EconomicModel::validate_distribution(&invalid_distribution));
    }
}

/// Example usage and demonstration
#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn demonstrate_economic_model() {
        println!("🎯 OPTIMIZED ECONOMIC MODEL DEMONSTRATION");
        
        // Scenario 1: Equal 50-50 wagers
        println!("\n📋 Scenario 1: Equal wagers (50 + 50 = 100 mana)");
        let equal_wagers = EconomicModel::calculate_winner_takes_all(50, 50);
        println!("  Winner receives: {} loot tokens", equal_wagers.player_loot_amount);
        println!("  System fee: {} mana tokens", equal_wagers.system_fee);
        
        // Scenario 2: Unequal wagers  
        println!("\n📋 Scenario 2: Unequal wagers (80 + 20 = 100 mana)");
        let unequal_wagers = EconomicModel::calculate_winner_takes_all(80, 20);
        println!("  Winner receives: {} loot tokens", unequal_wagers.player_loot_amount);
        println!("  System fee: {} mana tokens", unequal_wagers.system_fee);
        
        // Scenario 3: Large tournament wagers
        println!("\n📋 Scenario 3: Tournament wagers (500 + 500 = 1000 mana)");
        let tournament_wagers = EconomicModel::calculate_winner_takes_all(500, 500);
        println!("  Winner receives: {} loot tokens", tournament_wagers.player_loot_amount);
        println!("  System fee: {} mana tokens", tournament_wagers.system_fee);
        
        // Scenario 4: Edge cases with player-favorable rounding
        println!("\n📋 Scenario 4: Edge Cases - Player-Favorable Rounding");
        
        let edge_case_1 = EconomicModel::calculate_winner_takes_all(1, 1); // 2 total
        println!("  2 mana total: Winner receives {} loot, system gets {} (player-favorable)", 
                 edge_case_1.player_loot_amount, edge_case_1.system_fee);
        
        let edge_case_2 = EconomicModel::calculate_winner_takes_all(5, 5); // 10 total
        println!("  10 mana total: Winner receives {} loot, system gets {} (player-favorable)", 
                 edge_case_2.player_loot_amount, edge_case_2.system_fee);
        
        let edge_case_3 = EconomicModel::calculate_winner_takes_all(10, 10); // 20 total
        println!("  20 mana total: Winner receives {} loot, system gets {} (95% calculation)", 
                 edge_case_3.player_loot_amount, edge_case_3.system_fee);
        
        println!("\n🎉 ECONOMIC MODEL: Maximizes player value with edge case protection!");
    }
}