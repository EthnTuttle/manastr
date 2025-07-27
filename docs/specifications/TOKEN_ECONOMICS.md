# Token Economics - Final Specification

## Confirmed Economics Model

### Mana Token (Gameplay Currency)
- **Conversion Rate:** 5 mana per sat
- **Example:** 1000 sats → 5000 mana tokens
- **Fee:** 5% retained by system
- **Net to Player:** 1000 sats → 4750 mana (250 mana equivalent retained as fee)
- **Purpose:** Used to generate unit sets for matches
- **Non-meltable:** Cannot be converted back to sats

### Loot Token (Reward Currency)  
- **Source:** 5% fee from all mana purchases forms the loot pool
- **Example:** 1000 sat purchase → 50 sats worth goes to loot pool (250 loot tokens)
- **Distribution:** Periodic rewards to top leaderboard players
- **Locked:** Always locked to winner's Nostr public key (npub)
- **Meltable:** Can be redeemed for Lightning sats
- **Swappable:** Can be traded between players (but remains locked to original winner's npub for melting)

## Implementation Details

### Cashu Mint Configuration
```rust
// Token conversion rates
const MANA_PER_SAT: u64 = 5;
const FEE_PERCENTAGE: f64 = 0.05; // 5%

// Example mana purchase
fn calculate_mana_purchase(sats_paid: u64) -> (u64, u64) {
    let total_mana = sats_paid * MANA_PER_SAT;
    let fee_mana = (total_mana as f64 * FEE_PERCENTAGE) as u64;
    let player_mana = total_mana - fee_mana;
    
    (player_mana, fee_mana) // Returns (4750, 250) for 1000 sats
}
```

### Loot Pool Management
```rust
// Loot token creation from fees
fn create_loot_from_fees(fee_mana: u64) -> u64 {
    // Fee mana directly becomes loot tokens (1:1 ratio)
    fee_mana
}

// Periodic loot distribution (example: weekly)
fn distribute_weekly_loot(total_loot_pool: u64, top_players: Vec<Player>) -> Vec<LootReward> {
    // Example distribution:
    // 1st place: 40% of pool
    // 2nd place: 25% of pool  
    // 3rd place: 15% of pool
    // 4th-10th: 20% of pool split
}
```

### Unit Generation Economics
- **1 mana token = 1 unit set** (8 units from 32-byte token secret)
- **Match cost:** Players spend their chosen mana tokens (consumed in match)
- **Winner reward:** Comes from accumulated loot pool, not opponent's mana

### Fee Flow Example
```
Player pays 1000 sats
├── 95% → 4750 mana tokens (player receives)
└── 5% → 250 loot tokens (added to reward pool)

Weekly reward distribution:
├── Top 10 players split accumulated loot pool
└── Loot tokens can be melted to sats or traded
```

## Economic Balance

### Player Incentives
- **Play to Win:** Loot rewards incentivize competitive play
- **Skill Over Spending:** Better players earn more loot regardless of mana spending
- **Sustainable:** 5% fee funds ongoing rewards without depleting player value

### System Sustainability
- **Self-Funding:** Fee pool funds all rewards
- **No External Costs:** System doesn't need to inject additional value
- **Deflationary Pressure:** Mana consumed in matches, loot earned through skill

### Anti-Manipulation
- **Locked Loot:** Prevents easy value extraction
- **Skill-Based:** Rewards based on leaderboard performance
- **Time-Gated:** Periodic distribution prevents gaming short-term

This economic model creates a sustainable, competitive ecosystem where skill is rewarded and the system remains financially balanced.