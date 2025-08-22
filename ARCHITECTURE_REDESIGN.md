# Manastr Architecture Redesign: Independent Player States

## Problem Identified

The current architecture has a fundamental flaw: **shared local state between players**. Both players see the same `global_matches` HashMap, which means:

1. When Player A makes a move, Player B immediately sees it locally (not through Nostr)
2. State changes are shared in memory instead of being event-driven
3. This breaks the decentralized nature of Nostr-based gaming

## Correct Architecture: Nostr + Cashu

### Current (Flawed)
```
┌─────────────┐     ┌─────────────┐
│   Player A  │────▶│ Shared      │◀────│   Player B  │
│   Account   │     │ Local State │     │   Account   │
│             │     │             │     │             │
└─────────────┘     └─────────────┘     └─────────────┘
```

### Target (Correct - Stateless Client)
```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Player A  │     │             │     │   Player B  │
│   Client    │────▶│    NOSTR    │◀────│   Client    │
│ (Stateless) │     │   Events    │     │ (Stateless) │
└─────────────┘     │             │     └─────────────┘
      │             │  + CASHU    │           │
      └─────────────┤   Wallets   ├───────────┘
                    │             │
                    └─────────────┘
```

**Key Principle**: The client is just a UI that reads from Nostr and executes Cashu operations. No game state is stored locally.

## Required Changes

### 1. Stateless Client Design

**Before (Local State):**
```rust
struct Manastr {
    global_matches: HashMap<String, Match>,     // ❌ Local game state
    account_data: HashMap<String, PlayerData>,  // ❌ Local player data
    match_history: Vec<Match>,                  // ❌ Local history
}
```

**After (Stateless):**
```rust  
struct Manastr {
    // Only UI state and connections
    screen: Screen,
    current_match_id: Option<String>,
    
    // Connections to external state
    nostr_client: NostrClient,
    cashu_wallet: CashuWallet,
    
    // Temporary UI cache (cleared on refresh)
    ui_cache: UICache,
}

struct UICache {
    // Temporary cache of recently queried data for UI performance
    recent_challenges: Vec<NostrEvent>, 
    recent_matches: HashMap<String, Vec<NostrEvent>>,
    cache_expiry: Instant,
}
```

**Key Changes:**
- ❌ No persistent local game state
- ✅ All state read from Nostr on-demand  
- ✅ All sats operations through Cashu wallet
- ✅ UI cache only for performance, expires quickly

### 2. Nostr + Cashu Game Flow

**Complete Game Flow:**
1. **Challenge Creation**: 
   - Player A creates Cashu token for wager amount
   - Player A publishes KIND_MATCH_CHALLENGE event with token commitment
   - No local state - challenge exists only as Nostr event

2. **Challenge Discovery**: 
   - Player B queries Nostr for challenge events in real-time
   - UI displays challenges directly from Nostr events
   - No local challenge storage

3. **Match Acceptance**: 
   - Player B creates matching Cashu token for wager
   - Player B publishes KIND_MATCH_ACCEPTANCE with token commitment
   - Escrow mechanism holds both players' tokens

4. **Combat Phase**:
   - Each move publishes KIND_COMBAT_MOVE to Nostr
   - Game state reconstructed from chronological Nostr events
   - No local combat state - read from events each time

5. **Match Resolution**:
   - Winner determined by game engine or consensus mechanism
   - KIND_MATCH_RESULT published to Nostr
   - Cashu tokens automatically transferred to winner
   - Loser's tokens remain in escrow or are split as configured

6. **Real-time Updates**:
   - UI polls Nostr for new events every few seconds
   - No WebSocket needed - just periodic queries
   - State always fresh from source of truth

### 3. Stateless Implementation

**UI Renders Directly from Nostr Events:**

```rust
impl Manastr {
    fn render_matchmaking(&mut self, ui: &mut Ui) {
        // No local state - query Nostr every render
        let challenges = self.nostr_client.query_challenges(league_id);
        
        for event in challenges {
            let challenge = parse_challenge_event(event);
            ui.button(format!("Challenge: {} sats", challenge.wager));
            // Click handler publishes acceptance event directly
        }
        
        let my_matches = self.nostr_client.query_my_matches(my_pubkey);
        for match_events in group_by_match_id(my_matches) {
            let current_state = reconstruct_match_state(match_events);
            ui.button(format!("Match: {}", current_state.phase));
        }
    }
    
    fn handle_create_challenge(&mut self, wager: u64) {
        // 1. Create Cashu token
        let token = self.cashu_wallet.create_token(wager)?;
        
        // 2. Publish to Nostr immediately  
        let event = create_challenge_event(wager, token.commitment);
        self.nostr_client.publish(event)?;
        
        // 3. No local state stored - next render will show it via Nostr query
        self.success_message = Some("Challenge published to Nostr!");
    }
    
    fn handle_accept_challenge(&mut self, challenge_event: NostrEvent) {
        // 1. Create matching Cashu token
        let token = self.cashu_wallet.create_token(challenge.wager)?;
        
        // 2. Publish acceptance to Nostr
        let event = create_acceptance_event(challenge_event.id, token.commitment);
        self.nostr_client.publish(event)?;
        
        // 3. No local state - next render shows updated state from Nostr
    }
}

impl NostrClient {
    fn query_challenges(&self, league: u8) -> Vec<NostrEvent> {
        // Real-time query, no caching
        self.query([Filter::new()
            .kinds([KIND_MATCH_CHALLENGE])
            .tags([("league", league.to_string())])
            .since(now() - 3600) // Last hour only
        ])
    }
    
    fn query_match_events(&self, match_id: &str) -> Vec<NostrEvent> {
        // Get all events for this match, sorted by timestamp
        self.query([Filter::new()
            .kinds([KIND_MATCH_CHALLENGE, KIND_MATCH_ACCEPTANCE, 
                   KIND_COMBAT_MOVE, KIND_MATCH_RESULT])
            .tags([("match_id", match_id)])
        ]).sort_by(|a, b| a.created_at.cmp(&b.created_at))
    }
}

// Pure function - no state
fn reconstruct_match_state(events: Vec<NostrEvent>) -> MatchState {
    let mut state = MatchState::default();
    
    for event in events.iter().sorted_by_key(|e| e.created_at) {
        match event.kind {
            KIND_MATCH_CHALLENGE => state.phase = Phase::Challenge,
            KIND_MATCH_ACCEPTANCE => state.phase = Phase::Accepted,
            KIND_COMBAT_MOVE => state.apply_combat_move(event),
            KIND_MATCH_RESULT => state.phase = Phase::Completed,
        }
    }
    
    state
}
```

