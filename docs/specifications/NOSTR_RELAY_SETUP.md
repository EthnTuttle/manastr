# Nostr Relay Setup and Event Flow Specification

## Overview
Strfry-based Nostr relay running at `ws://localhost:7777` that stores all game data, coordinates match events, and serves as the single source of truth for game state.

## Strfry Relay Setup

### Installation and Build
```bash
# Install dependencies (Ubuntu/Debian)
sudo apt install -y git build-essential pkg-config libtool autoconf autoconf-archive automake
sudo apt install -y libyaml-cpp-dev libssl-dev zlib1g-dev liblmdb-dev

# Clone and build strfry
git clone https://github.com/damus-io/strfry.git
cd strfry
git submodule update --init
make setup-golpe
make -j4

# The binary will be at ./strfry
```

### Directory Structure
```
daemons/nostr-relay/
├── strfry              # Built binary
├── strfry.conf         # Configuration file
├── strfry-db/          # LMDB database (auto-created)
├── logs/               # Log files
└── scripts/            # Management scripts
```

### Configuration File (strfry.conf)
```yaml
##
## Strfry Config for Mana Strategy Game
##

relay:
    bind: "127.0.0.1:7777"
    nofiles: 1000000
    realIpHeader: ""

events:
    rejectEventsNewerThanSeconds: 900
    rejectEventsOlderThanSeconds: 86400
    rejectEphemeralEventsOlderThanSeconds: 60
    ephemeralEventsLifetimeSeconds: 300
    maxEventBytes: 65536
    
    # Game-specific: retain events for 24 hours
    maxNumTags: 100
    maxTagValSize: 1024

relay:
    compression:
        enabled: true
        slidingWindow: true
        maxWindowBits: 15

database:
    dbParams:
        maxreaders: 256
        mapsize: 268435456000  # 250GB for game data

logging:
    dumpInAll: false
    dumpInEvents: false
    dumpInReqs: false
    writePolicy: auto

negentropy:
    enabled: true
    maxSyncEvents: 1000000

plugin:
    writePolicy:
        plugin: ""
```

### Local Development Startup Script
```bash
#!/bin/bash
# daemons/nostr-relay/start.sh

cd "$(dirname "$0")"

# Ensure log directory exists
mkdir -p logs

# Start strfry relay
echo "Starting strfry Nostr relay on ws://localhost:7777"
echo "Database: ./strfry-db/"
echo "Config: ./strfry.conf"

./strfry relay --config=./strfry.conf 2>&1 | tee logs/strfry.log
```

## Game Event Schemas

### Base Event Structure
All game events follow standard Nostr event format:
```json
{
  "id": "event_id_hex",
  "pubkey": "player_npub_hex",
  "created_at": unix_timestamp,
  "kind": 1,
  "tags": [...],
  "content": "...",
  "sig": "signature_hex"
}
```

### Event Type 1: Challenge Request
```json
{
  "kind": 1,
  "content": "Challenge issued",
  "tags": [
    ["match_id", "uuid_string"],
    ["challenged", "npub_hex"],
    ["game", "mana-strategy"],
    ["challenge"]
  ]
}
```

### Event Type 2: Challenge Acceptance
```json
{
  "kind": 1,
  "content": "Challenge accepted",
  "tags": [
    ["match_id", "uuid_string"],
    ["challenger", "npub_hex"],
    ["game", "mana-strategy"], 
    ["challenge_accepted"]
  ]
}
```

### Event Type 3: Match Announcement (Bot Authority)
```json
{
  "kind": 1,
  "content": {
    "match_id": "uuid_string",
    "players": ["npub1", "npub2"],
    "status": "waiting_for_commitments",
    "created_at": "2024-01-27T12:00:00Z"
  },
  "tags": [
    ["match_id", "uuid_string"],
    ["game", "mana-strategy"],
    ["match_announcement"],
    ["p", "npub1"],
    ["p", "npub2"]
  ]
}
```

### Event Type 4: Unit Commitment
```json
{
  "kind": 1,
  "content": "sha256_commitment_hash",
  "tags": [
    ["match_id", "uuid_string"],
    ["round", "1"],
    ["game", "mana-strategy"],
    ["commitment"]
  ]
}
```

