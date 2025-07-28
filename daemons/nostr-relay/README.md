# Nostr Relay Daemon (Strfry)

Local Nostr relay for Mana Strategy Game development using strfry.

## Quick Start

```bash
# Initial setup (one time)
./setup.sh

# Start the relay
./start.sh

# Test functionality (in another terminal)
./test.sh
```

## Configuration

- **Port:** 7777 (ws://localhost:7777)
- **Database:** ./strfry-db/ (LMDB)
- **Config:** ./strfry.conf
- **Logs:** ./logs/strfry.log

## Game Event Types Supported

1. **Challenge Request** - Player challenges another player
2. **Challenge Acceptance** - Player accepts a challenge
3. **Match Announcement** - Game bot announces match start
4. **Unit Commitment** - Player commits to unit selection (hashed)
5. **Unit Reveal** - Player reveals unit selection with token proof
6. **Round Result** - Game bot publishes combat outcome
7. **Match Result** - Game bot publishes final match winner
8. **Loot Reward** - Game bot issues loot token to winner

## Event Retention

- **Game Events:** 24 hours
- **Match Events:** 24 hours  
- **Result Events:** 7 days (for longer reference)

## Development Features

- **Local Only:** Binds to 127.0.0.1 (localhost)
- **No Authentication:** Open relay for local testing
- **Compression:** WebSocket compression enabled
- **Fast Storage:** LMDB for high performance
- **Auto Cleanup:** Events expire based on retention policy

## Monitoring

```bash
# Check if relay is running
pgrep -f "strfry relay"

# View recent logs
tail -f logs/strfry.log

# Check database size
du -sh strfry-db/

# Export events (for backup/analysis)
./strfry export > backup.jsonl

# Import events (for testing/recovery)
cat backup.jsonl | ./strfry import
```

## Troubleshooting

### Relay won't start
- Check if port 7777 is already in use: `lsof -i :7777`
- Verify strfry binary exists: `ls -la strfry`
- Check config file syntax: `./strfry --config=strfry.conf --help`

### WebSocket connection fails
- Verify relay is running: `pgrep strfry`
- Test with websocat: `echo '["REQ","test",{}]' | websocat ws://localhost:7777`
- Check firewall settings (shouldn't be an issue for localhost)

### Database issues
- Check disk space: `df -h`
- Verify permissions: `ls -la strfry-db/`
- Reset database: `rm -rf strfry-db/` (will lose all events)

## Integration with Game

### Game Engine Bot
- Subscribes to: `{"#game": ["mana-strategy"]}`
- Publishes: Authority events (results, rewards)

### Web Client
- Subscribes to: `{"#match_id": ["specific-match"]}`
- Publishes: Player events (challenges, commitments, reveals)

This relay stores ALL game data - no separate database needed!