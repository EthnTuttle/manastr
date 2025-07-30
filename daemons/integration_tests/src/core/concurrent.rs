use anyhow::Result;
use tracing::info;

use super::shared::TestSuiteCore;
use crate::players::TestPlayer;
use crate::matches::MatchChallenge;

/// Tests multiple concurrent player-driven matches
/// 
/// Verifies that the system can handle multiple matches
/// running simultaneously without interference.
pub async fn test_concurrent_matches(core: &TestSuiteCore) -> Result<()> {
    let mut matches = Vec::new();
    
    // Create 3 concurrent matches
    for i in 0..3 {
        let player1 = core.create_test_player(&format!("Player{}A", i)).await?;
        let player2 = core.create_test_player(&format!("Player{}B", i)).await?;
        
        let (challenge, _challenge_event_id) = core.create_and_publish_match_challenge(&player1, 50, i % 4).await?;
        let (_acceptance, _acceptance_event_id) = core.create_and_publish_match_acceptance(&player2, &challenge).await?;
        
        matches.push((player1, player2, challenge));
    }
    
    // Process all matches concurrently
    let mut tasks = Vec::new();
    for (player1, player2, challenge) in matches {
        let task = process_concurrent_match(core, player1, player2, challenge);
        tasks.push(task);
    }
    
    // Wait for all matches to complete
    futures::future::try_join_all(tasks).await?;
    
    info!("✅ Concurrent player-driven matches completed successfully");
    Ok(())
}

/// Processes a single concurrent match
async fn process_concurrent_match(core: &TestSuiteCore, player1: TestPlayer, player2: TestPlayer, challenge: MatchChallenge) -> Result<()> {
    // Token reveals
    core.publish_token_reveal(&player1, &challenge.match_event_id).await?;
    core.publish_token_reveal(&player2, &challenge.match_event_id).await?;
    
    // Combat rounds
    core.simulate_combat_rounds(&player1, &player2, &challenge.match_event_id, 2).await?;
    
    // Match results
    let winner = player1.public_key.to_string();
    core.publish_match_result(&player1, &challenge.match_event_id, Some(winner.clone())).await?;
    core.publish_match_result(&player2, &challenge.match_event_id, Some(winner)).await?;
    
    Ok(())
} 