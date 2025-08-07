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
- **Quantum Web Client** (:8080) - Revolutionary React-based sci-fi interface

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

## ✅ **START HERE: INTEGRATION TESTS ARE THE REFERENCE**

**CRITICAL**: The integration tests in `daemons/integration_tests/` are the **definitive documentation** and **canonical reference implementation** of how Manastr works.

### 🚀 **Quick Start - Run the Integration Tests**

```bash
# 1. Build all services (including quantum web client)
just build-all

# 2. Start the quantum web client (dev mode)
just web-dev

# 3. Run the complete integration test suite (THE REFERENCE)
just integration
```

**Complete System Options:**
```bash
just serve        # 🚀 COMPLETE SYSTEM - Build all + run all services + web client  
just serve-dev    # 🚀 COMPLETE SYSTEM (skip build for faster iteration)
```

**Individual Interface Options:**
```bash
just web-dev      # Revolutionary sci-fi quantum web interface (services separate)
just play         # Educational trading card game interface  
just integration  # Complete system validation
```

**🎯 RECOMMENDED**: Use `just serve` for the complete Manastr gaming experience!

### 🎯 **What the Integration Tests Prove**

The `daemons/integration_tests/` directory contains the **complete working system** that demonstrates:

- ✅ **Complete 9-phase player-driven match flow** (all 7 Nostr event types: 31000-31006)
- ✅ **Real CDK integration** with deterministic Cashu token generation for authentic C values
- ✅ **Optimized economics**: 95% player rewards, 5% system fee validation
- ✅ **Revolutionary zero-coordination architecture**: Players control everything
- ✅ **Cryptographic anti-cheat**: Commitment/reveal working with real SHA256 validation
- ✅ **Rust-first service orchestration**: No shell scripts, robust process management
- ✅ **Event chain integrity**: Chronological Nostr event validation
- ✅ **Concurrent match processing**: Multiple matches handled simultaneously

### 📍 **Integration Tests = Canonical Reference**

**The integration tests ARE the authoritative system specification.** Everything else (docs, code comments) references these tests as the source of truth for how Manastr works.

**🔑 Key Insight**: Start with the integration tests to understand the system, then explore individual components.

## 🚀 Development Status

**✅ Revolutionary Architecture Complete!**
- **Player-Driven Match System**: 7 Nostr event types implemented
- **Pure Validation Engine**: Game engine refactored to zero-coordination
- **Cryptographic Security**: Real-time anti-cheat with commitment verification
- **Shared WASM Logic**: Deterministic client-server synchronization

**✅ Quantum Web Client Complete!** Revolutionary sci-fi interface with real NDK/Cashu integration

## 📁 Project Structure

```
manastr/
├── daemons/
│   ├── integration_tests/  # 🎯 THE REFERENCE - Complete system validation
│   ├── game-engine-bot/    # ✅ Pure validator with anti-cheat
│   ├── shared-game-logic/  # ✅ WASM-compatible deterministic logic  
│   ├── manastr-web/        # 🚀 Revolutionary quantum web client
│   ├── service-orchestrator/# 🎯 Rust service orchestration system
│   ├── cashu-ts/           # 📦 Cashu-TS library submodule
│   ├── ndk/                # 📦 NDK Nostr library submodule
│   ├── nostr-relay/        # ✅ Decentralized event coordination
│   └── cdk/                # ✅ Official CDK submodule with full mint functionality
├── docs/                   # ✅ Revolutionary architecture specifications
└── CLAUDE.md              # ✅ Complete breakthrough documentation
```

### 🎯 **Start with `daemons/integration_tests/`**

This directory contains:
- **Complete working system** with all 3 services orchestrated via Rust
- **Real CDK integration** using official Cashu Development Kit
- **Player-driven test scenarios** demonstrating the revolutionary architecture
- **Deterministic token generation** for reproducible C values and army generation
- **Rust-first service management** following architectural principles

**This represents a fundamental breakthrough in decentralized multiplayer gaming!** 🎯✨