# WASM Shared Game Logic Specification

**Status**: Architecture Complete  
**Dependencies**: Game Engine Bot (D3), Web Client (D4)  
**Purpose**: Ensure perfect synchronization between client and server game logic

## Overview

The WASM Shared Game Logic crate contains the core battle mechanics, unit generation, and game state logic compiled to WebAssembly. This approach eliminates client-server desynchronization by using identical logic on both sides.

## Architecture Benefits

### 1. Perfect Synchronization
- **Client-side Prediction**: Immediate visual feedback using WASM
- **Server Authority**: Identical logic ensures server results match client predictions
- **No Desync Issues**: Deterministic logic prevents state conflicts
- **Reduced Network Traffic**: Client can predict outcomes without server round-trips

### 2. Code Reuse
- **Single Implementation**: One Rust codebase for all game logic
- **Consistency**: No separate TypeScript implementation to maintain
- **Type Safety**: Rust's type system prevents logic bugs
- **Performance**: WASM provides near-native performance in browsers

### 3. Development Efficiency
- **Shared Types**: Data structures used by both client and server
- **Easy Testing**: Unit tests verify logic once for both platforms
- **Simplified Debugging**: Identical behavior across platforms
- **Hot Reloading**: WASM modules can be updated independently

## Technical Implementation

### Project Structure
```
shared-game-logic/
├── Cargo.toml              # WASM crate configuration
├── src/
│   ├── lib.rs             # WASM entry points and exports
│   ├── combat.rs          # Combat resolution logic
│   ├── units.rs           # Unit generation from tokens
│   ├── game_state.rs      # Shared data structures
│   ├── abilities.rs       # Unit abilities (Boost, Shield, Heal)
│   └── league.rs          # League modifiers and bonuses
├── pkg/                   # Generated WASM package (wasm-pack output)
└── tests/
    └── integration.rs     # Cross-platform logic tests
```

### Core WASM Exports

#### 1. Unit Generation
```rust
#[wasm_bindgen]
pub fn generate_units_from_token_secret(token_secret: &str, league_id: u8) -> JsValue {
    let units = units::generate_from_secret(token_secret, league_id);
    serde_wasm_bindgen::to_value(&units).unwrap()
}
```

**Purpose**: Generate 8 deterministic battle units from mana token secrets
**Input**: Token secret (string), League ID (0-15)
**Output**: Array of 8 Unit structs with stats and abilities
**Determinism**: Uses SHA256 to ensure identical results across platforms

#### 2. Combat Resolution
```rust
#[wasm_bindgen]
pub fn process_combat(
    unit1_js: JsValue, 
    unit2_js: JsValue, 
    player1_npub: &str, 
    player2_npub: &str
) -> JsValue {
    let unit1: Unit = serde_wasm_bindgen::from_value(unit1_js).unwrap();
    let unit2: Unit = serde_wasm_bindgen::from_value(unit2_js).unwrap();
    
    let result = combat::resolve_battle(unit1, unit2, player1_npub, player2_npub);
    serde_wasm_bindgen::to_value(&result).unwrap()
}
```

**Purpose**: Resolve combat between two units using identical server logic
**Input**: Two Unit structs, player NPubs
**Output**: RoundResult with final unit states, damage dealt, winner
**Abilities**: Handles Boost (2x attack), Shield (0 damage), Heal (50% restore)

#### 3. League Modifier Application
```rust
#[wasm_bindgen]
pub fn apply_league_modifiers(base_unit: JsValue, league_id: u8) -> JsValue {
    let mut unit: Unit = serde_wasm_bindgen::from_value(base_unit).unwrap();
    league::apply_modifiers(&mut unit, league_id);
    serde_wasm_bindgen::to_value(&unit).unwrap()
}
```

**Purpose**: Apply league-specific stat bonuses to units
**Input**: Base unit stats, league ID
**Output**: Modified unit with league bonuses applied
**Examples**: Fire (+10 ATK), Ice (+20 HP), Shadow (+5 ATK/DEF), Nature (+5 DEF/+15 HP)