### Event Type 5: Unit Reveal
```json
{
  "kind": 1,
  "content": {
    "mana_token_secret": "hex_string",
    "mana_token_signature": "hex_string",
    "unit_index": 3,
    "round": 1,
    "league_id": 5
  },
  "tags": [
    ["match_id", "uuid_string"],
    ["round", "1"],
    ["game", "mana-strategy"],
    ["reveal"],
    ["e", "commitment_event_id"]
  ]
}
```

### Event Type 6: Round Result (Bot Authority)
```json
{
  "kind": 1,
  "content": {
    "round": 1,
    "winner": "npub_hex_or_null",
    "damage_dealt": [25, 30],
    "units": {
      "player1": {
        "initial": {"attack": 120, "defense": 80, "health": 100, "ability": "boost"},
        "final": {"health": 70}
      },
      "player2": {
        "initial": {"attack": 100, "defense": 90, "health": 110, "ability": "none"},
        "final": {"health": 95}
      }
    }
  },
  "tags": [
    ["match_id", "uuid_string"],
    ["round", "1"],
    ["game", "mana-strategy"],
    ["round_result"],
    ["e", "reveal_event_id_1"],
    ["e", "reveal_event_id_2"]
  ]
}
```

### Event Type 7: Match Result (Bot Authority)
```json
{
  "kind": 1,
  "content": {
    "match_id": "uuid_string",
    "winner": "npub_hex_or_null",
    "score": [3, 2],
    "total_damage": [125, 98],
    "rounds": 5,
    "completed_at": "2024-01-27T12:30:00Z"
  },
  "tags": [
    ["match_id", "uuid_string"],
    ["game", "mana-strategy"],
    ["match_result"],
    ["e", "round_result_1"],
    ["e", "round_result_2"],
    ["e", "round_result_3"],
    ["e", "round_result_4"],
    ["e", "round_result_5"]
  ]
}
```

### Event Type 8: Loot Reward (Bot Authority)
```json
{
  "kind": 1,
  "content": {
    "match_id": "uuid_string", 
    "winner": "npub_hex",
    "loot_amount": 1000,
    "loot_token": {
      "secret": "hex_string",
      "signature": "hex_string",
      "locked_to_npub": "npub_hex"
    }
  },
  "tags": [
    ["match_id", "uuid_string"],
    ["game", "mana-strategy"],
    ["loot_reward"],
    ["p", "winner_npub"],
    ["e", "match_result_event_id"]
  ]
}
```

## Event Flow and Validation

### Challenge Flow
```
1. Player 1 → Challenge Request Event
2. Player 2 → Challenge Acceptance Event  
3. Game Bot → Match Announcement Event (authoritative)
```

### Match Round Flow
```
1. Players → Unit Commitment Events (both players)
2. Players → Unit Reveal Events (both players, after commitments)
3. Game Bot → Round Result Event (authoritative, after both reveals)
4. Repeat for rounds 2-5
```

### Match Completion Flow
```
1. Game Bot → Match Result Event (after 5 rounds or early termination)
2. Game Bot → Loot Reward Event (if there's a winner)
```

## Event Validation Rules

### Client-Side Validation (Before Publishing)
```rust
pub fn validate_game_event(event: &NostrEvent) -> Result<(), ValidationError> {
    // Basic Nostr validation
    if !event.verify_signature()? {
        return Err(ValidationError::InvalidSignature);
    }
    
    // Game-specific validation
    let game_tag = event.tags.iter()
        .find(|t| t.len() >= 2 && t[0] == "game")
        .ok_or(ValidationError::MissingGameTag)?;
    
    if game_tag[1] != "mana-strategy" {
        return Err(ValidationError::WrongGame);
    }
    
    // Event type specific validation
    if has_tag(event, "commitment") {
        validate_commitment_event(event)?;
    } else if has_tag(event, "reveal") {
        validate_reveal_event(event)?;
    }
    
    Ok(())
}

fn validate_commitment_event(event: &NostrEvent) -> Result<(), ValidationError> {
    // Content must be valid SHA-256 hash
    if event.content.len() != 64 || !is_hex_string(&event.content) {
        return Err(ValidationError::InvalidCommitmentHash);
    }
    
    // Must have match_id and round tags
    require_tag(event, "match_id")?;
    require_tag(event, "round")?;
    
    Ok(())
}

fn validate_reveal_event(event: &NostrEvent) -> Result<(), ValidationError> {
    let content: RevealContent = serde_json::from_str(&event.content)
        .map_err(|_| ValidationError::InvalidRevealFormat)?;
    
    // Validate token format
    if content.mana_token_secret.len() != 64 {
        return Err(ValidationError::InvalidTokenSecret);
    }
    
    // Validate unit index (0-7)
    if content.unit_index >= 8 {
        return Err(ValidationError::InvalidUnitIndex);
    }
    
    // Must reference commitment event
    require_tag(event, "e")?;
    
    Ok(())
}
```

