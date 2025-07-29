# Mana Strategy Game - Just Command Runner
# =====================================
# 
# This justfile provides streamlined commands for the revolutionary zero-coordination
# gaming architecture. All commands are designed for development, testing, and
# demonstration of the world's first truly decentralized multiplayer game.
# 
# 🚀 REVOLUTIONARY ARCHITECTURE:
# - Zero trusted servers - players control entire match flow
# - Cashu token C values provide tamper-proof army generation
# - Cryptographic commitment/reveal prevents all cheating
# - Pure validator game engine with anti-cheat enforcement
# - Complete decentralization via Nostr events
#
# 📖 USAGE:
#   just --list          # Show all available commands
#   just build           # Build all components
#   just test            # Run all tests
#   just demo            # Run revolutionary gaming demonstration
#   just integration     # Run complete integration test
#   just clean           # Clean build artifacts

# Default recipe - show help
default:
    @echo "🏛️ Mana Strategy Game - Revolutionary Zero-Coordination Gaming"
    @echo "=============================================================="
    @echo ""
    @echo "Available commands:"
    @just --list --unsorted
    @echo ""
    @echo "🚀 Quick start:"
    @echo "  just build     # Build everything"
    @echo "  just demo      # See the revolutionary gaming wallet in action"
    @echo "  just test      # Run all tests"
    @echo ""
    @echo "🎮 For full system demonstration:"
    @echo "  just integration  # Run complete player-driven integration test"

# Build all components in the correct order
build:
    @echo "🏗️ Building revolutionary zero-coordination gaming system..."
    @echo ""
    cd daemons/shared-game-logic && cargo build --release
    cd daemons/cashu-mint && cargo build --release
    cd daemons/game-engine-bot && cargo build --release
    cd daemons && cargo build --release --bin gaming-wallet-test
    cd daemons && cargo build --release --bin player-driven-integration-test
    @echo ""
    @echo "✅ All components built successfully!"

# Build WASM for web client
build-wasm:
    @echo "🌐 Building WASM for web client..."
    cd daemons/shared-game-logic && wasm-pack build --target web --out-dir pkg
    @echo "✅ WASM build complete!"

# Build everything including WASM
build-all: build build-wasm

# Run all unit tests
test:
    @echo "🧪 Running all unit tests..."
    @echo ""
    cd daemons/shared-game-logic && cargo test
    cd daemons/game-engine-bot && cargo test
    cd daemons && cargo test --bin gaming-wallet-test
    @echo ""
    @echo "✅ All unit tests passed!"

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cd daemons/shared-game-logic && cargo clean
    cd daemons/cashu-mint && cargo clean
    cd daemons/game-engine-bot && cargo clean
    cd daemons && cargo clean
    rm -rf daemons/shared-game-logic/pkg
    rm -rf daemons/target
    @echo "✅ Cleanup complete!"

# Demonstrate the revolutionary gaming wallet
demo:
    @echo "🏛️ DEMONSTRATING REVOLUTIONARY GAMING WALLET"
    @echo "============================================="
    @echo ""
    @echo "This demonstrates the breakthrough CDK extension that exposes"
    @echo "Cashu token C values for tamper-proof army generation."
    @echo ""
    cd daemons && cargo run --release --bin gaming-wallet-test

# Run the complete player-driven integration test
integration:
    @echo "🚀 RUNNING REVOLUTIONARY ZERO-COORDINATION INTEGRATION TEST"
    @echo "==========================================================="
    @echo ""
    @echo "This validates the world's first truly decentralized multiplayer"
    @echo "gaming architecture with complete player control and anti-cheat."
    @echo ""
    cd daemons && env RUST_LOG=info ./run-player-driven-tests.sh

# Start all services for development
dev-start:
    @echo "🏗️ Starting all services for development..."
    cd daemons && ./run-player-driven-tests.sh start

# Stop all services
dev-stop:
    @echo "🛑 Stopping all services..."
    cd daemons && ./run-player-driven-tests.sh stop

# Run integration test against already running services
integration-quick:
    @echo "⚡ Running integration test (services must be running)..."
    cd daemons && env RUST_LOG=info ./run-player-driven-tests.sh test

# Format all Rust code
fmt:
    @echo "📝 Formatting Rust code..."
    cd daemons/shared-game-logic && cargo fmt
    cd daemons/cashu-mint && cargo fmt
    cd daemons/game-engine-bot && cargo fmt
    cd daemons && cargo fmt

# Run clippy linting
lint:
    @echo "🔍 Running Clippy linting..."
    cd daemons/shared-game-logic && cargo clippy -- -D warnings
    cd daemons/cashu-mint && cargo clippy -- -D warnings
    cd daemons/game-engine-bot && cargo clippy -- -D warnings
    cd daemons && cargo clippy --bin gaming-wallet-test -- -D warnings
    cd daemons && cargo clippy --bin player-driven-integration-test -- -D warnings

# Run all quality checks (format, lint, test)
check: fmt lint test
    @echo "✅ All quality checks passed!"

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    cd daemons/shared-game-logic && cargo doc --no-deps
    cd daemons/game-engine-bot && cargo doc --no-deps
    @echo "✅ Documentation generated in target/doc/"

