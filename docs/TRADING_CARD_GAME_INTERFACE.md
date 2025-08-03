# 🎮 MANASTR: Revolutionary Trading Card Game Interface Design

## Overview

This document specifies the design for a **Trading Card Game (TCG) interface** built with **iced.rs** that layers on top of our revolutionary zero-coordination gaming integration test. The interface transforms technical blockchain operations into intuitive card game mechanics while maintaining full educational transparency.

## 🏗️ Architecture Design

### Core Principle: Integration Test as Interactive Tutorial
- **Base Layer**: Existing integration test with service orchestration (CDK mint, Game Engine, Nostr relay)
- **Interface Layer**: iced.rs GUI that presents both sides of a match interactively
- **Educational Layer**: Progressive disclosure from game mechanics to technical operations

### Service Integration Strategy
```rust
// Retain existing service setup but enable dual-player control
struct ManastrTCG {
    integration_runner: IntegrationRunner,  // Reuse existing service orchestration
    player_alice: PlayerInterface,         // Interactive control for Alice
    player_bob: PlayerInterface,           // Interactive control for Bob  
    game_state: MatchState,                // Current match progression
    ui_mode: UIDisplayMode,                // Simple/Advanced/Technical views
}
```

---

## 🎨 iced.rs Implementation Structure

### Main Application Structure
```rust
use iced::{Application, Command, Element, Settings, Subscription};

#[derive(Debug, Clone)]
pub enum ManastrTCG {
    Loading,
    ServiceSetup(ServiceSetupState),
    GameLobby(GameLobbyState), 
    ActiveMatch(ActiveMatchState),
    MatchComplete(MatchCompleteState),
}

#[derive(Debug, Clone)]
pub enum Message {
    // Service Management
    ServicesStarted,
    ServiceError(String),
    
    // Game Flow
    MintTokens(Player),
    CreateChallenge(ChallengeParams),
    AcceptChallenge(String), // match_id
    RevealArmy(Player),
    SubmitMove(Player, CombatMove),
    
    // UI Controls
    SwitchView(UIDisplayMode),
    ShowTechnicalDetails(bool),
    RestartMatch,
}
```

### State Management
```rust
#[derive(Debug, Clone)]
pub struct ActiveMatchState {
    // Integration test components (reused)
    pub test_suite: PlayerDrivenTestSuite,
    pub alice: TestPlayer,
    pub bob: TestPlayer,
    
    // Interactive game state
    pub current_phase: MatchPhase,
    pub active_player: Player,
    pub battlefield: BattlefieldState,
    pub ui_mode: UIDisplayMode,
    
    // Visual elements
    pub alice_cards: Vec<ArmyCard>,
    pub bob_cards: Vec<ArmyCard>,
    pub combat_log: Vec<CombatEvent>,
    pub economic_display: EconomicState,
}

#[derive(Debug, Clone)]
pub enum MatchPhase {
    Setup,                    // Services starting
    Minting,                 // Phase 1: Token minting
    Challenge,               // Phase 2: Match creation  
    Acceptance,              // Phase 3: Challenge acceptance
    ArmyReveal,             // Phase 4: Token revelation
    Combat(u32),            // Phase 5: Combat rounds (1-3)
    Resolution,             // Phase 6: Match result
    LootDistribution,       // Phase 7: Rewards
    Complete,               // Match finished
}
```

---

## 🃏 Visual Component Design

### Card Rendering System
```rust
pub fn render_army_card(card: &ArmyCard, theme: &Theme) -> Element<Message> {
    Container::new(
        Column::new()
            .push(render_card_header(card))
            .push(render_card_stats(card))
            .push(render_card_abilities(card))
            .push(render_c_value_proof(card))
    )
    .style(theme.card_style())
    .padding(10)
    .into()
}

pub fn render_battlefield() -> Element<Message> {
    Row::new()
        .push(render_player_side(Player::Alice))
        .push(render_combat_zone())
        .push(render_player_side(Player::Bob))
        .spacing(20)
        .into()
}
```

### Game Board Layout Components
```rust
pub struct GameBoard;

impl GameBoard {
    pub fn render(state: &ActiveMatchState) -> Element<Message> {
        Column::new()
            .push(Self::render_header(state))
            .push(Self::render_main_area(state))
            .push(Self::render_controls(state))
            .into()
    }
    
    fn render_header(state: &ActiveMatchState) -> Element<Message> {
        Row::new()
            .push(Self::render_mana_vault(&state.alice))
            .push(Self::render_match_info(state))
            .push(Self::render_mana_vault(&state.bob))
            .into()
    }
    
    fn render_main_area(state: &ActiveMatchState) -> Element<Message> {
        Row::new()
            .push(Self::render_army_deck(&state.alice_cards))
            .push(Self::render_battlefield(&state.battlefield))
            .push(Self::render_match_log(&state.combat_log))
            .into()
    }
}
```

---

## 🎯 Interactive Game Flow Implementation

