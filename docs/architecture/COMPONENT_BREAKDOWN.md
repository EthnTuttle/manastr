# Component Architecture Breakdown

## 1. Game Engine Core

### 1.1 Unit Generation Module
**Responsibilities:**
- VRF-based deterministic unit generation
- League modifier application
- Statistical balance validation

**Interfaces:**
```
generateUnitSet(secret, signature, league_id) -> UnitSet[8]
applyLeagueModifiers(units, league_id) -> UnitSet[8]
validateUnitBalance(units) -> bool
```

**Dependencies:**
- VRF Service for randomness
- League configuration data
- Cryptographic verification library

### 1.2 Combat Resolution Engine
**Responsibilities:**
- Turn-based combat calculation
- Ability effect processing
- Damage calculation and health tracking

**Interfaces:**
```
resolveCombat(unit_a, unit_b) -> CombatResult
processAbility(unit, ability_type) -> UnitState
calculateDamage(attacker, defender) -> int
```

**State Management:**
- Round-by-round unit health tracking
- Ability cooldowns and effects
- Combat log generation

### 1.3 Match State Machine
**Responsibilities:**
- Match progression tracking
- Round timing enforcement
- Winner determination logic

**States:**
- `WAITING_FOR_PLAYERS`
- `COMMITMENT_PHASE`
- `REVEAL_PHASE`
- `COMBAT_RESOLUTION`
- `ROUND_COMPLETE`
- `MATCH_COMPLETE`

## 2. Cashu Integration Layer

### 2.1 Token Lifecycle Manager
**Responsibilities:**
- Mana token minting with Lightning payment
- Loot token creation and locking
- Token verification and validation

**Core Operations:**
```
mintManaToken(payment_hash, league_id) -> (secret, signature)
createLootToken(winner_npub, amount) -> LockedToken
verifyTokenSignature(token) -> bool
```

### 2.2 Lightning Integration
**Responsibilities:**
- Invoice generation and payment verification
- Fee calculation (5% on inbound)
- Payment status tracking

**Payment Flow:**
1. Generate quote with amount and fee
2. Create Lightning invoice
3. Monitor payment confirmation
4. Trigger token minting on success

### 2.3 Blind Signature Implementation
**Responsibilities:**
- BDHKE key operations
- Message blinding/unblinding
- Signature verification

**Cryptographic Operations:**
```
blindMessage(secret, blinding_factor) -> BlindedMessage
signBlindedMessage(blinded_msg, private_key) -> BlindSignature
unblindSignature(blind_sig, blinding_factor) -> Signature
```

## 3. Nostr Protocol Integration

### 3.1 Event Schema Manager
**Responsibilities:**
- Event structure validation
- Tag standardization
- Content serialization/deserialization

**Event Types:**
- Match announcements (`kind: 1`)
- Unit commitments (`kind: 1` + `["match_id"]`)
- Unit reveals (`kind: 1` + `["reveal"]`)
- Round results (`kind: 1` + `["result"]`)
- Reward claims (`kind: 1` + `["claim"]`)

### 3.2 Relay Communication Handler
**Responsibilities:**
- WebSocket connection management
- Event publishing with retry logic
- Subscription filtering and management

**Connection Management:**
```
connectToRelay(relay_url) -> Connection
publishEvent(event, connection) -> EventId
subscribeToFilter(filter, callback) -> Subscription
```

### 3.3 Event Processing Pipeline
**Responsibilities:**
- Incoming event validation
- Event ordering and sequencing
- State synchronization

**Processing Flow:**
1. Receive event from relay
2. Validate signature and structure
3. Check event ordering constraints
4. Trigger appropriate state updates
5. Notify subscribed components

## 4. Tournament Management System

### 4.1 Swiss Pairing Engine
**Responsibilities:**
- Round-by-round player pairing
- Score-based matchmaking
- Repeat pairing prevention

**Pairing Algorithm:**
```
generatePairings(players, round_number) -> List[PlayerPair]
calculateStandings(tournament_results) -> RankedPlayerList
determineAdvancement(standings, cutoff) -> List[Player]
```

### 4.2 Scoring System
**Responsibilities:**
- Match result recording
- Point calculation (10 points/win + betting bonuses)
- Tiebreaker resolution

**Scoring Components:**
- Base match points (win/loss/tie)
- Betting accuracy bonuses
- Damage dealt tiebreakers
- Tournament placement points

### 4.3 Prize Pool Manager
**Responsibilities:**
- Tournament fee collection
- Prize distribution calculation
- Loot token allocation

**Prize Structure:**
- Entry fee collection in mana
- Winner takes pot structure
- Consolation prizes for top finishers

## 5. User Interface Components

### 5.1 Cross-Platform UI Framework
**Technology Stack:**
- React Native for mobile (iOS/Android)
- React for web interface
- Shared component library

**Core Components:**
- WalletConnector (Lightning + Nostr key management)
- UnitSelector (drag-and-drop unit selection)
- MatchViewer (real-time match state display)
- TournamentBracket (Swiss standings + elimination bracket)

### 5.2 State Management
**Architecture:**
- Redux for global application state
- Real-time updates via WebSocket connections
- Offline-first design with sync on reconnect

**State Slices:**
- Authentication and user profile
- Current match state
- Tournament standings
- Token balances (mana/loot)

### 5.3 Wallet Integration
**Responsibilities:**
- Lightning wallet connection
- Nostr key pair management
- Transaction signing and verification

**Supported Wallets:**
- WebLN-compatible browsers
- Mobile Lightning wallets (via deep linking)
- Hardware signing devices

## 6. Infrastructure Components

### 6.1 API Gateway
**Responsibilities:**
- Request routing and load balancing
- Authentication and rate limiting
- API versioning and documentation

**Endpoints:**
```
POST /api/v1/matches - Create new match
GET /api/v1/matches/{id} - Get match state
POST /api/v1/tokens/mint - Request mana token
POST /api/v1/tokens/claim - Claim loot token
```

### 6.2 Database Architecture
**Match State Storage:**
- PostgreSQL for transactional data
- Redis for session and cache data
- Event sourcing for audit trails

**Schema Design:**
- Players and profiles
- Tournament registrations
- Match history and results
- Token transaction logs

### 6.3 Monitoring and Observability
**Metrics Collection:**
- Match completion rates
- Token operation latency
- Nostr event delivery success
- Lightning payment success rates

**Alerting:**
- Failed payment notifications
- Nostr relay connectivity issues
- Database performance degradation
- Security event detection