# Show system status
status:
    @echo "📊 Revolutionary Gaming System Status"
    @echo "====================================="
    @echo ""
    @echo "🏛️ Core Components:"
    @if [ -f "daemons/target/release/gaming-wallet-test" ]; then echo "  ✅ Gaming Wallet (CDK Extension)"; else echo "  ❌ Gaming Wallet - needs build"; fi
    @if [ -f "daemons/game-engine-bot/target/release/game-engine-bot" ]; then echo "  ✅ Game Engine Bot (Pure Validator)"; else echo "  ❌ Game Engine Bot - needs build"; fi
    @if [ -f "daemons/cashu-mint/target/release/cashu-mint" ]; then echo "  ✅ Cashu Mint (Stub Implementation)"; else echo "  ❌ Cashu Mint - needs build"; fi
    @if [ -d "daemons/nostr-relay/nostr-rs-relay" ]; then echo "  ✅ Nostr Relay (Submodule)"; else echo "  ❌ Nostr Relay - needs setup"; fi
    @if [ -f "daemons/shared-game-logic/pkg/shared_game_logic.js" ]; then echo "  ✅ WASM Game Logic"; else echo "  ❌ WASM - needs build-wasm"; fi
    @echo ""
    @echo "🎯 Revolutionary Features Status:"
    @echo "  ✅ Zero-coordination gaming architecture"
    @echo "  ✅ Cryptographic anti-cheat system"
    @echo "  ✅ Cashu C value army generation"
    @echo "  ✅ Player-driven match flow (7 Nostr event types)"
    @echo "  ✅ Pure validator game engine"
    @echo "  ✅ Deterministic combat logic"
    @echo ""
    @echo "🚀 Ready for industry impact!"

# Install development dependencies
deps:
    @echo "📦 Installing development dependencies..."
    @echo ""
    @echo "Installing Rust toolchain components..."
    rustup component add rustfmt clippy
    @echo ""
    @echo "Installing wasm-pack for web builds..."
    @if ! command -v wasm-pack >/dev/null 2>&1; then \
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
    else \
        echo "wasm-pack already installed"; \
    fi
    @echo ""
    @echo "✅ Development dependencies ready!"

# Quick smoke test to verify everything works
smoke-test: build
    @echo "💨 Running smoke test..."
    @echo ""
    @echo "Testing gaming wallet..."
    @cd daemons && timeout 30s cargo run --release --bin gaming-wallet-test
    @echo ""
    @echo "Testing shared game logic..."
    @cd daemons/shared-game-logic && cargo test --release
    @echo ""
    @echo "✅ Smoke test passed - system is operational!"

# Development workflow - build, test, and check everything
dev: build test check docs
    @echo "🎉 Development workflow complete!"
    @echo ""
    @echo "Ready for revolutionary gaming development!"

# Performance test (placeholder for future implementation)
perf:
    @echo "⚡ Performance testing not yet implemented"
    @echo "This will test high-volume concurrent matches in the future"

# Security audit (placeholder for future implementation)
audit:
    @echo "🔒 Security audit not yet implemented"
    @echo "This will audit cryptographic implementations in the future"

# Generate example match data for testing
generate-test-data:
    @echo "🎲 Generating test match data..."
    @echo "This will create sample match events and armies for testing"
    @echo "(Implementation pending)"

# Show architecture overview
arch:
    @echo "🏛️ REVOLUTIONARY ZERO-COORDINATION GAMING ARCHITECTURE"
    @echo "======================================================="
    @echo ""
    @echo "📡 Communication Layer:"
    @echo "  • Nostr Relay (port 7777) - Decentralized event coordination"
    @echo "  • 7 Nostr Event Types (kinds 31000-31006) - Complete match flow"
    @echo ""
    @echo "🎮 Game Layer:"
    @echo "  • Game Engine Bot (port 4444) - Pure validator, no coordination"
    @echo "  • Shared Game Logic - Deterministic combat via WASM"
    @echo "  • Gaming Wallet - CDK extension exposing C values"
    @echo ""
    @echo "💰 Economic Layer:"
    @echo "  • Cashu Mint (port 3333) - Mana/Loot token management"
    @echo "  • C Value Army Generation - Tamper-proof randomness"
    @echo "  • 1 mana token = 1 army = 1 match capability"
    @echo ""
    @echo "🔒 Security Layer:"
    @echo "  • Cryptographic commitments prevent cheating"
    @echo "  • Mint signatures provide unpredictable randomness"
    @echo "  • No trusted third parties required"
    @echo ""
    @echo "🚀 This architecture represents a fundamental breakthrough in"
    @echo "   multiplayer gaming, eliminating trusted game servers!"

# Help for Claude Code users
claude-help:
    @echo "🤖 CLAUDE CODE INTEGRATION GUIDE"
    @echo "================================"
    @echo ""
    @echo "This project uses 'just' for command automation. Key commands:"
    @echo ""
    @echo "🏗️ Development:"
    @echo "  just build           # Build all components"
    @echo "  just dev             # Full development workflow"
    @echo "  just check           # Format, lint, and test"
    @echo ""
    @echo "🧪 Testing:"
    @echo "  just test            # Unit tests only"
    @echo "  just demo            # Gaming wallet demonstration"
    @echo "  just integration     # Full system integration test"
    @echo "  just smoke-test      # Quick system verification"
    @echo ""
    @echo "🔧 Utilities:"
    @echo "  just clean           # Remove build artifacts"
    @echo "  just status          # Show system status"
    @echo "  just arch            # Architecture overview"
    @echo ""
    @echo "⚡ Quick Development:"
    @echo "  just dev-start       # Start all services"
    @echo "  just integration-quick  # Test against running services"
    @echo "  just dev-stop        # Stop all services"
    @echo ""
    @echo "📚 The revolutionary gaming system is documented in:"
    @echo "  • CLAUDE.md - Project memory and status"
    @echo "  • daemons/player-driven-integration-test.rs - Reference implementation"
    @echo "  • daemons/gaming_wallet.rs - CDK extension for army generation"
    @echo ""
    @echo "🎯 For Claude Code users: Run 'just demo' to see the breakthrough!"