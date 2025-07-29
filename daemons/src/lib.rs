// 🚀 MANASTR DAEMONS LIBRARY
// ==========================
//
// This library provides shared modules for the Manastr gaming architecture.
// It includes service orchestration for integration testing and other shared utilities.

pub mod integration_runner;

// Player-driven integration test module path reference
#[path = "../player-driven-integration-test.rs"]
pub mod player_driven_integration_test;

// Economic model for optimized loot distribution
#[path = "../economic_model.rs"]
pub mod economic_model;

// Re-exports for convenience
pub use integration_runner::{IntegrationRunner, run_complete_integration_test};