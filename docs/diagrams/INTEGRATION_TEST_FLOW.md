# Integration Test Flow Diagrams
## Revolutionary Zero-Coordination Gaming Protocol

This document provides comprehensive diagrams showing the flow of the integration test and the revolutionary game protocol it demonstrates.

## High-Level Integration Test Flow

```mermaid
flowchart TD
    Start(["Integration Test Start"]) --> Services["Start Services"]
    Services --> |"Cashu Mint<br/>Game Engine<br/>Nostr Relay"| Ready{"All Services Ready?"}
    
    Ready -->|Yes| Test1["Test 1: Happy Path Match"]
    Ready -->|No| Services
    
    Test1 --> Test2["Test 2: Anti-Cheat Verification"]
    Test2 --> Test3["Test 3: Concurrent Matches"]
    Test3 --> Test4["Test 4: Edge Cases"]
    Test4 --> Test5["Test 5: Stress Test"]
    
    Test5 --> Success(["All Tests Passed"])
```

## Complete Match Lifecycle (Test 1: Happy Path)

```mermaid
sequenceDiagram
    participant P1 as Player 1 (Alice)
    participant P2 as Player 2 (Bob)
    participant Mint as Cashu Mint
    participant Relay as Nostr Relay
    participant Engine as Game Engine

    Note over P1,Engine: Phase 1: Player Creation with CDK Gaming Wallets
    P1->>Mint: Request mana tokens
    Mint-->>P1: Return tokens with C values
    P2->>Mint: Request mana tokens  
    Mint-->>P2: Return tokens with C values
    Note over P1,P2: Army generation from C values (4 units per player)

    Note over P1,Engine: Phase 2: Match Challenge (KIND 31000)
    P1->>P1: Create army commitment hash
    P1->>Relay: Publish KIND 31000 (Match Challenge)
    Relay-->>Engine: Event notification
    Relay-->>P2: Event notification

    Note over P1,Engine: Phase 3: Match Acceptance (KIND 31001)
    P2->>P2: Create army commitment hash
    P2->>Relay: Publish KIND 31001 (Match Acceptance)
    Relay-->>Engine: Event notification
    Relay-->>P1: Event notification

    Note over P1,Engine: Phase 4: Token Revelation (KIND 31002)
    P1->>Relay: Publish KIND 31002 (Token Reveal)
    P2->>Relay: Publish KIND 31002 (Token Reveal)
    Engine->>Relay: Listen for reveals
    Engine->>Engine: Validate commitments vs reveals
    Engine->>Mint: Verify token authenticity
    Mint-->>Engine: Token validation response

    Note over P1,Engine: Phase 5: Combat Rounds (KIND 31003/31004)
    loop Each Combat Round
        P1->>Relay: Publish KIND 31003 (Move Commitment)
        P2->>Relay: Publish KIND 31003 (Move Commitment) 
        P1->>Relay: Publish KIND 31004 (Move Reveal)
        P2->>Relay: Publish KIND 31004 (Move Reveal)
        Engine->>Engine: Validate moves and calculate results
    end

    Note over P1,Engine: Phase 6: Match Results (KIND 31005)
    P1->>Relay: Publish KIND 31005 (Match Result)
    P2->>Relay: Publish KIND 31005 (Match Result)

    Note over P1,Engine: Phase 7: Game Engine Authority (KIND 31006)
    Engine->>Engine: Validate entire match
    Engine->>Mint: Burn used mana tokens (exclusive authority)
    Engine->>Mint: Mint loot for winner
    Engine->>Relay: Publish KIND 31006 (Loot Distribution)

    Note over P1,Engine: Phase 8: Final Verification
    P1->>Relay: Query complete event chain
    P2->>Relay: Query complete event chain
    Relay-->>P1: All events verified
    Relay-->>P2: All events verified
```

## Service Architecture & Communication

