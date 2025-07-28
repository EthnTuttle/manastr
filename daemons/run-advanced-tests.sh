#!/bin/bash

# Advanced Manastr Integration Test Runner
# Supports multiple test scenarios and edge cases

set -e

echo "🧪 Advanced Manastr Integration Test Runner"
echo "==========================================="

# Ensure we're in the right directory
cd "$(dirname "$0")"

# Check if a test mode was provided
if [ $# -eq 0 ]; then
    echo "ℹ️ No test mode specified, showing usage..."
    echo ""
    cd integration_test && cargo run --release help
    exit 0
fi

TEST_MODE="$1"

echo "🎯 Test Mode: $TEST_MODE"
echo ""

# Build all components
echo "🏗️ Building all components..."

# Build Cashu mint
echo "📦 Building Cashu mint..."
cd cashu-mint
cargo build --release --quiet
cd ..

# Build Game engine bot  
echo "🎮 Building Game engine bot..."
cd game-engine-bot
cargo build --release --quiet
cd ..

# Build and run integration test
echo "🧪 Building integration test..."
cd integration_test
cargo build --release --quiet

echo ""
echo "🚀 Starting advanced integration test: $TEST_MODE"
echo "=================================================="

case $TEST_MODE in
    "help"|"--help"|"-h")
        echo "📖 Available test modes:"
        echo ""
        cargo run --release help
        ;;
    "normal")
        echo "🧪 Running standard integration test..."
        cargo run --release normal
        ;;
    "all")
        echo "🧪 Running ALL test scenarios (this may take a while)..."
        cargo run --release all
        ;;
    "edge-cases")
        echo "🧪 Running edge case tests..."
        cargo run --release edge-cases
        ;;
    "stress")
        echo "⚡ Running stress tests..."
        cargo run --release stress
        ;;
    "errors")
        echo "💥 Running error handling tests..."
        cargo run --release errors
        ;;
    *)
        echo "🎮 Running specific scenario: $TEST_MODE..."
        cargo run --release "$TEST_MODE"
        ;;
esac

echo ""
echo "✅ Advanced integration test completed!"
echo ""
echo "📋 Logs available in: ../logs/"
echo "📊 Check daemon outputs for detailed information"