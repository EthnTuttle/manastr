# Game Engine Bot Specification

## Overview
The Game Engine Bot is an authoritative daemon that orchestrates matches, processes Nostr events, resolves combat, and manages rewards. It runs at `:4444` and has special authority to create loot tokens via the Cashu mint.

## Bot Authority & Responsibilities

### Core Authority
- **Authoritative Match Resolution**: Final arbiter of combat outcomes
- **Loot Token Creation**: Only entity that can request loot tokens from mint
- **Event Publishing**: Publishes official match results to Nostr relay
- **Fraud Detection**: Validates commitments and prevents cheating
- **Timeout Management**: Handles player inactivity and forfeitures

### NOT Responsible For
- Player matchmaking (handled by web client directly)
- Token minting (handled by Cashu mint)
- Data persistence (handled by Nostr relay)

## Bot Architecture

### Main Event Loop
```rust
#[tokio::main]
async fn main() -> Result<(), GameEngineError> {
    let config = GameEngineConfig::load()?;
    
    // Connect to services
    let nostr_client = NostrClient::connect(&config.nostr_relay_url).await?;
    let cashu_client = CashuClient::connect(&config.mint_url).await?;
    
    // Subscribe to game events
    let filter = Filter::new()
        .kind(1)
        .tags(vec!["match_id", "commitment", "reveal", "challenge"]);
    nostr_client.subscribe(vec![filter]).await?;
    
    // Initialize match state manager
    let mut match_manager = MatchManager::new();
    
    // Main processing loop
    loop {
        match nostr_client.recv().await? {
            event => {
                let game_event = classify_event(&event)?;
                process_game_event(game_event, &mut match_manager, &cashu_client).await?;
            }
        }
    }
}
```

## Event Processing Logic

### Event Classification
```rust
#[derive(Debug, Clone)]
pub enum GameEvent {
    Challenge {
        challenger_npub: String,
        challenged_npub: String,
        match_id: String,
    },
    ChallengeAccepted {
        match_id: String,
        acceptor_npub: String,
    },
    UnitCommitment {
        match_id: String,
        player_npub: String,
        round: u8,
        commitment_hash: String,
    },
    UnitReveal {
        match_id: String,
        player_npub: String,
        round: u8,
        mana_token_secret: [u8; 32],
        mana_token_signature: String,
        unit_index: u8,
        commitment_event_id: String,
    },
    MatchVerification {
        match_id: String,
        player_npub: String,
        token_proofs: Vec<TokenProof>,
    },
}

pub fn classify_event(event: &NostrEvent) -> Result<GameEvent, ClassificationError> {
    let tags = parse_event_tags(&event.tags);
    
    match (tags.get("challenge"), tags.get("commitment"), tags.get("reveal")) {
        (Some(_), None, None) => parse_challenge_event(event),
        (None, Some(_), None) => parse_commitment_event(event),
        (None, None, Some(_)) => parse_reveal_event(event),
        _ => Err(ClassificationError::UnknownEventType),
    }
}
```

### Match State Management
```rust
#[derive(Debug, Clone)]
pub struct MatchState {
    pub match_id: String,
    pub players: [String; 2], // npubs
    pub current_round: u8,
    pub state: MatchPhase,
    pub rounds: Vec<RoundResult>,
    pub commitments: HashMap<(String, u8), CommitmentData>, // (npub, round) -> commitment
    pub reveals: HashMap<(String, u8), RevealData>,
    pub created_at: DateTime<Utc>,
    pub timeout_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPhase {
    WaitingForAcceptance,
    WaitingForCommitments,
    WaitingForReveals,
    ProcessingCombat,
    RoundComplete,
    MatchComplete,
    Abandoned,
}
```

