# Mana Strategy Game - Claude Memory System

## Project Context & Status

### Project Overview
**Name:** Mana Strategy Game  
**Architecture:** Rust backend services + React/React Native frontend  
**Core Concept:** Cashu-powered strategy game with Nostr coordination, player-choice matchmaking, and periodic leaderboard rewards  

### Key Architectural Decisions Made
- ✅ **Simplified Matchmaking:** Player-choice challenges (no Swiss tournaments)
- ✅ **Rust Backend:** All services in Rust using axum, sqlx, tokio
- ✅ **Cashu Integration:** Handles both payments AND VRF for unit generation
- ✅ **Nostr Coordination:** Asynchronous match events and state sync
- ✅ **Periodic Rewards:** Leaderboard-based loot distribution

### Project Structure
```
manastr/
├── docs/                    # ✅ Created - Architecture & specifications
├── planning/                # ✅ Created - Work breakdown & dependencies  
├── implementation/          # 🔄 Future - Code implementation
└── CLAUDE.md               # 📍 THIS FILE - Memory & status tracking
```

## Current Task Status Board (Daemon-Focused)

### 🟢 FOUNDATION TASKS (Ready to Start)
| Task ID | Agent | Daemon Target | Status | Progress |
|---------|-------|---------------|--------|----------|
| F1 | crypto-specialist | Cashu Mint | ⭕ Not Started | Mint modification requirements |
| F2 | game-engine | Game Engine Bot | ⭕ Not Started | Bot logic specification |
| F3 | nostr-dev | Nostr Relay | ⭕ Not Started | Relay setup and event flow |
| F4 | ui-dev | Web Client | ⭕ Not Started | React + Cashu wallet integration |

### ✅ ALL HUMAN DECISIONS RESOLVED
| Decision ID | Description | Resolution |
|-------------|-------------|------------|
| H1 | Choose Cashu library | **cashubtc/cdk** (Rust CDK) |
| H6 | Token economics | **5 mana per sat, 5% fee = loot pool** |
| H7 | Choose Nostr relay | **strfry** (damus-io/strfry) |
| H8 | Choose web Cashu library | **cashubtc/cashu-ts** (TypeScript) |
| H9 | UI complexity for MVP | **Basic UI, no graphics** |

### 🟢 DAEMON IMPLEMENTATION (All Prerequisites Available!)
| Task ID | Agent | Daemon | Prerequisites | Port | Status |
|---------|-------|--------|---------------|------|--------|
| D1 | crypto-specialist | Cashu Mint | F1 ✅ | :3333 | 🟢 Ready to start! |
| D2 | nostr-dev | Nostr Relay | F3 ✅ | :7777 | 🟢 Ready to start! |
| D3 | game-engine | Game Engine Bot | F2 + D1 + D2 | :4444 | Waiting for D1+D2 |
| D4 | ui-dev | Web Client | F4 + D1 + D2 | :8080 | Waiting for D1+D2 |

### 🟡 WAITING FOR DEPENDENCIES
All implementation tasks (C1-C8) wait for foundation tasks and human decisions.

## Agent Memory Templates

### Task Execution Memory Format
```markdown
## [AGENT_NAME] - [TASK_ID]: [TASK_NAME]

### Task Context
- **Started:** [DATE]
- **Last Updated:** [DATE]  
- **Dependencies Met:** [LIST]
- **Current Status:** [In Progress/Blocked/Complete]

### Progress Log
- [DATE] - Started task with [specific approach/methodology]
- [DATE] - Completed [specific milestone/deliverable]
- [DATE] - Encountered [issue/decision point] - [resolution/next steps]

### Current State
- **Files Created/Modified:** [list with paths]
- **Key Decisions Made:** [architectural/implementation choices]
- **Blockers:** [what's preventing progress]
- **Next Steps:** [immediate next actions]

### Deliverable Status
- **Expected Output:** [file path or deliverable description]
- **Completion Criteria:** [how to know task is done]
- **Quality Checks:** [validation steps before marking complete]
```

