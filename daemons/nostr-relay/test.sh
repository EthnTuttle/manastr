#!/bin/bash
# Test strfry Nostr relay functionality

echo "🧪 Testing strfry Nostr relay..."

# Check if relay is running
if ! pgrep -f "strfry relay" > /dev/null; then
    echo "❌ Strfry relay is not running. Start it first:"
    echo "   ./start.sh"
    exit 1
fi

# Test WebSocket connection using curl (if available)
if command -v websocat &> /dev/null; then
    echo "📡 Testing WebSocket connection..."
    
    # Create a simple subscription request
    SUB_REQ='["REQ", "test", {"kinds": [1], "limit": 1}]'
    
    # Test connection (timeout after 5 seconds)
    timeout 5s websocat ws://localhost:7777 <<< "$SUB_REQ" && echo "✅ WebSocket connection successful"
    
elif command -v wscat &> /dev/null; then
    echo "📡 Testing WebSocket connection with wscat..."
    echo '["REQ", "test", {"kinds": [1], "limit": 1}]' | timeout 5s wscat -c ws://localhost:7777 && echo "✅ WebSocket connection successful"
    
else
    echo "⚠️  WebSocket test tools not available (websocat/wscat)"
    echo "   Install with: sudo apt install websocat"
fi

# Test basic relay info endpoint (if supported)
if command -v curl &> /dev/null; then
    echo "📊 Testing relay health..."
    if curl -s --connect-timeout 3 http://localhost:7777 > /dev/null; then
        echo "✅ HTTP endpoint responding"
    else
        echo "⚠️  HTTP endpoint not responding (normal for strfry)"
    fi
fi

# Check database directory
if [ -d "./strfry-db" ]; then
    echo "✅ Database directory exists"
    DB_SIZE=$(du -sh strfry-db 2>/dev/null | cut -f1)
    echo "   Database size: $DB_SIZE"
else
    echo "❌ Database directory not found"
fi

# Check log file
if [ -f "./logs/strfry.log" ]; then
    echo "✅ Log file exists"
    echo "   Recent log entries:"
    tail -n 3 ./logs/strfry.log | sed 's/^/   /'
else
    echo "❌ Log file not found"
fi

echo ""
echo "🎮 Ready for game events at ws://localhost:7777"