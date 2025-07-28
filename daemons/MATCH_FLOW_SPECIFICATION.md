# Manastr Match Flow Specification - Player-Driven Commitment/Reveal

## Overview

The Manastr game uses a **completely decentralized** approach where:
- **Players** initiate, wager, and execute matches via Nostr events
- **Game Engine Bot** only validates outcomes and issues Loot rewards
- **All game state** is transmitted through signed Nostr events
- **Commitment/Reveal scheme** ensures fair play without centralized coordination

## Future Enhancement Opportunities

### Multi-Round Wager System (Future Implementation)
Instead of single token wagers, players could commit multiple mana amounts for:
- **Multi-match tournaments**: Wager covers N matches, winner determined by best-of-N
- **Progressive wagers**: Each round escalates the stakes
- **Custom victory conditions**: Players define winning criteria in commitment schema

Example commitment structure for future:
```rust
pub struct EnhancedMatchChallenge {
    pub total_wager: u64,          // Total mana at stake
    pub round_count: u8,           // Number of rounds/matches to play  
    pub victory_condition: String,  // "best_of_5", "total_damage", "survival", etc.
    pub escalation_rule: String,   // "linear", "exponential", "fixed", etc.
    // ... existing fields
}
```

### Mid-Match Status Reporting (Future Enhancement)
While clients handle all match resolution, optional status reporting could provide:
- **Live match updates** for spectators/analytics
- **Progress tracking** for long tournaments
- **Health checks** to detect abandoned matches

This is **not critical** for MVP since focus is client-driven resolution with authoritative reward validation.

---

## Match Flow Phases

### Phase 1: Match Creation (Player-Initiated)

**Actor**: Player 1 (Challenge Creator)

1. **Player 1 creates match challenge**:
   ```typescript
   // Nostr Event Kind: 31000 (Match Challenge)
   {
     kind: 31000,
     tags: [
       ["d", match_id],           // Replaceable event identifier
       ["wager", "100"],          // Mana amount wagered
       ["league", "0"],           // League ID (0-15)
       ["expires", "1690000000"]  // Challenge expiration timestamp
     ],
     content: JSON.stringify({
       challenger_npub: player1_npub,
       wager_amount: 100,
       league_id: 0,
       cashu_token_commitment: hash(cashu_token_secrets), // Commitment to tokens
       army_commitment: hash(army_data + nonce),          // Commitment to army
       created_at: timestamp
     }),
     pubkey: player1_npub,
     // ... standard Nostr signature
   }
   ```

### Phase 2: Match Acceptance (Player Response)

**Actor**: Player 2 (Challenge Acceptor)

2. **Player 2 accepts challenge**:
   ```typescript
   // Nostr Event Kind: 31001 (Match Acceptance)
   {
     kind: 31001,
     tags: [
       ["e", match_challenge_event_id], // References challenge event
       ["p", player1_npub],             // Mentions challenger
       ["wager", "100"]                 // Confirms wager amount
     ],
     content: JSON.stringify({
       acceptor_npub: player2_npub,
       match_id: match_id,
       cashu_token_commitment: hash(cashu_token_secrets), // Player 2's token commitment
       army_commitment: hash(army_data + nonce),          // Player 2's army commitment
       accepted_at: timestamp
     }),
     pubkey: player2_npub,
     // ... standard Nostr signature
   }
   ```

### Phase 3: Token Revelation (Both Players)

**Actors**: Both Players

3. **Both players reveal their Cashu tokens**:
   ```typescript
   // Nostr Event Kind: 31002 (Token Reveal)
   {
     kind: 31002,
     tags: [
       ["e", match_acceptance_event_id], // References match
       ["p", other_player_npub],         // Mentions opponent
       ["phase", "token_reveal"]
     ],
     content: JSON.stringify({
       player_npub: player_npub,
       match_id: match_id,
       cashu_tokens: [cashu_token_1, cashu_token_2, ...], // Actual tokens
       token_secrets_nonce: nonce,  // Nonce used in commitment
       revealed_at: timestamp
     }),
     pubkey: player_npub,
     // ... standard Nostr signature
   }
   ```

### Phase 4: Army Generation & Commitment Verification

