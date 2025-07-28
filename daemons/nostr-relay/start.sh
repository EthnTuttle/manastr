#!/bin/bash
# Start nostr-rs-relay for Mana Strategy Game

cd "$(dirname "$0")"

# Check if nostr-rs-relay binary exists
if [ ! -f "./nostr-rs-relay/target/release/nostr-rs-relay" ]; then
    echo "❌ nostr-rs-relay binary not found. Building..."
    cd nostr-rs-relay && cargo build --release && cd ..
fi

# Create config if it doesn't exist
if [ ! -f "./config.toml" ]; then
    echo "📋 Creating default config.toml..."
    cat > config.toml << 'EOF'
[info]
relay_url = "ws://localhost:7777"
name = "Mana Strategy Game Relay"
description = "Nostr relay for decentralized gaming"
pubkey = ""
contact = ""

[database]
data_directory = "./nostr-relay-db"
engine = "sqlite"

[network]
port = 7777
address = "127.0.0.1"

[limits]
max_message_length = 131072
max_subscriptions = 20
max_filters = 10
max_event_tags = 2000

[authorization]
# pubkey_whitelist = []  # Commented out to allow all pubkeys for testing

[verified_users]

[limits.messages]

[limits.subscriptions]

[grpc]

[logging]
tracing_level = "debug"

[diagnostics]

[metrics]

[reject]
kinds = []

[pay_to_relay]
enabled = false

[options]
reject_future_seconds = 1800
EOF
fi

# Ensure db directory exists
mkdir -p logs
mkdir -p nostr-relay-db

# Start nostr-rs-relay
echo "🚀 Starting nostr-rs-relay..."
echo "📡 WebSocket: ws://localhost:7777"  
echo "📁 Database: ./nostr-relay-db/"
echo "📋 Config: ./config.toml"
echo "📝 Logs: ./logs/nostr-relay.log"
echo ""
echo "Press Ctrl+C to stop"
echo ""

# Start the relay and log output
./nostr-rs-relay/target/release/nostr-rs-relay --config config.toml 2>&1 | tee logs/nostr-relay.log