# Daemon-Focused Task Dependencies

## Revised Project Structure (Daemon-Based)

```
manastr/
├── daemons/                    # All runnable services (4 daemons)
│   ├── cashu-mint/            # Modified Cashu mint (mana/loot) :3333
│   ├── game-engine/           # Authoritative game bot :4444
│   ├── nostr-relay/           # Local Nostr relay (stores all data) :7777
│   └── web-client/            # React app + Cashu wallet :8080
├── justfile                   # Single command to start all 4 daemons
├── tests/                     # Integration and e2e tests
├── docs/                      # Documentation (existing)
└── CLAUDE.md                  # Agent memory system (existing)
```

## Updated Task Board (Daemon Implementation Focus)

### 🟢 FOUNDATION TASKS (Ready to Start)

| Task ID | Agent | Deliverable | MVP Contribution |
|---------|-------|-------------|------------------|
| F1 | crypto-specialist | Cashu mint modification requirements | Daemon 1: Cashu Mint |
| F2 | game-engine | Game bot logic specification | Daemon 2: Game Engine Bot |
| F3 | nostr-dev | Nostr event flow and relay setup | Daemon 3: Nostr Relay (stores all data) |
| F4 | ui-dev | Web client + wallet integration plan | Daemon 4: Web Client |

### 🔴 CRITICAL HUMAN DECISIONS (Blocking Daemon Implementation)

| Decision ID | Description | Affects Daemon | Impact |
|-------------|-------------|----------------|--------|
| H1 | Choose Cashu Rust library to fork | Cashu Mint | Core functionality |
| H6 | Approve mana/loot token economics | Cashu Mint | Token issuance logic |
| H7 | Choose Nostr relay implementation | Nostr Relay | Message infrastructure |

### 🟡 DAEMON IMPLEMENTATION TASKS

| Task ID | Agent | Daemon | Prerequisites | Deliverable |
|---------|-------|--------|---------------|-------------|
| D1 | crypto-specialist | Cashu Mint | F1 + H1 + H6 | Working mint at :3333 |
| D2 | nostr-dev | Nostr Relay | F3 + H7 | Relay at ws://localhost:7777 |
| D3 | game-engine | Game Engine Bot | F2 + D1 + D2 | Authoritative bot at :4444 |
| D4 | ui-dev | Web Client | F4 + D1 + D2 | React app at :8080 |

### 🔵 INTEGRATION TASKS

| Task ID | Agent | Prerequisites | Deliverable |
|---------|-------|---------------|-------------|
| I1 | architect | All D1-D5 | Justfile configuration |
| I2 | qa-lead | I1 | Integration test suite |
| I3 | qa-lead | I2 | Self-play acceptance test |

## Daemon-Specific Requirements

### D1: Cashu Mint Daemon (`crypto-specialist`)
**Base Repository:** Fork `cashubtc/cashu` Rust implementation  
**Key Modifications:**
```rust
// Replace Bitcoin with mana/loot token types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManaToken {
    pub league_id: u8,        // 0-15 for league-specific keys
    pub secret: [u8; 32],     // Deterministic unit generation
    pub signature: Signature, // Blind signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct LootToken {
    pub amount: u64,          // Reward amount
    pub locked_to_npub: String, // Winner's Nostr pubkey
    pub secret: [u8; 32],
    pub signature: Signature,
}

// Stubbed Lightning for local testing
impl LightningStub {
    pub async fn create_invoice(&self, amount: u64) -> Invoice {
        // Auto-approve all invoices for local testing
        Invoice { paid: true, amount }
    }
}
```

**API Endpoints:**
- `POST /mint/mana` - Purchase mana tokens (stubbed payment)
- `POST /mint/loot` - Create locked loot tokens (game engine only)
- `GET /keysets` - League-specific keysets (16 keys)
- `POST /swap` - Token swapping between players

### D2: Nostr Relay Daemon (`nostr-dev`)
**Options:**
1. **strfry** (recommended): Production-ready C++ relay
2. **nostr-rs-relay**: Rust alternative for easier customization

**Configuration:**
```toml
# config.toml
[relay]
name = "Mana Strategy Local Relay"
port = 7777
host = "127.0.0.1"

[storage]
engine = "lmdb"  # Local database
max_events = 10000
retention_hours = 24

[limits]
max_connections = 100
max_subscriptions_per_connection = 20
```

**Game-Specific Event Kinds:**
- Kind 1: All game events (match, commitment, reveal, result)
- Custom tags: `["match_id"]`, `["commitment"]`, `["reveal"]`, `["game"]`

