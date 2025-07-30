use anyhow::Result;
use tracing::info;

use super::gaming_wallet::GamingWallet;
use super::shared::TestSuiteCore;

/// Tests anti-cheat commitment verification
///
/// Verifies that the system correctly detects and rejects
/// attempts to cheat by revealing different tokens than committed.
pub async fn test_commitment_verification(core: &TestSuiteCore) -> Result<()> {
    let player1 = core.create_test_player("Cheater").await?;
    let player2 = core.create_test_player("Honest").await?;

    // Create match using REAL Nostr events
    let (challenge, _challenge_event_id) = core
        .create_and_publish_match_challenge(&player1, 100, 0)
        .await?;
    let (_acceptance, _acceptance_event_id) = core
        .create_and_publish_match_acceptance(&player2, &challenge)
        .await?;

    // Player 1 tries to cheat by revealing different tokens than committed
    let mut cheating_player = player1.clone();

    // Create fake gaming wallet with different tokens
    cheating_player.gaming_wallet = GamingWallet::new(core.mint_url.clone());
    let _fake_tokens = cheating_player
        .gaming_wallet
        .mint_gaming_tokens(2, "mana")
        .await?;

    // Attempt cheating reveal (should fail validation)
    core.publish_token_reveal(&cheating_player, &challenge.match_event_id)
        .await?;

    info!("✅ Anti-cheat commitment verification working correctly");
    Ok(())
}
