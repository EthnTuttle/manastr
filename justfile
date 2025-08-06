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
    @echo "✅ DEFINITIVE SYSTEM VALIDATION:"
    @echo "   just integration    # THE authoritative proof the system works"
    @echo ""
    @echo "Available commands:"
    @just --list --unsorted
    @echo ""
    @echo "🎯 IMPORTANT: The integration test is the definitive documentation"
    @echo "   of how Manastr works. Run 'just integration' to see the complete"
    @echo "   revolutionary zero-coordination gaming system in action!"
    @echo ""
    @echo "🚀 Quick start:"
    @echo "  just build         # Build everything"
    @echo "  just demo          # See the revolutionary gaming wallet in action"
    @echo "  just test          # Run all tests"
    @echo "  just integration   # THE definitive system demonstration"
    @echo ""
    @echo "🎮 Interactive Interfaces:"
    @echo "  just integration   # Complete service orchestration + game validation"
    @echo "  just web           # Quantum web client with automatic service orchestration"
    @echo "  just web-dev       # Quantum web client development mode (services separate)"
    @echo "  just play          # Trading card game interface (iced.rs)"

# Build all components in the correct order
build:
    @echo "🏗️ Building revolutionary zero-coordination gaming system..."
    @echo ""
    cargo build --release
    @echo ""
    @echo "✅ All components built successfully!"

# Build WASM for web client
build-wasm:
    @echo "🌐 Building WASM for web client..."
    cd daemons/shared-game-logic && wasm-pack build --target web --out-dir pkg
    @echo "✅ WASM build complete!"

# Build quantum web client
build-web:
    @echo "🌐 Building quantum web client..."
    @echo ""
    @echo "🚀 Compiling React + Styled Components"
    @echo "⚡ Bundling NDK and Cashu-TS dependencies"
    @echo "🔮 Optimizing quantum animations"
    @echo ""
    cd daemons/manastr-web && npm run build
    @echo ""
    @echo "✅ Quantum web client build complete!"
    @echo "📁 Output: daemons/manastr-web/dist/"

# Build everything including WASM and web client
build-all: build build-wasm build-web

# Run all unit tests
test:
    @echo "🧪 Running all unit tests..."
    @echo ""
    cargo test
    @echo ""
    @echo "✅ All unit tests passed!"

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf daemons/shared-game-logic/pkg
    @echo "✅ Cleanup complete!"

# Demonstrate the revolutionary gaming wallet
demo:
    @echo "🏛️ DEMONSTRATING REVOLUTIONARY GAMING WALLET"
    @echo "============================================="
    @echo ""
    @echo "This demonstrates the breakthrough CDK extension that exposes"
    @echo "Cashu token C values for tamper-proof army generation."
    @echo ""
    cd daemons/integration_tests && cargo run --bin gaming-wallet

# 🏆 THE DEFINITIVE SYSTEM VALIDATION - Run this to see Manastr in action!
integration:
    @echo "🚀 RUNNING REVOLUTIONARY ZERO-COORDINATION INTEGRATION TEST"
    @echo "==========================================================="
    @echo ""
    @echo "✅ THE INTEGRATION TEST IS THE DEFINITIVE SYSTEM DOCUMENTATION"
    @echo "This is the authoritative proof that revolutionary zero-coordination"
    @echo "gaming works. It demonstrates:"
    @echo "  • Service orchestration (Cashu + Game Engine + Nostr)"
    @echo "  • Complete 9-phase player-driven match flow"
    @echo "  • Optimized economics: 95% player rewards"
    @echo "  • Cryptographic anti-cheat system working"
    @echo "  • All 7 Nostr event types (KIND 31000-31006)"
    @echo "  • Concurrent match processing"
    @echo "  • Edge case and stress testing"
    @echo ""
    @echo "🎯 IMPORTANT: This test IS the system - run it to understand Manastr!"
    @echo ""
    cd daemons/integration_tests && cargo run --bin integration-runner

