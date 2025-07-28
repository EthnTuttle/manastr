use anyhow::Result;
use serde_json::{json, Value};
use std::collections::HashMap;

/// Test scenario configuration
#[derive(Debug, Clone)]
pub struct TestScenario {
    pub name: String,
    pub description: String,
    pub player1_config: PlayerConfig,
    pub player2_config: PlayerConfig,
    pub match_config: MatchConfig,
    pub expected_outcome: ExpectedOutcome,
}

#[derive(Debug, Clone)]
pub struct PlayerConfig {
    pub name: String,
    pub mana_amount: u64,
    pub private_key: String,
    pub public_key: String,
    pub should_fail_minting: bool,
    pub should_timeout: bool,
}

#[derive(Debug, Clone)]
pub struct MatchConfig {
    pub rounds: u32,
    pub should_error: bool,
    pub force_draw: bool,
    pub custom_seed: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ExpectedOutcome {
    Player1Wins,
    Player2Wins,
    Draw,
    MintError,
    MatchError,
    Timeout,
    Any, // For edge case testing
}

impl TestScenario {
    /// Create all test scenarios
    pub fn all_scenarios() -> Vec<TestScenario> {
        vec![
            Self::normal_match(),
            Self::asymmetric_armies(),
            Self::large_armies(),
            Self::minimal_armies(),
            Self::draw_scenario(),
            Self::mint_failure(),
            Self::timeout_scenario(),
            Self::edge_case_zero_amount(),
            Self::edge_case_max_amount(),
            Self::identical_keys(),
            Self::rapid_succession(),
            Self::concurrent_matches(),
        ]
    }

    /// Normal balanced match
    pub fn normal_match() -> TestScenario {
        TestScenario {
            name: "Normal Match".to_string(),
            description: "Standard balanced match between two equal players".to_string(),
            player1_config: PlayerConfig {
                name: "Alice".to_string(),
                mana_amount: 100,
                private_key: "0000000000000000000000000000000000000000000000000000000000000003".to_string(),
                public_key: "npub1alice".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "Bob".to_string(),
                mana_amount: 100,
                private_key: "0000000000000000000000000000000000000000000000000000000000000004".to_string(),
                public_key: "npub1bob".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 3,
                should_error: false,
                force_draw: false,
                custom_seed: None,
            },
            expected_outcome: ExpectedOutcome::Any,
        }
    }

    /// Asymmetric armies (different mana amounts)
    pub fn asymmetric_armies() -> TestScenario {
        TestScenario {
            name: "Asymmetric Armies".to_string(),
            description: "One player has significantly more mana (200 vs 50)".to_string(),
            player1_config: PlayerConfig {
                name: "BigSpender".to_string(),
                mana_amount: 200,
                private_key: "0000000000000000000000000000000000000000000000000000000000000005".to_string(),
                public_key: "npub1bigspender".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "SmallArmy".to_string(),
                mana_amount: 50,
                private_key: "0000000000000000000000000000000000000000000000000000000000000006".to_string(),
                public_key: "npub1smallarmy".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 5,
                should_error: false,
                force_draw: false,
                custom_seed: Some("asymmetric_test".to_string()),
            },
            expected_outcome: ExpectedOutcome::Player1Wins,
        }
    }

    /// Large armies test
    pub fn large_armies() -> TestScenario {
        TestScenario {
            name: "Large Armies".to_string(),
            description: "Both players mint maximum armies (1000 mana each)".to_string(),
            player1_config: PlayerConfig {
                name: "MegaArmy1".to_string(),
                mana_amount: 1000,
                private_key: "0000000000000000000000000000000000000000000000000000000000000007".to_string(),
                public_key: "npub1megaarmy1".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "MegaArmy2".to_string(),
                mana_amount: 1000,
                private_key: "0000000000000000000000000000000000000000000000000000000000000008".to_string(),
                public_key: "npub1megaarmy2".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 10,
                should_error: false,
                force_draw: false,
                custom_seed: Some("large_armies".to_string()),
            },
            expected_outcome: ExpectedOutcome::Any,
        }
    }

