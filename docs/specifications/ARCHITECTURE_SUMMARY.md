# Mana Strategy Game - Architecture Summary

## Core Understanding

**Cashu provides both:**
1. **Lightning Payment Processing** - Handles the Lightning Network integration and 5% fee collection
2. **VRF (Verifiable Random Function)** - Deterministic unit generation via token secrets

This significantly simplifies our implementation since we don't need separate Lightning or VRF services.

## Simplified Service Architecture

### Backend Services (Rust)

#### 1. Cashu Mint Integration (`cashu-mint/`)
- **Purpose:** Interface with existing Cashu mint for token operations
- **Key Functions:**
  - Request mana tokens (Cashu handles Lightning payment + 5% fee)
  - Parse token secrets into 32-byte unit sets (Cashu provides deterministic randomness)
  - Create locked loot tokens (using Cashu's NUT-11 implementation)
  - Token verification and validation

#### 2. Game Engine (`game-engine/`)
- **Purpose:** Pure game logic implementation
- **Key Functions:**
  - Parse 32-byte token → 8 units (4 bytes each: attack, defense, health, ability)
  - Apply league modifiers (+10 attack, +20 health, etc.)
  - Combat resolution (damage = max(0, attack - defense))
  - Ability processing (Boost, Shield, Heal)
  - Match state management

#### 3. Nostr Client (`nostr-client/`)
- **Purpose:** Asynchronous match coordination
- **Key Functions:**
  - Publish match announcements
  - Handle commitment/reveal events
  - Broadcast match results and rewards
  - Event validation and sequencing

#### 4. Tournament System (`tournament-system/`)
- **Purpose:** Swiss tournament management  
- **Key Functions:**
  - Swiss pairing algorithm
  - Scoring and standings calculation
  - Prize pool management
  - Tournament progression

#### 5. API Gateway (`api-gateway/`)
- **Purpose:** Orchestrate all services via REST API
- **Key Functions:**
  - Route requests to appropriate services
  - Coordinate match lifecycle
  - Handle authentication and rate limiting

### Frontend (Web/Mobile)
- **Web:** React with TypeScript
- **Mobile:** React Native
- **Purpose:** User interface for all game interactions

## Implementation Priorities

### Phase 1: Requirements & Design (2 weeks)
All specifications and mockups - **no code references needed**

### Phase 2: Core Development (8 weeks)
#### Definite Implementation (no references needed):
- Game Engine - Standard Rust game logic
- API Gateway - Standard `axum` web service  
- Database - Standard `sqlx` with PostgreSQL
- Frontend - Standard React/React Native

#### Need Code References:
1. **Cashu Integration** - Which Rust Cashu library and usage patterns
2. **Swiss Tournament Algorithm** - Rust implementation reference

### Phase 3: Tournament System (4 weeks)
- Mostly standard Rust implementation once Swiss algorithm reference is provided

## Code References Needed

### 1. Cashu Rust Library Integration
```rust
// Need guidance on:
// - Which Cashu Rust crate to use
// - How to interface with existing Cashu mint
// - Token parsing patterns for unit generation
use cashu::???;

let token = cashu_mint.request_mana_token(lightning_payment, league_id).await?;
let unit_bytes: [u8; 32] = token.extract_secret(); // How to get the 32-byte secret
let units: [Unit; 8] = parse_units_from_bytes(unit_bytes, league_id);
```

### 2. Swiss Tournament Pairing Algorithm  
```rust
// Need reference implementation for:
fn generate_swiss_pairings(
    players: &[Player], 
    round: u32,
    previous_pairings: &[Pairing]
) -> Vec<Pairing> {
    // Swiss system logic with anti-repeat constraints
}
```

## Project Status

✅ **Complete Project Planning:**
- Folder structure created
- Team roles and Claude agents defined  
- Architecture broken down into components
- Work breakdown structure for all 5 phases
- Technical specifications documented

🎯 **Ready for Implementation** once we have:
1. Cashu Rust library reference
2. Swiss tournament algorithm reference

The waterfall planning is complete and agent-ready. Each phase has clear deliverables, success criteria, and validation steps that Claude agents can execute autonomously.