```mermaid
graph TB
    subgraph "Revolutionary Gaming Architecture"
        subgraph "Players (Zero Trust Required)"
            P1["Player 1<br/>- Nostr Keys<br/>- Gaming Wallet<br/>- C Value Army Generation"]
            P2["Player 2<br/>- Nostr Keys<br/>- Gaming Wallet<br/>- C Value Army Generation"]
        end
        
        subgraph "Decentralized Infrastructure"
            Relay["Nostr Relay<br/>- Event Broadcasting<br/>- Real-time Communication<br/>- Persistence"]
            
            Mint["Cashu Mint<br/>- Token Minting/Burning<br/>- C Value Generation<br/>- Game Engine Authorization"]
            
            Engine["Game Engine<br/>- Pure Validator<br/>- Anti-Cheat Detection<br/>- Loot Distribution"]
        end
    end

    P1 -.->|KIND 31000-31005<br/>Player Events| Relay
    P2 -.->|KIND 31000-31005<br/>Player Events| Relay
    Relay -.->|Event Notifications| P1
    Relay -.->|Event Notifications| P2

    Engine <-.->|Event Processing<br/>KIND 31000-31006| Relay
    Engine <-->|Token Validation<br/>Burn/Mint Operations<br/>Nostr Signatures| Mint

    P1 <-->|Mana Minting<br/>C Value Access| Mint
    P2 <-->|Mana Minting<br/>C Value Access| Mint
```

## Anti-Cheat & Validation Flow

```mermaid
flowchart TD
    Start(["Anti-Cheat Validation"]) --> TokenReveal["Player Reveals Tokens"]
    
    TokenReveal --> CommitmentCheck{"Commitment Hash<br/>Matches Reveal?"}
    CommitmentCheck -->|No| InvalidateMatch["Invalidate Match<br/>Cheating Detected"]
    CommitmentCheck -->|Yes| TokenValidation["Validate Tokens with Mint"]
    
    TokenValidation --> MintCheck{"Mint Confirms<br/>Token Authenticity?"}
    MintCheck -->|No| InvalidateMatch
    MintCheck -->|Yes| DoubleSpendCheck["Check for Double-Spending"]
    
    DoubleSpendCheck --> SpendCheck{"Token Already<br/>Spent Elsewhere?"}
    SpendCheck -->|Yes| InvalidateMatch
    SpendCheck -->|No| ArmyGeneration["Generate Army from C Values"]
    
    ArmyGeneration --> ArmyValidation["Validate Army Determinism"]
    ArmyValidation --> ValidationResult{"All Validations<br/>Passed?"}
    
    ValidationResult -->|No| InvalidateMatch
    ValidationResult -->|Yes| CombatProcessing["Process Combat"]
    
    CombatProcessing --> BurnTokens["Burn Used Mana Tokens<br/>(Game Engine Authority)"]
    BurnTokens --> DistributeLoot["Distribute Loot to Winner"]
    DistributeLoot --> Success(["Match Validated<br/>Loot Distributed"])
    
    InvalidateMatch --> LogCheating["Log Cheating Attempt"]
    LogCheating --> Failure(["Match Invalid<br/>No Loot Distributed"])
```

## Revolutionary Paradigm Features

### Zero-Coordination Gaming
- **Players Control Everything**: Complete match flow driven by player Nostr events
- **Game Engine Cannot Cheat**: Acts as pure validator, cannot manipulate outcomes
- **No Trusted Servers**: All authority derived from cryptographic proofs

### Perfect Anti-Cheat System
- **Commitment/Reveal Scheme**: Prevents strategic cheating via cryptographic binding
- **Mint-Based Randomness**: Army generation from unbiased Cashu token C values
- **Economic Constraints**: Real mana tokens required, preventing Sybil attacks

### Complete Economic Cycle
1. **Mana Investment**: Players spend real mana tokens for match participation
2. **Army Generation**: Deterministic armies from cryptographic C values  
3. **Combat Resolution**: Shared logic ensures identical outcomes
4. **Loot Distribution**: Winners receive valuable loot tokens

### Production-Ready Architecture
- **Concurrent Match Support**: Multiple matches processed simultaneously
- **Runtime Authorization**: Hot-swappable game engine permissions
- **Cross-Platform Compatibility**: Rust-first implementation works everywhere
- **Comprehensive Testing**: 5-tier test suite validates all scenarios

## Test Coverage Summary

| Test Category | Purpose | Key Validations |
|---------------|---------|-----------------|
| **Happy Path** | Complete match lifecycle | All 8 phases, deterministic outcomes |
| **Anti-Cheat** | Commitment verification | Cheating detection, match invalidation |
| **Concurrent** | Multiple matches | Isolated processing, unique armies |
| **Edge Cases** | Malformed events | Error handling, graceful degradation |
| **Stress Test** | High-volume processing | Performance, scalability, reliability |

This represents the **world's first working zero-coordination multiplayer game** with perfect fairness and complete decentralization!