### Event Processing Functions
```rust
pub async fn process_challenge(
    event: &NostrEvent,
    match_manager: &mut MatchManager,
) -> Result<(), ProcessingError> {
    let challenge = parse_challenge_event(event)?;
    
    // Create new match state
    let match_state = MatchState::new(
        challenge.match_id.clone(),
        [challenge.challenger_npub, challenge.challenged_npub],
    );
    
    match_manager.add_match(match_state);
    
    // Publish match announcement
    publish_match_announcement(&challenge.match_id, &match_state.players).await?;
    
    Ok(())
}

pub async fn process_commitment(
    event: &NostrEvent,
    match_manager: &mut MatchManager,
) -> Result<(), ProcessingError> {
    let commitment = parse_commitment_event(event)?;
    
    // Validate commitment format
    if !is_valid_sha256_hash(&commitment.commitment_hash) {
        return Err(ProcessingError::InvalidCommitment);
    }
    
    // Store commitment
    match_manager.add_commitment(
        &commitment.match_id,
        &commitment.player_npub,
        commitment.round,
        CommitmentData {
            hash: commitment.commitment_hash,
            event_id: event.id.clone(),
            timestamp: event.created_at,
        }
    )?;
    
    // Check if both players have committed for this round
    if match_manager.both_players_committed(&commitment.match_id, commitment.round)? {
        // Advance to reveal phase
        match_manager.set_phase(&commitment.match_id, MatchPhase::WaitingForReveals)?;
        publish_reveal_request(&commitment.match_id, commitment.round).await?;
    }
    
    Ok(())
}

pub async fn process_reveal(
    event: &NostrEvent,
    match_manager: &mut MatchManager,
    cashu_client: &CashuClient,
) -> Result<(), ProcessingError> {
    let reveal = parse_reveal_event(event)?;
    
    // Verify token signature with Cashu mint
    let token_valid = cashu_client.verify_mana_token(
        &reveal.mana_token_secret,
        &reveal.mana_token_signature,
    ).await?;
    
    if !token_valid {
        return Err(ProcessingError::InvalidToken);
    }
    
    // Verify commitment matches reveal
    let expected_commitment = calculate_commitment_hash(
        &reveal.mana_token_secret,
        &reveal.mana_token_signature,
        reveal.unit_index,
        reveal.round,
        &reveal.match_id,
    );
    
    let stored_commitment = match_manager.get_commitment(
        &reveal.match_id,
        &reveal.player_npub,
        reveal.round,
    )?;
    
    if stored_commitment.hash != expected_commitment {
        return Err(ProcessingError::CommitmentMismatch);
    }
    
    // Generate units from token
    let units = generate_units_from_mana_token(
        &reveal.mana_token_secret,
        reveal.league_id, // extracted from token signature
    );
    
    if reveal.unit_index >= 8 {
        return Err(ProcessingError::InvalidUnitIndex);
    }
    
    // Store reveal
    match_manager.add_reveal(
        &reveal.match_id,
        &reveal.player_npub,
        reveal.round,
        RevealData {
            unit: units[reveal.unit_index as usize],
            token_secret: reveal.mana_token_secret,
            event_id: event.id.clone(),
        }
    )?;
    
    // Check if both players have revealed
    if match_manager.both_players_revealed(&reveal.match_id, reveal.round)? {
        // Process combat
        process_combat(&reveal.match_id, reveal.round, match_manager, cashu_client).await?;
    }
    
    Ok(())
}
```

## Combat Resolution Engine

