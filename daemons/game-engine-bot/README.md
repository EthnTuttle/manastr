# Game Engine Bot

An **authoritative game engine daemon** for the Mana Strategy Game that orchestrates matches, processes combat, and manages loot distribution. The bot runs on port `:4444` and serves as the final arbiter of all game outcomes.

## Overview

The Game Engine Bot is responsible for:

- **Authoritative Match Resolution**: Final arbiter of combat outcomes
- **Loot Token Creation**: Authority to request loot tokens from the Cashu mint  
- **Combat Processing**: Deterministic battle resolution using mana token secrets
- **Match State Management**: Tracking player commitments, reveals, and rounds
- **Nostr Event Processing**: (Full implementation) Subscribe to and process game events

## Current Implementation Status

✅ **Core Architecture**: Match state management, combat engine, Cashu integration  
✅ **HTTP API**: Status endpoints and test functionality  
⏳ **Nostr Integration**: Event subscription and processing (planned)  
⏳ **Full Game Loop**: Complete commit-reveal game flow (planned)  

## API Endpoints

### Status and Health
- `GET /health` - Bot health check
- `GET /status` - Bot status and active match count
- `GET /match/{match_id}` - Get specific match details

### Test Endpoints (MVP)
- `GET /test/create_match` - Create a test match between two players
- `GET /test/award_loot` - Simulate loot token creation

## Architecture

### Core Components

```rust
pub struct GameEngineBot {
    config: GameEngineConfig,
    match_manager: Arc<tokio::sync::Mutex<MatchManager>>,
    cashu_client: Arc<CashuClient>,
}
```

### Match State Management
- **MatchManager**: Tracks all active matches and their states
- **MatchState**: Individual match data (players, rounds, commitments, reveals)
- **MatchPhase**: Current phase of each match (commitments, reveals, combat, etc.)

### Combat Engine
- **Unit Generation**: Creates 8 battle units from mana token secrets
- **Combat Resolution**: Processes unit vs unit battles with abilities
- **League Modifiers**: Applies bonuses based on token league (simplified)
- **Winner Determination**: Decides round and match winners

### Cashu Integration
- **CashuClient**: Communicates with the CDK mint for loot creation
- **Token Verification**: Validates mana tokens (client-side logic)
- **Loot Minting**: Requests loot tokens for match winners

## Configuration

The bot is configured via `game-engine.toml`:

```toml
[server]
host = "127.0.0.1"
port = 4444

[nostr]
relay_url = "ws://localhost:7777"
private_key = "game_engine_bot_private_key_hex"

[cashu]
mint_url = "http://localhost:3333"

[game]
max_concurrent_matches = 100
round_timeout_seconds = 300
match_timeout_seconds = 1800
loot_reward_per_match = 1000
```

## Running the Bot

### Development
```bash
cargo run
```

### Testing
```bash
cargo test
```

### Example Usage

1. **Check Bot Status**:
```bash
curl http://localhost:4444/status
```

2. **Create Test Match**:
```bash
curl http://localhost:4444/test/create_match
```

3. **Award Test Loot**:
```bash
curl http://localhost:4444/test/award_loot
```

## Integration Points

### With Cashu Mint (D1)
- **Health Check**: Ensures mint is available for loot creation
- **Loot Token Creation**: Requests loot tokens for match winners
- **Token Verification**: Validates mana tokens used in battles

### With Nostr Relay (D2)
- **Event Subscription**: Listens for challenge, commitment, and reveal events
- **Result Publishing**: Publishes authoritative match results
- **Player Communication**: Announces match phases and timeouts

### With Web Client (D4)
- **Match Status**: Provides current match states to clients
- **Result Display**: Match outcomes and combat details
- **Debug Interface**: Development and testing endpoints

## Game Flow (Full Implementation)

1. **Challenge Phase**: Players initiate matches via Nostr
2. **Commitment Phase**: Players commit to unit selections (hash)
3. **Reveal Phase**: Players reveal their mana tokens and unit choices
4. **Combat Phase**: Bot processes deterministic combat resolution
5. **Result Phase**: Bot publishes results and awards loot tokens

## Combat Resolution

### Unit Generation
Units are generated deterministically from mana token secrets:
- 8 units per token
- Base stats: attack (10-29), defense (5-19), health (20-49)
- League modifiers applied (Fire: +10 attack, Ice: +20 health, etc.)
- Random abilities: None, Boost, Shield, Heal

### Combat Mechanics
- **Boost**: Double attack for one round
- **Shield**: Negate all damage for one round  
- **Heal**: Restore 50% max health after combat
- **Damage**: `attack - defense` (minimum 0)
- **Winner**: Last unit standing, or higher health if both survive

### Match Victory
- **Best of 5**: First to win 3 rounds wins the match
- **Tiebreaker**: Total damage dealt if rounds are tied
- **Loot Reward**: Winner receives 1000 loot tokens (meltable to Lightning)

## Development Notes

This implementation provides the foundation for an authoritative game engine. The current version focuses on:

- ✅ Core architecture and data structures
- ✅ Combat resolution engine with full game mechanics
- ✅ Cashu mint integration for loot distribution  
- ✅ HTTP API for testing and debugging

Future development will add:
- 🔄 Complete Nostr event processing
- 🔄 Commit-reveal game protocol
- 🔄 Advanced match management (timeouts, forfeitures)
- 🔄 Fraud detection and validation

---

**Status**: Core implementation complete ✅  
**Port**: 4444  
**Dependencies**: Cashu Mint (D1), Nostr Relay (D2)  
**Integration**: Ready for Web Client (D4)