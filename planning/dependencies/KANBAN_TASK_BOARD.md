# Mana Strategy Game - Kanban Task Board

## Task Status Board

### 🟢 READY TO START (No Dependencies)

| Task ID | Task Name | Agent | Deliverable |
|---------|-----------|-------|-------------|
| F1 | Cryptographic Specification | crypto-specialist | `/docs/specifications/crypto_requirements.md` |
| F2 | Game Mechanics Specification | game-engine | `/docs/specifications/game_mechanics.md` |
| F3 | API Design Specification | architect | `/docs/specifications/api_spec.yaml` |

### 🟡 WAITING FOR DEPENDENCIES

| Task ID | Task Name | Agent | Waiting For | Ready When |
|---------|-----------|-------|-------------|------------|
| F4 | Database Schema Design | architect | F3 | API design complete |
| F5 | UI Component Specifications | ui-dev | F2, F3 | Game mechanics + API complete |
| C1 | Cashu Mint Service | crypto-specialist | F1, F4 | Crypto spec + database schema |
| C2 | Game Engine Service | game-engine | F2, F4 | Game mechanics + database schema |
| C3 | Nostr Client Service | nostr-dev | F1, F2 | Crypto spec + game mechanics |
| C4 | Database Setup | architect | F4 | Database schema complete |
| C5 | Matchmaking Service | matchmaking-dev | C2, C4 | Game engine + database |
| C6 | Leaderboard Service | leaderboard-dev | C5, C1 | Matchmaking + Cashu mint |
| C7 | API Gateway | architect | C1, C2, C3, C5, C6 | All core services |
| C8 | Frontend Implementation | ui-dev | F5, C7 | UI specs + API gateway |

### 🔵 IN PROGRESS

| Task ID | Task Name | Agent | Started | Progress |
|---------|-----------|-------|---------|----------|
| | | | | |

### ✅ COMPLETED

| Task ID | Task Name | Agent | Completed | Output |
|---------|-----------|-------|-----------|--------|
| | | | | |

## Next Wave Predictions

### When F1 (Crypto Spec) Completes:
**Becomes Ready:**
- C3 (Nostr Client) - if F2 also complete
- C1 (Cashu Mint) - if F4 also complete

### When F2 (Game Mechanics) Completes:
**Becomes Ready:**
- F5 (UI Specs) - if F3 also complete
- C2 (Game Engine) - if F4 also complete
- C3 (Nostr Client) - if F1 also complete

### When F3 (API Design) Completes:
**Becomes Ready:**
- F4 (Database Schema)
- F5 (UI Specs) - if F2 also complete

## Critical Path Analysis

### Longest Dependency Chain:
```
F3 → F4 → C4 → C5 → C6 → C7 → C8 → T2 → T3 → D1
```

### Parallel Development Opportunities:
- **Foundation Phase:** F1, F2, F3 (all parallel)
- **Service Development:** C1, C2, C3 (parallel once prereqs met)
- **Testing:** T1 can start as soon as any service completes

## Agent Availability Optimization

### High-Priority First Assignments:
1. **crypto-specialist** → F1 (blocks C1, C3)
2. **game-engine** → F2 (blocks C2, C3, F5)
3. **architect** → F3 (blocks F4, F5, critical path)

### Agent Pipeline Planning:
- **crypto-specialist:** F1 → C1 → E2 (crypto enhancement)
- **game-engine:** F2 → C2 → (available for other tasks)
- **architect:** F3 → F4 → C4 → C7 (API orchestration)
- **ui-dev:** (wait for F5) → C8 → E3 (social features)
- **nostr-dev:** (wait for F1+F2) → C3 → (available)
- **matchmaking-dev:** (wait for C2+C4) → C5 → E3 (social)
- **leaderboard-dev:** (wait for C5+C1) → C6 → E1 → E2
- **qa-lead:** (parallel testing) → T1 → T2 → T3 → D1

## Task Completion Triggers

### Automated Task Status Updates:
```bash
# When a task completes, automatically check:
# 1. Mark task as complete
# 2. Check all waiting tasks for newly satisfied dependencies
# 3. Move newly ready tasks to "READY TO START"
# 4. Assign available agents to ready tasks

update_task_status(task_id: str, status: "completed") {
    mark_complete(task_id)
    newly_ready = check_dependencies_satisfied()
    move_to_ready(newly_ready)
    assign_available_agents()
}
```

## Code References Needed Tracker

### Blocking on External References:
- **C1 (Cashu Mint):** Need Cashu Rust library integration patterns
- **C5 (Matchmaking):** ~~Need Swiss tournament algorithm~~ ✅ REMOVED

### Self-Contained Implementation:
- All other tasks use well-established Rust patterns
- Game mechanics are straightforward algorithmic implementation  
- Nostr integration uses existing `nostr-sdk` crate
- Database and API patterns are standard

## Risk Mitigation

### Single Points of Failure:
- **F3 → F4 → C4:** Architect dependency chain (critical path)
- **C1 (Cashu):** External library dependency

### Mitigation Strategies:
- Prioritize F3 completion to unblock F4
- Research Cashu Rust library options early
- Have backup agents familiar with architect tasks

This kanban approach lets Claude agents pick up ready tasks immediately without waiting for predetermined schedules!