### Core Combat Logic
```rust
#[derive(Debug, Clone)]
pub struct Unit {
    pub attack: u8,
    pub defense: u8,
    pub health: u8,
    pub max_health: u8,
    pub ability: Ability,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ability {
    None,
    Boost,      // Double attack this round
    Shield,     // Negate damage this round
    Heal,       // Restore 50% max health post-combat
}

pub async fn process_combat(
    match_id: &str,
    round: u8,
    match_manager: &mut MatchManager,
    cashu_client: &CashuClient,
) -> Result<(), CombatError> {
    let reveals = match_manager.get_round_reveals(match_id, round)?;
    let (player1_reveal, player2_reveal) = reveals;
    
    let mut unit1 = player1_reveal.unit.clone();
    let mut unit2 = player2_reveal.unit.clone();
    
    // Apply abilities (pre-combat)
    if unit1.ability == Ability::Boost {
        unit1.attack *= 2;
    }
    if unit2.ability == Ability::Boost {
        unit2.attack *= 2;
    }
    
    // Calculate damage
    let damage_to_unit2 = if unit2.ability == Ability::Shield {
        0
    } else {
        unit1.attack.saturating_sub(unit2.defense)
    };
    
    let damage_to_unit1 = if unit1.ability == Ability::Shield {
        0
    } else {
        unit2.attack.saturating_sub(unit1.defense)
    };
    
    // Apply damage
    unit1.health = unit1.health.saturating_sub(damage_to_unit1);
    unit2.health = unit2.health.saturating_sub(damage_to_unit2);
    
    // Apply healing (post-combat)
    if unit1.ability == Ability::Heal && unit1.health > 0 {
        let heal_amount = (unit1.max_health / 2).max(1);
        unit1.health = (unit1.health + heal_amount).min(unit1.max_health);
    }
    if unit2.ability == Ability::Heal && unit2.health > 0 {
        let heal_amount = (unit2.max_health / 2).max(1);
        unit2.health = (unit2.health + heal_amount).min(unit2.max_health);
    }
    
    // Determine round winner
    let round_winner = determine_round_winner(&unit1, &unit2, &player1_reveal.player_npub, &player2_reveal.player_npub);
    
    // Store round result
    let round_result = RoundResult {
        round,
        player1_unit: unit1,
        player2_unit: unit2,  
        damage_dealt: [damage_to_unit2, damage_to_unit1],
        winner: round_winner.clone(),
    };
    
    match_manager.add_round_result(match_id, round_result.clone())?;
    
    // Publish round result
    publish_round_result(match_id, &round_result).await?;
    
    // Check if match is complete (5 rounds or early termination)
    if round >= 5 || match_manager.has_match_winner(match_id)? {
        finalize_match(match_id, match_manager, cashu_client).await?;
    } else {
        // Advance to next round
        match_manager.advance_round(match_id)?;
    }
    
    Ok(())
}

fn determine_round_winner(
    unit1: &Unit,
    unit2: &Unit,
    player1_npub: &str,
    player2_npub: &str,
) -> Option<String> {
    match (unit1.health > 0, unit2.health > 0) {
        (true, false) => Some(player1_npub.to_string()),
        (false, true) => Some(player2_npub.to_string()),
        (true, true) => {
            // Both alive, higher health wins
            if unit1.health > unit2.health {
                Some(player1_npub.to_string())
            } else if unit2.health > unit1.health {
                Some(player2_npub.to_string())
            } else {
                None // Tie
            }
        },
        (false, false) => None, // Both dead, tie
    }
}
```

### Match Finalization
```rust
pub async fn finalize_match(
    match_id: &str,
    match_manager: &mut MatchManager,
    cashu_client: &CashuClient,
) -> Result<(), FinalizationError> {
    let match_state = match_manager.get_match(match_id)?;
    let round_results = &match_state.rounds;
    
    // Calculate match winner
    let mut player1_rounds_won = 0;
    let mut player2_rounds_won = 0;
    let mut total_damage = [0u32, 0u32];
    
    for round_result in round_results {
        if let Some(winner) = &round_result.winner {
            if winner == &match_state.players[0] {
                player1_rounds_won += 1;
            } else {
                player2_rounds_won += 1;
            }
        }
        total_damage[0] += round_result.damage_dealt[0] as u32;
        total_damage[1] += round_result.damage_dealt[1] as u32;
    }
    
    // Determine match winner
    let match_winner = if player1_rounds_won > player2_rounds_won {
        Some(match_state.players[0].clone())
    } else if player2_rounds_won > player1_rounds_won {
        Some(match_state.players[1].clone())
    } else {
        // Tiebreaker: total damage dealt
        if total_damage[0] > total_damage[1] {
            Some(match_state.players[0].clone())
        } else if total_damage[1] > total_damage[0] {
            Some(match_state.players[1].clone())
        } else {
            None // Perfect tie
        }
    };
    
    // Create loot token for winner (if any)
    if let Some(winner_npub) = &match_winner {
        let loot_amount = 1000; // Base loot reward per match
        
        let loot_token = cashu_client.create_loot_token(
            winner_npub,
            loot_amount,
            match_id,
        ).await?;
        
        // Publish reward event
        publish_loot_reward(match_id, winner_npub, loot_amount, &loot_token).await?;
    }
    
    // Publish final match result
    let final_result = MatchResult {
        match_id: match_id.to_string(),
        winner: match_winner,
        score: [player1_rounds_won, player2_rounds_won],
        total_damage,
        rounds: round_results.clone(),
    };
    
    publish_match_result(&final_result).await?;
    
    // Mark match as complete
    match_manager.set_phase(match_id, MatchPhase::MatchComplete)?;
    
    Ok(())
}
```

