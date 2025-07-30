use anyhow::Result;
use tracing::info;

use super::shared::TestSuiteCore;
use crate::players::TestPlayer;

/// Tests high-volume match processing
///
/// Verifies that the system can handle a large number
/// of matches running simultaneously under stress conditions.
pub async fn test_stress_scenarios(core: &TestSuiteCore) -> Result<()> {
    info!("🧪 Running stress test with high-volume matches");

    const STRESS_MATCH_COUNT: usize = 5;
    let mut stress_tasks = Vec::new();

    for i in 0..STRESS_MATCH_COUNT {
        let player1 = core.create_test_player(&format!("Stress{i}A")).await?;
        let player2 = core.create_test_player(&format!("Stress{i}B")).await?;

        let task = run_stress_match(core, player1, player2, i);
        stress_tasks.push(task);
    }

    // Process all stress matches concurrently
    futures::future::try_join_all(stress_tasks).await?;

    info!(
        "✅ Stress test completed - {} matches processed",
        STRESS_MATCH_COUNT
    );
    Ok(())
}

/// Runs a single stress match
async fn run_stress_match(
    core: &TestSuiteCore,
    player1: TestPlayer,
    player2: TestPlayer,
    match_index: usize,
) -> Result<()> {
    let (challenge, _challenge_event_id) = core
        .create_and_publish_match_challenge(&player1, 25, (match_index % 4) as u8)
        .await?;
    let (_acceptance, _acceptance_event_id) = core
        .create_and_publish_match_acceptance(&player2, &challenge)
        .await?;

    // Fast-track match completion for stress test
    core.publish_token_reveal(&player1, &challenge.match_event_id)
        .await?;
    core.publish_token_reveal(&player2, &challenge.match_event_id)
        .await?;

    let winner = if match_index % 2 == 0 {
        &player1
    } else {
        &player2
    };
    let winner_npub = winner.keys.public_key().to_string();

    core.publish_match_result(
        &player1,
        &challenge.match_event_id,
        Some(winner_npub.clone()),
    )
    .await?;
    core.publish_match_result(&player2, &challenge.match_event_id, Some(winner_npub))
        .await?;

    Ok(())
}
