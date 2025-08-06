# Mana Strategy Game - Claude Memory System

## Project Overview

**Manastr:** Revolutionary decentralized strategy game with zero-coordination multiplayer architecture
- **Core Innovation:** Players control entire match flow via cryptographic commitment/reveal schemes
- **Game Engine Role:** Pure validator - only validates outcomes and distributes loot
- **Architecture:** Player-driven decentralized game with Cashu+Nostr backend

### Project Structure
```
manastr/
├── docs/                    # ✅ Complete - Revolutionary player-driven architecture
│   └── TRADING_CARD_GAME_INTERFACE.md  # 🎮 iced.rs TCG interface design
├── daemons/                 # ✅ Implemented - Pure validation backend services
│   ├── game-engine-bot/     # ✅ Player-driven match validator with anti-cheat
│   ├── shared-game-logic/   # ✅ WASM-compatible deterministic game logic
│   ├── nostr-relay/         # ✅ Decentralized event coordination (strfry)
│   ├── cdk/                 # ✅ Official CDK submodule with full mint functionality
│   ├── config/              # ✅ Manastr-specific configurations for all services
│   ├── gaming-wallet/       # ✅ CDK extension for C value access
│   ├── manastr-web/         # 🚀 NEW - Revolutionary quantum web client
│   ├── cashu-ts/            # 📦 Cashu-TS library submodule  
│   ├── ndk/                 # 📦 NDK Nostr library submodule
│   └── integration_tests/   # ✅ Complete - Consolidated integration test suite
└── CLAUDE.md               # 📍 THIS FILE - Memory & status tracking
```

### Quick Commands
```bash
just build          # Build all components  
just build-web      # Build quantum web client
just web-dev        # Start quantum web client (dev mode)
just web            # Start web client + services
just integration    # Run complete system integration test (THE REFERENCE)
just dev            # Full development workflow (build + test + check)
just --list         # Show all available commands
```

**📍 INTEGRATION TESTING:**
Integration tests are consolidated in `daemons/integration_tests/` directory for comprehensive system validation.

## Revolutionary Architecture Status ✅

### Core Breakthroughs Complete
- **✅ Zero-Coordination Gaming:** Players control everything via cryptographically-secured Nostr events
- **✅ Pure Validator Engine:** Game engine cannot cheat - only validates player-submitted outcomes
- **✅ Cryptographic Anti-Cheat:** Commitment/reveal prevents all cheating without trusted authority
- **✅ Complete Decentralization:** No centralized matchmaking or coordination required
- **✅ Shared WASM Logic:** Client-server synchronization via identical Rust/WASM game logic
- **✅ Nostr-First Architecture:** All data uses Nostr format except CDK-required types

### Implementation Status
| Component | Status | Revolutionary Feature |
|-----------|--------|---------------------|
| **Player-Driven Match Flow** | ✅ Complete | 7 Nostr event types with commitment/reveal |
| **Game Engine State Machine** | ✅ Complete | Concurrent match tracking with formal transitions |
| **Shared WASM Logic** | ✅ Complete | Deterministic Rust logic for client-server sync |
| **Anti-Cheat System** | ✅ Complete | Cryptographic commitment verification |
| **Economic Model** | ✅ Complete | 95% player rewards, 5% system fee |
| **Integration Testing** | ✅ Complete | Air-tight player-driven test suite |
| **Rust-First Architecture** | ✅ Complete | Service orchestration via Rust, not shell scripts |
| **Mint Authorization** | ✅ Complete | Hot-swappable game engine authorization |

### 🎯 **CONSOLIDATED INTEGRATION TESTING**
**CRITICAL**: Integration tests are located in `daemons/integration_tests/` directory for consolidated testing. The integration test suite is the **definitive documentation** proving the revolutionary zero-coordination gaming architecture works.

## Core Architectural Principles

### 1. Cashu Token C Values for Army Generation 🏛️
**CRITICAL RULE**: Army units MUST be generated from Cashu token unblinded signature C values.

- **Mint-Provided Randomness**: Use only mint-generated C values, never player randomness
- **Anti-Cheat by Design**: Players cannot manipulate army composition
- **Deterministic Generation**: Same C value always generates same unit stats
- **Economic Model**: 1 mana token = 1 army (4 units) = 1 match capability

**Implementation** (256-bit C value chunked into 4 u64 seeds for 4 units):
```rust
// ✅ CORRECT: Use shared combat logic
use shared_game_logic::combat::generate_army_from_cashu_c_value;
let army = generate_army_from_cashu_c_value(c_value_bytes, league_id);
```

### 2. Real Nostr Events for All Communication 📡
**CRITICAL RULE**: ALL match communication MUST use real Nostr events with proper EventIds.

