# Stateless Manastr Implementation Plan

## Goal
Create a stateless client that operates like the integration tests - each player queries Nostr independently, no shared local state, all communication through Nostr events.

## Step-by-Step Implementation

### Phase 1: Remove Shared State (Foundation)
1. **Strip down Manastr struct**
   - Remove `global_matches` HashMap
   - Remove `account_data` with persistent state
   - Keep only: `screen`, `current_match_id`, `nostr_client`, UI state

2. **Create stateless data structures**
   - `MatchView` - reconstructed from Nostr events each time
   - `ChallengeView` - parsed from challenge events
   - `UICache` - temporary cache, expires every 30 seconds

3. **Update UI to query-on-render**
   - Matchmaking screen queries challenges in real-time
   - Active match screen reconstructs state from events
   - No persistent storage between renders

### Phase 2: Basic Nostr Event Publishing
4. **Implement real challenge creation**
   - Publish KIND_MATCH_CHALLENGE with proper tags
   - Include match_id, wager, league, army_hash
   - Test with two notedeck accounts

5. **Implement challenge acceptance**
   - Publish KIND_MATCH_ACCEPTANCE event
   - Reference original challenge event
   - Include acceptor army configuration

6. **Basic event publishing infrastructure**
   - Use real nostrdb publishing (not placeholders)
   - Proper event signing with account keys
   - Error handling for network failures

### Phase 3: Nostr Event Reading & Parsing
7. **Query available challenges**
   - Real nostrdb Filter for KIND_MATCH_CHALLENGE
   - Parse event content and tags
   - Filter out own challenges

8. **Query match events by ID**
   - Get all events for specific match_id
   - Sort chronologically by created_at
   - Handle missing/partial event sets

9. **State reconstruction from events**
   - Parse challenge → accepted → combat → result flow
   - Determine current match phase
   - Identify player roles (challenger vs acceptor)

### Phase 4: Combat Events
10. **Combat move publishing**
    - KIND_COMBAT_MOVE events for each action
    - Include round number, unit positions, abilities
    - Commit-reveal scheme for simultaneous moves

11. **Combat state reconstruction**
    - Parse combat moves chronologically
    - Apply game rules to determine outcomes
    - Handle incomplete/missing moves

12. **Match completion**
    - KIND_MATCH_RESULT event for game end
    - Include winner, final scores, loot distribution

### Phase 5: Integration & Testing
13. **Two-player testing flow**
    - Account A creates challenge
    - Account B sees and accepts challenge
    - Both players take combat turns
    - Match completes with winner declared

14. **Error handling & edge cases**
    - Network failures during event publishing
    - Missing events from other players
    - Invalid/malformed events
    - Timeout handling for unresponsive players

15. **Performance optimization**
    - Cache recent events to avoid re-querying
    - Batch queries where possible
    - Rate limiting for UI updates

## Implementation Details

### Data Flow (Like Integration Tests)
```rust
// Player A (Challenger)
1. Click "Create Challenge" 
2. → Publish KIND_MATCH_CHALLENGE event to Nostr
3. → UI shows "Waiting for opponent" (queries own events)

// Player B (Acceptor) 
4. Open matchmaking screen
5. → Query Nostr for KIND_MATCH_CHALLENGE events
6. → See Player A's challenge, click "Accept"
7. → Publish KIND_MATCH_ACCEPTANCE event

// Both Players
8. UI polls Nostr every 5 seconds
9. → Query events for match_id
10. → Reconstruct current match state
11. → Update UI based on reconstructed state
```

### Key Principles
1. **No persistent local state** - Everything from Nostr
2. **Query on every render** - Fresh state always
3. **Pure functions** - Event parsing is stateless
4. **Independent clients** - Each player queries separately
5. **Event-driven** - All communication via Nostr events

### File Changes Required
- `src/lib.rs` - Strip down Manastr struct, remove shared state
- `src/nostr_client.rs` - Implement real nostrdb integration  
- `src/match_state.rs` - Add pure reconstruction functions
- `src/event_parser.rs` - New module for parsing Nostr events
- Integration tests as reference for expected behavior

### Success Criteria
- [ ] Two different accounts can play independently
- [ ] No shared memory state between players
- [ ] All game state reconstructed from Nostr events
- [ ] Player A's moves appear to Player B only via Nostr
- [ ] Client can restart and resume from Nostr state
- [ ] Works exactly like integration_tests but in real UI

## Next Steps
Start with Phase 1: Remove all shared state and implement query-on-render UI pattern. This will immediately fix the "both players see same state" issue.