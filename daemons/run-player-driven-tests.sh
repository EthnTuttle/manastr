#!/bin/bash

# Player-Driven Integration Test Runner
# Comprehensive testing of revolutionary zero-coordination architecture

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Player-Driven Integration Test Suite${NC}"
echo -e "${BLUE}Testing revolutionary zero-coordination gaming architecture${NC}"
echo ""

# Configuration
CASHU_MINT_PORT=3333
GAME_ENGINE_PORT=4444
NOSTR_RELAY_PORT=7777
TEST_TIMEOUT=300 # 5 minutes

# Function to check if service is running
check_service() {
    local service_name=$1
    local port=$2
    local max_attempts=30
    local attempt=1

    echo -e "${YELLOW}⏳ Waiting for $service_name to be ready on port $port...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s "http://localhost:$port/health" > /dev/null 2>&1 || nc -z localhost $port 2>/dev/null; then
            echo -e "${GREEN}✅ $service_name is ready${NC}"
            return 0
        fi
        
        echo -e "   Attempt $attempt/$max_attempts - waiting..."
        sleep 2
        ((attempt++))
    done
    
    echo -e "${RED}❌ $service_name failed to start within $((max_attempts * 2)) seconds${NC}"
    return 1
}

# Function to check if game engine state machine is running
check_game_engine_state_machine() {
    local max_attempts=30
    local attempt=1

    echo -e "${YELLOW}⏳ Waiting for Game Engine State Machine to be ready...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        # Check if process is running and log shows successful initialization
        # Look for the log file in the current directory where it's actually created
        if kill -0 $GAME_ENGINE_PID 2>/dev/null && grep -q "Game Engine Bot fully operational" game-engine.log 2>/dev/null; then
            echo -e "${GREEN}✅ Game Engine State Machine is ready${NC}"
            return 0
        fi
        
        # Alternative check: if process is running and listening on expected port or has key messages
        if kill -0 $GAME_ENGINE_PID 2>/dev/null && (grep -q "Started Nostr match event processing loop" game-engine.log 2>/dev/null || grep -q "OPTIMIZED FILTERING" game-engine.log 2>/dev/null); then
            echo -e "${GREEN}✅ Game Engine State Machine is ready (alternative check)${NC}"
            return 0
        fi
        
        echo -e "   Attempt $attempt/$max_attempts - waiting..."
        sleep 2
        ((attempt++))
    done
    
    echo -e "${RED}❌ Game Engine State Machine failed to start within $((max_attempts * 2)) seconds${NC}"
    echo -e "${YELLOW}Debug: Checking if log file exists and process is running...${NC}"
    if [ -f "game-engine.log" ]; then
        echo -e "${YELLOW}Log file contents (last 10 lines):${NC}"
        tail -10 game-engine.log || echo "Could not read log file"
    else
        echo -e "${RED}Log file game-engine.log does not exist${NC}"
    fi
    
    if kill -0 $GAME_ENGINE_PID 2>/dev/null; then
        echo -e "${YELLOW}Game Engine process is still running (PID: $GAME_ENGINE_PID)${NC}"
    else
        echo -e "${RED}Game Engine process is not running${NC}"
    fi
    
    return 1
}

# Function to start required services
start_services() {
    echo -e "${PURPLE}🏗️ Starting required services...${NC}"
    
    # Start Cashu Mint
    echo -e "${YELLOW}Starting Cashu Mint...${NC}"
    cd cashu-mint
    cargo run --release > ../cashu-mint.log 2>&1 &
    CASHU_MINT_PID=$!
    cd ..
    
    # Start Game Engine Bot
    echo -e "${YELLOW}Starting Game Engine Bot...${NC}"
    cd game-engine-bot
    cargo run --release > ../game-engine.log 2>&1 &
    GAME_ENGINE_PID=$!
    cd ..
    
    # Start Nostr Relay
    echo -e "${YELLOW}Starting Nostr Relay...${NC}"
    cd nostr-relay
    ./start.sh > ../nostr-relay.log 2>&1 &
    NOSTR_RELAY_PID=$!
    cd ..
    
    # Wait for services to be ready
    check_service "Cashu Mint" $CASHU_MINT_PORT
    check_game_engine_state_machine
    check_service "Nostr Relay" $NOSTR_RELAY_PORT
    
    echo -e "${GREEN}✅ All services started successfully${NC}"
    
    # Give game engine extra time to process historical Nostr events
    echo -e "${YELLOW}⏳ Allowing game engine to process historical events...${NC}"
    sleep 3
}

