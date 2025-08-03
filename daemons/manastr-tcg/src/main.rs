use iced::{Application, Settings};

mod app;

fn main() -> iced::Result {
    // Initialize tracing for debugging
    tracing_subscriber::fmt::init();
    
    // Run the Manastr TCG application
    app::ManastrTCG::run(Settings::default())
}