**Actors**: Both Players + Game Engine Bot (Validation)

4. **Players generate armies and verify commitments**:
   ```typescript
   // Each player generates army using shared-game-logic WASM
   const army = generateUnitsFromTokenSecrets(cashu_token_secrets, league_id);
   
   // Verify opponent's commitments match revealed data
   const opponent_token_commitment = hash(opponent_cashu_tokens);
   const opponent_army_commitment = hash(opponent_army + opponent_nonce);
   
   // Both must match the original commitments or match is invalid
   ```

5. **Game Engine Bot validates commitments** (listening to Nostr):
   ```typescript
   // Game Engine verifies:
   // - Both players revealed valid Cashu tokens
   // - Token commitments match revelations
   // - Cashu tokens are valid (not double-spent)
   // - Army generation is deterministic from token secrets
   ```

### Phase 5: Round-by-Round Combat (Player Moves)

**Actors**: Both Players

6. **For each combat round (typically 3-5 rounds)**:
   
   **6a. Move Commitment**:
   ```typescript
   // Nostr Event Kind: 31003 (Move Commitment)
   {
     kind: 31003,
     tags: [
       ["e", match_acceptance_event_id],
       ["p", opponent_npub],
       ["round", "1"],
       ["phase", "move_commit"]
     ],
     content: JSON.stringify({
       player_npub: player_npub,
       match_id: match_id,
       round_number: 1,
       move_commitment: hash(unit_positions + unit_abilities + nonce), // Commit to moves
       committed_at: timestamp
     }),
     pubkey: player_npub
   }
   ```

   **6b. Move Revelation** (after both players committed):
   ```typescript
   // Nostr Event Kind: 31004 (Move Reveal)
   {
     kind: 31004,
     tags: [
       ["e", match_acceptance_event_id],
       ["p", opponent_npub],
       ["round", "1"],
       ["phase", "move_reveal"]
     ],
     content: JSON.stringify({
       player_npub: player_npub,
       match_id: match_id,
       round_number: 1,
       unit_positions: [pos1, pos2, ...],    // Actual unit positions
       unit_abilities: [ability1, ability2, ...], // Abilities used this round
       moves_nonce: nonce,                   // Nonce from commitment
       revealed_at: timestamp
     }),
     pubkey: player_npub
   }
   ```

   **6c. Round Resolution** (deterministic, both players compute):
   ```typescript
   // Both players run identical WASM combat logic
   const round_result = processCombat(
     player1_units, player1_moves,
     player2_units, player2_moves,
     round_number
   );
   
   // Round result is deterministic - both players get same outcome
   ```

### Phase 6: Match Completion (Final Validation)

**Actors**: Both Players + Game Engine Bot

7. **Match outcome determination**:
   ```typescript
   // Both players publish final match state
   // Nostr Event Kind: 31005 (Match Result)
   {
     kind: 31005,
     tags: [
       ["e", match_acceptance_event_id],
       ["p", opponent_npub],
       ["winner", winner_npub || "draw"],
       ["phase", "match_complete"]
     ],
     content: JSON.stringify({
       player_npub: player_npub,
       match_id: match_id,
       final_army_state: final_units,
       all_round_results: [round1, round2, round3],
       calculated_winner: winner_npub,
       match_completed_at: timestamp
     }),
     pubkey: player_npub
   }
   ```

8. **Game Engine Bot validates complete match**:
   ```typescript
   // Game Engine validates entire match flow:
   // - All Nostr events are properly signed
   // - Commitments match revelations in every round  
   // - Combat calculations are correct (re-runs WASM logic)
   // - Winner determination is accurate
   // - Both players agree on outcome
   ```

### Phase 7: Loot Distribution (Game Engine Authority)

**Actor**: Game Engine Bot (Only step requiring authority)