    /// Minimal armies (edge case)
    pub fn minimal_armies() -> TestScenario {
        TestScenario {
            name: "Minimal Armies".to_string(),
            description: "Both players mint minimal armies (1 mana each)".to_string(),
            player1_config: PlayerConfig {
                name: "TinyArmy1".to_string(),
                mana_amount: 1,
                private_key: "0000000000000000000000000000000000000000000000000000000000000009".to_string(),
                public_key: "npub1tinyarmy1".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "TinyArmy2".to_string(),
                mana_amount: 1,
                private_key: "000000000000000000000000000000000000000000000000000000000000000a".to_string(),
                public_key: "npub1tinyarmy2".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 1,
                should_error: false,
                force_draw: false,
                custom_seed: Some("minimal_armies".to_string()),
            },
            expected_outcome: ExpectedOutcome::Any,
        }
    }

    /// Force draw scenario
    pub fn draw_scenario() -> TestScenario {
        TestScenario {
            name: "Forced Draw".to_string(),
            description: "Match engineered to end in a draw".to_string(),
            player1_config: PlayerConfig {
                name: "DrawPlayer1".to_string(),
                mana_amount: 100,
                private_key: "000000000000000000000000000000000000000000000000000000000000000b".to_string(),
                public_key: "npub1drawplayer1".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "DrawPlayer2".to_string(),
                mana_amount: 100,
                private_key: "000000000000000000000000000000000000000000000000000000000000000c".to_string(),
                public_key: "npub1drawplayer2".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 3,
                should_error: false,
                force_draw: true,
                custom_seed: Some("draw_match".to_string()),
            },
            expected_outcome: ExpectedOutcome::Draw,
        }
    }

    /// Mint failure scenario
    pub fn mint_failure() -> TestScenario {
        TestScenario {
            name: "Mint Failure".to_string(),
            description: "Test graceful handling of minting failures".to_string(),
            player1_config: PlayerConfig {
                name: "FailPlayer".to_string(),
                mana_amount: 100,
                private_key: "invalid_key".to_string(),
                public_key: "npub1failplayer".to_string(),
                should_fail_minting: true,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "NormalPlayer".to_string(),
                mana_amount: 100,
                private_key: "000000000000000000000000000000000000000000000000000000000000000d".to_string(),
                public_key: "npub1normalplayer".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 3,
                should_error: false,
                force_draw: false,
                custom_seed: None,
            },
            expected_outcome: ExpectedOutcome::MintError,
        }
    }

    /// Timeout scenario
    pub fn timeout_scenario() -> TestScenario {
        TestScenario {
            name: "Timeout Handling".to_string(),
            description: "Test handling of player timeouts during match".to_string(),
            player1_config: PlayerConfig {
                name: "TimeoutPlayer".to_string(),
                mana_amount: 100,
                private_key: "000000000000000000000000000000000000000000000000000000000000000e".to_string(),
                public_key: "npub1timeoutplayer".to_string(),
                should_fail_minting: false,
                should_timeout: true,
            },
            player2_config: PlayerConfig {
                name: "PunctualPlayer".to_string(),
                mana_amount: 100,
                private_key: "000000000000000000000000000000000000000000000000000000000000000f".to_string(),
                public_key: "npub1punctualplayer".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 3,
                should_error: false,
                force_draw: false,
                custom_seed: None,
            },
            expected_outcome: ExpectedOutcome::Player2Wins,
        }
    }

    /// Edge case: Zero amount
    pub fn edge_case_zero_amount() -> TestScenario {
        TestScenario {
            name: "Zero Amount Edge Case".to_string(),
            description: "Test system behavior with zero mana amount".to_string(),
            player1_config: PlayerConfig {
                name: "ZeroPlayer".to_string(),
                mana_amount: 0,
                private_key: "0000000000000000000000000000000000000000000000000000000000000010".to_string(),
                public_key: "npub1zeroplayer".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "SmallPlayer".to_string(),
                mana_amount: 10,
                private_key: "0000000000000000000000000000000000000000000000000000000000000011".to_string(),
                public_key: "npub1smallplayer".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 1,
                should_error: false,
                force_draw: false,
                custom_seed: Some("zero_test".to_string()),
            },
            expected_outcome: ExpectedOutcome::Player2Wins,
        }
    }