### Shared Data Structures

#### Unit Definition
```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Unit {
    pub attack: u8,      // 10-29 base + league modifiers
    pub defense: u8,     // 5-19 base + league modifiers  
    pub health: u8,      // Current health (damage applied)
    pub max_health: u8,  // 20-49 base + league modifiers
    pub ability: Ability, // Special combat ability
}
```

#### Ability System
```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Ability {
    None,    // No special ability
    Boost,   // Double attack damage this round
    Shield,  // Take no damage this round
    Heal,    // Restore 50% max health after combat
}
```

#### Combat Result
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct RoundResult {
    pub round: u8,
    pub player1_unit: Unit,    // Final state after combat
    pub player2_unit: Unit,    // Final state after combat
    pub damage_dealt: [u8; 2], // [damage to unit2, damage to unit1]
    pub winner: Option<String>, // Winner's NPub or None for tie
}
```

## Integration Points

### Server-Side Integration (Game Engine Bot)
```rust
// In game-engine-bot/Cargo.toml
[dependencies]
shared-game-logic = { path = "../shared-game-logic" }

// In game-engine-bot/src/main.rs
use shared_game_logic::{combat, units, game_state::*};

impl GameEngineBot {
    pub async fn resolve_match_round(&self, match_id: &str, round: u8) -> Result<RoundResult> {
        let (reveal1, reveal2) = self.get_round_reveals(match_id, round).await?;
        
        // Use shared logic (not WASM, native Rust)
        let result = combat::process_combat(
            reveal1.unit,
            reveal2.unit, 
            &reveal1.player_npub,
            &reveal2.player_npub
        )?;
        
        Ok(result)
    }
}
```

### Client-Side Integration (Web Client)
```typescript
// In web-client/package.json dependencies
"@manastr/shared-game-logic": "file:../shared-game-logic/pkg"

// In web-client/src/services/battleEngine.ts
import init, { 
  generate_units_from_token_secret, 
  process_combat 
} from '@manastr/shared-game-logic';

export class BattleEngine {
  private static wasmInitialized = false;
  
  static async initialize() {
    if (!this.wasmInitialized) {
      await init();
      this.wasmInitialized = true;
    }
  }
  
  static generateUnits(tokenSecret: string, leagueId: number): Unit[] {
    const result = generate_units_from_token_secret(tokenSecret, leagueId);
    return JSON.parse(result);
  }
  
  static simulateCombat(unit1: Unit, unit2: Unit, player1: string, player2: string): RoundResult {
    const result = process_combat(
      JSON.stringify(unit1),
      JSON.stringify(unit2),
      player1,
      player2
    );
    return JSON.parse(result);
  }
}
```

## Build Process

### WASM Compilation
```bash
# In shared-game-logic directory
wasm-pack build --target web --out-dir pkg --scope manastr

# Generated files in pkg/:
# - shared_game_logic.js        # JavaScript bindings
# - shared_game_logic.d.ts      # TypeScript definitions  
# - shared_game_logic_bg.wasm   # WebAssembly binary
# - package.json                # NPM package configuration
```

### Development Workflow
```bash
# 1. Modify shared logic in Rust
vim shared-game-logic/src/combat.rs

# 2. Rebuild WASM package
cd shared-game-logic && wasm-pack build --target web

# 3. Update is automatically picked up by:
# - Game Engine Bot (native Rust dependency)
# - Web Client (WASM import from pkg/)

# 4. Test both platforms have identical behavior
cargo test -p shared-game-logic
npm test -p web-client
```

### Continuous Integration
```yaml
# .github/workflows/wasm-build.yml
name: WASM Build
on: [push, pull_request]

jobs:
  build-wasm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
      - name: Build WASM package
        run: cd shared-game-logic && wasm-pack build --target web
      
      - name: Test server logic
        run: cargo test -p game-engine-bot
      
      - name: Test client WASM integration
        run: cd web-client && npm install && npm test
