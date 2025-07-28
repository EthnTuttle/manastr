#!/bin/bash
# Start strfry Nostr relay for Mana Strategy Game

cd "$(dirname "$0")"

# Check if strfry binary exists
if [ ! -f "./strfry" ]; then
    echo "❌ strfry binary not found. Run setup.sh first:"
    echo "   ./setup.sh"
    exit 1
fi

# Check if config exists
if [ ! -f "./strfry.conf" ]; then
    echo "❌ strfry.conf not found"
    exit 1
fi

# Ensure log directory exists
mkdir -p logs

# Start strfry relay
echo "🚀 Starting strfry Nostr relay..."
echo "📡 WebSocket: ws://localhost:7777"
echo "📁 Database: ./strfry-db/"
echo "📋 Config: ./strfry.conf"
echo "📝 Logs: ./logs/strfry.log"
echo ""
echo "Press Ctrl+C to stop"
echo ""

# Start the relay and log output
./strfry relay --config=./strfry.conf 2>&1 | tee logs/strfry.log