**7 Event Types for Complete Player-Driven Flow:**
- **Kind 31000**: Match Challenge (Player creates match)
- **Kind 31001**: Match Acceptance (Player accepts challenge)
- **Kind 31002**: Token Reveal (Player reveals Cashu tokens)
- **Kind 31003**: Move Commitment (Player commits to moves)
- **Kind 31004**: Move Reveal (Player reveals actual moves)
- **Kind 31005**: Match Result (Player submits final state)
- **Kind 31006**: Loot Distribution (Game Engine's ONLY authoritative event)

### 3. Nostr-First Data Architecture
**RULE**: All data types MUST use Nostr format except CDK-required types.
- **Player Identity**: Always use Nostr `PublicKey` and `SecretKey` types
- **Event Data**: All game data transmitted via Nostr events
- **Match Identification**: Use real EventIds from actual Nostr events

### 4. Economic Model (95% Player Rewards)
- **Player-Friendly**: Winners receive 95% of total mana wagered as loot tokens
- **Minimal Fees**: 5% system fee for operational sustainability
- **Edge Case Protection**: Minimum 2 mana total per match
- **Transparent Math**: `total_wager * 95 / 100 = loot_tokens`

## Game Engine as Pure Validator

### State Machine Architecture
**Match States:**
1. **Challenged** → 2. **Accepted** → 3. **InCombat** → 4. **AwaitingValidation** → 5. **Completed**
   - Alternative: **Invalid** (cheating detected)

### Critical Anti-Cheat Features
- **Commitment Verification**: SHA256 commitment/reveal validation
- **Mint Token Validation**: Query mint to prevent double-spending
- **Event Chain Integrity**: Chronological validation of all events
- **Shared Logic Re-execution**: Re-run combat using shared WASM

### Game Engine ONLY Authority
- **Validates** player-submitted outcomes (never coordinates)
- **Distributes** loot after successful validation
- **Burns** mana tokens via authorized Nostr signatures
- **Publishes** KIND 31006 loot distribution events

## Quantum Web Client Architecture

**TECHNOLOGY STACK**:
- **React 17**: Modern component architecture with hooks
- **Styled Components**: CSS-in-JS for dynamic quantum styling
- **Vite**: Lightning-fast development and optimized production builds
- **NDK**: Real Nostr client with full protocol support
- **Cashu-TS**: Authentic Cashu operations with C value access

**DESIGN PRINCIPLES**:
- **Sci-Fi Aesthetic**: Arwes-inspired quantum interface design
- **Real Integration**: No mocking - authentic library usage
- **Responsive**: Works on desktop, tablet, and mobile
- **Performance**: Optimized animations and lazy loading

**KEY FEATURES**:
1. **Quantum Visual Effects**: Scanning lines, pulsing glows, matrix animations
2. **Real-time Status**: Live connection monitoring for all services
3. **Terminal Logging**: Sci-fi style activity log with quantum terminology
4. **Touch-Friendly**: Mobile-optimized button sizes and interactions

## Gaming Wallet Strategy

**PRINCIPLE**: Maximize CDK API usage, minimize custom deviations.

**CDK-First Approach:**
- Use existing CDK APIs wherever possible
- Only extend where CDK doesn't expose needed data (C values)
- Document deviations clearly for future compatibility

**Required Extensions:**
1. **C Value Access**: Extract unblinded signature C values for unit generation
2. **Gaming Token Metadata**: Track C values alongside CDK proof structure

## Implementation Notes

### Economic Model Usage
```rust
use crate::economic_model::EconomicModel;
let distribution = EconomicModel::calculate_loot_distribution(total_wager);
assert!(EconomicModel::validate_distribution(&distribution));
```

### Integration Test Priority
- **Use integration test as acceptance criteria** for all changes
- **9-phase flow** must complete successfully with economic validation
- **Focus on player-driven match lifecycle**

### 🔑 **CRITICAL INTEGRATION TEST PRINCIPLE**: No Mocking in Production Tests

**RULE**: Integration tests MUST use all real crates and daemons exactly as they run in production, relying on deterministic cryptography for reproducible testing.

**Why This Matters:**
- **Real System Testing**: Integration tests validate the actual production system behavior
- **Cryptographic Determinism**: Real mints with controlled keys provide deterministic C values
- **No Mock Fragility**: Mocks break when real APIs change - real services don't
- **True Integration**: Tests prove all components work together correctly
- **Production Confidence**: What works in tests works in production

**Implementation Requirements:**
- ✅ **Real CDK Mint**: Use official cdk-mintd with fake Lightning backend for testing
- ✅ **Real Nostr Relay**: Use actual nostr-rs-relay, not mock events
- ✅ **Real Game Engine**: Use actual state machine with real Nostr processing
- ✅ **Real Cryptography**: Use deterministic mint keys for reproducible C values
- ⚠️ **No Mocking**: Never mock CDK APIs, Nostr events, or cryptographic operations

**Deterministic Testing Strategy:**
- **Controlled Mint Keys**: Use fixed mnemonic/seed for deterministic signatures
- **Predictable Inputs**: Use fixed x values (blind factors) for deterministic C values
- **Real C Values**: Actual mint signatures provide authentic randomness for army generation
- **Reproducible Tests**: Same inputs always produce same outputs across test runs

**Examples:**
```rust
// ✅ CORRECT: Real CDK mint with deterministic configuration
let mint_config = MintConfig {
    mnemonic: "abandon abandon abandon...", // Fixed seed for deterministic keys
    fake_wallet: true, // Auto-fills Lightning quotes for testing
};

// ✅ CORRECT: Real army generation from real C values
let c_value_bytes = real_cashu_token.get_c_value(); // From actual mint operation
let army = generate_army_from_cashu_c_value(c_value_bytes, league_id);

// ❌ WRONG: Mocking CDK or using fake C values
let fake_c_value = [0u8; 32]; // This doesn't test real cryptography!
let army = generate_army_from_cashu_c_value(&fake_c_value, league_id);
```

This principle ensures integration tests validate the complete production system with authentic cryptographic operations.

### Army Generation Constraint
- **Minimum 2 mana total** per match (1 per player for armies)
- **System panics** for invalid wagers < 2 mana
- **UI should prevent** invalid wagers

## Revolutionary Achievement Summary

### Architectural Breakthrough: Zero-Coordination Gaming
**Problem Solved**: Traditional multiplayer games require trusted central servers that can manipulate outcomes.

**Revolutionary Solution**: 
- **Players control everything** via cryptographically-secured Nostr events
- **Game engine cannot cheat** - only validates player-submitted outcomes
- **Perfect decentralization** aligned with Bitcoin/Nostr principles

### Technical Innovation Impact
- **🎯 Zero Trust Required**: Players don't need to trust the game engine
- **🔒 Cryptographically Secure**: Commitment/reveal prevents all cheating
- **📡 Fully Decentralized**: No central authority controls match flow
- **⚡ Future-Proof**: Architecture supports complex tournament formats

**This is not just a game - it's a new paradigm for decentralized multiplayer gaming that could revolutionize the industry.**

## Integration Tests (Separate Repository)

**🏗️ MOVED TO SEPARATE REPO:** Integration tests are now maintained independently and demonstrate:
1. **Complete 9-Phase Player-Driven Match Flow** - All 7 Nostr event types
2. **Optimized Economics** - 95% player rewards validated
3. **Revolutionary Architecture** - Zero-coordination gaming operational
4. **Cryptographic Security** - Anti-cheat working in real scenarios
5. **Service Orchestration** - All services coordinated via Rust
6. **Real CDK Integration** - Authentic Cashu token C values for deterministic army generation

**Benefits of Separate Repository:**
- **Better CI/CD**: Independent testing pipeline
- **Modular Development**: Core services and tests can evolve independently
- **Cleaner Architecture**: Separation of concerns between implementation and validation

---

**Status**: Revolutionary zero-coordination multiplayer gaming architecture **COMPLETE AND OPERATIONAL** with canonical reference implementation! 🎮✨

## 🚀 REVOLUTIONARY QUANTUM WEB CLIENT

**COMPLETED**: React-based sci-fi web interface operational with `just web-dev` command:
- **Futuristic UI**: Arwes-inspired quantum aesthetic with cyber animations
- **React Architecture**: Modern component-based structure with Styled Components
- **Real Integrations**: Authentic NDK Nostr client and Cashu-TS wallet
- **Sci-Fi Effects**: Pulsing glows, scanning lines, quantum terminology
- **Responsive Design**: Works on all devices with touch-friendly controls
- **Hot Reload**: Instant development feedback with Vite build system

## 🎮 Interactive Gaming Interfaces

**Multiple Interface Options**:
- **Quantum Web Client** (`just web-dev`): Modern React sci-fi interface
- **Trading Card Game** (`just play`): iced.rs-based educational interface

Both interfaces provide:
- **Service Orchestration**: Automatic backend startup, health checking, and cleanup
- **Real Backend Integration**: All game actions execute authentic Nostr/CDK operations
- **Complete Experience**: Full 9-phase match flow implementation

## 🚀 STRATEGIC EVOLUTION: BEVY + MATCHBOX INTEGRATION

**Next Phase**: Professional game engine upgrade while preserving all cryptographic security:

**New Architecture**:
- **Foundation**: Keep all Manastr security (Nostr events, commitment/reveal, Cashu tokens)
- **Engine**: Replace iced.rs with Bevy for professional game capabilities  
- **Networking**: Add matchbox_nostr for WebRTC P2P with graceful Nostr fallback
- **UI**: Implement bevy_lunex responsive layouts for modern interface design

**Strategic Document**: `BEVY_INTEGRATION_STRATEGY.md` - Complete implementation roadmap

**Revolutionary Outcome**: World's first secure + responsive decentralized game that rivals centralized alternatives in UX while surpassing them in security and player ownership.