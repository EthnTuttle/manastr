# Nostr Relay for Mana Strategy Game

This directory contains the Nostr relay setup using **nostr-rs-relay** for the decentralized Mana Strategy Game.

## Overview

The Nostr relay serves as the decentralized communication backbone for the game, handling:
- Match challenge/acceptance events
- Player move commitments and reveals
- Game state synchronization
- Anti-cheat verification events

## Quick Start

```bash
# Initial setup (one time)
./setup.sh

# Start the relay
./start.sh

# Test functionality (in another terminal)
./test.sh
```

The relay will be available at `ws://localhost:7777`

## Architecture

- **Backend**: nostr-rs-relay (Rust-based Nostr relay)
- **Port**: 7777 (ws://localhost:7777)
- **Protocol**: WebSocket (Nostr NIP-01)
- **Database**: SQLite (local file)
- **Platform Support**: ✅ macOS, ✅ Linux, ✅ Windows

## Event Types

The relay handles these custom event kinds for the revolutionary player-driven architecture:

- **Kind 31000**: Match Challenge
- **Kind 31001**: Match Acceptance  
- **Kind 31002**: Token Reveal
- **Kind 31003**: Move Commitment
- **Kind 31004**: Move Reveal
- **Kind 31005**: Match Result
- **Kind 31006**: Loot Distribution

## Configuration

The relay uses `config.toml` (auto-generated) for configuration:
- Max message size: 128KB
- Database: `./nostr-relay-db/`
- Logs: `./logs/nostr-relay.log`
- Port: 7777 (localhost only for development)

## macOS Compatibility ✅

The relay is fully compatible with macOS through:
- Native Rust compilation
- SQLite database (no external dependencies)
- Automatic configuration generation
- No Linux-specific dependencies required

## Development

- **Logs**: Check `./logs/nostr-relay.log` for relay activity
- **Database**: SQLite file at `./nostr-relay-db/`
- **Config**: Auto-generated `config.toml` on first run
- **Binary**: Built at `./nostr-rs-relay/target/release/nostr-rs-relay`

## Integration Testing

✅ **Player-Driven Tests**: Use `../run-player-driven-tests.sh` (RECOMMENDED)
- Tests revolutionary zero-coordination architecture
- Uses 7 Nostr event types per current spec
- Validates cryptographic commitment/reveal anti-cheat
- **Status**: ✅ Working on macOS and Linux

❌ **Legacy Tests**: Avoid `../run-integration-test.sh` and `../run-advanced-tests.sh`
- These use outdated centralized architecture
- Not compatible with current player-driven specification
- Should be removed in future cleanup

## Troubleshooting

### Relay won't start
- Check if port 7777 is already in use: `lsof -i :7777`
- Verify binary exists: `ls -la nostr-rs-relay/target/release/nostr-rs-relay`
- Run setup: `./setup.sh` to build nostr-rs-relay

### WebSocket connection fails
- Verify relay is running: `pgrep nostr-rs-relay`
- Test connection: `nc -z localhost 7777`
- Check macOS firewall if needed

### macOS-specific issues
- Grant network access if prompted by macOS
- Ensure Rust toolchain is installed: `rustc --version`
- Build failures: Update Xcode command line tools

### Database issues
- Check disk space: `df -h`
- Verify permissions: `ls -la nostr-relay-db/`
- Reset database: `rm -rf nostr-relay-db/` (will lose all events)

## Integration

The relay integrates with:
- **Game Engine Bot**: Validates events and publishes results
- **Web Client**: Sends player events and receives updates  
- **Player-Driven Integration Tests**: Revolutionary zero-coordination testing

This relay is the **backbone of the world's first trustless multiplayer gaming architecture**!