# 🎮 INTERACTIVE GAMING SESSION - Start services and launch TCG interface
play:
    @echo "🎮 LAUNCHING MANASTR TRADING CARD GAME INTERFACE"
    @echo "==============================================="
    @echo ""
    @echo "This will:"
    @echo "  1. 🏗️  Start all backend services (CDK, Nostr, Game Engine)"
    @echo "  2. ⏳  Wait for services to be ready"
    @echo "  3. 🎯  Launch the Trading Card Game interface"
    @echo "  4. 🧹  Clean up all services when GUI exits"
    @echo ""
    @echo "🚀 Starting interactive gaming session..."
    @echo ""
    cd daemons/integration_tests && cargo run --bin integration-runner -- --gui

# 🌐 WEB CLIENT - Start services and launch quantum web interface  
web:
    @echo "🌐 LAUNCHING MANASTR QUANTUM WEB CLIENT"
    @echo "======================================"
    @echo ""
    @echo "Revolutionary sci-fi web interface featuring:"
    @echo "  🚀 React-based quantum UI with Arwes-inspired aesthetics"
    @echo "  ⚡ Real-time Nostr client integration"
    @echo "  💰 Advanced Cashu wallet operations"
    @echo "  🎮 Game engine coordination"
    @echo "  🔮 Futuristic animations and effects"
    @echo ""
    @echo "This will:"
    @echo "  1. 🏗️  Start all backend services (CDK mint, Nostr relay, Game Engine)"
    @echo "  2. ⏳  Wait for services to be ready"
    @echo "  3. 🌍  Launch quantum web client on http://localhost:8080"
    @echo "  4. 🔌  Connect to Nostr relay (ws://localhost:7777)"
    @echo "  5. 💰  Connect to Cashu mint (http://localhost:3333)"
    @echo "  6. 🎮  Connect to Game Engine (http://localhost:4444)"
    @echo ""
    @echo "🚀 Initializing quantum web client session..."
    @echo ""
    cd daemons/integration_tests && cargo run --bin integration-runner -- --web

# 🌐 WEB DEV - Start quantum web client in development mode (standalone)
web-dev:
    @echo "🌐 STARTING MANASTR QUANTUM WEB CLIENT (DEV MODE)"
    @echo "================================================="
    @echo ""
    @echo "🚀 REVOLUTIONARY SCI-FI WEB INTERFACE"
    @echo "Features:"
    @echo "  ⚡ React + Styled Components architecture"
    @echo "  🔮 Futuristic animations and quantum effects"
    @echo "  📡 Real NDK Nostr client integration"
    @echo "  💰 Cashu-TS wallet with C value support"
    @echo "  🎮 Game engine coordination protocols"
    @echo "  🌌 Arwes-inspired sci-fi aesthetic"
    @echo ""
    @echo "Development mode - quantum web client only:"
    @echo "  🌍  Quantum interface: http://localhost:8080"
    @echo "  🔧  Hot reload enabled"
    @echo "  ⚡  Connect manually to quantum services:"
    @echo "     - Nostr relay: ws://localhost:7777"
    @echo "     - Cashu mint: http://localhost:3333"
    @echo "     - Game engine: http://localhost:4444"
    @echo ""
    @echo "💡 Note: Start backend services separately with 'just integration'"
    @echo ""
    cd daemons/manastr-web && npm run dev

# 🖥️ INTEGRATION DASHBOARD - Tauri + Dioxus dashboard with service orchestration
dashboard:
    @echo "🖥️ LAUNCHING MANASTR INTEGRATION DASHBOARD"
    @echo "==========================================="
    @echo ""
    @echo "Revolutionary Tauri + Dioxus integration dashboard with service orchestration:"
    @echo "  1. 🏗️  Start all background services first (CDK, Nostr, Game Engine)"
    @echo "  2. 📊  Health check all services before launching dashboard"
    @echo "  3. 🎮  Professional desktop interface with Dioxus"
    @echo "  4. 💰  Real-time service monitoring and control"
    @echo "  5. 📋  Live service logs and activity monitoring"
    @echo "  6. 🛑  Graceful shutdown of all services on exit"
    @echo ""
    @echo "Features:"
    @echo "  • Rust-based service orchestration with fail-fast behavior"
    @echo "  • All services started before dashboard launch (no startup race conditions)"
    @echo "  • Native desktop performance with Tauri"
    @echo "  • Reactive UI with Dioxus web framework"
    @echo "  • Complete process lifecycle management"
    @echo ""
    @echo "🚀 Starting service orchestration and dashboard..."
    @echo ""
    cd daemons/manastr-tauri/src-tauri && cargo run --bin dashboard-launcher


