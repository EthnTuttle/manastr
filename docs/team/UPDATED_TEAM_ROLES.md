# Updated Team Roles (Simplified Architecture)

## Revised Claude Agent Team

### Core Development Agents (6 agents)

#### 1. Technical Architect (Agent: `architect`)
**Unchanged** - System architecture and API design
- Component integration planning
- API gateway implementation
- Database schema design

#### 2. Cryptography Specialist (Agent: `crypto-specialist`)  
**Unchanged** - Cashu integration focus
- Cashu protocol implementation
- Token lifecycle management
- Cryptographic verification

#### 3. Nostr Integration Lead (Agent: `nostr-dev`)
**Unchanged** - Asynchronous communication
- Nostr event handling
- Match coordination via events
- Real-time communication

#### 4. Game Engine Developer (Agent: `game-engine`)
**Unchanged** - Core game mechanics
- Combat resolution
- Unit generation and balancing
- Match state management

#### 5. Matchmaking Developer (Agent: `matchmaking-dev`) **[REPLACES tournament-dev]**
**New Focus** - Player-driven match creation
- Challenge system implementation
- Player lobby management
- Match history tracking
- ELO rating system

#### 6. Leaderboard Developer (Agent: `leaderboard-dev`) **[NEW ROLE]**
**Purpose** - Ranking and reward distribution
- Real-time leaderboard calculation
- Seasonal reward distribution
- Player statistics and analytics
- Performance tracking

### Support Agents (2 agents)

#### 7. Frontend/UI Developer (Agent: `ui-dev`)
**Simplified scope** - Streamlined UI components
- Player challenge interfaces
- Leaderboard and statistics views
- Match history and replay
- Reward claiming interface

#### 8. Quality Assurance Lead (Agent: `qa-lead`)
**Reduced complexity** - Simpler testing scenarios
- Match flow testing
- Leaderboard accuracy validation
- UI/UX testing
- Performance validation

## Removed Roles

### ❌ No Longer Needed:
- ~~Tournament System Engineer~~ (complex Swiss pairing algorithms)
- ~~DevOps Engineer~~ (reduced to QA responsibilities)

## Updated Responsibility Matrix (RACI)

| Component | Architect | Crypto | Nostr | Game | Matchmaking | Leaderboard | UI | QA |
|-----------|-----------|--------|-------|------|-------------|-------------|----|----|
| System Architecture | R | C | C | C | C | C | C | I |
| Cashu Integration | C | R | I | I | I | I | I | A |
| Nostr Events | C | I | R | C | I | I | I | A |
| Game Mechanics | C | I | I | R | C | I | C | A |
| Matchmaking System | C | I | I | C | R | I | C | A |
| Leaderboard & Rewards | C | C | I | I | I | R | C | A |
| User Interface | I | I | I | C | C | C | R | A |
| Testing & QA | I | C | C | C | C | C | C | R |

## Agent Collaboration Patterns

### Phase 2: Core Development
**Primary pairs:**
- `crypto-specialist` + `architect` (Cashu integration)
- `game-engine` + `matchmaking-dev` (match creation flow)
- `nostr-dev` + `leaderboard-dev` (event-driven rankings)
- `ui-dev` + `matchmaking-dev` (challenge interfaces)

### Phase 3: Leaderboard & Rewards  
**Primary pairs:**
- `leaderboard-dev` + `crypto-specialist` (reward distribution)
- `ui-dev` + `leaderboard-dev` (ranking displays)
- `architect` + `leaderboard-dev` (API design)

## Simplified Agent Workload

### Reduced Complexity:
- **No tournament algorithms** to implement or test
- **No bracket management** systems
- **No complex scheduling** coordination
- **Fewer edge cases** to handle in testing

### Streamlined Development:
- Direct player-to-player interactions
- Simple leaderboard calculations
- Flexible reward timing
- Organic community growth

## Team Communication

### Daily Coordination:
- `matchmaking-dev` ↔ `game-engine` (match flow integration)
- `leaderboard-dev` ↔ `crypto-specialist` (reward token creation)
- `ui-dev` ↔ `matchmaking-dev` (challenge UI/UX)

### Weekly Reviews:
- All agents review leaderboard accuracy
- Performance metrics validation
- User experience optimization

This simplified team structure eliminates the most complex algorithmic work while maintaining all core functionality!