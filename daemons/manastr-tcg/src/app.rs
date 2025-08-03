use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, column, container, row, text, space, progress_bar},
    Application, Command, Element, Length, Theme, Color, Background, Subscription,
    time,
};
use iced::widget::Button;
use std::time::{Duration, Instant};
use tracing::{info, error};

#[derive(Debug, Clone)]
pub enum ManastrTCG {
    Loading,
    ServicesStarting,
    GameLobby(GameLobbyState),
    ActiveMatch(ActiveMatchState),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct GameLobbyState {
    services_ready: bool,
    mint_url: String,
    relay_url: String,
}

#[derive(Debug, Clone)]
pub struct ActiveMatchState {
    current_phase: MatchPhase,
    alice_ready: bool,
    bob_ready: bool,
    active_player: Player,
    animation_progress: f32,
    last_tick: Option<Instant>,
    // Economics - 100 MANA = 1000 LOOT
    total_mana_wagered: u64,
    winner_loot_amount: u64,
}

#[derive(Debug, Clone)]
pub enum MatchPhase {
    Setup,
    Minting,
    Challenge,
    Acceptance,
    ArmyReveal,
    Combat(u32),
    Resolution,
    Complete,
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    Alice,
    Bob,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Application lifecycle
    StartServices,
    ServicesReady,
    ServiceError(String),
    
    // Game flow
    StartMatch,
    MintTokens(Player),
    TokensMinted(Player),
    CreateChallenge,
    AcceptChallenge,
    RevealArmy(Player),
    NextPhase,
    
    // UI controls
    SwitchPlayer,
    RestartMatch,
    
    // Animation
    Tick,
}

impl Application for ManastrTCG {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        info!("🎮 Starting Manastr TCG Application");
        (
            ManastrTCG::Loading,
            Command::perform(async {}, |_| Message::StartServices)
        )
    }

    fn title(&self) -> String {
        String::from("Manastr - Revolutionary Trading Card Game")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartServices => {
                info!("🏗️ Starting backend services...");
                *self = ManastrTCG::ServicesStarting;
                Command::perform(start_services(), |result| {
                    match result {
                        Ok(_) => Message::ServicesReady,
                        Err(e) => Message::ServiceError(e.to_string()),
                    }
                })
            }
            
            Message::ServicesReady => {
                info!("✅ Services ready! Moving to game lobby");
                *self = ManastrTCG::GameLobby(GameLobbyState {
                    services_ready: true,
                    mint_url: "http://localhost:3333".to_string(),
                    relay_url: "ws://localhost:7777".to_string(),
                });
                Command::none()
            }
            
            Message::ServiceError(error) => {
                error!("❌ Service error: {}", error);
                *self = ManastrTCG::Error(error);
                Command::none()
            }
            
            Message::StartMatch => {
                info!("Starting new match!");
                *self = ManastrTCG::ActiveMatch(ActiveMatchState {
                    current_phase: MatchPhase::Minting,
                    alice_ready: false,
                    bob_ready: false,
                    active_player: Player::Alice,
                    animation_progress: 0.0,
                    last_tick: None,
                    total_mana_wagered: 200, // 100 MANA per player
                    winner_loot_amount: 1000, // 100 MANA = 1000 LOOT
                });
                Command::none()
            }
            
            Message::MintTokens(player) => {
                info!("🪙 {} minting tokens...", player_name(&player));
                Command::perform(mint_tokens_async(player.clone()), Message::TokensMinted)
            }
            
            Message::TokensMinted(player) => {
                info!("✅ {} tokens minted successfully!", player_name(&player));
                if let ManastrTCG::ActiveMatch(ref mut state) = self {
                    match player {
                        Player::Alice => state.alice_ready = true,
                        Player::Bob => state.bob_ready = true,
                    }
                    
                    if state.alice_ready && state.bob_ready {
                        state.current_phase = MatchPhase::Challenge;
                    }
                }
                Command::none()
            }
            
            Message::CreateChallenge => {
                info!("⚔️ Creating match challenge...");
                if let ManastrTCG::ActiveMatch(ref mut state) = self {
                    state.current_phase = MatchPhase::Acceptance;
                }
                Command::none()
            }
            
            Message::AcceptChallenge => {
                info!("🤝 Challenge accepted!");
                if let ManastrTCG::ActiveMatch(ref mut state) = self {
                    state.current_phase = MatchPhase::ArmyReveal;
                }
                Command::none()
            }
            
            Message::RevealArmy(player) => {
                info!("🔮 {} revealing army...", player_name(&player));
                if let ManastrTCG::ActiveMatch(ref mut state) = self {
                    state.current_phase = MatchPhase::Combat(1);
                }
                Command::none()
            }
            
            Message::NextPhase => {
                if let ManastrTCG::ActiveMatch(ref mut state) = self {
                    state.current_phase = match state.current_phase {
                        MatchPhase::Combat(round) if round < 3 => MatchPhase::Combat(round + 1),
                        MatchPhase::Combat(_) => MatchPhase::Resolution,
                        MatchPhase::Resolution => MatchPhase::Complete,
                        _ => state.current_phase.clone(),
                    };
                }
                Command::none()
            }
            
            Message::SwitchPlayer => {
                if let ManastrTCG::ActiveMatch(ref mut state) = self {
                    state.active_player = match state.active_player {
                        Player::Alice => Player::Bob,
                        Player::Bob => Player::Alice,
                    };
                }
                Command::none()
            }
            
            Message::RestartMatch => {
                info!("🔄 Restarting match...");
                *self = ManastrTCG::GameLobby(GameLobbyState {
                    services_ready: true,
                    mint_url: "http://localhost:3333".to_string(),
                    relay_url: "ws://localhost:7777".to_string(),
                });
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            ManastrTCG::Loading => render_loading(),
            ManastrTCG::ServicesStarting => render_services_starting(),
            ManastrTCG::GameLobby(state) => render_game_lobby(state),
            ManastrTCG::ActiveMatch(state) => render_active_match(state),
            ManastrTCG::Error(error) => render_error(error),
        }
    }
}

