# Phase 2: Core Development (8 weeks)

## Overview
Build game engine, Cashu mint, Nostr client, VRF module, reward system, UI with agent-executable tasks and measurable deliverables.

## Work Packages

### WP2.1: Cashu Mint Implementation (12 days)
**Agent:** crypto-specialist  
**Dependencies:** Phase 1 crypto specifications  
**Input:** `/docs/specifications/crypto_requirements.md`  
**Output:** `/implementation/cashu-mint/` (complete service)

**Deliverables:**
- [ ] NUT-00 BDHKE implementation with secp256k1
- [ ] NUT-01/02 keyset management with 16 league keys
- [ ] NUT-03 token minting and swapping endpoints
- [ ] NUT-04/20 Lightning Network integration
- [ ] NUT-05 loot token melting with Lightning payouts
- [ ] NUT-07 token verification and fraud detection
- [ ] NUT-11 locked token implementation (npub-locked loot)

**Agent Tasks:**
```
Week 1-2: Core cryptographic primitives
1. Implement secp256k1 BDHKE operations
2. Create keyset management with league-specific keys
3. Build blind signature creation and verification
4. Implement SHA-256 hashing for commitments
5. Create deterministic unit generation via VRF

Week 3: Token lifecycle management
6. Build mana token minting with 5% fee calculation
7. Implement loot token creation with npub locking
8. Create token verification and signature validation
9. Build token swapping between players

Week 4: Lightning integration
10. Implement Lightning invoice generation
11. Build payment verification and callback handling
12. Create loot token melting to Lightning payouts
13. Add payment status tracking and error handling
```

**Automated Validation:**
```bash
# Unit tests must achieve 95% coverage
pytest cashu-mint/ --cov=95
# Integration tests with real Lightning testnet
python test_lightning_integration.py
# NUT compliance verification
python validate_nut_compliance.py
# Performance benchmarks
python benchmark_token_operations.py  # <2s target
```

**Definition of Done:**
- All NUT-* specifications pass compliance tests
- Lightning integration works with major wallets
- Token operations meet performance targets (<2s)
- Security audit passes with zero critical issues

### WP2.2: Game Engine Core (10 days)
**Agent:** game-engine  
**Dependencies:** Phase 1 game mechanics spec  
**Input:** `/docs/specifications/game_mechanics.md`  
**Output:** `/implementation/game-engine/` (complete service)

**Deliverables:**
- [ ] VRF-based unit generation engine
- [ ] Combat resolution algorithms
- [ ] Match state machine implementation
- [ ] Betting system with accuracy calculations
- [ ] League modifier application logic

**Agent Tasks:**
```
Week 1: Unit generation and stats
1. Implement VRF: Hash(seed||k_a) -> 32-byte unit sets
2. Build unit stat parsing: [attack, defense, health, ability]
3. Create league modifier application (+10 attack, +20 health, etc.)
4. Implement ability effects (Boost, Shield, Heal)
5. Build unit validation and balance checking

Week 2: Combat resolution
6. Implement damage calculation: max(0, attack - defense)
7. Build health tracking and unit elimination
8. Create ability processing pipeline
9. Implement round winner determination
10. Build combat logging and replay generation

Week 3: Match orchestration
11. Implement match state machine (5 states)
12. Build betting system with accuracy scoring
13. Create tiebreaker logic (total damage dealt)
14. Implement match completion and winner determination
15. Build fraud detection and verification
```

**Automated Validation:**
```bash
# Deterministic unit generation tests
python test_unit_generation.py --iterations=1000
# Combat resolution correctness
python test_combat_scenarios.py --comprehensive
# Performance benchmarks
python benchmark_match_resolution.py  # <30s per round
# Game balance validation
python validate_unit_balance.py --statistical
```

**Definition of Done:**
- Unit generation is deterministic and verifiable
- Combat resolution matches specification exactly
- Match resolution completes within performance targets
- Game balance passes statistical fairness tests

### WP2.3: Nostr Client Integration (8 days)
**Agent:** nostr-dev  
**Dependencies:** Phase 1 integration plan  
**Input:** `/docs/specifications/integration_plan.md`  
**Output:** `/implementation/nostr-client/` (complete service)

**Deliverables:**
- [ ] WebSocket relay connection management
- [ ] Event publishing with signature validation
- [ ] Subscription filtering and event processing
- [ ] Commitment/reveal event coordination
- [ ] Match state synchronization

**Agent Tasks:**
```
Week 1: Core Nostr protocol
1. Implement WebSocket connection to relay
2. Build event publishing with secp256k1 signing
3. Create subscription management and filtering
4. Implement event validation and verification
5. Build retry logic and connection recovery

Week 2: Game-specific events
6. Implement match announcement events
7. Build commitment event publishing and validation
8. Create reveal event processing and verification
9. Implement result event publishing
10. Build reward claim event handling
```

**Automated Validation:**
```bash
# Nostr protocol compliance
python test_nip01_compliance.py
# Event publishing performance
python benchmark_event_publishing.py  # <1s target
# Connection reliability
python test_connection_recovery.py --duration=1h
# Event ordering validation
python test_event_sequencing.py
```

**Definition of Done:**
- NIP-01 compliance verified with test suite
- Event publishing meets performance targets
- Connection recovery works under network failures
- Event ordering maintains consistency

### WP2.4: API Gateway and Orchestration (6 days)
**Agent:** architect  
**Dependencies:** WP2.1, WP2.2, WP2.3  
**Input:** `/docs/specifications/api_spec.yaml`  
**Output:** `/implementation/api-gateway/` (complete service)

