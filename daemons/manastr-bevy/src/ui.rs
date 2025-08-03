use bevy::prelude::*;
use crate::game::{GameEvent, PlayerAction};
use crate::manastr_core::{GameState, MatchPhase};
use tracing::info;

/// UI Plugin - Modern responsive interface with Bevy's built-in UI
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui_systems)
            .add_systems(Update, (
                handle_ui_interactions,
                update_ui_display,
            ))
            .init_resource::<UiState>();
    }
}

/// UI state management
#[derive(Resource, Default)]
pub struct UiState {
    pub current_screen: UiScreen,
    pub animation_progress: f32,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum UiScreen {
    #[default]
    Loading,
    MainMenu,
    GameLobby,
    ActiveMatch,
    MatchComplete,
}

/// UI Components
#[derive(Component)]
pub struct MainMenuButton {
    pub action: MenuAction,
}

#[derive(Component)]
pub struct MainMenuRoot;

#[derive(Component)]
pub struct GameStatusText;

#[derive(Clone, Debug)]
pub enum MenuAction {
    StartMatch,
    ViewStats,
    Settings,
    Exit,
}

fn setup_ui_systems(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("🎨 Setting up modern UI with Bevy's built-in UI");
    
    // Create camera for UI
    commands.spawn(Camera2dBundle::default());
    
    // Create main menu UI
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.2, 0.95).into(),
            ..default()
        },
        MainMenuRoot,
    )).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "MANASTR",
            TextStyle {
                font_size: 64.0,
                color: Color::WHITE,
                ..default()
            },
        ));
        
        // Subtitle
        parent.spawn(TextBundle::from_section(
            "Revolutionary Trading Card Game",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(0.8, 0.8, 0.8),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        }));
        
        // Start Match Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.6, 0.2).into(),
                ..default()
            },
            MainMenuButton { action: MenuAction::StartMatch },
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start New Match",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
        
        // Game Status Text
        parent.spawn((
            TextBundle::from_section(
                "Services starting...",
                TextStyle {
                    font_size: 16.0,
                    color: Color::rgb(0.7, 0.7, 0.9),
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            }),
            GameStatusText,
        ));
    });
}

fn handle_ui_interactions(
    mut interaction_query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut player_actions: EventWriter<PlayerAction>,
    mut ui_state: ResMut<UiState>,
) {
    for (interaction, menu_button) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                match menu_button.action {
                    MenuAction::StartMatch => {
                        info!("🎯 UI: Start Match button pressed");
                        player_actions.send(PlayerAction::StartMatch);
                        ui_state.current_screen = UiScreen::ActiveMatch;
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                // Could add hover effects here
            }
            Interaction::None => {}
        }
    }
}

fn update_ui_display(
    game_state: Res<GameState>,
    mut query: Query<&mut Text, With<GameStatusText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(ref current_match) = game_state.current_match {
            text.sections[0].value = match current_match.phase {
                MatchPhase::Minting => "Phase: Army Forging - Players minting tokens".to_string(),
                MatchPhase::Challenge => "Phase: Challenge Creation".to_string(),
                MatchPhase::Acceptance => "Phase: Challenge Acceptance".to_string(),
                MatchPhase::ArmyReveal => "Phase: Army Revelation".to_string(),
                MatchPhase::Combat(round) => format!("Phase: Combat Round {}/3", round),
                MatchPhase::Resolution => "Phase: Match Resolution".to_string(),
                MatchPhase::Complete => format!("Match Complete - {} LOOT awarded!", current_match.winner_loot),
            };
        } else {
            text.sections[0].value = "Ready to start new match".to_string();
        }
    }
}