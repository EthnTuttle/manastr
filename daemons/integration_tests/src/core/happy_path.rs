use anyhow::Result;
use tracing::info;

use super::shared::TestSuiteCore;

/// Tests the complete happy path of a player-driven match
///
/// Executes a full match lifecycle from player creation through
/// loot distribution, demonstrating the complete zero-coordination
/// gaming paradigm.
pub async fn test_happy_path_match(core: &TestSuiteCore) -> Result<()> {
    info!("🚀 Testing complete player-driven match lifecycle");

    // Phase 1: Player Creation
    let mut player1 = core.create_test_player("Alice").await?;
    let mut player2 = core.create_test_player("Bob").await?;
    info!("📋 Phase 1 Complete: Players created with Cashu C value armies");

    // Phase 2: Match Challenge
    let (challenge, _challenge_event_id) = core
        .create_and_publish_match_challenge(&player1, 100, 0)
        .await?;
    info!("📋 Phase 2 Complete: Alice published match challenge with army commitment");

    // Phase 3: Match Acceptance
    let (_acceptance, _acceptance_event_id) = core
        .create_and_publish_match_acceptance(&player2, &challenge)
        .await?;
    info!("📋 Phase 3 Complete: Bob accepted challenge with his army commitment");

    // Phase 4: Token Revelation
    core.publish_token_reveal(&player1, &challenge.match_event_id)
        .await?;
    core.publish_token_reveal(&player2, &challenge.match_event_id)
        .await?;
    info!("📋 Phase 4 Complete: Both players revealed Cashu tokens for army verification");

    // Phase 5: Combat Rounds
    core.simulate_combat_rounds(&player1, &player2, &challenge.match_event_id, 3)
        .await?;
    info!("📋 Phase 5 Complete: 3 combat rounds with turn-based moves and event chaining");

    // Phase 6: Match Results
    let winner_npub = player1.public_key.to_string();
    core.publish_match_result(
        &player1,
        &challenge.match_event_id,
        Some(winner_npub.clone()),
    )
    .await?;
    core.publish_match_result(
        &player2,
        &challenge.match_event_id,
        Some(winner_npub.clone()),
    )
    .await?;
    info!("📋 Phase 6 Complete: Both players submitted agreed match outcome");

    // Phase 7: Game Engine Validation & Loot Distribution
    core.verify_loot_distribution(
        &challenge.match_event_id, 
        &winner_npub,
        &mut player1,
        &mut player2,
    )
    .await?;
    info!("📋 Phase 7 Complete: Game engine validated match and issued actual loot tokens");

    info!("🎉 Happy path test completed successfully!");
    Ok(())
}
