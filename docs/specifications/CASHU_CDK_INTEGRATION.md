# Cashu CDK Integration Requirements

## Overview
Integration plan for modifying `cashubtc/cdk` to support mana/loot tokens instead of Bitcoin, with stubbed Lightning integration for local development.

## CDK Architecture Analysis

### Key Crates to Modify
- **`cdk`** - Core protocol logic, token structures
- **`cdk-mint-rpc`** - Mint API endpoints  
- **`cdk-sqlite`** - Storage for token state
- **`cdk-fake-wallet`** - Perfect for stubbed Lightning integration
- **`cdk-axum`** - Web server framework (keep as-is)

### Token Structure Modifications

#### Current CDK Token Structure
```rust
// Standard Cashu token (Bitcoin-based)
pub struct Token {
    pub mint_url: String,
    pub proofs: Vec<Proof>,
}

pub struct Proof {
    pub amount: u64,        // Bitcoin amount
    pub secret: String,
    pub C: String,          // Blinded signature  
    pub id: String,         // Keyset ID
}
```

#### Modified Token Structures for Game
```rust
// Mana token (gameplay currency)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManaToken {
    pub mint_url: String,
    pub league_id: u8,           // 0-15 for league-specific keys
    pub secret: [u8; 32],        // 32-byte secret for unit generation
    pub signature: String,       // Blind signature  
    pub keyset_id: String,       // League keyset ID
    pub amount: u64,             // Always 1 mana per token
}

// Loot token (reward currency)  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootToken {
    pub mint_url: String,
    pub locked_to_npub: String,  // Nostr public key lock
    pub amount: u64,             // Loot amount
    pub secret: [u8; 32],        
    pub signature: String,       // Locked signature
    pub keyset_id: String,       // Loot keyset ID
}
```

## Required CDK Modifications

### 1. Custom Token Types (cdk crate)
```rust
// src/types.rs - New token enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameToken {
    Mana(ManaToken),
    Loot(LootToken),
}

// Token economics constants
pub const MANA_PER_SAT: u64 = 5;
pub const FEE_PERCENTAGE: f64 = 0.05;
pub const MANA_AMOUNT_PER_TOKEN: u64 = 1; // Each mana token = 1 mana
```

### 2. Modified Mint API (cdk-mint-rpc crate)
```rust
// src/mint_api.rs - Game-specific endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct MintManaRequest {
    pub sats_paid: u64,
    pub league_id: u8,          // 0-15 league selection
    pub blinded_messages: Vec<BlindedMessage>,
}

#[derive(Debug, Serialize, Deserialize)]  
pub struct MintManaResponse {
    pub mana_tokens: Vec<ManaToken>,
    pub fee_collected: u64,     // 5% fee in mana equivalent
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLootRequest {
    pub winner_npub: String,
    pub amount: u64,
    pub match_id: String,       // For audit trail
}
```

### 3. Stubbed Lightning Integration (cdk-fake-wallet)
```rust
// src/fake_lightning.rs - Auto-approve all payments for local testing
pub struct StubbedLightning {
    pub auto_approve: bool,
}

impl StubbedLightning {
    pub async fn create_invoice(&self, sats: u64) -> Result<Invoice, Error> {
        // Always create successful invoice for local testing
        Ok(Invoice {
            payment_hash: generate_fake_hash(),
            payment_request: format!("fake_invoice_{}", sats),
            amount_sats: sats,
            paid: true,  // Auto-approve
            settled_at: Some(Utc::now()),
        })
    }
    
    pub async fn check_payment(&self, payment_hash: &str) -> Result<PaymentStatus, Error> {
        // Always return paid for testing
        Ok(PaymentStatus::Paid)
    }
}
```

### 4. League Keyset Management (cdk crate)
```rust
// src/league_keysets.rs - 16 league-specific keysets
pub struct LeagueKeysets {
    pub keysets: HashMap<u8, Keyset>, // 0-15 league IDs
}

impl LeagueKeysets {
    pub fn new() -> Self {
        let mut keysets = HashMap::new();
        
        // Generate 16 league-specific keysets
        for league_id in 0..16 {
            let keyset = Keyset::generate_league_specific(league_id);
            keysets.insert(league_id, keyset);
        }
        
        Self { keysets }
    }
    
    pub fn get_league_keyset(&self, league_id: u8) -> Option<&Keyset> {
        self.keysets.get(&league_id)
    }
}
```