### Relay-Side Storage Rules
```yaml
# Store all game events for 24 hours minimum
retention_policy:
  game_events: 86400  # 24 hours in seconds
  match_events: 86400
  result_events: 604800  # 1 week for results

# Index tags for efficient querying
indexed_tags:
  - "match_id"
  - "game" 
  - "round"
  - "p"  # participant
  - "e"  # event reference

# Rate limiting per pubkey
rate_limits:
  max_events_per_minute: 30
  max_events_per_hour: 500
```

## Integration with Game Engine Bot

### Bot Subscription Filters
```rust
// Game engine bot subscribes to these filters
let filters = vec![
    // All game events
    Filter::new()
        .kind(Kind::TextNote)
        .tag_values_containing("game", vec!["mana-strategy"]),
    
    // Challenge events
    Filter::new()
        .kind(Kind::TextNote)
        .tag_values_containing("challenge", vec![""]),
        
    // Commitment events
    Filter::new()
        .kind(Kind::TextNote)
        .tag_values_containing("commitment", vec![""]),
        
    // Reveal events
    Filter::new()
        .kind(Kind::TextNote)
        .tag_values_containing("reveal", vec![""]),
];
```

### Bot Event Publishing
```rust
// Bot publishes authoritative events
pub async fn publish_authoritative_event(
    event_type: AuthorityEventType,
    content: serde_json::Value,
    tags: Vec<Tag>
) -> Result<String, PublishError> {
    let mut event_tags = vec![
        Tag::Generic(TagKind::Custom("game".to_string()), vec!["mana-strategy".to_string()]),
        Tag::Generic(TagKind::Custom("authority".to_string()), vec!["game-engine-bot".to_string()]),
    ];
    event_tags.extend(tags);
    
    let event = EventBuilder::new(
        Kind::TextNote,
        content.to_string(),
        &event_tags
    );
    
    publish_to_relay(event).await
}
```

## Query Patterns for Game Data

### Get Match History
```javascript
// Web client queries for player's match history
{
  "kinds": [1],
  "authors": ["player_npub"],
  "#game": ["mana-strategy"],
  "#match_result": [""],
  "limit": 50
}
```

### Get Active Match State
```javascript
// Get all events for a specific match
{
  "kinds": [1], 
  "#match_id": ["specific_match_uuid"],
  "#game": ["mana-strategy"]
}
```

### Get Leaderboard Data
```javascript
// Get all match results for leaderboard calculation
{
  "kinds": [1],
  "#game": ["mana-strategy"],
  "#match_result": [""],
  "since": unix_timestamp_24h_ago
}
```

## Performance and Monitoring

### Expected Load (MVP)
- **Concurrent matches**: ~10-50
- **Events per match**: ~15-25 events
- **Daily events**: ~500-2000 events
- **Storage growth**: ~1MB per day

### Monitoring Endpoints
```bash
# Check relay health
curl http://localhost:7777/health

# Get relay info
curl http://localhost:7777/relay-info

# Event count by type
./strfry export | jq '.tags[][] | select(. == "game")' | wc -l
```

### Database Maintenance
```bash
# Export game events for backup
./strfry export --filter='{"#game": ["mana-strategy"]}' > game_events_backup.jsonl

# Import events (for testing or recovery)
cat game_events_backup.jsonl | ./strfry import

# Check database size
du -sh strfry-db/
```

This specification provides everything needed to implement D2 (Nostr Relay daemon) using strfry as the foundation, storing all game data and coordinating match events.