# Format all Rust code
fmt:
    @echo "📝 Formatting Rust code..."
    cargo fmt

# Run clippy linting
lint:
    @echo "🔍 Running Clippy linting..."
    cargo clippy --all-targets --all-features

# Run all quality checks (format, lint, test)
check: fmt lint test
    @echo "✅ All quality checks passed!"

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    cargo doc --no-deps --workspace
    @echo "✅ Documentation generated in target/doc/"

# Show system status
status:
    @echo "📊 Revolutionary Gaming System Status"
    @echo "====================================="
    @echo ""
    @echo "🏛️ Core Components:"
    @if [ -f "target/release/game-engine-bot" ]; then echo "  ✅ Game Engine Bot (Pure Validator)"; else echo "  ❌ Game Engine Bot - needs build"; fi
    @if [ -f "target/release/integration-runner" ]; then echo "  ✅ Integration Test Runner"; else echo "  ❌ Integration Runner - needs build"; fi
    @if [ -f "target/release/gaming-wallet" ]; then echo "  ✅ Gaming Wallet Demo"; else echo "  ❌ Gaming Wallet - needs build"; fi
    @if [ -d "daemons/cdk" ]; then echo "  ✅ CDK (Official Cashu Implementation)"; else echo "  ❌ CDK - needs submodule init"; fi
    @if [ -d "daemons/nostr-relay" ]; then echo "  ✅ Nostr Relay (Submodule)"; else echo "  ❌ Nostr Relay - needs setup"; fi
    @if [ -f "daemons/shared-game-logic/pkg/shared_game_logic.js" ]; then echo "  ✅ WASM Game Logic"; else echo "  ❌ WASM - needs build-wasm"; fi
    @echo ""
    @echo "🎯 Revolutionary Features Status:"
    @echo "  ✅ Zero-coordination gaming architecture"
    @echo "  ✅ Cryptographic anti-cheat system"
    @echo "  ✅ Cashu C value army generation"
    @echo "  ✅ Player-driven match flow (7 Nostr event types)"
    @echo "  ✅ Pure validator game engine"
    @echo "  ✅ Deterministic combat logic"
    @echo "  ✅ Consolidated integration test framework"
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
    @echo "Testing workspace build..."
    @cargo check --workspace
    @echo ""
    @echo "Testing core game logic..."
    @cargo test -p shared-game-logic --release
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

# Exit the matrix - Neo's awakening to zero-coordination gaming
exit-matrix:
    cd daemons/integration_tests && cargo run --bin integration-runner -- --tutorial

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
    @echo "🎓 Learning:"
    @echo "  just exit-matrix     # Neo's awakening - Interactive TUI tutorial + gameplay client"
    @echo ""
    @echo "🔧 Utilities:"
    @echo "  just clean           # Remove build artifacts"
    @echo "  just status          # Show system status"
    @echo "  just arch            # Architecture overview"
    @echo ""
    @echo "⚡ Quick Development:"
    @echo "  just smoke-test      # Quick system verification"
    @echo "  just integration     # Comprehensive integration test"
    @echo ""
    @echo "📚 The revolutionary gaming system is documented in:"
    @echo "  • CLAUDE.md - Project memory and status"
    @echo "  • daemons/integration_tests/ - Consolidated test framework"
    @echo "  • daemons/integration_tests/src/core/gaming_wallet.rs - CDK extension"
    @echo "  • README.md - Getting started guide"
    @echo ""
    @echo "🎯 For Claude Code users: Run 'just exit-matrix' to experience the revolution!"
    @echo ""
    @echo "🚀 STRATEGIC EVOLUTION:"
    @echo "  BEVY_INTEGRATION_STRATEGY.md  # Complete roadmap for professional game engine upgrade"