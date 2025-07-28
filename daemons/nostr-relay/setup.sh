#!/bin/bash
# Strfry Nostr Relay Setup for Mana Strategy Game

set -e

echo "🔨 Setting up strfry Nostr relay for local development..."

# Check if we're in the right directory
if [ ! -f "setup.sh" ]; then
    echo "❌ Error: Run this script from daemons/nostr-relay/ directory"
    exit 1
fi

# Install dependencies (Ubuntu/Debian)
echo "📦 Installing build dependencies..."
sudo apt update
sudo apt install -y git build-essential pkg-config libtool autoconf autoconf-archive automake
sudo apt install -y libyaml-cpp-dev libssl-dev zlib1g-dev liblmdb-dev

# Clone strfry if not already present
if [ ! -d "strfry" ]; then
    echo "📥 Cloning strfry repository..."
    git clone https://github.com/damus-io/strfry.git
fi

# Build strfry
echo "🏗️  Building strfry..."
cd strfry
git submodule update --init
make setup-golpe
make -j$(nproc)
cd ..

# Copy binary to daemon directory
cp strfry/strfry ./strfry
chmod +x strfry

# Create necessary directories
mkdir -p logs
mkdir -p strfry-db

echo "✅ Strfry setup complete!"
echo "📍 Binary: ./strfry"
echo "📁 Database: ./strfry-db/"
echo "📋 Config: ./strfry.conf"
echo "📝 Logs: ./logs/"
echo ""
echo "To start the relay:"
echo "  ./start.sh"