# Phase 2: Core Development - Simplified (6 weeks)

## Overview
Build game engine, Cashu mint, Nostr client, matchmaking system, leaderboard service, and UI with player-choice matches and periodic rewards.

## Work Packages

### WP2.1: Cashu Mint Integration (12 days)
**Agent:** crypto-specialist  
**Dependencies:** Phase 1 crypto specifications  
**Output:** `/implementation/cashu-mint/` (complete service)
**UNCHANGED** - Same as original plan

### WP2.2: Game Engine Core (10 days)
**Agent:** game-engine  
**Dependencies:** Phase 1 game mechanics spec  
**Output:** `/implementation/game-engine/` (complete service)
**UNCHANGED** - Same as original plan

### WP2.3: Nostr Client Integration (8 days)
**Agent:** nostr-dev  
**Dependencies:** Phase 1 integration plan  
**Output:** `/implementation/nostr-client/` (complete service)
**UNCHANGED** - Same as original plan

### WP2.4: Matchmaking Service (8 days) **[NEW - REPLACES TOURNAMENT]**
**Agent:** matchmaking-dev  
**Dependencies:** WP2.2 (game engine), Phase 1 specifications  
**Output:** `/implementation/matchmaking/` (complete service)

**Deliverables:**
- [ ] Player challenge system (create, send, accept, decline)
- [ ] Player lobby with availability status
- [ ] Match history and statistics tracking
- [ ] ELO rating system implementation
- [ ] Anti-spam and rate limiting for challenges

**Agent Tasks:**
```
Week 1: Challenge system
1. Implement challenge creation and management
2. Build challenge acceptance/decline workflow
3. Create challenge expiration and cleanup
4. Implement challenge notifications via Nostr
5. Build anti-spam protection (rate limiting)

Week 2: Lobby and ratings
6. Create player lobby with online status
7. Implement match history tracking
8. Build ELO rating system
9. Create player statistics aggregation
10. Implement matchmaking suggestions based on rating
```

**Challenge System Design:**
```rust
// src/challenges.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub challenger_id: PlayerId,
    pub challenged_id: PlayerId,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub status: ChallengeStatus,
    pub bet_percentage: Option<u8>, // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeStatus {
    Pending,
    Accepted,
    Declined, 
    Expired,
    InProgress,
    Completed,
}
```

**Automated Validation:**
```bash
# Challenge system testing
pytest matchmaking/ --cov=90%
# ELO rating accuracy
python test_elo_calculations.py --iterations=1000
# Performance benchmarks
python benchmark_lobby_operations.py  # <100ms response
# Anti-spam validation
python test_rate_limiting.py --concurrent=50
```

**Definition of Done:**
- Challenge workflow supports all game features
- ELO ratings calculate accurately and fairly
- Lobby operations meet performance targets
- Anti-spam protection prevents abuse

### WP2.5: Leaderboard Service (6 days) **[NEW SERVICE]**
**Agent:** leaderboard-dev  
**Dependencies:** WP2.4 (matchmaking), WP2.1 (Cashu integration)  
**Output:** `/implementation/leaderboard/` (complete service)

**Deliverables:**
- [ ] Real-time player rankings calculation
- [ ] Seasonal period management (weekly/monthly cycles)
- [ ] Top-player reward distribution algorithms
- [ ] Historical statistics and analytics
- [ ] Leaderboard API endpoints with pagination

**Agent Tasks:**
```
Week 1: Core leaderboard system
1. Implement real-time ranking calculations
2. Build seasonal period management
3. Create leaderboard data persistence
4. Implement efficient ranking queries
5. Build leaderboard API endpoints

Week 2: Rewards and analytics
6. Create reward calculation algorithms
7. Implement automated reward distribution
8. Build player statistics aggregation
9. Create historical data analysis
10. Add leaderboard caching for performance
```

**Leaderboard Design:**
```rust
// src/rankings.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRanking {
    pub player_id: PlayerId,
    pub season_id: SeasonId,
    pub wins: u32,
    pub losses: u32,
    pub rating: u32,
    pub total_loot_earned: u64,
    pub last_match_at: DateTime<Utc>,
    pub rank: u32,
}

// src/seasons.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: SeasonId,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub reward_pool: u64,
    pub status: SeasonStatus,
}
```

**Reward Distribution:**
```rust
// src/rewards.rs
pub fn calculate_season_rewards(
    rankings: &[PlayerRanking], 
    total_pool: u64
) -> Vec<PlayerReward> {
    // Top 10% of active players get rewards
    // Exponential decay: 1st = 25%, 2nd = 15%, 3rd = 10%, etc.
}
```

