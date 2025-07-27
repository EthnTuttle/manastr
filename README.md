# Mana Strategy Game

A strategy game built with Cashu tokens for gameplay currency and Nostr for asynchronous match coordination.

## Quick Start

```bash
# Start all 4 daemons locally
just dev

# Game available at http://localhost:8080
# Self-play by opening multiple browser tabs
```

## Architecture

- **Cashu Mint** (:3333) - Modified `cashubtc/cdk` with mana/loot tokens
- **Game Engine Bot** (:4444) - Authoritative match orchestrator
- **Nostr Relay** (:7777) - `strfry` relay storing all game data  
- **Web Client** (:8080) - React app with integrated Cashu wallet

## Token Economics

- **5 mana per sat** (1000 sats = 5000 mana tokens)
- **5% fee** creates loot reward pool
- **Skill-based rewards** distributed periodically to top players

## Game Flow

1. **Purchase mana** via Lightning (stubbed for local development)
2. **Challenge players** through the web interface
3. **Commit and reveal units** derived from mana token secrets
4. **Automated combat resolution** by game engine bot
5. **Loot rewards** for winners from accumulated fee pool

## Development Status

**Current Phase:** Foundation tasks and daemon implementation

See `/docs/` for complete specifications and `/planning/` for task dependencies.

## Project Structure

```
manastr/
├── daemons/          # 4 runnable services
├── docs/             # Architecture and specifications  
├── planning/         # Work breakdown and dependencies
├── tests/           # Integration and e2e tests
├── justfile         # Single command to start all services
└── CLAUDE.md        # Agent memory and status tracking
```