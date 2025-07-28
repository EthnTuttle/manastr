#!/bin/bash

# Generate test configuration files with deterministic keys
# This script creates config files for testing the full integration

echo "🔧 Generating test configuration files..."

# Create game engine test config
cat > game-engine-bot/game-engine.toml << EOF
[server]
host = "127.0.0.1"
port = 4444

[nostr]
relay_url = "ws://localhost:7777"
private_key = "0000000000000000000000000000000000000000000000000000000000000002"

[cashu]
mint_url = "http://localhost:3333"

[game]
max_concurrent_matches = 10
round_timeout_seconds = 30
match_timeout_seconds = 300
loot_reward_per_match = 100
EOF

echo "✅ Created game-engine-bot/game-engine.toml"

# Create mint test config (if needed)
mkdir -p cashu-mint/test-data
cat > cashu-mint/mint.toml << EOF
[server]
host = "127.0.0.1" 
port = 3333

[mint]
# Test mint configuration
private_key = "0000000000000000000000000000000000000000000000000000000000000001"
database_url = "sqlite:test-mint.db"

[lightning]
# Stub lightning backend for testing
backend = "stub"
EOF

echo "✅ Created cashu-mint/mint.toml"

echo "🎮 Test configuration files generated!"
echo "🔑 Using deterministic keys for reproducible testing"
echo ""
echo "Next steps:"
echo "1. Start services: ./start-test-services.sh"
echo "2. Run integration test: cargo test --test integration"