**Automated Validation:**
```bash
# Ranking accuracy tests
python test_ranking_calculations.py --scenarios=comprehensive
# Reward distribution validation
python test_reward_algorithms.py --fairness-check
# Performance testing
python benchmark_leaderboard.py --players=10000  # <200ms queries
# Season transition testing
python test_season_management.py --automated
```

**Definition of Done:**
- Rankings update in real-time accurately
- Reward distribution is fair and mathematically correct
- Leaderboard queries meet performance targets
- Season transitions work automatically

### WP2.6: API Gateway - Simplified (4 days) **[REDUCED SCOPE]**
**Agent:** architect  
**Dependencies:** WP2.4, WP2.5  
**Output:** `/implementation/api-gateway/` (complete service)

**Simplified API Endpoints:**
```yaml
/api/v1/players:
  GET /{id}                 # Player profile
  GET /{id}/stats           # Player statistics
  GET /leaderboard          # Current rankings
  GET /lobby               # Available players

/api/v1/matches:
  POST /challenge          # Challenge another player
  POST /{id}/accept        # Accept challenge  
  GET /{id}               # Match details
  GET /history            # Match history

/api/v1/rewards:
  GET /current            # Current season info
  GET /{player_id}        # Player's earned rewards
  POST /claim             # Claim loot rewards
```

**Agent Tasks:**
```
Week 1: Core API implementation
1. Implement player and match endpoints
2. Build challenge creation and acceptance APIs
3. Create leaderboard and statistics endpoints
4. Implement reward claiming APIs
5. Add authentication and rate limiting
6. Build API documentation and validation
7. Create health checks and monitoring
8. Implement graceful error handling
```

**Definition of Done:**
- All API endpoints implemented per specification
- Authentication and rate limiting working
- API documentation complete and accurate
- Health checks report system status

### WP2.7: Frontend Implementation - Simplified (8 days) **[REDUCED SCOPE]**
**Agent:** ui-dev  
**Dependencies:** WP2.6 (API Gateway)  
**Output:** `/implementation/frontend/` (web and mobile apps)

**Simplified UI Components:**
- `PlayerLobby` - Browse online players and send challenges
- `ChallengeManager` - Manage incoming/outgoing challenges
- `Leaderboard` - Rankings, statistics, and seasonal info
- `MatchViewer` - Watch matches in progress (unchanged)
- `RewardCenter` - View and claim seasonal loot rewards
- `PlayerProfile` - Statistics, match history, achievements

**Agent Tasks:**
```
Week 1: Core matchmaking UI
1. Implement PlayerLobby with online player list
2. Build ChallengeManager for challenge workflow
3. Create Leaderboard with rankings and filters
4. Implement real-time updates via WebSocket
5. Build responsive layouts for mobile

Week 2: Enhanced features
6. Create PlayerProfile with detailed statistics
7. Implement RewardCenter for loot claiming
8. Build match history and replay viewing
9. Add accessibility features (WCAG 2.1 AA)
10. Implement push notifications for mobile
```

**Definition of Done:**
- All UI components work seamlessly together
- Real-time updates via WebSocket integration
- Mobile responsiveness on all screen sizes
- Accessibility compliance verified

## Phase 2 Integration Testing (Final 2 days)

### WP2.8: Simplified Integration Testing
**Agent:** qa-lead  
**Output:** `/tests/integration/` (complete test suite)

**Test Scenarios:**
- [ ] Complete match flow: challenge → accept → play → rewards
- [ ] Leaderboard updates in real-time during matches
- [ ] Seasonal reward distribution to top players
- [ ] Player lobby and challenge system under load
- [ ] Cashu integration for mana purchase and loot claiming

**Definition of Done:**
- All integration tests pass consistently
- Performance targets met under load
- Error scenarios recover gracefully

## Phase 2 Success Metrics

### Functional Metrics
- [ ] Players can challenge each other successfully (100% success rate)
- [ ] Leaderboard updates accurately in real-time
- [ ] Reward distribution works automatically
- [ ] All Cashu and Nostr integrations function correctly

### Performance Metrics  
- [ ] API response time: <200ms (95th percentile)
- [ ] Leaderboard queries: <200ms (10,000 players)
- [ ] Challenge operations: <100ms
- [ ] Real-time UI updates: <1 second

### Quality Metrics
- [ ] Unit test coverage: >90% (all services)
- [ ] Integration test coverage: >85% (critical user flows)
- [ ] Zero critical security vulnerabilities
- [ ] WCAG 2.1 AA accessibility compliance

**Total Phase 2 Duration: 6 weeks** (reduced from 8 weeks by removing tournament complexity)