### Phase 1: Army Forging (Token Minting)
```rust
pub fn handle_minting_phase(state: &mut ActiveMatchState, player: Player) -> Command<Message> {
    // Reuse integration test minting logic
    let gaming_wallet = &mut match player {
        Player::Alice => &mut state.alice.gaming_wallet,
        Player::Bob => &mut state.bob.gaming_wallet,
    };
    
    // Execute real CDK minting operation
    Command::perform(
        gaming_wallet.mint_gaming_tokens(100, "mana"),
        move |result| Message::TokensMinted(player, result)
    )
}

pub fn render_minting_interface(player: Player) -> Element<Message> {
    Container::new(
        Column::new()
            .push(Text::new("🏛️ THE GREAT MINT"))
            .push(Text::new("Forge your army with cryptographic randomness"))
            .push(Button::new("⚡ Mint Army - 100 sats")
                .on_press(Message::MintTokens(player)))
            .push(Text::new("🎯 Your army will be: UNPREDICTABLE, FAIR, TAMPER-PROOF"))
    )
    .style(theme.mint_chamber_style())
    .into()
}
```

### Phase 2-3: Challenge System
```rust
pub fn handle_challenge_creation(state: &mut ActiveMatchState) -> Command<Message> {
    // Reuse integration test challenge logic
    Command::perform(
        state.test_suite.core.create_and_publish_match_challenge(
            &state.alice, 100, 1
        ),
        Message::ChallengeCreated
    )
}

pub fn render_challenge_interface(state: &ActiveMatchState) -> Element<Message> {
    match state.current_phase {
        MatchPhase::Challenge => render_challenge_creation(),
        MatchPhase::Acceptance => render_challenge_acceptance(state),
        _ => Container::new(Text::new("")).into()
    }
}
```

### Phase 4: Army Revelation with Visual Generation
```rust
pub fn handle_army_reveal(state: &mut ActiveMatchState, player: Player) -> Command<Message> {
    // Execute real token revelation (reuse integration test)
    let reveal_command = Command::perform(
        state.test_suite.core.publish_token_reveal(&state.alice, &state.match_id),
        Message::ArmyRevealed
    );
    
    // Generate visual army cards from C values
    let cards_command = Command::perform(
        generate_army_cards_from_c_values(&state.alice.gaming_wallet),
        Message::ArmyCardsGenerated
    );
    
    Command::batch(vec![reveal_command, cards_command])
}

async fn generate_army_cards_from_c_values(wallet: &GamingWallet) -> Vec<ArmyCard> {
    let tokens = wallet.get_all_gaming_tokens();
    tokens.iter().take(4).map(|token| {
        // Use real shared game logic for army generation
        let unit_stats = shared_game_logic::combat::generate_unit_from_c_value(
            &token.c_value, 0
        );
        
        ArmyCard {
            unit_type: unit_stats.unit_type,
            hp: unit_stats.hp,
            attack: unit_stats.attack,
            defense: unit_stats.defense,
            special_ability: unit_stats.special_ability,
            c_value_proof: token.c_value.clone(),
        }
    }).collect()
}
```

### Phase 5: Combat System with Real Nostr Events
```rust
pub fn handle_combat_move(
    state: &mut ActiveMatchState, 
    player: Player, 
    combat_move: CombatMove
) -> Command<Message> {
    // Execute real combat move via integration test
    Command::perform(
        state.test_suite.core.publish_combat_move(
            match player {
                Player::Alice => &state.alice,
                Player::Bob => &state.bob,
            },
            &state.match_id,
            state.current_round,
            state.previous_event_hash.clone()
        ),
        Message::CombatMovePublished
    )
}

pub fn render_combat_interface(state: &ActiveMatchState) -> Element<Message> {
    Column::new()
        .push(Text::new("⚔️ COMBAT ROUND {}/3").size(24))
        .push(render_battlefield_combat(state))
        .push(render_combat_actions(state))
        .push(render_timer(state.phase_timer))
        .into()
}
```

---

## 🎓 Educational Integration

### Progressive Disclosure System
```rust
#[derive(Debug, Clone)]
pub enum UIDisplayMode {
    Simple,      // Pure game interface
    Educational, // Game + explanations
    Technical,   // Game + technical details
    Advanced,    // Full integration test view
}

pub fn render_with_educational_overlay(
    base_element: Element<Message>,
    mode: UIDisplayMode,
    technical_info: &TechnicalInfo
) -> Element<Message> {
    match mode {
        UIDisplayMode::Simple => base_element,
        UIDisplayMode::Educational => {
            Column::new()
                .push(base_element)
                .push(render_educational_explanation(technical_info))
                .into()
        },
        UIDisplayMode::Technical => {
            Row::new()
                .push(base_element)
                .push(render_technical_panel(technical_info))
                .into()
        },
        UIDisplayMode::Advanced => {
            render_integration_test_view(technical_info)
        }
    }
}
```

