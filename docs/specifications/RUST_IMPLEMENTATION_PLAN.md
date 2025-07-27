# Rust Implementation Technical Specifications

## Technology Stack Overview

### Backend Services (Rust)
- **Web Framework:** `axum` for REST APIs and WebSocket handling
- **Database:** `sqlx` with PostgreSQL for type-safe database access
- **Async Runtime:** `tokio` for async/await concurrency
- **Cryptography:** `secp256k1`, `sha2` crates (Cashu provides VRF)
- **Cashu Integration:** Use existing Cashu Rust libraries
- **Nostr Integration:** `nostr-sdk` crate for protocol implementation
- **Serialization:** `serde` with JSON for data formats

### Frontend (Web/Mobile)
- **Web:** React with TypeScript
- **Mobile:** React Native or native iOS/Android
- **State Management:** Redux Toolkit
- **WebSocket Client:** Native WebSocket API

## Rust Crate Dependencies

### Core Dependencies
```toml
[dependencies]
# Web framework and HTTP
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
tower-http = "0.5"

# Database and persistence
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }
uuid = "1.0"
chrono = { version = "0.4", features = ["serde"] }

# Cashu integration (use existing Rust implementation)
cashu = "0.1"  # or appropriate Cashu Rust crate

# Nostr integration
nostr-sdk = "0.29"

# Basic cryptography (supplementary to Cashu)
secp256k1 = "0.28"
sha2 = "0.10"

# Serialization and utilities
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Service Architecture in Rust

### 1. Game Engine Service (`game-engine/`)

**Main Components:**
```rust
// src/units.rs
pub fn parse_unit_from_token(token: &CashuToken, league_id: LeagueId) -> [Unit; 8] {
    // Parse 32-byte token into 8 units (4 bytes each)
    // Apply league modifiers (+10 attack, +20 health, etc.)
}

// src/combat.rs  
pub fn resolve_combat(unit_a: &Unit, unit_b: &Unit) -> CombatResult {
    // damage = max(0, attack - defense)
    // Apply abilities (Boost, Shield, Heal)
}

// src/match_engine.rs
pub struct Match {
    pub players: [Player; 2],
    pub rounds: Vec<Round>,
    pub state: MatchState,
}
```

### 2. Tournament System (`tournament-system/`)

**Main Components:**
```rust
// src/swiss.rs
pub fn generate_swiss_pairings(
    players: &[Player], 
    round: u32,
    previous_pairings: &[Pairing]
) -> Vec<Pairing> {
    // Swiss system: pair players with similar records
    // Avoid repeat pairings when possible
}

// src/scoring.rs
pub fn calculate_standings(results: &[MatchResult]) -> Vec<PlayerStanding> {
    // 10 points per match win
    // Betting accuracy bonus: 0.5 * accuracy_percentage  
    // Tiebreakers: opponent win%, total damage dealt
}
```

### 3. API Gateway with Cashu/Nostr Integration (`api-gateway/`)

**Main Components:**
```rust
// src/cashu_service.rs
use cashu::{Mint, Token};

pub struct CashuService {
    mint: Mint,
}

impl CashuService {
    pub async fn mint_mana_token(&self, payment: LightningPayment, league: LeagueId) -> Result<Token, CashuError> {
        // Handle Lightning payment (5% fee)
        // Mint token using Cashu library
        // Token deterministically generates 32-byte unit set
    }
    
    pub async fn create_loot_token(&self, winner_npub: &str, amount: u64) -> Result<LockedToken, CashuError> {
        // Create locked token using Cashu's NUT-11 implementation
    }
}

// src/nostr_service.rs
use nostr_sdk::{Client, Event, EventBuilder};

pub struct NostrService {
    client: Client,
}

impl NostrService {
    pub async fn publish_match_announcement(&self, match_id: &str, players: [&str; 2]) -> Result<(), NostrError> {
        // Publish match creation event
    }
    
    pub async fn handle_commitment(&self, event: Event) -> Result<Commitment, NostrError> {
        // Process unit commitment events
    }
    
    pub async fn handle_reveal(&self, event: Event) -> Result<Reveal, NostrError> {
        // Process unit reveal events
        // Verify commitment matches reveal
    }
}
```

## Required Code References

The main areas where we need specific implementation guidance:

### 1. Cashu Rust Integration
- Which Cashu Rust crate/library to use
- How to integrate Lightning payments with the mint
- Token parsing for unit generation (32-byte → 8 units)

### 2. Swiss Tournament Algorithm
- Rust implementation reference for Swiss pairing
- Handling edge cases (byes, dropouts, tiebreakers)

Everything else (Nostr integration, web APIs, database access, game mechanics) has well-established Rust patterns and libraries.

## Implementation Priority

### Phase 2 Core Development:
1. **Game Engine** - Combat resolution and unit parsing (straightforward)
2. **Cashu Integration** - Need library reference  
3. **Nostr Client** - Use `nostr-sdk` (well documented)
4. **API Gateway** - Standard `axum` patterns
5. **Database** - Standard `sqlx` patterns

### Phase 3 Tournament System:
1. **Swiss Pairing** - Need algorithm reference
2. **Scoring System** - Straightforward implementation
3. **Tournament API** - Standard REST patterns

The Rust ecosystem provides excellent libraries for web services, databases, and cryptography. The main integration points needing specific guidance are:
- Cashu library usage and Lightning integration
- Swiss tournament pairing algorithm implementation