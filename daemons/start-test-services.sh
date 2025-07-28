#!/bin/bash

# Start all services for Manastr integration testing
# This script starts: Cashu mint, Game engine bot, and Nostr relay (stub)

set -e

echo "🚀 Starting Manastr Test Services"
echo "================================="

# Generate test configurations
./generate-test-configs.sh

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo "⚠️ Port $port is already in use"
        return 1
    fi
    return 0
}

# Function to wait for service to be ready
wait_for_service() {
    local name=$1
    local url=$2
    local max_attempts=${3:-30}
    
    echo "⏳ Waiting for $name to be ready..."
    for i in $(seq 1 $max_attempts); do
        if curl -s "$url" >/dev/null 2>&1; then
            echo "✅ $name is ready"
            return 0
        fi
        sleep 1
    done
    echo "❌ $name failed to start after $max_attempts seconds"
    return 1
}

# Check if required ports are available
echo "🔍 Checking port availability..."
check_port 3333 || { echo "❌ Mint port 3333 in use"; exit 1; }
check_port 4444 || { echo "❌ Game engine port 4444 in use"; exit 1; }
check_port 7777 || { echo "❌ Relay port 7777 in use (would be used if we had a relay)"; }

# Start Cashu mint
echo "🏪 Starting Cashu Stub Mint on port 3333..."
cd cashu-mint
cargo run --release > ../logs/mint.log 2>&1 &
MINT_PID=$!
echo "📝 Mint PID: $MINT_PID"
cd ..

# Start Game Engine Bot
echo "🎮 Starting Game Engine Bot on port 4444..."
cd game-engine-bot
cargo run --release > ../logs/bot.log 2>&1 &
BOT_PID=$!
echo "📝 Bot PID: $BOT_PID"
cd ..

# Create logs directory if it doesn't exist
mkdir -p logs

# Save PIDs for cleanup
echo "$MINT_PID" > logs/mint.pid
echo "$BOT_PID" > logs/bot.pid

echo "🔄 Services starting..."

# Wait for services to be ready
wait_for_service "Mint" "http://localhost:3333/health" 30
wait_for_service "Game Engine Bot" "http://localhost:4444/health" 30

echo ""
echo "🎉 All services are running!"
echo "=========================="
echo "🏪 Cashu Mint:       http://localhost:3333"
echo "🎮 Game Engine Bot:  http://localhost:4444" 
echo "📊 Mint Status:      http://localhost:3333/health"
echo "🤖 Bot Status:       http://localhost:4444/status"
echo ""
echo "📋 Service PIDs saved in logs/ directory"
echo "🛑 To stop services: ./stop-test-services.sh"
echo "🧪 To run integration test: cargo run --bin integration-test"
echo ""
echo "✅ Ready for integration testing!"