9. **Game Engine issues Loot to winner**:
   ```typescript
   // ONLY if validation passes, Game Engine:
   // 1. Mints Loot tokens for winner
   // 2. Publishes loot distribution event
   
   // Nostr Event Kind: 31006 (Loot Distribution)
   {
     kind: 31006,
     tags: [
       ["e", match_acceptance_event_id],
       ["p", winner_npub],
       ["loot_amount", "95"], // 100 mana - 5% fee
       ["match_id", match_id]
     ],
     content: JSON.stringify({
       game_engine_npub: bot_npub,
       match_id: match_id,
       winner_npub: winner_npub,
       loot_cashu_token: loot_token,     // Actual Loot token for winner
       match_fee: 5,                     // 5% fee taken
       loot_issued_at: timestamp,
       validation_summary: {
         commitments_valid: true,
         combat_verified: true,
         signatures_valid: true,
         winner_confirmed: true
       }
     }),
     pubkey: game_engine_bot_npub
   }
   ```

## Key Validation Rules

### Commitment Verification
```typescript
// All commitments must be verifiable:
const token_commitment_valid = (
  hash(revealed_tokens) === original_token_commitment
);

const army_commitment_valid = (
  hash(generated_army + nonce) === original_army_commitment  
);

const move_commitment_valid = (
  hash(revealed_moves + nonce) === round_move_commitment
);
```

### Combat Determinism
```typescript
// Combat must be deterministic - same inputs = same outputs
const combat_valid = (
  player_calculated_result === bot_calculated_result &&
  shared_wasm_logic.processCombat(inputs) === expected_result
);
```

### Signature Chain Validation
```typescript
// All events must be properly signed and reference previous events
const signature_chain_valid = (
  challenge.verify_signature() &&
  acceptance.references(challenge) &&
  acceptance.verify_signature() &&
  token_reveals.both_reference(acceptance) &&
  // ... all subsequent events properly chained
);
```

## Nostr Event Types Summary

| Kind  | Name | Purpose | Required Fields |
|-------|------|---------|----------------|
| 31000 | Match Challenge | Player creates match | wager, league, army_commitment |
| 31001 | Match Acceptance | Player accepts challenge | match_ref, army_commitment |
| 31002 | Token Reveal | Reveal Cashu tokens | cashu_tokens, nonce |
| 31003 | Move Commitment | Commit to round moves | round, move_commitment |
| 31004 | Move Reveal | Reveal round moves | round, positions, abilities, nonce |
| 31005 | Match Result | Final match outcome | winner, round_results |
| 31006 | Loot Distribution | Game Engine issues Loot | loot_token, validation |

## Security Properties

### Prevents Cheating
- **No move changes**: Commitments prevent players from changing moves after seeing opponent's
- **No army manipulation**: Army commitments prevent changing units after opponent reveals
- **No double-spending**: Game Engine validates Cashu tokens haven't been spent elsewhere
- **No result manipulation**: Combat is deterministic using shared WASM logic

### Ensures Fairness  
- **Both players commit first**: No player has information advantage
- **All moves are signed**: Can't deny making moves or claim different moves
- **Public validation**: Anyone can verify match was played fairly
- **Automated loot distribution**: No human intervention in rewards

### Maintains Decentralization
- **Players drive all gameplay**: Game Engine only validates and rewards
- **No single point of failure**: Match continues even if Game Engine is offline temporarily
- **Censorship resistant**: Uses Nostr's decentralized relay network
- **Open source validation**: Anyone can run validation logic

## Implementation Requirements

### Client Requirements (Player Implementation)
- [ ] Cashu wallet integration for token management
- [ ] WASM module for shared combat logic
- [ ] Nostr client for event publishing/subscribing  
- [ ] Commitment/reveal cryptographic functions
- [ ] Army generation from token secrets
- [ ] Round-by-round move commitment UI

### Game Engine Bot Requirements (Validator Implementation)
- [ ] Nostr relay monitoring for all match events
- [ ] Cashu token validation (not double-spent)
- [ ] WASM combat logic (identical to client)
- [ ] Commitment verification functions  
- [ ] Complete match replay capability
- [ ] Loot token minting and distribution

### Integration Test Requirements  
- [ ] Full commitment/reveal flow simulation
- [ ] Multiple concurrent matches
- [ ] Invalid commitment detection
- [ ] Combat result verification
- [ ] Loot distribution validation
- [ ] Edge case handling (timeouts, invalid moves, etc.)

This specification ensures true decentralization while maintaining game integrity through cryptographic commitments and deterministic combat resolution.