**Deliverables:**
- [ ] REST API implementation matching OpenAPI spec
- [ ] Request routing to backend services
- [ ] Authentication and authorization middleware
- [ ] Rate limiting and request validation
- [ ] WebSocket event broadcasting

**Agent Tasks:**
```
Week 1: Core API framework
1. Implement REST endpoints per OpenAPI spec
2. Build request validation middleware
3. Create authentication using Nostr keys
4. Implement rate limiting (100 req/min per user)
5. Build error handling and logging

Week 2: Service orchestration
6. Create service discovery and routing
7. Implement WebSocket event broadcasting
8. Build request/response transformation
9. Create health checks and monitoring
10. Implement graceful shutdown and recovery
```

**Automated Validation:**
```bash
# API specification compliance
openapi-validator api_spec.yaml --endpoint-coverage=100%
# Load testing
artillery run load_test.yml  # 1000 concurrent users
# Security testing
zap-baseline.py -t http://localhost:8080/api/v1
```

**Definition of Done:**
- OpenAPI specification 100% implemented
- Load tests pass at target concurrency
- Security scan shows zero high-risk vulnerabilities
- Health checks report all services operational

### WP2.5: Database and State Management (4 days)
**Agent:** architect  
**Dependencies:** Phase 1 database schema  
**Input:** `/docs/specifications/database_schema.sql`  
**Output:** `/implementation/database/` (migrations and queries)

**Deliverables:**
- [ ] PostgreSQL database setup and migrations
- [ ] Query optimization and indexing
- [ ] Connection pooling and transaction management
- [ ] Backup and recovery procedures
- [ ] Data archival and cleanup policies

**Agent Tasks:**
```
Week 1: Database implementation
1. Execute DDL scripts and create all tables
2. Implement database migrations and versioning
3. Create optimized queries for all API operations
4. Build connection pooling and transaction handling
5. Implement backup automation and recovery testing
```

**Automated Validation:**
```bash
# Schema validation
pg_dump --schema-only | diff - database_schema.sql
# Performance testing
pgbench -c 50 -j 2 -T 300 manastr_db
# Migration testing
python test_migrations.py --all-versions
```

**Definition of Done:**
- Database schema matches specification exactly
- Query performance meets targets (<100ms)
- Migration tests pass for all version transitions
- Backup and recovery procedures validated

### WP2.6: Frontend Implementation (16 days)
**Agent:** ui-dev  
**Dependencies:** WP2.4, Phase 1 UI specifications  
**Input:** `/design/component_library.md`, `/docs/specifications/api_spec.yaml`  
**Output:** `/implementation/frontend/` (web and mobile apps)

**Deliverables:**
- [ ] React web application with responsive design
- [ ] React Native mobile applications (iOS/Android)
- [ ] Shared component library implementation
- [ ] WebSocket integration for real-time updates
- [ ] Lightning wallet integration
- [ ] Accessibility compliance (WCAG 2.1 AA)

**Agent Tasks:**
```
Week 1-2: Core component library
1. Implement UnitCard component with stats display
2. Build MatchViewer with real-time state updates
3. Create TournamentBracket with Swiss standings
4. Implement TokenBalance with mana/loot display
5. Build WalletConnector with Lightning integration

Week 3: Web application
6. Create responsive layout system
7. Implement routing and navigation
8. Build state management with Redux
9. Add WebSocket integration for live updates
10. Implement error handling and loading states

Week 4: Mobile applications
11. Set up React Native project structure
12. Implement native navigation (React Navigation)
13. Build platform-specific components
14. Add push notifications for match updates
15. Implement deep linking for match joins
```

**Automated Validation:**
```bash
# Component testing
npm test -- --coverage=90%
# Accessibility testing
axe-core --tags wcag21aa
# Cross-browser testing
playwright test --browser=all
# Mobile testing
detox test --configuration ios.sim.release
```

**Definition of Done:**
- All components pass automated testing with 90% coverage
- Accessibility audit passes WCAG 2.1 AA standards
- Cross-browser compatibility verified on major browsers
- Mobile apps build and run on iOS/Android simulators

## Phase 2 Integration Testing (Week 8)

### WP2.7: End-to-End Integration (5 days)
**Agent:** qa-lead  
**Dependencies:** All WP2.1-WP2.6  
**Input:** All implementation outputs  
**Output:** `/tests/integration/` (complete test suite)

**Integration Test Scenarios:**
- [ ] Complete match flow: mana purchase → unit commitment → reveal → combat → loot claim
- [ ] Tournament registration and Swiss pairing
- [ ] Lightning payment integration with multiple wallets
- [ ] Nostr event synchronization across multiple clients
- [ ] Database consistency under concurrent load
- [ ] API error handling and recovery

**Automated Test Execution:**
```bash
# Full integration test suite
python -m pytest tests/integration/ --parallel=4
# Performance integration tests
python tests/performance/full_match_simulation.py
# Security integration tests
python tests/security/end_to_end_security.py
```

**Definition of Done:**
- All integration tests pass consistently
- Performance targets met under simulated load
- Security tests validate cryptographic correctness
- Error scenarios recover gracefully

## Phase 2 Success Metrics

### Technical Metrics
- Token operations: <2 seconds (target met)
- Match resolution: <30 seconds per round (target met)
- VRF verification: <1 second (target met)
- API response time: <200ms (target met)
- Database queries: <100ms (target met)

### Quality Metrics
- Unit test coverage: >90% (all components)
- Integration test coverage: >80% (critical paths)
- Security scan: Zero critical vulnerabilities
- Accessibility compliance: WCAG 2.1 AA (100%)

### Functional Metrics
- All Cashu NUTs implemented and compliant
- All Nostr events publish and verify correctly
- Game mechanics match specification exactly
- UI supports all user workflows completely