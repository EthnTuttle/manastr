use serde::{Deserialize, Serialize};

/// Summary of match validation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub commitments_valid: bool,
    pub combat_verified: bool,
    pub signatures_valid: bool,
    pub winner_confirmed: bool,
    pub match_integrity_score: u8,
}

impl Default for ValidationSummary {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationSummary {
    /// Creates a new validation summary with default values
    pub fn new() -> Self {
        Self {
            commitments_valid: false,
            combat_verified: false,
            signatures_valid: false,
            winner_confirmed: false,
            match_integrity_score: 0,
        }
    }

    /// Creates a validation summary for a successful match
    pub fn success() -> Self {
        Self {
            commitments_valid: true,
            combat_verified: true,
            signatures_valid: true,
            winner_confirmed: true,
            match_integrity_score: 100,
        }
    }

    /// Checks if the match validation was completely successful
    pub fn is_valid(&self) -> bool {
        self.commitments_valid
            && self.combat_verified
            && self.signatures_valid
            && self.winner_confirmed
            && self.match_integrity_score >= 100
    }
}
