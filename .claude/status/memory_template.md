# Agent Memory Template

## [AGENT_NAME] - Task Memory

### Current Assignment
- **Task ID:** [e.g., F1, C2, E1]
- **Task Name:** [descriptive name]
- **Started:** [date]
- **Last Active:** [date]
- **Status:** [Not Started/In Progress/Blocked/Complete]

### Task Context
- **Dependencies Required:** [list what must be complete first]
- **Dependencies Met:** [✅/❌ for each dependency]
- **Blocks These Tasks:** [what's waiting for this to complete]
- **Human Decisions Needed:** [any approvals/inputs required]

### Progress Journal
```
[DATE] [TIME] - [What was accomplished/discovered/decided]
[DATE] [TIME] - [Next significant update]
```

**Example:**
```
2024-01-15 09:00 - Started F1 (Cashu NUT Analysis). Reading through NUT-00 to NUT-20 specifications.
2024-01-15 11:30 - Completed NUT-00 analysis. Key finding: BDHKE requires secp256k1 curve operations.
2024-01-15 14:15 - Analyzed NUT-01/02 keyset management. Decision: Will use single keyset with 16 league-specific keys.
2024-01-15 16:45 - Blocked on NUT-04 Lightning integration - need to research Rust Lightning libraries.
```

### Current State
- **Files Created:** [paths to any files created/modified]
- **Code/Documentation Written:** [specific outputs]
- **Architectural Decisions Made:** [key choices that affect other components]
- **Research Completed:** [findings that inform implementation]

### Immediate Next Steps
1. [Specific next action]
2. [Following action]
3. [Third priority action]

### Blockers & Dependencies
- **Currently Blocked By:** [what's preventing progress]
- **Waiting For:** [human decisions, other agent completions]
- **Can Proceed When:** [specific conditions to resume]

### Handoff Information
- **For Next Agent:** [what the next person needs to know]
- **Interface Contracts:** [any APIs/protocols defined]
- **Assumptions Made:** [decisions that might need revisiting]

### Quality Checklist
- [ ] Deliverable matches task specification
- [ ] All acceptance criteria met
- [ ] Documentation is complete and clear
- [ ] Dependencies for next tasks are satisfied
- [ ] Changes communicated to affected agents

## Cross-Agent Context

### Shared Decisions Made
[Any architectural or implementation decisions that affect other agents]

### Interface Definitions
[APIs, data structures, or protocols defined that other agents will use]

### Open Questions  
[Things that need clarification or decisions from other agents/human]

---

## Memory Maintenance Notes

**Last Memory Update:** [timestamp]
**Memory Validated By:** [agent name]
**Next Review Due:** [date]

### Memory Health Check
- [ ] All dates are current and accurate
- [ ] Status reflects actual progress
- [ ] Dependencies are up-to-date
- [ ] Next steps are actionable
- [ ] No contradictions with other agent memories