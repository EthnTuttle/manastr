use anyhow::Result;
use tracing::info;

use crate::core::anti_cheat::test_commitment_verification;
use crate::core::concurrent::test_concurrent_matches;
use crate::core::edge_cases::test_edge_cases;
use crate::core::happy_path::test_happy_path_match;
use crate::core::stress::test_stress_scenarios;
use crate::core::TestSuiteCore;
use crate::gaming_auth_test::test_gaming_authorization;

/// Main test suite for player-driven integration tests
///
/// This struct orchestrates the complete integration test flow,
/// managing player creation, match execution, and validation.
pub struct PlayerDrivenTestSuite {
    core: TestSuiteCore,
}

impl PlayerDrivenTestSuite {
    /// Creates a new test suite instance with configured clients
    pub async fn new() -> Result<Self> {
        let core = TestSuiteCore::new().await?;
        Ok(Self { core })
    }

    /// Runs the complete integration test suite
    ///
    /// Executes all test scenarios in sequence:
    /// - Happy path match execution
    /// - Anti-cheat validation
    /// - Concurrent match processing
    /// - Edge case handling
    /// - Stress testing
    pub async fn run_comprehensive_tests(&self) -> Result<()> {
        info!("🚀 Starting Player-Driven Integration Test Suite");

        self.core.wait_for_services().await?;

        info!("📋 Test 1: Happy Path Player-Driven Match");
        test_happy_path_match(&self.core).await?;

        info!("📋 Test 2: Anti-Cheat Commitment Verification");
        test_commitment_verification(&self.core).await?;

        info!("📋 Test 3: Concurrent Player-Driven Matches");
        test_concurrent_matches(&self.core).await?;

        info!("📋 Test 4: Edge Cases and Malicious Events");
        test_edge_cases(&self.core).await?;

        info!("📋 Test 5: High-Volume Match Processing");
        test_stress_scenarios(&self.core).await?;

        info!("📋 Test 6: Gaming Token Authorization Enforcement");
        test_gaming_authorization().await?;

        info!("✅ All Player-Driven Integration Tests Passed!");
        Ok(())
    }
}
