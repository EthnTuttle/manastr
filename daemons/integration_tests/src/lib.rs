// Integration tests for player-driven gaming architecture
//
// This module contains comprehensive tests for the world's first
// truly decentralized multiplayer gaming system.

pub mod core;
pub mod gaming_auth_test;
pub mod matches;
pub mod players;
pub mod test_suite;
pub mod utils;
pub mod validation;

pub use test_suite::PlayerDrivenTestSuite;
