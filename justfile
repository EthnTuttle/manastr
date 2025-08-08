# Mana Strategy Game - Essential Commands
# ======================================
# 
# Streamlined command set for the revolutionary zero-coordination gaming architecture.
# Focus on integration testing and individual service management.

# Default recipe - show help
default:
    @echo "🏛️ Mana Strategy Game - Revolutionary Zero-Coordination Gaming"
    @echo "=============================================================="
    @echo ""
    @echo "✅ DEFINITIVE SYSTEM VALIDATION:"
    @echo "   just integration           # THE authoritative proof the system works"
    @echo ""
    @echo "🔧 INDIVIDUAL SERVICES:"
    @echo "   just serve-cdk-mint        # 💰 Start CDK mint with logging"
    @echo "   just serve-nostr-relay     # 📡 Start Nostr relay with logging"
    @echo "   just serve-game-engine     # 🎮 Start game engine with logging"
    @echo "   just serve-web-dev         # 🌐 Start web dev server with logging"
    @echo ""
    @echo "🔥 PROCESS MANAGEMENT:"
    @echo "   just kill-all              # Kill all running services"
    @echo ""
    @echo "🚀 ESSENTIAL COMMANDS:"
    @echo "   just build                 # Build all components"
    @echo "   just clean                 # Clean build artifacts"
    @echo ""

# Build all components in the correct order
build:
    @echo "🔨 Building Manastr revolutionary gaming system..."
    @echo ""
    @echo "📦 Building shared game logic (WASM compatible)..."
    cd daemons/shared-game-logic && cargo build --release
    @echo "✅ Shared game logic built successfully"
    @echo ""
    @echo "🎮 Building game engine bot..."
    cd daemons/game-engine-bot && cargo build --release
    @echo "✅ Game engine bot built successfully"
    @echo ""
    @echo "💰 Building CDK mint..."
    cd daemons/cdk && cargo build --release --bin cdk-mintd
    @echo "✅ CDK mint built successfully"
    @echo ""
    @echo "📡 Building Nostr relay..."
    cd daemons/nostr-relay/nostr-rs-relay && cargo build --release
    @echo "✅ Nostr relay built successfully"
    @echo ""
    @echo "🌐 Building web client..."
    cd daemons/manastr-web && npm install && npm run build
    @echo "✅ Web client built successfully"
    @echo ""
    @echo "🚀 All components built successfully!"

# Clean all build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    find . -name target -type d -exec rm -rf {} + 2>/dev/null || true
    find . -name node_modules -type d -exec rm -rf {} + 2>/dev/null || true
    find . -name dist -type d -exec rm -rf {} + 2>/dev/null || true
    @echo "✅ Clean complete"

# 🏆 THE DEFINITIVE SYSTEM VALIDATION - Run this to see Manastr in action!
integration:
    @echo "🏆 MANASTR INTEGRATION TEST - THE DEFINITIVE SYSTEM VALIDATION"
    @echo "=============================================================="
    @echo ""
    @echo "This integration test is the CANONICAL DOCUMENTATION of how"
    @echo "the revolutionary zero-coordination gaming system works."
    @echo ""
    @echo "🚀 What you'll see:"
    @echo "  • 9-phase player-driven match lifecycle"
    @echo "  • Cryptographic commitment/reveal anti-cheat"
    @echo "  • Real Cashu token C values for army generation"
    @echo "  • Complete Nostr event chain (7 event types)"
    @echo "  • 95% player rewards economic model validation"
    @echo "  • Game engine as pure validator (no coordination)"
    @echo ""
    @echo "🎯 Starting integration test..."
    @echo ""
    cd daemons/integration_tests && cargo run --bin main

# 💰 SERVE CDK MINT - Start CDK mint with timestamped logging
serve-cdk-mint:
    #!/usr/bin/env bash
    set -euo pipefail
    cd daemons/cdk
    echo "💰 Starting CDK mint with deterministic config..."
    ./target/release/cdk-mintd --config ../config/cdk-mintd-deterministic.toml 2>&1 | tee "cdk-mint-$(date +%Y%m%d-%H%M%S).log"

# 📡 SERVE NOSTR RELAY - Start Nostr relay with timestamped logging  
serve-nostr-relay:
    #!/usr/bin/env bash
    set -euo pipefail
    cd daemons/nostr-relay
    echo "📡 Starting Nostr relay..."
    ./nostr-rs-relay/target/release/nostr-rs-relay --config config.toml 2>&1 | tee "nostr-relay-$(date +%Y%m%d-%H%M%S).log"

# 🎮 SERVE GAME ENGINE - Start game engine bot with timestamped logging
serve-game-engine:
    #!/usr/bin/env bash
    set -euo pipefail  
    cd daemons/game-engine-bot
    echo "🎮 Starting game engine bot..."
    ./target/release/game-engine-bot 2>&1 | tee "game-engine-$(date +%Y%m%d-%H%M%S).log"

# 🌐 SERVE WEB DEV - Start web development server with timestamped logging
serve-web-dev:
    #!/usr/bin/env bash
    set -euo pipefail
    cd daemons/manastr-web
    echo "🌐 Starting web development server..."
    npm run dev 2>&1 | tee "web-dev-$(date +%Y%m%d-%H%M%S).log"

# 🔥 KILL ALL - Kill all running Manastr processes
kill-all:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔥 Killing all Manastr processes..."
    
    # Kill CDK mint
    pkill -f "cdk-mintd" || echo "  💰 CDK mint not running"
    
    # Kill Nostr relay
    pkill -f "nostr-rs-relay" || echo "  📡 Nostr relay not running"
    
    # Kill Game Engine
    pkill -f "game-engine-bot" || echo "  🎮 Game engine not running"
    
    # Kill Web dev server (Vite)
    pkill -f "vite" || echo "  🌐 Vite dev server not running"
    pkill -f "npm.*dev" || echo "  🌐 NPM dev server not running"
    
    # Kill any Node processes from this project
    pkill -f "manastr-web" || echo "  🌐 Manastr web processes not running"
    
    echo "✅ All Manastr processes terminated"

# 🧨 NUKE RELAY EVENTS - Delete all Nostr events from local relay database
nuke-relay:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🧨 Nuking Nostr relay event database..."
    cd daemons/nostr-relay
    # Ensure relay is stopped before deleting DB
    pkill -f "nostr-rs-relay" || true
    # Remove sqlite files
    rm -f ./nostr-relay-db/nostr.db ./nostr-relay-db/nostr.db-shm ./nostr-relay-db/nostr.db-wal
    echo "✅ Relay database cleared"