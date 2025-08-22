# Manastr Game Flow Test Guide

## Fixed Issues

1. **State Synchronization**: Matches now properly update in both local player data and global match pool
2. **Better UI Feedback**: Added clearer instructions and phase descriptions  
3. **Debug Information**: Added debugging info to help troubleshoot match issues
4. **Improved Flow**: Enhanced button descriptions and user guidance

## Testing the Game Flow

### Setup
1. Start notedeck with the manastr client integrated
2. Make sure you have at least 2 accounts/keys configured in notedeck

### Test Steps

#### Player 1 (Challenger):
1. Go to Manastr app in notedeck
2. Click "⚔️ Battle Arena" 
3. Configure wager (e.g., 100 sats) and league
4. Click "⚔️ Create Challenge"
5. **Should see**: Match in "Challenge" phase, waiting for opponent
6. Note the Match ID for sharing

#### Player 2 (Acceptor):
1. Switch to second account in notedeck
2. Go to Manastr → Battle Arena
3. **Should see**: The challenge in "Available Challenges" section
4. Click "⚡ Accept Challenge"
5. **Should see**: Match moves to "Accepted" phase
6. Click "🔓 Reveal Tokens"
7. **Should see**: Match moves to "TokensRevealed" phase
8. Click "🚀 Start Battle"
9. **Should see**: Match moves to "Combat" phase with round 1

#### Both Players:
- In Combat phase, you should see:
  - Combat interface with ability buttons (Boost, Shield, Heal)
  - Current round indicator
  - Turn-based combat options
  - Submit button to advance rounds

## Key Improvements Made

### 1. Fixed State Management
```rust
// Now updates both local and global match states
if let Some(player_data) = self.get_current_player_data_mut() {
    // Update local match
}
if let Some(global_match) = self.global_matches.get_mut(&match_id) {
    // Update global match - visible to all players
}
```

### 2. Enhanced UI Guidance
- Added helpful tooltips and instructions
- Better phase descriptions
- Debug information when matches can't be found
- Clear button labels explaining next steps

### 3. Debugging Support
- Added debug logging for match phase transitions
- Debug panel showing available matches when match not found
- Tracing logs for troubleshooting

## Expected Behavior

1. **Match Creation**: Creates challenge visible to all players
2. **Match Acceptance**: Moves match to "Accepted" phase for both players
3. **Token Reveal**: Each player clicks to reveal tokens (phase progression)
4. **Combat Start**: Click to begin turn-based combat
5. **Combat Rounds**: Take turns using abilities until match completion

## Troubleshooting

If the match still gets stuck:

1. **Check Debug Info**: Look at the debug panel when match not found
2. **Check Logs**: Look for tracing logs starting with "Manastr:"
3. **Account Switching**: Make sure you're switching accounts properly in notedeck
4. **Match IDs**: Verify the match ID is consistent between players

The key fix was ensuring that match state updates are applied to both the player's local active matches AND the global match pool, so all players can see the same match state.