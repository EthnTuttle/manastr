# Mana Strategy Game 🚀

**Revolutionary decentralized multiplayer game** where players have complete control over matches via cryptographically-secured Nostr events. The first truly trustless gaming experience.

## 🎯 Revolutionary Architecture

**Zero-Coordination Gaming**: Players drive the entire match flow while the game engine only validates outcomes - no central authority required!

### ✅ What Makes This Revolutionary
- **Players control everything** - create matches, wager, execute gameplay via Nostr events
- **Game engine cannot cheat** - only validates player-submitted outcomes and distributes rewards
- **Cryptographic anti-cheat** - commitment/reveal scheme prevents all forms of cheating
- **Perfect decentralization** - aligned with Bitcoin/Nostr principles, no trusted third parties

## 🏗️ Architecture Overview

- **Game Engine Bot** (:4444) - Pure validator, zero coordination authority
- **Shared WASM Logic** - Deterministic game logic for client-server synchronization  
- **Nostr Relay** (:7777) - Decentralized event coordination using `strfry`
- **Pure CDK Mint** (:3333) - Standard Cashu protocol with dual currencies (mana/loot)
- **Web Client** (:8080) - Player-driven interface with WASM integration

## 🎮 Player-Driven Match Flow

1. **Challenge** - Player creates match via Nostr (kind 31000)
2. **Accept** - Another player accepts challenge (kind 31001)  
3. **Commit** - Players commit to token secrets and army choices
4. **Reveal** - Players reveal actual data for validation (kinds 31002-31004)
5. **Validate** - Game engine validates all commitments and distributes loot (kind 31006)

**No game engine coordination required** - players control the entire process!

## 💰 Token Economics

- **Dual Currency System**: "mana" (game currency) and "loot" (rewards)
- **5 mana per sat** - Purchase mana via Lightning for gameplay
- **Loot distribution** - Winners receive loot tokens from match fees
- **Pure Cashu protocol** - Standard NUT implementations, no game logic in mint

## 🎮 Cryptographic Anti-Cheat System

### Commitment/Reveal Protocol
1. **Commit Phase**: Players publish SHA256 hashes of their secret data
2. **Reveal Phase**: Players reveal actual data for validation
3. **Validation**: Game engine verifies reveals match commitments
4. **Anti-Cheat**: Any mismatch invalidates the match automatically

**Result**: Impossible to cheat without detection, no trusted authority required!

## 🚀 Development Status

**✅ Revolutionary Architecture Complete!**
- **Player-Driven Match System**: 7 Nostr event types implemented
- **Pure Validation Engine**: Game engine refactored to zero-coordination
- **Cryptographic Security**: Real-time anti-cheat with commitment verification
- **Shared WASM Logic**: Deterministic client-server synchronization

**⏳ Next: Complete pure CDK mint and WASM web client**

## 📁 Project Structure

```
manastr/
├── daemons/
│   ├── game-engine-bot/    # ✅ Pure validator with anti-cheat
│   ├── shared-game-logic/  # ✅ WASM-compatible deterministic logic  
│   ├── nostr-relay/        # ✅ Decentralized event coordination
│   └── cashu-mint/         # ⏳ Pure CDK dual-currency implementation
├── docs/                   # ✅ Revolutionary architecture specifications
└── CLAUDE.md              # ✅ Complete breakthrough documentation
```

**This represents a fundamental breakthrough in decentralized multiplayer gaming!** 🎯✨