### Technical Information Display
```rust
pub fn render_technical_panel(info: &TechnicalInfo) -> Element<Message> {
    Scrollable::new(
        Column::new()
            .push(Text::new("🔐 CRYPTOGRAPHIC STATE"))
            .push(Text::new(format!("Event ID: {}", info.current_event_id)))
            .push(Text::new(format!("Nostr Kind: {}", info.event_kind)))
            .push(Text::new(format!("Commitment Hash: {}", info.commitment_hash)))
            .push(Text::new("📡 NETWORK STATE"))
            .push(Text::new(format!("Mint Status: {}", info.mint_status)))
            .push(Text::new(format!("Relay Connected: {}", info.relay_status)))
            .push(Text::new("💰 ECONOMIC STATE"))
            .push(Text::new(format!("Total Wagered: {} MANA", info.total_wager)))
            .push(Text::new(format!("Winner Gets: {} LOOT", info.winner_amount)))
    )
    .into()
}
```

---

## 🚀 Service Integration Layer

### Reuse Existing Integration Test Infrastructure
```rust
pub struct ServiceManager {
    integration_runner: IntegrationRunner,
    services_ready: bool,
}

impl ServiceManager {
    pub async fn initialize() -> Result<Self, ServiceError> {
        let mut runner = IntegrationRunner::new()
            .add_cashu_mint()
            .add_game_engine()
            .add_nostr_relay();
            
        runner.start_all_services().await?;
        
        Ok(Self {
            integration_runner: runner,
            services_ready: true,
        })
    }
    
    pub fn get_test_suite(&self) -> PlayerDrivenTestSuite {
        // Reuse existing test suite creation
        PlayerDrivenTestSuite::new().await.unwrap()
    }
}
```

### Dual Player Control System
```rust
pub struct DualPlayerController {
    pub alice_interface: PlayerInterface,
    pub bob_interface: PlayerInterface,
    pub active_player: Player,
}

impl DualPlayerController {
    pub fn switch_active_player(&mut self) {
        self.active_player = match self.active_player {
            Player::Alice => Player::Bob,
            Player::Bob => Player::Alice,
        };
    }
    
    pub fn render_player_controls(&self, state: &ActiveMatchState) -> Element<Message> {
        Row::new()
            .push(self.render_alice_controls(state))
            .push(Button::new("🔄 Switch Player")
                .on_press(Message::SwitchActivePlayer))
            .push(self.render_bob_controls(state))
            .into()
    }
}
```

---

## 📁 File Structure for iced.rs Implementation

```
daemons/manastr-tcg/
├── Cargo.toml                    # iced.rs dependencies
├── src/
│   ├── main.rs                   # Application entry point
│   ├── app.rs                    # Main iced application
│   ├── state/
│   │   ├── mod.rs
│   │   ├── match_state.rs        # Match progression state
│   │   ├── player_state.rs       # Individual player state
│   │   └── ui_state.rs          # UI display state
│   ├── components/
│   │   ├── mod.rs
│   │   ├── game_board.rs        # Main game board layout
│   │   ├── army_cards.rs        # Card rendering system
│   │   ├── battlefield.rs       # Combat visualization
│   │   ├── economic_display.rs  # Rewards/economics panel
│   │   └── technical_panel.rs   # Educational overlays
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── service_manager.rs   # Reuse integration test services
│   │   ├── test_bridge.rs       # Bridge to existing test suite
│   │   └── dual_player.rs       # Control both sides of match
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── colors.rs           # TCG color scheme
│   │   ├── styles.rs           # Card and UI styles
│   │   └── assets.rs           # Icons and graphics
│   └── utils/
│       ├── mod.rs
│       ├── c_value_display.rs  # Cryptographic proof visualization
│       └── nostr_events.rs     # Event chain display
```

---

## 🎯 Implementation Roadmap

### Phase 1: Basic iced.rs Structure
1. Create basic iced application with service integration
2. Implement state management for match progression
3. Create basic card rendering system

### Phase 2: Game Flow Integration
1. Integrate with existing integration test services
2. Implement dual-player control system
3. Add real Nostr event publishing/receiving

### Phase 3: Visual Polish
1. Design beautiful card animations
2. Add combat visualizations
3. Implement economic dashboard

### Phase 4: Educational Features
1. Add progressive disclosure system
2. Implement technical overlays
3. Create tutorial progression system

---

## 🔧 Key Dependencies

```toml
[dependencies]
iced = "0.10"
iced_aw = "0.7"  # Additional widgets
tokio = "1.0"
serde = "1.0"
serde_json = "1.0"

# Reuse existing project dependencies
anyhow = { workspace = true }
tracing = { workspace = true }
nostr = { workspace = true }
shared-game-logic = { path = "../shared-game-logic" }

# Integration with existing test infrastructure
integration-tests = { path = "../integration_tests" }
```

This design creates a **revolutionary educational gaming experience** that teaches users about decentralized gaming while they play actual matches with real economic stakes, all built on top of our proven integration test infrastructure! 🎮✨