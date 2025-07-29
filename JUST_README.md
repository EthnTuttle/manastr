# Just Command Runner Integration

This project uses [`just`](https://just.systems/) for streamlined command automation and development workflows.

## 🚀 Quick Start

```bash
# Install just (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin
# Or via package manager: brew install just, cargo install just-cli, etc.

# Show all available commands
just --list

# Quick start sequence
just build      # Build everything
just demo       # See the revolutionary gaming wallet
just test       # Run all tests
```

## 📋 Command Categories

### 🏗️ **Build Commands**
- `just build` - Build all Rust components
- `just build-wasm` - Build WASM for web client  
- `just build-all` - Build everything including WASM
- `just clean` - Remove all build artifacts

### 🧪 **Testing Commands**
- `just test` - Run all unit tests
- `just demo` - Demonstrate gaming wallet with army generation
- `just integration` - Run complete player-driven integration test
- `just smoke-test` - Quick system verification
- `just integration-quick` - Test against already running services

### 🔧 **Development Commands**
- `just dev` - Full development workflow (build + test + check)
- `just dev-start` - Start all services for development
- `just dev-stop` - Stop all services
- `just check` - Format, lint, and test everything
- `just fmt` - Format all Rust code
- `just lint` - Run clippy linting

### 📊 **Information Commands**
- `just status` - Show system component status
- `just arch` - Display architecture overview
- `just claude-help` - Integration guide for Claude Code users
- `just deps` - Install development dependencies

## 🎯 **Revolutionary Gaming Workflows**

### **First Time Setup**
```bash
just deps       # Install development dependencies
just build      # Build all components
just demo       # See the breakthrough in action!
```

### **Development Cycle**
```bash
just dev-start        # Start services
# ... make changes ...
just check           # Verify code quality
just integration-quick  # Test changes
just dev-stop        # Stop services
```

### **Full System Validation**
```bash
just build           # Ensure everything is built
just integration     # Run complete integration test
```

### **Code Quality**
```bash
just fmt            # Format code
just lint           # Check for issues
just test           # Run unit tests
just check          # All of the above
```

## 🏛️ **Revolutionary Architecture Commands**

The justfile includes commands specifically designed for the breakthrough zero-coordination gaming architecture:

- **`just demo`** - Demonstrates tamper-proof army generation from Cashu C values
- **`just integration`** - Validates complete player-driven match flow via 7 Nostr event types
- **`just arch`** - Shows the revolutionary architecture overview

## 🤖 **Claude Code Integration**

For Claude Code users, the justfile provides:

1. **Comprehensive automation** - No need to remember complex cargo commands
2. **Self-documenting workflows** - Each command explains what it does
3. **Revolutionary gaming focus** - Commands highlight the breakthrough architecture
4. **Development efficiency** - Quick iteration and testing cycles

Run `just claude-help` for the complete Claude Code integration guide.

## 📚 **Additional Resources**

- **Main Documentation**: `CLAUDE.md` - Project memory and current status
- **Reference Implementation**: `daemons/player-driven-integration-test.rs` 
- **Gaming Wallet**: `daemons/gaming_wallet.rs` - CDK extension for army generation
- **Architecture Docs**: `docs/` directory - Complete specifications

## 🎮 **Ready for Industry Impact**

This justfile enables rapid development and demonstration of the world's first working zero-coordination multiplayer gaming architecture. Every command is designed to showcase the revolutionary breakthrough that eliminates trusted game servers!