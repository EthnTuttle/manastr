# Manastr Service Orchestrator

**Revolutionary Rust-based service orchestration for the complete Manastr gaming system.**

## 🚀 Features

- **Complete System Management**: Builds and runs all Manastr components in one command
- **Health Monitoring**: Automatic health checks for all services
- **Graceful Shutdown**: Ctrl+C cleanly stops all services
- **Web Server**: Serves the quantum web client with static file handling
- **Process Lifecycle**: Robust process management with error handling
- **Centralized Logging**: All services managed with unified logging

## 🎯 What It Does

The service orchestrator:

1. **Builds All Components**:
   - Rust workspace (game-engine, shared logic, etc.)
   - CDK mint (separate build)
   - Nostr relay (separate build) 
   - WASM shared game logic
   - Quantum web client (npm build)

2. **Starts All Services**:
   - Nostr Relay (ws://localhost:7777)
   - CDK Mint (http://localhost:3333)
   - Game Engine (http://localhost:4444)

3. **Serves Web Client**: 
   - Quantum web interface (http://localhost:8080)
   - Static file serving with CORS
   - Built-in development server

## 🔧 Usage

### Complete System (Build + Run)
```bash
just serve
# or directly:
cargo run --release --bin manastr-serve
```

### Development Mode (Skip Build)
```bash
just serve-dev
# or directly:
cargo run --release --bin manastr-serve -- --skip-build
```

### Command Options
```
USAGE:
    manastr-serve [OPTIONS]

OPTIONS:
    -p, --port <PORT>    Port to serve the web client on [default: 8080]
        --skip-build     Skip building (useful for development)
    -v, --verbose        Enable verbose logging
    -h, --help           Print help information
```

## 🏗️ Architecture

```
manastr-serve
├── Build Phase
│   ├── Rust workspace build
│   ├── CDK mint build  
│   ├── Nostr relay build
│   ├── WASM build (shared-game-logic)
│   └── Web client build (npm)
├── Service Management
│   ├── Nostr Relay (port 7777)
│   ├── CDK Mint (port 3333)
│   └── Game Engine (port 4444)
└── Web Server
    └── Static file server (port 8080)
```

## 🛡️ Error Handling

- **Build Failures**: Stops execution with clear error messages
- **Service Startup**: Health checks ensure services are ready
- **Process Management**: Handles child process lifecycle
- **Graceful Shutdown**: Kills all child processes on exit

## 🔍 Health Checks

Services with HTTP endpoints are automatically health-checked:
- CDK Mint: `GET http://localhost:3333/v1/info`  
- Game Engine: `GET http://localhost:4444/health`

Services without HTTP endpoints (Nostr relay) use timeout-based readiness.

## 📦 Dependencies

- **axum**: Web server framework
- **tokio**: Async runtime
- **reqwest**: HTTP client for health checks
- **tower-http**: File serving and CORS
- **tracing**: Structured logging
- **clap**: Command-line interface

This orchestrator represents the **definitive way** to run the complete Manastr system with a single command!