### Example Agent Memory Entry
```markdown
## crypto-specialist - F1: Cashu NUT Compliance Analysis

### Task Context
- **Started:** 2024-01-15
- **Last Updated:** 2024-01-15
- **Dependencies Met:** None (foundation task)
- **Current Status:** In Progress

### Progress Log
- 2024-01-15 - Started NUT analysis, reviewing NUT-00 through NUT-20
- 2024-01-15 - Completed NUT-00 (BDHKE) analysis, documented secp256k1 requirements
- 2024-01-15 - Working on NUT-03 (token swapping) implementation requirements

### Current State
- **Files Created:** `/docs/specifications/crypto_requirements.md` (partial)
- **Key Decisions Made:** 
  - Will use `secp256k1` crate for all elliptic curve operations
  - SHA-256 for all hashing operations (commitments, VRF)
- **Blockers:** None
- **Next Steps:** Complete NUT-04/NUT-05 Lightning integration analysis

### Deliverable Status
- **Expected Output:** `/docs/specifications/crypto_requirements.md`
- **Completion Criteria:** All NUTs documented with Rust implementation requirements
- **Quality Checks:** Implementation requirements are specific enough for coding
```

## Cross-Agent Context Sharing

### Shared Knowledge Base
**Location:** `/docs/specifications/SHARED_CONTEXT.md`

**Contains:**
- Common architectural decisions affecting multiple agents
- Interface definitions between services
- Shared data structures and types
- Integration points and protocols

### Inter-Agent Dependencies
```markdown
## Service Interface Contracts

### Game Engine → Matchmaking
- `create_match(player1_id, player2_id) -> MatchId`
- `get_match_state(match_id) -> MatchState`
- `resolve_combat(unit_a, unit_b) -> CombatResult`

### Cashu Mint → Game Engine  
- `parse_token_to_units(token: CashuToken) -> [Unit; 8]`
- `verify_token_signature(token: CashuToken) -> bool`

### Matchmaking → Leaderboard
- `record_match_result(match_result: MatchResult)`
- `update_player_rating(player_id, new_rating)`
```

## Progress Tracking System

### Task State Transitions
```
⭕ Not Started → 🔄 In Progress → ✅ Complete
                      ↓
                   ⛔ Blocked (waiting for dependency/decision)
```

### Completion Verification
Before marking any task as ✅ Complete:

1. **Deliverable Check:** Expected output file exists and is complete
2. **Quality Gate:** Meets all specified acceptance criteria  
3. **Dependency Update:** Any tasks waiting for this one become ready
4. **Memory Update:** Progress log shows completion and handoff info

### Status Reporting Format
```markdown
## Weekly Status Report - [DATE]

### Completed This Period
| Task ID | Agent | Deliverable | Impact |
|---------|-------|-------------|--------|
| F1 | crypto-specialist | Crypto requirements spec | Unblocks C1, C3 |

### In Progress
| Task ID | Agent | % Complete | Blocker | ETA |
|---------|-------|------------|---------|-----|
| F2 | game-engine | 60% | None | 2 days |

### Newly Ready (Dependencies Met)
| Task ID | Agent | Can Start | Priority |
|---------|-------|-----------|----------|
| C3 | nostr-dev | Now | High |

### Human Decisions Needed
| Decision | Impact | Urgency |
|----------|--------|---------|
| H1 (Cashu library) | Blocks C1 implementation | Critical |
```

## Memory Persistence Strategy

### File-Based Memory System
Each agent maintains its memory in dedicated files:
- `/docs/agent-memory/[AGENT_NAME]_memory.md`
- `/docs/agent-memory/[AGENT_NAME]_task_[TASK_ID].md`

### Context Restoration Protocol
When an agent resumes work:

1. **Read Project Context:** This CLAUDE.md file for overall status
2. **Read Agent Memory:** Agent's specific memory file for task history
3. **Check Dependencies:** Verify all prerequisites are still met
4. **Load Shared Context:** Read shared knowledge base for interface changes
5. **Resume Work:** Continue from last recorded state

### Memory Update Triggers
Agents must update memory when:
- Starting a new task
- Completing a major milestone  
- Making architectural decisions
- Encountering blockers
- Completing a task
- Handing off work to another agent

## Quality Assurance for Memory

### Memory Validation Checklist
- [ ] Current task status is accurately reflected
- [ ] All architectural decisions are documented
- [ ] Dependencies and blockers are clearly stated
- [ ] Next steps are specific and actionable
- [ ] Deliverable status is up-to-date

### Memory Consistency Checks
- Agent memories don't contradict each other
- Shared context reflects all agents' decisions
- Task dependencies are consistent across agents
- Status board matches individual agent memories

This memory system ensures agents can pick up exactly where they left off and maintain consistency across the entire project!