# Function to stop services
stop_services() {
    echo -e "${YELLOW}🛑 Stopping services...${NC}"
    
    if [ ! -z "${CASHU_MINT_PID:-}" ]; then
        kill $CASHU_MINT_PID 2>/dev/null || true
        echo -e "   Stopped Cashu Mint"
    fi
    
    if [ ! -z "${GAME_ENGINE_PID:-}" ]; then
        kill $GAME_ENGINE_PID 2>/dev/null || true
        echo -e "   Stopped Game Engine Bot"
    fi
    
    if [ ! -z "${NOSTR_RELAY_PID:-}" ]; then
        kill $NOSTR_RELAY_PID 2>/dev/null || true
        echo -e "   Stopped Nostr Relay"
    fi
    
    # Kill any remaining processes
    pkill -f "cashu-mint" 2>/dev/null || true
    pkill -f "game-engine-bot" 2>/dev/null || true
    pkill -f "nostr-rs-relay" 2>/dev/null || true
    
    echo -e "${GREEN}✅ Services stopped${NC}"
}

# Function to run integration tests
run_tests() {
    echo -e "${PURPLE}🧪 Running Player-Driven Integration Tests...${NC}"
    echo ""
    
    # Build test binary
    echo -e "${YELLOW}Building integration test...${NC}"
    cargo build --bin player-driven-integration-test --release
    
    # Run comprehensive test suite
    echo -e "${BLUE}🚀 Executing comprehensive test suite...${NC}"
    timeout $TEST_TIMEOUT ./target/release/player-driven-integration-test
    
    local test_result=$?
    
    if [ $test_result -eq 0 ]; then
        echo ""
        echo -e "${GREEN}🎉 ALL PLAYER-DRIVEN INTEGRATION TESTS PASSED!${NC}"
        echo -e "${GREEN}✅ Revolutionary zero-coordination architecture validated${NC}"
        echo -e "${GREEN}✅ Cryptographic anti-cheat system working${NC}"
        echo -e "${GREEN}✅ Concurrent match processing validated${NC}"
        echo -e "${GREEN}✅ Edge cases and malicious events handled${NC}"
        echo -e "${GREEN}✅ Stress testing completed successfully${NC}"
    else
        echo ""
        echo -e "${RED}❌ INTEGRATION TESTS FAILED${NC}"
        echo -e "${RED}Check logs for details:${NC}"
        echo -e "   Game Engine: $(pwd)/game-engine.log"
        echo -e "   Nostr Relay: $(pwd)/nostr-relay.log"
        return 1
    fi
}

# Function to show test summary
show_summary() {
    echo ""
    echo -e "${BLUE}📊 Test Summary${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
    echo -e "🎯 ${GREEN}Revolutionary Architecture:${NC} Zero-coordination gaming"
    echo -e "🔒 ${GREEN}Anti-Cheat System:${NC} Cryptographic commitment/reveal"
    echo -e "📡 ${GREEN}Event Types Tested:${NC} 7 Nostr kinds (31000-31006)"
    echo -e "⚔️ ${GREEN}Match Scenarios:${NC} Happy path, cheating, concurrent, edge cases"
    echo -e "🧪 ${GREEN}Stress Testing:${NC} High-volume match processing"
    echo ""
    echo -e "${PURPLE}This validates the world's first trustless multiplayer gaming architecture!${NC}"
}

# Function to cleanup on exit
cleanup() {
    echo ""
    echo -e "${YELLOW}🧹 Cleaning up...${NC}"
    stop_services
    
    # Remove test artifacts
    rm -f *.log 2>/dev/null || true
    
    echo -e "${GREEN}✅ Cleanup completed${NC}"
}

# Set up cleanup trap
trap cleanup EXIT

# Main execution
main() {
    echo -e "${BLUE}Starting Player-Driven Integration Test Pipeline${NC}"
    echo -e "${BLUE}═════════════════════════════════════════════════════${NC}"
    
    # Start services
    start_services
    
    # Run tests
    run_tests
    
    # Show summary
    show_summary
    
    echo ""
    echo -e "${GREEN}🎉 Player-Driven Integration Testing Complete!${NC}"
}

# Parse command line arguments
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Player-Driven Integration Test Runner"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  (none)    Run full integration test suite"
        echo "  start     Start services only"
        echo "  stop      Stop services"
        echo "  test      Run tests (assumes services running)"
        echo "  help      Show this help"
        echo ""
        echo "This script tests the revolutionary zero-coordination gaming architecture."
        ;;
    "start")
        start_services
        echo -e "${GREEN}Services started. Run tests with: $0 test${NC}"
        trap - EXIT # Don't cleanup on exit
        ;;
    "stop")
        stop_services
        ;;
    "test")
        run_tests
        show_summary
        ;;
    *)
        main
        ;;
esac