fn render_loading() -> Element<'static, Message> {
    container(
        column![
            text("MANASTR").size(64),
            space().height(16),
            text("Revolutionary Trading Card Game").size(20),
            space().height(32),
            text("Initializing...").size(16).color(Color::from_rgb(0.7, 0.7, 0.7)),
        ]
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

fn render_services_starting() -> Element<'static, Message> {
    container(
        column![
            text("Starting Backend Services").size(32),
            space().height(24),
            column![
                text("CDK Cashu Mint").size(16),
                text("Game Engine Bot").size(16),
                text("Nostr Relay").size(16),
            ]
            .spacing(8),
            space().height(24),
            text("Please wait...").size(16).color(Color::from_rgb(0.7, 0.7, 0.7)),
        ]
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

fn render_game_lobby(state: &GameLobbyState) -> Element<Message> {
    container(
        column![
            // Title section
            column![
                text("THE GREAT ARENA").size(48),
                space().height(8),
                text("Welcome, Champion!").size(24),
            ]
            .align_items(iced::Alignment::Center),
            
            space().height(40),
            
            // Description
            container(
                text("Experience revolutionary zero-coordination gaming")
                    .size(18)
                    .color(Color::from_rgb(0.8, 0.8, 0.8))
            )
            .width(Length::Fill)
            .center_x(),
            
            space().height(40),
            
            // Main action button
            if state.services_ready {
                Button::new(text("Start New Match").size(20))
                    .padding([16, 32])
                    .on_press(Message::StartMatch)
            } else {
                Button::new(text("Services Not Ready").size(20))
                    .padding([16, 32])
            },
            
            space().height(24),
            
            // Info text
            container(
                text("Learn decentralized gaming through actual gameplay")
                    .size(14)
                    .color(Color::from_rgb(0.6, 0.6, 0.6))
            )
            .width(Length::Fill)
            .center_x(),
        ]
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

fn render_active_match(state: &ActiveMatchState) -> Element<Message> {
    container(
        column![
            // Header
            render_match_header(state),
            
            // Main content based on phase
            render_phase_content(state),
            
            // Controls
            render_match_controls(state),
        ]
        .spacing(30)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(20)
    .into()
}

fn render_match_header(state: &ActiveMatchState) -> Element<Message> {
    row![
        text("👤 Alice").size(20),
        text(format!("🎯 Phase: {}", phase_name(&state.current_phase))).size(24),
        text("👤 Bob").size(20),
    ]
    .spacing(50)
    .align_items(iced::Alignment::Center)
    .into()
}

fn render_phase_content(state: &ActiveMatchState) -> Element<Message> {
    match &state.current_phase {
        MatchPhase::Minting => {
            column![
                text("🏛️ THE GREAT MINT").size(32),
                text("Each player needs to mint 100 MANA tokens to forge their army").size(16),
                text("Tokens contain cryptographic randomness for fair army generation").size(14),
                
                row![
                    render_player_minting(Player::Alice, state.alice_ready),
                    render_player_minting(Player::Bob, state.bob_ready),
                ]
                .spacing(100),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        MatchPhase::Challenge => {
            column![
                text("⚔️ CHALLENGE ARENA").size(32),
                text("Alice creates a match challenge").size(16),
                button("📡 Broadcast Challenge")
                    .padding(15)
                    .on_press(Message::CreateChallenge),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        MatchPhase::Acceptance => {
            column![
                text("🤝 INCOMING CHALLENGE").size(32),
                text("Bob can accept Alice's challenge").size(16),
                text("💰 Wager: 100 MANA each").size(14),
                text("🏆 Winner gets: 190 MANA (95%)").size(14),
                button("✅ Accept Challenge")
                    .padding(15)
                    .on_press(Message::AcceptChallenge),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        MatchPhase::ArmyReveal => {
            column![
                text("🔮 ARMY REVELATION").size(32),
                text("Both players reveal their armies").size(16),
                button("🔓 Reveal Armies")
                    .padding(15)
                    .on_press(Message::RevealArmy(Player::Alice)),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        MatchPhase::Combat(round) => {
            column![
                text(format!("⚔️ COMBAT ROUND {}/3", round)).size(32),
                text("Battle in progress...").size(16),
                button("➡️ Next Round")
                    .padding(15)
                    .on_press(Message::NextPhase),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        MatchPhase::Resolution => {
            column![
                text("🏆 BATTLE RESOLVED").size(32),
                text("Calculating winner...").size(16),
                button("💰 Distribute Rewards")
                    .padding(15)
                    .on_press(Message::NextPhase),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        MatchPhase::Complete => {
            column![
                text("🎉 MATCH COMPLETE!").size(32),
                text("Winner: Alice").size(20),
                text("💎 190 LOOT tokens awarded").size(16),
                text("✅ Zero-coordination gaming cycle complete!").size(14),
            ]
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
        }
        
        _ => text("Game in progress...").into(),
    }
}

fn render_match_controls(state: &ActiveMatchState) -> Element<Message> {
    row![
        button("🔄 Switch Player View")
            .padding(10)
            .on_press(Message::SwitchPlayer),
        text(format!("Active: {}", player_name(&state.active_player))).size(16),
        button("🏠 Return to Lobby")
            .padding(10)
            .on_press(Message::RestartMatch),
    ]
    .spacing(20)
    .align_items(iced::Alignment::Center)
    .into()
}

fn render_error(error: &str) -> Element<'static, Message> {
    container(
        column![
            text("❌ Error").size(32),
            text(error).size(16),
            text("Check that all services are running properly").size(14),
        ]
        .spacing(20)
        .align_items(iced::Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

// Helper functions
fn render_player_minting(player: Player, ready: bool) -> Element<'static, Message> {
    let status_element: Element<'static, Message> = if ready {
        text("✅ Army Ready").size(14).into()
    } else {
        button("🪙 Mint Army")
            .padding(10)
            .on_press(Message::MintTokens(player))
            .into()
    };

    column![
        text(player_name(&player)).size(18),
        status_element
    ]
    .spacing(10)
    .into()
}

fn player_name(player: &Player) -> &'static str {
    match player {
        Player::Alice => "Alice",
        Player::Bob => "Bob",
    }
}

fn phase_name(phase: &MatchPhase) -> String {
    match phase {
        MatchPhase::Setup => "Setup".to_string(),
        MatchPhase::Minting => "Army Forging".to_string(),
        MatchPhase::Challenge => "Challenge".to_string(),
        MatchPhase::Acceptance => "Acceptance".to_string(),
        MatchPhase::ArmyReveal => "Army Reveal".to_string(),
        MatchPhase::Combat(round) => format!("Combat {}/3", round),
        MatchPhase::Resolution => "Resolution".to_string(),
        MatchPhase::Complete => "Complete".to_string(),
    }
}

// Async functions that integrate with our backend
async fn start_services() -> anyhow::Result<()> {
    info!("🏗️ Services are managed by integration runner");
    
    // Use iced-compatible async sleep
    use std::time::Duration;
    async_std::task::sleep(Duration::from_secs(2)).await;
    
    info!("✅ Services assumed ready (started by integration runner)");
    Ok(())
}

async fn mint_tokens_async(player: Player) -> Player {
    info!("🪙 Minting tokens for {}...", player_name(&player));
    
    // Use iced-compatible async sleep for demo
    use std::time::Duration;
    async_std::task::sleep(Duration::from_secs(1)).await;
    
    info!("✅ {} tokens minted successfully (demo mode)", player_name(&player));
    player
}