### D3: Game Engine Bot Daemon (`game-engine`)
**Purpose:** Authoritative game orchestrator with mint access  
**Architecture:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    
    // Connect to services
    let nostr = NostrClient::connect(&config.nostr_relay_url).await?;
    let mint = CashuMintClient::connect(&config.mint_url).await?;
    let db = Database::connect(&config.database_url).await?;
    
    // Subscribe to game events
    let filter = Filter::new()
        .kind(1)
        .tags(vec!["match_id", "commitment", "reveal"]);
    nostr.subscribe(vec![filter]).await?;
    
    // Main event processing loop
    let mut match_states = HashMap::new();
    loop {
        let event = nostr.recv().await?;
        match classify_event(&event) {
            GameEvent::MatchRequest => handle_match_creation(event, &mut match_states).await?,
            GameEvent::Commitment => handle_commitment(event, &mut match_states).await?,
            GameEvent::Reveal => handle_reveal_and_combat(event, &mut match_states, &mint).await?,
        }
    }
}
```

**Authority Powers:**
- Create locked loot tokens via mint API
- Publish authoritative match results
- Resolve disputes and invalid commitments
- Manage match timeouts and forfeitures

### D4: Database Daemon (`architect`)
**Technology:** PostgreSQL with Docker  
**Setup:**
```sql
-- Core tables for local MVP
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    npub TEXT NOT NULL UNIQUE,
    mana_balance INTEGER DEFAULT 0,
    loot_balance INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    challenger_id UUID REFERENCES players(id),
    challenged_id UUID REFERENCES players(id),
    status TEXT NOT NULL DEFAULT 'pending',
    winner_id UUID REFERENCES players(id),
    rounds_data JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE seasons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    reward_pool INTEGER DEFAULT 0,
    status TEXT DEFAULT 'active'
);
```

### D5: Web Client Daemon (`ui-dev`)
**Technology:** React + TypeScript + Integrated Cashu Wallet  
**Key Libraries:**
- `@cashu/cashu-ts` - Cashu wallet functionality
- `nostr-tools` - Nostr client
- `@nostr-dev-kit/ndk` - Advanced Nostr features

**Integrated Wallet Component:**
```typescript
// src/wallet/ManaWallet.ts
export class ManaWallet {
  private cashuWallet: CashuWallet;
  private mintUrl = 'http://localhost:3333';
  
  async purchaseMana(amount: number): Promise<ManaToken[]> {
    // Create payment request (stubbed)
    const invoice = await this.createStubbedInvoice(amount);
    
    // Mint mana tokens
    const tokens = await this.cashuWallet.mintTokens({
      amount,
      invoice,
      league_id: this.selectedLeague
    });
    
    return tokens;
  }
  
  async claimLoot(lootToken: LootToken): Promise<void> {
    // Verify token is locked to our npub
    if (lootToken.locked_to_npub !== this.nostrPublicKey) {
      throw new Error('Loot token not locked to this player');
    }
    
    // Process loot claim
    await this.cashuWallet.processLootClaim(lootToken);
  }
}
```

**Game UI Components:**
- `PlayerLobby` - Challenge creation and acceptance
- `MatchViewer` - Real-time match state from Nostr events
- `UnitSelector` - Choose units from mana token
- `WalletView` - Mana/loot balance and transactions

## MVP Definition of Done

### ✅ Single Command Success
```bash
just dev
# ✅ 5 daemons start without errors
# ✅ Web client loads at http://localhost:8080
# ✅ All health checks pass
```

### ✅ Self-Play Integration Test
```bash
just test-integration
# ✅ Two browser tabs can challenge each other
# ✅ Complete match with mana purchase → commitment → reveal → loot reward
# ✅ All Nostr events properly sequenced
# ✅ Database shows complete match history
```

### ✅ Daemon Health Checks
- **Cashu Mint:** `curl http://localhost:3333/health` returns 200
- **Nostr Relay:** WebSocket connection at `ws://localhost:7777` accepts subscriptions
- **Game Engine:** `curl http://localhost:4444/status` shows "processing events"
- **Web Client:** React app loads without errors
- **Database:** Accepts connections and queries

## Critical Path for MVP

```
F1,F2,F3,F4,F5 → H1,H6,H7 → D1,D2,D4 → D3 → D5 → I1 → I2 → I3
```

**Key Dependencies:**
- D3 (Game Engine) needs D1 (Mint) and D2 (Relay) first
- D5 (Web Client) needs D1 (Mint) and D2 (Relay) first  
- I1 (Justfile) needs all daemons working
- I2/I3 (Tests) validate the complete system

This daemon-focused approach gives us a concrete, testable MVP with a clear definition of done!