```

## Performance Considerations

### WASM Optimization
- **Binary Size**: ~50KB compressed WASM binary for game logic
- **Load Time**: <100ms initialization on modern browsers
- **Execution Speed**: Near-native performance for combat calculations
- **Memory Usage**: Minimal allocation, mostly stack-based operations

### Caching Strategy
```typescript
// Client-side WASM module caching
const WASM_CACHE_KEY = 'manastr-game-logic-v1';

export async function loadCachedWasm(): Promise<WebAssembly.Module> {
  // Check IndexedDB cache first
  const cached = await getCachedWasmModule(WASM_CACHE_KEY);
  if (cached) return cached;
  
  // Load and cache new module
  const wasmModule = await WebAssembly.compileStreaming(
    fetch('/shared_game_logic_bg.wasm')
  );
  
  await cacheWasmModule(WASM_CACHE_KEY, wasmModule);
  return wasmModule;
}
```

## Testing Strategy

### Cross-Platform Verification
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic_unit_generation() {
        let secret = "test_token_secret_123";
        let league_id = 0;
        
        // Generate units multiple times
        let units1 = generate_units_from_token_secret(secret, league_id);
        let units2 = generate_units_from_token_secret(secret, league_id);
        
        // Must be identical
        assert_eq!(units1, units2);
    }
    
    #[test] 
    fn test_combat_consistency() {
        let unit1 = Unit { attack: 20, defense: 10, health: 50, max_health: 50, ability: Ability::None };
        let unit2 = Unit { attack: 15, defense: 5, health: 40, max_health: 40, ability: Ability::Shield };
        
        let result1 = process_combat(unit1, unit2, "player1", "player2");
        let result2 = process_combat(unit1, unit2, "player1", "player2");
        
        // Combat results must be deterministic
        assert_eq!(result1.damage_dealt, result2.damage_dealt);
        assert_eq!(result1.winner, result2.winner);
    }
}
```

### Integration Testing
```typescript
// web-client/tests/wasm-integration.test.ts
describe('WASM Game Logic Integration', () => {
  beforeAll(async () => {
    await BattleEngine.initialize();
  });
  
  test('generates identical units to server', async () => {
    const tokenSecret = 'test_secret_456';
    const leagueId = 1;
    
    // Generate units using WASM
    const clientUnits = BattleEngine.generateUnits(tokenSecret, leagueId);
    
    // Verify against known server results
    const expectedUnits = await fetchServerGeneratedUnits(tokenSecret, leagueId);
    
    expect(clientUnits).toEqual(expectedUnits);
  });
  
  test('combat results match server authority', async () => {
    const unit1 = { attack: 25, defense: 8, health: 45, max_health: 45, ability: 'Boost' };
    const unit2 = { attack: 18, defense: 12, health: 38, max_health: 38, ability: 'None' };
    
    // Simulate combat client-side
    const clientResult = BattleEngine.simulateCombat(unit1, unit2, 'player1', 'player2');
    
    // Compare with server result
    const serverResult = await fetchServerCombatResult(unit1, unit2);
    
    expect(clientResult.winner).toBe(serverResult.winner);
    expect(clientResult.damage_dealt).toEqual(serverResult.damage_dealt);
  });
});
```

## Security Considerations

### Deterministic Behavior
- **No Random Sources**: All randomness derived from token secrets
- **Consistent Hashing**: SHA256 ensures identical results across platforms  
- **No Side Channels**: Pure functions with no external dependencies
- **Audit Trail**: Every combat result can be reproduced from inputs

### WASM Security
- **Sandboxed Execution**: WASM runs in browser security context
- **No Network Access**: WASM modules cannot make external requests
- **Memory Isolation**: WASM has isolated linear memory space
- **Type Safety**: Rust's memory safety carries over to WASM

---

**Status**: Architecture complete, ready for implementation  
**Next Steps**: 
1. Implement shared-game-logic crate with WASM exports
2. Integrate into Game Engine Bot as native dependency
3. Integrate into Web Client as WASM module
4. Add comprehensive cross-platform testing