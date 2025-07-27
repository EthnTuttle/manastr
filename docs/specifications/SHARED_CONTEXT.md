# Shared Context - Cross-Agent Knowledge Base

## Project-Wide Architectural Decisions

### Technology Stack (FINALIZED)
- **Backend Language:** Rust
- **Web Framework:** `axum` for REST APIs + WebSocket
- **Database:** PostgreSQL with `sqlx` for type-safe queries
- **Async Runtime:** `tokio`
- **Frontend:** React (web) + React Native (mobile)
- **Cryptography:** `secp256k1` + `sha2` crates
- **Nostr Integration:** `nostr-sdk` crate

### Core System Simplifications (FINALIZED)
- ❌ **No Swiss Tournament System** - Removed complex pairing algorithms
- ✅ **Player-Choice Matchmaking** - Direct challenge system
- ✅ **Periodic Leaderboard Rewards** - Weekly/monthly top-player payouts
- ✅ **Cashu Provides VRF** - No separate randomness service needed

## Service Interface Contracts

### Data Types (Shared Across All Services)
```rust
// Core identifiers
pub type PlayerId = uuid::Uuid;
pub type MatchId = uuid::Uuid;
pub type SeasonId = uuid::Uuid;
pub type LeagueId = u8; // 0-15 for 16 league keys

// Game structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub attack: u8,
    pub defense: u8, 
    pub health: u8,
    pub ability: Ability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ability {
    None,
    Boost,    // Double attack this round
    Shield,   // Negate damage this round  
    Heal,     // Restore 50% max health post-combat
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub npub: String,
    pub rating: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: MatchId,
    pub player1_id: PlayerId,
    pub player2_id: PlayerId,
    pub state: MatchState,
    pub rounds: Vec<Round>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchState {
    WaitingForCommitments,
    WaitingForReveals,
    CombatResolution,
    RoundComplete,
    MatchComplete,
}
```

### Service Interfaces

#### Game Engine → Matchmaking
```rust
// Functions the matchmaking service can call
pub async fn create_match(player1_id: PlayerId, player2_id: PlayerId) -> Result<MatchId, GameError>;
pub async fn get_match_state(match_id: MatchId) -> Result<Match, GameError>;
pub async fn process_unit_commitment(match_id: MatchId, player_id: PlayerId, commitment: String) -> Result<(), GameError>;
pub async fn process_unit_reveal(match_id: MatchId, player_id: PlayerId, reveal: UnitReveal) -> Result<(), GameError>;
```

#### Cashu Mint → Game Engine
```rust  
// Functions the game engine can call
pub async fn parse_token_to_units(token: CashuToken, league_id: LeagueId) -> Result<[Unit; 8], CashuError>;
pub async fn verify_token_signature(token: CashuToken) -> Result<bool, CashuError>;
pub async fn create_loot_token(winner_npub: String, amount: u64) -> Result<LootToken, CashuError>;
```

#### Matchmaking → Leaderboard
```rust
// Functions the leaderboard service can call
pub async fn record_match_result(result: MatchResult) -> Result<(), LeaderboardError>;
pub async fn update_player_rating(player_id: PlayerId, new_rating: u32) -> Result<(), LeaderboardError>;
pub async fn get_player_stats(player_id: PlayerId) -> Result<PlayerStats, LeaderboardError>;
```

#### Nostr Client → All Services
```rust
// Functions all services can call for event publishing
pub async fn publish_match_announcement(match_id: MatchId, players: [PlayerId; 2]) -> Result<NostrEventId, NostrError>;
pub async fn publish_commitment(match_id: MatchId, player_id: PlayerId, commitment: String) -> Result<NostrEventId, NostrError>;
pub async fn publish_reveal(match_id: MatchId, player_id: PlayerId, reveal: UnitReveal) -> Result<NostrEventId, NostrError>;
pub async fn publish_match_result(match_id: MatchId, result: MatchResult) -> Result<NostrEventId, NostrError>;
```

## Database Schema Agreements

