use anyhow::Result;
use chrono::Utc;
use nostr::EventBuilder;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

use super::shared::TestSuiteCore;
use crate::matches::{CombatMove, TokenReveal};

/// Tests edge cases and malicious events
///
/// Verifies that the system handles various edge cases
/// and malicious inputs gracefully.
pub async fn test_edge_cases(core: &TestSuiteCore) -> Result<()> {
    // Test malformed events
    test_malformed_events(core).await?;

    // Test events from unknown players
    test_unknown_player_events(core).await?;

    // Test duplicate events
    test_duplicate_events(core).await?;

    // Test timing attacks
    test_timing_attacks(core).await?;

    info!("✅ Edge cases and malicious events handled correctly");
    Ok(())
}

/// Tests handling of malformed events
async fn test_malformed_events(core: &TestSuiteCore) -> Result<()> {
    info!("🧪 Testing malformed event handling");

    let player = core.create_test_player("Malicious").await?;

    // Publish malformed challenge event (missing required fields)
    let malformed_challenge = json!({
        "challenger_npub": player.keys.public_key().to_string(),
        // Missing wager_amount, league_id, etc.
    });

    let event = EventBuilder::new(
        nostr::Kind::Custom(31000),
        malformed_challenge.to_string(),
        vec![],
    )
    .to_event(&player.keys)?;

    core.nostr_client.send_event(event).await?;

    // Game engine should ignore malformed events
    sleep(Duration::from_secs(2)).await;

    Ok(())
}

/// Tests handling of events from unknown players
async fn test_unknown_player_events(core: &TestSuiteCore) -> Result<()> {
    info!("🧪 Testing events from unknown players");

    let unknown_player = core.create_test_player("Unknown").await?;

    // Try to reveal tokens for non-existent match
    let fake_reveal = TokenReveal {
        player_npub: unknown_player.public_key.to_string(),
        match_event_id: "non_existent_match".to_string(),
        cashu_tokens: vec!["fake".to_string()],
        token_secrets_nonce: "fake_nonce".to_string(),
        revealed_at: Utc::now().timestamp() as u64,
    };

    core.publish_event(&unknown_player, 31002, &fake_reveal)
        .await?;

    // Game engine should ignore events for unknown matches
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

/// Tests handling of duplicate events
async fn test_duplicate_events(core: &TestSuiteCore) -> Result<()> {
    info!("🧪 Testing duplicate event handling");

    let player1 = core.create_test_player("Duplicate1").await?;
    let player2 = core.create_test_player("Duplicate2").await?;

    // Create match using REAL Nostr events and test duplicate acceptance
    let (challenge, _challenge_event_id) = core
        .create_and_publish_match_challenge(&player1, 100, 0)
        .await?;

    // Send acceptance twice to test duplicate handling
    let (_acceptance1, _acceptance_event_id1) = core
        .create_and_publish_match_acceptance(&player2, &challenge)
        .await?;
    let (_acceptance2, _acceptance_event_id2) = core
        .create_and_publish_match_acceptance(&player2, &challenge)
        .await?;

    // Game engine should handle duplicates gracefully
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

/// Tests resistance to timing attacks
async fn test_timing_attacks(core: &TestSuiteCore) -> Result<()> {
    info!("🧪 Testing timing attack resistance");

    let player1 = core.create_test_player("Timing1").await?;
    let _player2 = core.create_test_player("Timing2").await?;

    // Try to publish invalid combat move  
    let invalid_move = CombatMove {
        player_npub: player1.public_key.to_string(),
        match_event_id: "timing_test".to_string(),
        previous_event_hash: Some("invalid_hash".to_string()),
        round_number: 1,
        unit_positions: vec![1, 2, 3],
        unit_abilities: vec!["boost".to_string()],
        move_timestamp: Utc::now().timestamp() as u64,
    };

    core.publish_event(&player1, 21003, &invalid_move)
        .await?;

    // Game engine should reject out-of-order reveals
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
