# Manastr Configuration Files

This directory contains configuration files for all Manastr daemons and services.

## Files

### `cashu-mint.toml`
Configuration for the CDK Cashu mint daemon (`cdk-mintd`).

**Key Features:**
- **Fake Lightning Backend**: Uses `cdk-fake-wallet` for testing (auto-fills payment quotes)
- **Deterministic Keys**: Fixed mnemonic ensures reproducible C values for army generation
- **Gaming-Optimized**: Configured for mana/loot dual currency gaming tokens
- **Port 3333**: Standard mint port for compatibility with existing tests

**Usage:**
```bash
cd daemons/cdk/crates/cdk-mintd
cargo run --release -- --config ../../config/cashu-mint.toml
```

**Important:** This configuration uses a test mnemonic for deterministic behavior. 
Never use this configuration in production with real Lightning backends.

## Integration with CDK Submodule

Our configuration files are kept in our repository (`daemons/config/`) rather than 
inside the CDK submodule (`daemons/cdk/`) to:

1. **Version Control**: Track our specific configurations in our repository
2. **Submodule Independence**: Avoid conflicts when updating the CDK submodule
3. **Configuration Management**: Centralize all Manastr-specific configurations
4. **Security**: Keep sensitive configurations out of external repositories

## Testing Configuration

All configurations are optimized for integration testing with:
- **Deterministic behavior** for reproducible test results
- **Fast startup times** for efficient test execution  
- **Real services** rather than mocks for authentic system validation
- **Local ports** (3333, 4444, 7777) for isolated testing