# Efficient Stateless Manastr Design

## Key Insights from Notedeck Investigation

### Notedeck's Event Architecture:
1. **Shared ndb**: All apps use the same nostrdb instance - very efficient for cross-app caching
2. **60fps update cycle**: Apps query data in their `update()` method every frame
3. **Transaction-based queries**: Each query uses `Transaction::new(ndb)` for consistency
4. **Smart caching**: NoteCache prevents recomputation of processed notes
5. **Subscription management**: Long-lived subscriptions for feeds, one-off queries for specific data

## Efficient Stateless Manastr Strategy

### Leverage Existing Infrastructure ✅
```rust
impl Manastr {
    fn update(&mut self, ctx: &mut AppContext<'_>, ui: &mut Ui) -> Option<AppAction> {
        // Use the same ndb that Dave and columns use - very efficient!
        let txn = Transaction::new(ctx.ndb).unwrap();
        
        match self.screen {
            Screen::MatchMaking => self.render_matchmaking(ctx, &txn, ui),
            Screen::ActiveMatch(id) => self.render_match(ctx, &txn, &id, ui),
            // ...
        }
    }
    
    fn render_matchmaking(&mut self, ctx: &AppContext, txn: &Transaction, ui: &mut Ui) {
        // Query fresh each render - ndb caching makes this efficient
        let challenges = self.query_available_challenges(txn, ctx.ndb);
        let my_matches = self.query_my_matches(txn, ctx.ndb);
        
        // Render directly from query results - no local state
        for challenge in challenges {
            ui.button(format!("Challenge: {} sats", challenge.wager));
        }
    }
}
```

### Smart Caching Strategy 🎯
```rust
impl Manastr {
    fn query_available_challenges(&self, txn: &Transaction, ndb: &Ndb) -> Vec<ChallengeView> {
        // Frequent queries are OK - ndb caches results internally
        let filter = Filter::new()
            .kinds([KIND_MATCH_CHALLENGE])
            .since(Timestamp::now() - 3600) // Last hour only
            .limit(50)
            .build();
            
        let results = ndb.query(txn, [filter], 1000).unwrap_or_default();
        
        results.into_iter()
            .filter_map(|qr| parse_challenge_event(&qr.note))
            .collect()
    }
    
    fn query_match_state(&self, txn: &Transaction, ndb: &Ndb, match_id: &str) -> Option<MatchView> {
        // Query all events for this match
        let filter = Filter::new()
            .kinds([KIND_MATCH_CHALLENGE, KIND_MATCH_ACCEPTANCE, KIND_COMBAT_MOVE, KIND_MATCH_RESULT])
            .generic_tags([("match_id", match_id)])
            .build();
            
        let events = ndb.query(txn, [filter], 1000).unwrap_or_default();
        reconstruct_match_from_events(events, self.get_current_pubkey(ctx))
    }
}
```

## Performance Benefits

### Why This is Efficient:
1. **Shared ndb cache**: When Dave queries events, Manastr benefits from cached results
2. **Transaction reuse**: Single transaction per render cycle
3. **Smart filtering**: Only query recent events, specific match IDs
4. **No duplicate state**: Events stored once in ndb, not duplicated in app state
5. **Real-time fresh data**: Always see latest state from network

### Query Frequency Strategy:
```rust
// High frequency (every frame): UI state reconstruction
- Current match state from events
- Available challenges list
- Match phase determination

// Medium frequency (every few seconds): Background sync  
- New challenge discovery
- Match updates from opponents
- Combat move synchronization

// Low frequency (on user action): Event publishing
- Create challenge
- Accept match
- Submit combat moves
```

## Implementation Plan

### Phase 1: Use Existing ndb Infrastructure ✅
1. Remove all local game state storage
2. Query ndb directly in render methods using transactions
3. Parse events into view objects on-demand
4. Leverage notedeck's existing caching and subscription system

### Phase 2: Smart Event Management
1. Subscribe to relevant event kinds via existing subscription system
2. Use ndb's built-in caching for frequently accessed data
3. Implement pure functions for event parsing and state reconstruction

### Phase 3: Optimize Query Patterns
1. Batch related queries in single transaction
2. Use time-based filtering to limit query scope
3. Implement incremental event processing for large matches

## Code Structure

```rust
// Stateless - no game state stored
pub struct Manastr {
    screen: Screen,
    current_match_id: Option<String>,
    // UI state only
}

// Pure functions - no side effects
fn query_challenges(txn: &Transaction, ndb: &Ndb, league: u8) -> Vec<ChallengeView>
fn query_match_events(txn: &Transaction, ndb: &Ndb, match_id: &str) -> Vec<NostrEvent>
fn reconstruct_match(events: Vec<NostrEvent>, my_pubkey: &str) -> MatchView

// Event-driven rendering
fn render_matchmaking(&self, txn: &Transaction, ndb: &Ndb, ui: &mut Ui) {
    let challenges = query_challenges(txn, ndb, selected_league);
    // Render UI directly from query results
}
```

This approach gives us:
- ✅ True stateless architecture (fixes shared state issue)
- ✅ Leverages notedeck's efficient caching infrastructure  
- ✅ Real-time data always fresh from Nostr
- ✅ No duplicate storage or complex state management
- ✅ Works exactly like integration tests but in live UI