### 5. Unit Generation Integration
```rust
// src/unit_generation.rs - Convert token secret to game units
pub fn generate_units_from_token(token: &ManaToken) -> [Unit; 8] {
    let mut units = [Unit::default(); 8];
    
    // Use token secret as deterministic seed
    for (i, chunk) in token.secret.chunks(4).enumerate() {
        if i >= 8 { break; } // Only 8 units per token
        
        units[i] = Unit {
            attack: chunk[0],
            defense: chunk[1], 
            health: chunk[2],
            ability: match chunk[3] % 4 {
                0 => Ability::None,
                1 => Ability::Boost,
                2 => Ability::Shield,
                3 => Ability::Heal,
                _ => Ability::None,
            }
        };
    }
    
    // Apply league modifiers
    apply_league_modifiers(&mut units, token.league_id);
    
    units
}
```

## Database Schema Modifications (cdk-sqlite)

### Additional Tables
```sql
-- Game-specific token tracking
CREATE TABLE mana_tokens (
    secret_hash TEXT PRIMARY KEY,
    league_id INTEGER NOT NULL,
    player_npub TEXT,
    created_at INTEGER NOT NULL,
    consumed_in_match TEXT  -- match_id when used
);

CREATE TABLE loot_tokens (
    secret_hash TEXT PRIMARY KEY,
    locked_to_npub TEXT NOT NULL,
    amount INTEGER NOT NULL,
    match_id TEXT,
    claimed BOOLEAN DEFAULT FALSE,
    created_at INTEGER NOT NULL
);

-- Fee tracking for loot pool
CREATE TABLE fee_pool (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount_collected INTEGER NOT NULL,
    collected_at INTEGER NOT NULL,
    distributed BOOLEAN DEFAULT FALSE
);
```

## API Endpoints for Game Integration

### Mint Endpoints (:3333)
```
POST /mint/mana
  - Request: { sats_paid: u64, league_id: u8, blinded_messages: [...] }
  - Response: { mana_tokens: [...], fee_collected: u64 }

POST /mint/loot  
  - Request: { winner_npub: string, amount: u64, match_id: string }
  - Response: { loot_token: LootToken }
  - Auth: Game engine bot only

GET /keysets/league/{league_id}
  - Response: League-specific keyset for client verification

POST /loot/claim
  - Request: { loot_token: LootToken, signature: string }
  - Response: { success: bool, melted_sats: u64 }

GET /stats/fee-pool
  - Response: { total_collected: u64, available_for_distribution: u64 }
```

## Configuration for Local Development

### mint.conf
```toml
[server]
host = "127.0.0.1"
port = 3333

[lightning]
backend = "fake"
auto_approve = true

[game]
mana_per_sat = 5
fee_percentage = 0.05
max_leagues = 16

[database]
type = "sqlite"
path = "./mint_data.db"
```

## Integration Points with Game Engine

### Token Creation Flow
1. **Game Engine** requests loot token creation via `/mint/loot`
2. **Mint** creates locked loot token using winner's npub
3. **Mint** publishes reward via Nostr (game engine handles this)

### Unit Generation Flow  
1. **Player** purchases mana via web client
2. **Mint** returns mana token with 32-byte secret
3. **Game Engine** calls unit generation function when needed
4. **Units** derived deterministically from token secret + league modifiers

## Development Priority

### Phase 1 - Core Modifications
1. Fork `cashubtc/cdk` repository
2. Implement custom token structures
3. Modify mint APIs for mana/loot
4. Set up stubbed Lightning backend

### Phase 2 - Game Integration
1. Add unit generation functions  
2. Implement league keyset management
3. Create game-specific API endpoints
4. Add database schema for tracking

### Phase 3 - Testing & Integration
1. Test with fake Lightning backend
2. Integrate with game engine bot
3. Validate token economics (5 mana per sat, 5% fee)
4. End-to-end testing with web client

This specification provides everything needed to implement D1 (Cashu Mint daemon) using the CDK as a foundation.