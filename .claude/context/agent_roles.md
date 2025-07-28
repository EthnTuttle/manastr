# Claude Agents & Team Roles for Mana Strategy Game

## Core Development Team Personas

### 1. Technical Architect (Agent: `architect`)
**Primary Responsibilities:**
- System architecture design and validation
- Component integration planning
- Technical risk assessment
- Cross-platform compatibility decisions

**Skills:**
- Distributed systems design
- Cryptographic protocol implementation
- Performance optimization
- Security architecture

**Phase Focus:** Phase 1-2 (Requirements & Core Development)

### 2. Cryptography Specialist (Agent: `crypto-specialist`)
**Primary Responsibilities:**
- Cashu protocol implementation (NUT-00 through NUT-20)
- VRF implementation for unit generation
- Blind signature scheme implementation
- Security audit of cryptographic components

**Skills:**
- secp256k1 curve operations
- BDHKE (Blind Diffie-Hellman Key Exchange)
- SHA-256 hashing implementations
- Locked token mechanisms

**Phase Focus:** Phase 2 (Core Development)

### 3. Nostr Integration Lead (Agent: `nostr-dev`)
**Primary Responsibilities:**
- Nostr event schema design and implementation
- Relay communication protocols
- Asynchronous event handling
- Event verification and validation

**Skills:**
- NIP-01 protocol expertise
- WebSocket management
- Event-driven architecture
- Real-time communication patterns

**Phase Focus:** Phase 2 (Core Development)

### 4. Game Engine Developer (Agent: `game-engine`)
**Primary Responsibilities:**
- Combat resolution algorithms
- Unit stat generation and balancing
- Match state management
- Betting system implementation

**Skills:**
- Game mechanics design
- State machine implementation
- Algorithm optimization
- Fair randomness generation

**Phase Focus:** Phase 2-3 (Core Development & Tournament System)

### 5. Frontend/UI Developer (Agent: `ui-dev`)
**Primary Responsibilities:**
- Cross-platform UI implementation
- User experience optimization
- Mobile and web interface design
- Responsive design implementation

**Skills:**
- React/React Native development
- Cross-platform frameworks
- Mobile app development
- Web3 wallet integration

**Phase Focus:** Phase 2-4 (Core Development through Testing)

### 6. Tournament System Engineer (Agent: `tournament-dev`)
**Primary Responsibilities:**
- Swiss-system pairing algorithms
- Tournament scoring and ranking
- Match scheduling and coordination
- Prize pool management

**Skills:**
- Algorithm design
- Database optimization
- Concurrent system design
- Tournament management systems

**Phase Focus:** Phase 3 (Tournament System)

### 7. DevOps/Infrastructure Engineer (Agent: `devops`)
**Primary Responsibilities:**
- Deployment pipeline setup
- Monitoring and logging systems
- Performance optimization
- Security hardening

**Skills:**
- Cloud infrastructure
- Container orchestration
- CI/CD pipeline design
- System monitoring

**Phase Focus:** Phase 4-5 (Testing & Deployment)

### 8. Quality Assurance Lead (Agent: `qa-lead`)
**Primary Responsibilities:**
- Test strategy development
- Automated testing implementation
- Security testing
- Performance testing

**Skills:**
- Test automation frameworks
- Security testing methodologies
- Load testing
- Bug tracking and reporting

**Phase Focus:** Phase 4 (Testing & Refinement)

## Agent Collaboration Matrix

| Phase | Primary Agents | Supporting Agents |
|-------|---------------|-------------------|
| Phase 1 | architect, crypto-specialist | All (requirements gathering) |
| Phase 2 | crypto-specialist, nostr-dev, game-engine, ui-dev | architect, qa-lead |
| Phase 3 | tournament-dev, game-engine | ui-dev, qa-lead |
| Phase 4 | qa-lead, devops | All (testing support) |
| Phase 5 | devops, ui-dev | architect, qa-lead |

## Agent Communication Protocols

### Daily Standups (Virtual)
- Each agent reports progress on assigned components
- Blockers and dependencies identified
- Cross-team coordination requirements

### Technical Reviews
- Architecture decisions require `architect` + 2 specialist approvals
- Security implementations require `crypto-specialist` + `qa-lead` review
- UI/UX decisions require `ui-dev` + user testing validation

### Code Reviews
- All cryptographic code: `crypto-specialist` mandatory review
- All Nostr integration: `nostr-dev` mandatory review  
- All game mechanics: `game-engine` mandatory review
- All deployment code: `devops` mandatory review