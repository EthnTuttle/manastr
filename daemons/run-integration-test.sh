#!/bin/bash

# Run the Manastr integration test
# This script builds all components and runs the full integration test

set -e

echo "🧪 Manastr Integration Test Runner"
echo "=================================="

# Ensure we're in the right directory
cd "$(dirname "$0")"

echo "🏗️ Building all components..."

# Build Cashu mint
echo "📦 Building Cashu mint..."
cd cashu-mint
cargo build --release
cd ..

# Build Game engine bot
echo "🎮 Building Game engine bot..."
cd game-engine-bot
cargo build --release
cd ..

# Build and run integration test
echo "🧪 Building and running integration test..."
cd integration_test
cargo build --release

echo ""
echo "🚀 Starting integration test..."
echo "==============================="
echo "ℹ️  This test will:"
echo "   1. Start all required daemons"
echo "   2. Run comprehensive API tests"
echo "   3. Simulate a full match flow"
echo "   4. Clean up all processes"
echo ""
echo "🛑 Press Ctrl+C to interrupt and clean up"
echo ""

# Run the integration test
cargo run --release

echo ""
echo "✅ Integration test completed!"