### 4. Cashu Integration

**Wager Management:**
```rust
impl CashuWallet {
    fn create_wager_token(&mut self, amount: u64) -> Result<CashuToken, Error> {
        // Lock tokens in escrow for the match
        let token = self.create_token(amount)?;
        self.mark_escrowed(token.clone(), "match_wager")?;
        Ok(token)
    }
    
    fn release_tokens_to_winner(&mut self, winner_npub: &str, amount: u64) -> Result<(), Error> {
        // Transfer escrowed tokens to winner's wallet
        let tokens = self.get_escrowed_tokens("match_wager", amount)?;
        self.send_to_npub(winner_npub, tokens)?;
        Ok(())
    }
    
    fn return_tokens_on_draw(&mut self, player_npubs: Vec<String>) -> Result<(), Error> {
        // Return tokens to both players on draw/timeout
        for npub in player_npubs {
            let original_amount = self.get_player_wager_amount(&npub)?;
            let tokens = self.get_escrowed_tokens("match_wager", original_amount)?;
            self.send_to_npub(&npub, tokens)?;
        }
        Ok(())
    }
}
```

**Game Economy Flow:**
1. **Challenge Creation**: `cashu_wallet.create_wager_token(100)` → Tokens locked
2. **Match Acceptance**: Second player locks matching tokens  
3. **During Match**: Tokens remain in escrow, tracked by match_id
4. **Match End**: 
   - Winner takes all: `release_tokens_to_winner(winner, 200)`
   - Draw: `return_tokens_on_draw([player1, player2])`
   - Timeout: Automatic return after 24 hours

**Nostr Event Integration:**
```rust
// Challenge event includes Cashu token commitment
{
  "kind": 21000,
  "content": "",
  "tags": [
    ["match_id", "uuid"],
    ["wager", "100"],
    ["league", "1"], 
    ["cashu_token", "base64_encoded_token_commitment"],
    ["mint_url", "https://mint.example.com"]
  ]
}
```

## Benefits of Nostr + Cashu Architecture

1. **True Decentralization**: No local game state, everything on Nostr
2. **Financial Sovereignty**: Cashu tokens are bearer assets, no trusted third party  
3. **Instant Settlement**: Winners receive sats immediately via Cashu
4. **Censorship Resistance**: Game state distributed across Nostr relays
5. **Stateless Clients**: App can be restarted/refreshed without losing state
6. **Interoperability**: Any Nostr client can read game state
7. **Scalability**: No server state to manage or synchronize
8. **Privacy**: Cashu provides privacy for financial transactions
9. **Auditability**: All game moves are publicly verifiable on Nostr
10. **Resilience**: Game continues even if some relays go offline

## Implementation Priority

### Immediate (Fixes Current Issue)
1. **Remove all local game state sharing** - Fix the "both players see same state" problem
2. **Make UI query Nostr directly** - Each render reads fresh from Nostr events  
3. **Implement basic event publishing** - Create/accept challenges via Nostr events

### Short Term (Full Nostr Integration)
4. **Real Nostr event parsing** - Read challenges and matches from actual events
5. **Event-driven state reconstruction** - Build match state from event history
6. **Basic match flow** - Challenge → Accept → Combat → Result via Nostr

### Medium Term (Cashu Integration)  
7. **Cashu wallet integration** - Lock/release tokens for wagers
8. **Economic game flow** - Winners receive sats automatically
9. **Escrow mechanisms** - Handle draws, timeouts, disputes

### Long Term (Polish)
10. **Performance optimization** - Smart caching, rate limiting
11. **Advanced features** - Tournaments, rankings, achievements
12. **Multi-mint support** - Support different Cashu mints

## Current Status

- 🚧 **In Progress**: Removing shared local state
- ❌ **CRITICAL**: Players currently share state instead of reading from Nostr
- ❌ **TODO**: Implement stateless architecture 
- ❌ **TODO**: Real Nostr event integration
- ❌ **TODO**: Cashu wallet integration

## The Core Issue

**Right now**: Both players manipulate the same `global_matches` HashMap
**Should be**: Each player queries Nostr independently and sees their own view

This is why you see synchronized state - it's because it literally IS the same state object in memory, not separate clients reading from a shared Nostr feed.