## Nostr Event Publishing

### Event Publishing Functions
```rust
pub async fn publish_match_announcement(
    match_id: &str,
    players: &[String; 2],
) -> Result<String, PublishError> {
    let content = json!({
        "match_id": match_id,
        "players": players,
        "status": "waiting_for_commitments"
    });
    
    let event = EventBuilder::new(Kind::TextNote, content.to_string(), &[
        Tag::Generic(TagKind::Custom("match_id".to_string()), vec![match_id.to_string()]),
        Tag::Generic(TagKind::Custom("game".to_string()), vec!["mana-strategy".to_string()]),
    ]);
    
    publish_event(event).await
}

pub async fn publish_round_result(
    match_id: &str,
    round_result: &RoundResult,
) -> Result<String, PublishError> {
    let content = json!({
        "winner": round_result.winner,
        "damage_dealt": round_result.damage_dealt,
        "round": round_result.round,
        "units": {
            "player1": round_result.player1_unit,
            "player2": round_result.player2_unit
        }
    });
    
    let event = EventBuilder::new(Kind::TextNote, content.to_string(), &[
        Tag::Generic(TagKind::Custom("match_id".to_string()), vec![match_id.to_string()]),
        Tag::Generic(TagKind::Custom("round_result".to_string()), vec![round_result.round.to_string()]),
    ]);
    
    publish_event(event).await
}
```

## Bot Configuration

### Configuration File (game-engine.toml)
```toml
[server]
host = "127.0.0.1"
port = 4444

[nostr]
relay_url = "ws://localhost:7777"
private_key = "bot_private_key_hex"

[cashu]
mint_url = "http://localhost:3333"

[game]
max_concurrent_matches = 100
round_timeout_seconds = 300  # 5 minutes per round
match_timeout_seconds = 1800 # 30 minutes total match
loot_reward_per_match = 1000

[logging]
level = "info"
file = "./logs/game-engine.log"
```

## Error Handling & Recovery

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum GameEngineError {
    #[error("Nostr connection failed: {0}")]
    NostrConnectionError(String),
    
    #[error("Cashu mint communication failed: {0}")]
    CashuError(String),
    
    #[error("Invalid event format: {0}")]
    EventParsingError(String),
    
    #[error("Match not found: {0}")]
    MatchNotFound(String),
    
    #[error("Invalid game state transition")]
    InvalidStateTransition,
    
    #[error("Combat resolution failed: {0}")]
    CombatError(String),
}
```

### Recovery Strategies
- **Nostr disconnection**: Auto-reconnect with exponential backoff
- **Invalid events**: Log and ignore, don't crash the bot
- **Cashu mint unavailable**: Queue loot creation, retry when available
- **Match timeouts**: Auto-forfeit inactive players after timeout

## Integration Points

### With Cashu Mint
- **Verify mana tokens**: `POST /verify-token`
- **Create loot tokens**: `POST /mint/loot` (bot authority only)
- **Health check**: `GET /health`

### With Nostr Relay
- **Subscribe to events**: WebSocket subscription to game events
- **Publish results**: Authoritative match results and rewards
- **Event validation**: Verify event signatures and structure

### With Web Client
- **Status endpoint**: `GET /status` for bot health and active matches
- **Match details**: `GET /match/{match_id}` for current match state
- **Debug endpoints**: Various debugging and monitoring endpoints

This specification provides everything needed to implement D3 (Game Engine Bot daemon) with full authority over match resolution and loot distribution.