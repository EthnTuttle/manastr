#!/bin/bash
# nostr-rs-relay Setup for Mana Strategy Game

set -e

echo "🔨 Setting up nostr-rs-relay for local development..."

# Check if we're in the right directory
if [ ! -f "setup.sh" ]; then
    echo "❌ Error: Run this script from daemons/nostr-relay/ directory"
    exit 1
fi

# Clone nostr-rs-relay if not already present
if [ ! -d "nostr-rs-relay" ]; then
    echo "📥 Cloning nostr-rs-relay repository..."
    git clone https://github.com/scsibug/nostr-rs-relay nostr-rs-relay
fi

# Build nostr-rs-relay
echo "🏗️  Building nostr-rs-relay..."
cd nostr-rs-relay
cargo build --release
cd ..

# Create necessary directories
mkdir -p logs
mkdir -p nostr-relay-db

echo "✅ nostr-rs-relay setup complete!"
echo "📍 Binary: ./nostr-rs-relay/target/release/nostr-rs-relay"
echo "📁 Database: ./nostr-relay-db/"
echo "📋 Config: ./config.toml (auto-generated)"
echo "📝 Logs: ./logs/"
echo ""
echo "🖥️  macOS Compatible: ✅"
echo "🐧 Linux Compatible: ✅"
echo ""
echo "To start the relay:"
echo "  ./start.sh"