### Core Tables (All Agents Must Use These)
```sql
-- Players table
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    npub TEXT NOT NULL UNIQUE,
    rating INTEGER NOT NULL DEFAULT 1200,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Matches table  
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player1_id UUID NOT NULL REFERENCES players(id),
    player2_id UUID NOT NULL REFERENCES players(id),
    state TEXT NOT NULL,
    winner_id UUID REFERENCES players(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Match rounds table
CREATE TABLE match_rounds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    match_id UUID NOT NULL REFERENCES matches(id),
    round_number INTEGER NOT NULL,
    player1_unit_index INTEGER,
    player2_unit_index INTEGER,
    winner_id UUID REFERENCES players(id),
    damage_dealt JSONB, -- {player1: int, player2: int}
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Event Schemas (Nostr)

### Match Announcement Event
```json
{
  "kind": 1,
  "content": {
    "match_id": "uuid-string",
    "players": ["npub1...", "npub2..."],
    "league_ids": [3, 7]
  },
  "tags": [
    ["match_id", "uuid-string"],
    ["game", "mana-strategy"]
  ]
}
```

### Unit Commitment Event
```json
{
  "kind": 1,
  "content": "sha256-commitment-hash",
  "tags": [
    ["match_id", "uuid-string"],
    ["round", "1"],
    ["commitment"]
  ]
}
```

### Unit Reveal Event
```json
{
  "kind": 1,
  "content": {
    "token_secret": "hex-string",
    "token_signature": "hex-string", 
    "unit_index": 3,
    "round": 1
  },
  "tags": [
    ["match_id", "uuid-string"],
    ["round", "1"],
    ["reveal"],
    ["e", "commitment-event-id"]
  ]
}
```

## API Endpoint Standards

### Authentication
All API endpoints use Nostr public key authentication:
```
Authorization: Bearer npub1[...public-key...]
```

### Response Format
```json
{
  "success": true,
  "data": { /* response payload */ },
  "error": null,
  "timestamp": "2024-01-15T12:00:00Z"
}
```

### Error Format
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "MATCH_NOT_FOUND",
    "message": "Match with ID xyz not found",
    "details": {}
  },
  "timestamp": "2024-01-15T12:00:00Z"
}
```

## Configuration Standards

### Environment Variables (All Services)
```bash
# Database
DATABASE_URL=postgresql://user:pass@localhost/manastr

# Nostr
NOSTR_RELAY_URL=wss://relay.example.com
NOSTR_PRIVATE_KEY=hex-private-key

# Cashu
CASHU_MINT_URL=https://mint.example.com
CASHU_MINT_PRIVATE_KEY=hex-private-key

# Service-specific ports
GAME_ENGINE_PORT=3001
MATCHMAKING_PORT=3002
LEADERBOARD_PORT=3003
API_GATEWAY_PORT=3000
```

## Testing Standards

### Unit Test Requirements
- Minimum 90% code coverage
- All public functions must have tests
- Error cases must be tested
- Use `tokio-test` for async testing

### Integration Test Patterns
```rust
#[tokio::test]
async fn test_full_match_flow() {
    let test_db = setup_test_database().await;
    let game_engine = GameEngine::new(test_db.clone());
    let matchmaking = Matchmaking::new(test_db.clone());
    
    // Test complete match lifecycle
    let match_id = matchmaking.create_challenge(player1, player2).await?;
    let match_result = game_engine.play_match(match_id).await?;
    
    assert!(match_result.winner.is_some());
}
```

## Human Decision Tracking

### Decisions Made
- ✅ **Architecture:** Rust + React confirmed
- ✅ **Matchmaking:** Player-choice system confirmed  
- ✅ **VRF:** Cashu provides randomness confirmed

### Decisions Pending
- ❓ **H1:** Cashu Rust library selection
- ❓ **H2:** Reward pool economics and distribution
- ❓ **H3:** UI/UX design approval process
- ❓ **H4:** Game balance parameters
- ❓ **H5:** Seasonal reward timing

## Communication Protocols

### Agent-to-Agent Updates
When making changes that affect other agents:
1. Update this shared context file
2. Notify affected agents in their memory files
3. Update the main CLAUDE.md status board

### Interface Change Process
1. Propose change in shared context
2. Get approval from affected agents  
3. Update all relevant documentation
4. Implement change across services

---

**Last Updated:** [Agent should update this when making changes]  
**Updated By:** [Agent name]  
**Changes Made:** [Summary of what was modified]