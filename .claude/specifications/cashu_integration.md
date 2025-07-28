# Cashu CDK Integration Specification

**Agent**: `crypto-specialist`  
**Status**: Foundation Complete  
**Dependencies**: None  
**Integrates With**: F2 (Game Engine), F4 (Web Client)

## Overview

The Mana Strategy Game uses a **pure Cashu CDK mint** (https://github.com/cashubtc/cdk) with two custom currencies: **"mana"** and **"loot"**. The mint is completely game-agnostic and only handles standard Cashu protocol operations. All game logic (unit generation, leagues, NPub locking) is handled client-side.

## Pure CDK Architecture

### Currency Configuration
The mint supports exactly **two currencies** using CDK's standard `CurrencyUnit::Custom`:

- **"mana"** - Primary game currency (5 mana per sat, mint-only)
- **"loot"** - Reward currency (meltable back to Lightning)

### Standard CDK Endpoints
The mint exposes only standard Cashu protocol endpoints:

#### NUT-00: Mint Info
- `GET /v1/info` - Standard mint information

#### NUT-01: Keysets
- `GET /v1/keysets` - Available keysets for both currencies

#### NUT-04: Mint Tokens  
- `POST /v1/mint/quote/bolt11` - Request mint quotes
- `POST /v1/mint/bolt11` - Mint tokens after Lightning payment

#### NUT-05: Melt Tokens (loot only)
- `POST /v1/melt/quote/bolt11` - Request melt quotes (loot currency only)
- `POST /v1/melt/bolt11` - Melt loot tokens back to Lightning

## CDK Mint Implementation

### Rust Implementation
```rust
use cdk::{
    mint::{Mint, MintBuilder},
    nuts::{CurrencyUnit, MintInfo},
    cdk_database::MintDatabase,
    cdk_lightning::MintLightning,
};

pub struct DualCurrencyMint {
    mint: Mint,
}

impl DualCurrencyMint {
    pub async fn new(
        database: Arc<dyn MintDatabase>,
        lightning: Arc<dyn MintLightning>
    ) -> Result<Self, cdk::Error> {
        let mint_info = MintInfo {
            name: "Dual Currency Mint".to_string(),
            description: Some("CDK mint supporting mana and loot currencies".to_string()),
            description_long: None,
            contact: vec![],
            motd: Some("Standard Cashu mint with dual currency support".to_string()),
            nuts: Default::default(),
        };

        let mint = MintBuilder::new(database)
            .with_mint_info(mint_info)
            .with_lightning_backend(lightning)
            .build()
            .await?;

        // Generate keysets for both currencies
        mint.generate_keyset(CurrencyUnit::Custom("mana".to_string())).await?;
        mint.generate_keyset(CurrencyUnit::Custom("loot".to_string())).await?;

        Ok(Self { mint })
    }

    // Standard CDK methods only - no game-specific logic
    pub async fn mint_quote_bolt11(&self, request: MintQuoteBolt11Request) -> Result<MintQuoteBolt11Response, cdk::Error> {
        self.mint.mint_quote_bolt11(request).await
    }

    pub async fn mint_bolt11(&self, request: MintBolt11Request) -> Result<MintBolt11Response, cdk::Error> {
        self.mint.mint_bolt11(request).await
    }

    pub async fn melt_quote_bolt11(&self, request: MeltQuoteBolt11Request) -> Result<MeltQuoteBolt11Response, cdk::Error> {
        // Only allow melting loot currency
        if request.unit != CurrencyUnit::Custom("loot".to_string()) {
            return Err(cdk::Error::UnsupportedUnit);
        }
        self.mint.melt_quote_bolt11(request).await
    }

    pub async fn melt_bolt11(&self, request: MeltBolt11Request) -> Result<MeltBolt11Response, cdk::Error> {
        self.mint.melt_bolt11(request).await
    }
}
```

### Configuration (mint.toml)
```toml
[mint]
name = "Dual Currency Mint"
description = "Standard CDK mint with mana and loot currencies"

[server]
host = "127.0.0.1"
port = 3333

[lightning]
backend = "fake"  # For local development
auto_approve = true

[database]
type = "sqlite" 
path = "./mint.db"

[currencies]
# Configuration handled by CDK automatically
# Currencies: ["mana", "loot"]
```

## Client-Side Game Logic

### Unit Generation (Client-Side)
```typescript
// Client handles all game logic
import { Proof } from '@cashu/cashu-ts';

export function generateBattleUnits(proof: Proof, leagueId: number): BattleUnit[] {
    // Client-side logic to generate 8 units from proof secret
    const units = [];
    const hash = sha256(proof.secret);
    
    for (let i = 0; i < 8; i++) {
        const chunk = hash.slice(i * 4, i * 4 + 4);
        units.push({
            attack: applyLeagueModifier(chunk[0], leagueId, 'attack'),
            defense: applyLeagueModifier(chunk[1], leagueId, 'defense'),  
            health: applyLeagueModifier(chunk[2], leagueId, 'health'),
            ability: determineAbility(chunk[3])
        });
    }
    
    return units;
}
```

### NPub Locking (Client-Side)
```typescript
// Clients handle NPub locking for loot tokens
export function createNPubLockedSecret(npub: string, matchId: string, amount: number): string {
    return `loot_${npub}_${matchId}_${amount}_${Date.now()}`;
}

export function canClaimLoot(proof: Proof, myNpub: string): boolean {
    // Parse secret to check if locked to this npub
    const parts = proof.secret.split('_');
    return parts[1] === myNpub;
}
```

## Integration Points

### Game Engine Bot
- Requests loot mint quotes for winners
- Uses standard CDK endpoints only
- No special authentication - mint is permissionless

### Web Client  
- Uses standard CDK wallet for both currencies
- Handles all game logic client-side
- Determines unit generation, league selection, etc.

### Nostr Relay
- Publishes token proofs and match results
- No direct integration with mint

## Development Setup

### Dependencies
```toml
[dependencies]
cdk = { git = "https://github.com/cashubtc/cdk" }
# Standard CDK dependencies...
```

### Running the Mint
```bash
cargo run
```

### Testing
```bash
# Standard CDK mint operations
curl -X POST http://localhost:3333/v1/mint/quote/bolt11 \
  -H "Content-Type: application/json" \
  -d '{"amount": 500, "unit": "mana"}'

curl -X POST http://localhost:3333/v1/melt/quote/bolt11 \
  -H "Content-Type: application/json" \
  -d '{"amount": 100, "unit": "loot", "request": "lnbc..."}'
```

## Key Differences from Game-Aware Approach

| Aspect | Game-Aware Mint | Pure CDK Mint |
|--------|----------------|---------------|
| Unit Generation | Mint generates units | Client generates units |
| League Keysets | 16 league-specific keysets | 2 currency keysets only |
| NPub Locking | Mint enforces NPub locks | Client handles NPub logic |
| Game API | Custom game endpoints | Standard CDK endpoints only |
| Battle Logic | Mint knows about battles | Mint is game-agnostic |
| Complexity | High (game + protocol) | Low (protocol only) |

---

**Implementation Status**: Ready for pure CDK implementation  
**Next Steps**: Implement standard CDK mint with dual currency support only