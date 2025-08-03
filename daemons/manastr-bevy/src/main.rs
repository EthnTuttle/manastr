use bevy::prelude::*;
use tracing::info;

mod game;
mod networking;
mod ui;
mod manastr_core;

use game::GamePlugin;
use networking::NetworkingPlugin;
use ui::UIPlugin;
use manastr_core::ManastrCorePlugin;

fn main() {
    // Initialize tracing for debugging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting Manastr Bevy - Revolutionary Decentralized Gaming");
    
    App::new()
        // Bevy built-in plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "MANASTR - Revolutionary Trading Card Game".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        
        // Custom Manastr plugins
        .add_plugins((
            ManastrCorePlugin,    // Security core (Nostr, Cashu, validation)
            NetworkingPlugin,     // Hybrid networking (Nostr + WebRTC)
            GamePlugin,          // Game logic and state management
            UIPlugin,            // Modern responsive UI
        ))
        
        .run();
}