    /// Edge case: Maximum amount
    pub fn edge_case_max_amount() -> TestScenario {
        TestScenario {
            name: "Maximum Amount Edge Case".to_string(),
            description: "Test system limits with maximum mana amount".to_string(),
            player1_config: PlayerConfig {
                name: "MaxPlayer".to_string(),
                mana_amount: 1_000_000, // Mint's max amount
                private_key: "0000000000000000000000000000000000000000000000000000000000000012".to_string(),
                public_key: "npub1maxplayer".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "RegularPlayer".to_string(),
                mana_amount: 100,
                private_key: "0000000000000000000000000000000000000000000000000000000000000013".to_string(),
                public_key: "npub1regularplayer".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 1,
                should_error: false,
                force_draw: false,
                custom_seed: Some("max_test".to_string()),
            },
            expected_outcome: ExpectedOutcome::Player1Wins,
        }
    }

    /// Edge case: Identical keys
    pub fn identical_keys() -> TestScenario {
        TestScenario {
            name: "Identical Keys Edge Case".to_string(),
            description: "Test system behavior with identical player keys".to_string(),
            player1_config: PlayerConfig {
                name: "Twin1".to_string(),
                mana_amount: 100,
                private_key: "0000000000000000000000000000000000000000000000000000000000000014".to_string(),
                public_key: "npub1twin".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "Twin2".to_string(),
                mana_amount: 100,
                private_key: "0000000000000000000000000000000000000000000000000000000000000014".to_string(), // Same key
                public_key: "npub1twin".to_string(), // Same pubkey
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 3,
                should_error: false,
                force_draw: true, // Identical armies should draw
                custom_seed: Some("identical_test".to_string()),
            },
            expected_outcome: ExpectedOutcome::Draw,
        }
    }

    /// Rapid succession matches
    pub fn rapid_succession() -> TestScenario {
        TestScenario {
            name: "Rapid Succession".to_string(),
            description: "Test rapid match creation and resolution".to_string(),
            player1_config: PlayerConfig {
                name: "SpeedPlayer1".to_string(),
                mana_amount: 50,
                private_key: "0000000000000000000000000000000000000000000000000000000000000015".to_string(),
                public_key: "npub1speedplayer1".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "SpeedPlayer2".to_string(),
                mana_amount: 50,
                private_key: "0000000000000000000000000000000000000000000000000000000000000016".to_string(),
                public_key: "npub1speedplayer2".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 1,
                should_error: false,
                force_draw: false,
                custom_seed: Some("speed_test".to_string()),
            },
            expected_outcome: ExpectedOutcome::Any,
        }
    }

    /// Concurrent matches
    pub fn concurrent_matches() -> TestScenario {
        TestScenario {
            name: "Concurrent Matches".to_string(),
            description: "Test system handling of multiple simultaneous matches".to_string(),
            player1_config: PlayerConfig {
                name: "ConcurrentPlayer1".to_string(),
                mana_amount: 75,
                private_key: "0000000000000000000000000000000000000000000000000000000000000017".to_string(),
                public_key: "npub1concurrent1".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            player2_config: PlayerConfig {
                name: "ConcurrentPlayer2".to_string(),
                mana_amount: 75,
                private_key: "0000000000000000000000000000000000000000000000000000000000000018".to_string(),
                public_key: "npub1concurrent2".to_string(),
                should_fail_minting: false,
                should_timeout: false,
            },
            match_config: MatchConfig {
                rounds: 2,
                should_error: false,
                force_draw: false,
                custom_seed: Some("concurrent_test".to_string()),
            },
            expected_outcome: ExpectedOutcome::Any,
        }
    }
}

/// Get scenarios by category
impl TestScenario {
    pub fn normal_scenarios() -> Vec<TestScenario> {
        vec![
            Self::normal_match(),
            Self::asymmetric_armies(),
            Self::large_armies(),
        ]
    }

    pub fn edge_case_scenarios() -> Vec<TestScenario> {
        vec![
            Self::minimal_armies(),
            Self::edge_case_zero_amount(),
            Self::edge_case_max_amount(),
            Self::identical_keys(),
        ]
    }

    pub fn error_scenarios() -> Vec<TestScenario> {
        vec![
            Self::mint_failure(),
            Self::timeout_scenario(),
        ]
    }

    pub fn stress_test_scenarios() -> Vec<TestScenario> {
        vec![
            Self::rapid_succession(),
            Self::concurrent_matches(),
            Self::large_armies(),
        ]
    }
}