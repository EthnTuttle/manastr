# Team Roles & Responsibilities Matrix

## Executive Level

### Project Manager
**Human Role** - Overall project coordination and timeline management
- Phase gate approvals
- Resource allocation
- Stakeholder communication
- Risk mitigation strategy

### Product Owner
**Human Role** - Game design decisions and user experience validation
- Feature prioritization
- User story validation
- Market requirements
- Beta testing coordination

## Technical Leadership

### Technical Architect (Claude Agent)
**Accountability:** System design integrity and scalability
- Component interface definitions
- Technology stack decisions
- Performance requirements validation
- Integration architecture

### Security Lead (Claude Agent: crypto-specialist)
**Accountability:** Cryptographic security and compliance
- Cashu protocol compliance
- Private key management
- Audit trail implementation
- Penetration testing coordination

## Development Teams

### Backend Core Team
- **Cashu Integration Specialist** (Claude Agent: crypto-specialist)
- **Nostr Protocol Developer** (Claude Agent: nostr-dev)
- **Game Engine Developer** (Claude Agent: game-engine)

### Frontend Team
- **UI/UX Developer** (Claude Agent: ui-dev)
- **Mobile App Developer** (Claude Agent: ui-dev)
- **Web Interface Developer** (Claude Agent: ui-dev)

### Systems Team
- **Tournament System Engineer** (Claude Agent: tournament-dev)
- **DevOps Engineer** (Claude Agent: devops)
- **Performance Engineer** (Claude Agent: devops)

### Quality Assurance
- **QA Lead** (Claude Agent: qa-lead)
- **Security Tester** (Claude Agent: qa-lead)
- **Performance Tester** (Claude Agent: qa-lead)

## Accountability Matrix (RACI)

| Component | Architect | Crypto | Nostr | Game | UI | Tournament | DevOps | QA |
|-----------|-----------|--------|-------|------|----|-----------  |--------|----| 
| System Architecture | R | C | C | C | C | C | C | I |
| Cashu Integration | C | R | I | I | I | I | I | A |
| Nostr Events | C | I | R | C | I | I | I | A |
| Game Mechanics | C | I | I | R | C | C | I | A |
| User Interface | I | I | I | C | R | I | I | A |
| Tournament System | C | I | I | C | C | R | I | A |
| Deployment | I | I | I | I | I | I | R | C |
| Testing Strategy | I | C | C | C | C | C | C | R |

**Legend:**
- R = Responsible (does the work)
- A = Accountable (ultimately answerable)
- C = Consulted (input required)
- I = Informed (kept in the loop)