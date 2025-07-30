use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

use crate::errors::GameEngineError;
use crate::match_state_machine::{GameEngineAction, MatchEvent, MatchState};
use crate::nostr_client::PlayerMatchEvent;

/// Concurrent match tracker using state machines
pub struct MatchTracker {
    /// Active matches tracked by match_event_id
    matches: Arc<RwLock<HashMap<String, TrackedMatch>>>,
    /// Action queue for processing state transitions
    action_sender: mpsc::UnboundedSender<TrackedAction>,
    /// Configuration
    max_concurrent_matches: usize,
    match_timeout_minutes: u64,
}

/// A match being tracked with its state machine
#[derive(Debug, Clone)]
pub struct TrackedMatch {
    pub state: MatchState,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub action_count: u64,
}

/// Action to be processed with context
#[derive(Debug, Clone)]
pub struct TrackedAction {
    pub match_id: String,
    pub action: GameEngineAction,
    pub triggered_at: DateTime<Utc>,
}

impl MatchTracker {
    /// Create new match tracker
    pub fn new(
        max_concurrent_matches: usize,
        match_timeout_minutes: u64,
    ) -> (Self, mpsc::UnboundedReceiver<TrackedAction>) {
        let (action_sender, action_receiver) = mpsc::unbounded_channel();

        let tracker = Self {
            matches: Arc::new(RwLock::new(HashMap::new())),
            action_sender,
            max_concurrent_matches,
            match_timeout_minutes,
        };

        (tracker, action_receiver)
    }

    /// Process a Nostr match event through the state machine
    pub async fn process_event(&self, event: PlayerMatchEvent) -> Result<(), GameEngineError> {
        let (match_id, match_event) = self.convert_to_match_event(event).await?;

        debug!("🔄 Processing event for match {}", match_id);

        // Get or create match state
        let mut matches = self.matches.write().await;

        // Check concurrent match limit
        if matches.len() >= self.max_concurrent_matches && !matches.contains_key(&match_id) {
            warn!(
                "🚫 Maximum concurrent matches ({}) reached",
                self.max_concurrent_matches
            );
            return Err(GameEngineError::Internal(
                "Too many concurrent matches".to_string(),
            ));
        }

        let current_state = matches
            .get(&match_id)
            .map(|tm| tm.state.clone())
            .unwrap_or_else(|| {
                // Create initial state based on event type
                match &match_event {
                    MatchEvent::ChallengePosted(challenge) => {
                        MatchState::new_challenge(challenge.clone())
                    }
                    _ => {
                        warn!(
                            "🚨 Received non-challenge event for unknown match: {}",
                            match_id
                        );
                        MatchState::Invalid {
                            reason: "Unknown match received non-challenge event".to_string(),
                            failed_at: Utc::now(),
                        }
                    }
                }
            });

        // Process state transition
        let transition_result = current_state.transition(match_event);

        // Update match state
        let tracked_match = TrackedMatch {
            state: transition_result.new_state.clone(),
            created_at: matches
                .get(&match_id)
                .map(|tm| tm.created_at)
                .unwrap_or_else(Utc::now),
            last_updated: Utc::now(),
            action_count: matches
                .get(&match_id)
                .map(|tm| tm.action_count + transition_result.actions.len() as u64)
                .unwrap_or(transition_result.actions.len() as u64),
        };

        matches.insert(match_id.clone(), tracked_match);

        // Log state transition
        info!(
            "🎮 Match {} transitioned to: {}",
            match_id,
            transition_result.new_state.phase_name()
        );

        // Queue actions for processing
        for action in transition_result.actions {
            let tracked_action = TrackedAction {
                match_id: match_id.clone(),
                action,
                triggered_at: Utc::now(),
            };

            if let Err(e) = self.action_sender.send(tracked_action) {
                error!("Failed to queue action: {}", e);
            }
        }

        // Log any errors
        for error in transition_result.errors {
            warn!("🚨 Transition error for match {}: {}", match_id, error);
        }

        // Clean up terminal matches after delay
        if transition_result.new_state.is_terminal() {
            let matches_clone = Arc::clone(&self.matches);
            let match_id_clone = match_id.clone();

            tokio::spawn(async move {
                // Wait 5 minutes before cleaning up completed matches
                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;

                let mut matches = matches_clone.write().await;
                if let Some(tracked_match) = matches.get(&match_id_clone) {
                    if tracked_match.state.is_terminal() {
                        matches.remove(&match_id_clone);
                        info!("🧹 Cleaned up terminal match: {}", match_id_clone);
                    }
                }
            });
        }

        Ok(())
    }

    /// Convert PlayerMatchEvent to internal MatchEvent
    async fn convert_to_match_event(
        &self,
        event: PlayerMatchEvent,
    ) -> Result<(String, MatchEvent), GameEngineError> {
        match event {
            PlayerMatchEvent::Challenge(challenge) => {
                let match_id = format!("challenge_{}", challenge.challenger_npub);
                Ok((match_id, MatchEvent::ChallengePosted(challenge)))
            }
            PlayerMatchEvent::Acceptance(acceptance) => {
                let match_id = acceptance.match_event_id.clone();
                Ok((match_id, MatchEvent::ChallengeAccepted(acceptance)))
            }
            PlayerMatchEvent::TokenReveal(reveal) => {
                let match_id = reveal.match_event_id.clone();
                Ok((match_id, MatchEvent::TokenRevealed(reveal)))
            }
            PlayerMatchEvent::MoveCommitment(commitment) => {
                let match_id = commitment.match_event_id.clone();
                Ok((match_id, MatchEvent::MoveCommitted(commitment)))
            }
            PlayerMatchEvent::MoveReveal(reveal) => {
                let match_id = reveal.match_event_id.clone();
                Ok((match_id, MatchEvent::MoveRevealed(reveal)))
            }
            PlayerMatchEvent::MatchResult(result) => {
                let match_id = result.match_event_id.clone();
                Ok((match_id, MatchEvent::ResultSubmitted(result)))
            }
        }
    }

    /// Get current match state
    pub async fn get_match_state(&self, match_id: &str) -> Option<MatchState> {
        let matches = self.matches.read().await;
        matches.get(match_id).map(|tm| tm.state.clone())
    }

    /// Get match statistics
    pub async fn get_statistics(&self) -> MatchStatistics {
        let matches = self.matches.read().await;

        let mut stats = MatchStatistics {
            total_matches: matches.len(),
            challenged: 0,
            accepted: 0,
            in_combat: 0,
            awaiting_validation: 0,
            completed: 0,
            invalid: 0,
            oldest_match: None,
        };

        let mut oldest_time = None;

        for tracked_match in matches.values() {
            // Update oldest match time
            if oldest_time.is_none() || tracked_match.created_at < oldest_time.unwrap() {
                oldest_time = Some(tracked_match.created_at);
            }

            // Count by state
            match tracked_match.state {
                MatchState::Challenged { .. } => stats.challenged += 1,
                MatchState::Accepted { .. } => stats.accepted += 1,
                MatchState::InCombat { .. } => stats.in_combat += 1,
                MatchState::AwaitingValidation { .. } => stats.awaiting_validation += 1,
                MatchState::Completed { .. } => stats.completed += 1,
                MatchState::Invalid { .. } => stats.invalid += 1,
            }
        }

        stats.oldest_match = oldest_time;
        stats
    }

    /// Clean up expired matches
    pub async fn cleanup_expired_matches(&self) {
        let now = Utc::now();
        let timeout_duration = chrono::Duration::minutes(self.match_timeout_minutes as i64);

        let mut matches = self.matches.write().await;
        let mut expired_matches = Vec::new();

        for (match_id, tracked_match) in matches.iter() {
            if now.signed_duration_since(tracked_match.last_updated) > timeout_duration {
                expired_matches.push(match_id.clone());
            }
        }

        for match_id in expired_matches {
            if let Some(tracked_match) = matches.remove(&match_id) {
                warn!(
                    "⏰ Expired match removed: {} (last updated: {})",
                    match_id, tracked_match.last_updated
                );

                // Queue invalidation action
                let action = TrackedAction {
                    match_id: match_id.clone(),
                    action: GameEngineAction::InvalidateMatch {
                        match_id,
                        reason: "Match timeout expired".to_string(),
                    },
                    triggered_at: now,
                };

                if let Err(e) = self.action_sender.send(action) {
                    error!("Failed to queue timeout invalidation: {}", e);
                }
            }
        }
    }

    /// Trigger manual match invalidation
    pub async fn invalidate_match(
        &self,
        match_id: &str,
        reason: String,
    ) -> Result<(), GameEngineError> {
        let mut matches = self.matches.write().await;

        if let Some(tracked_match) = matches.get_mut(match_id) {
            let transition_result = tracked_match
                .state
                .clone()
                .transition(MatchEvent::InvalidationTriggered(reason.clone()));

            tracked_match.state = transition_result.new_state;
            tracked_match.last_updated = Utc::now();

            info!("🚨 Manually invalidated match {}: {}", match_id, reason);

            // Queue invalidation actions
            for action in transition_result.actions {
                let tracked_action = TrackedAction {
                    match_id: match_id.to_string(),
                    action,
                    triggered_at: Utc::now(),
                };

                if let Err(e) = self.action_sender.send(tracked_action) {
                    error!("Failed to queue invalidation action: {}", e);
                }
            }

            Ok(())
        } else {
            Err(GameEngineError::MatchNotFound(match_id.to_string()))
        }
    }

    /// Get all matches in a specific state
    pub async fn get_matches_in_state(&self, target_state: &str) -> Vec<(String, TrackedMatch)> {
        let matches = self.matches.read().await;

        matches
            .iter()
            .filter(|(_, tracked_match)| tracked_match.state.phase_name() == target_state)
            .map(|(id, tm)| (id.clone(), tm.clone()))
            .collect()
    }
}

/// Statistics about current matches
#[derive(Debug, Clone)]
pub struct MatchStatistics {
    pub total_matches: usize,
    pub challenged: usize,
    pub accepted: usize,
    pub in_combat: usize,
    pub awaiting_validation: usize,
    pub completed: usize,
    pub invalid: usize,
    pub oldest_match: Option<DateTime<Utc>>,
}

impl MatchStatistics {
    /// Get active (non-terminal) match count
    pub fn active_matches(&self) -> usize {
        self.challenged + self.accepted + self.in_combat + self.awaiting_validation
    }
}

/// Background task to periodically clean up expired matches
pub async fn run_cleanup_task(tracker: Arc<MatchTracker>) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes

    loop {
        interval.tick().await;
        tracker.cleanup_expired_matches().await;

        let stats = tracker.get_statistics().await;
        debug!(
            "🧹 Cleanup cycle: {} total matches, {} active",
            stats.total_matches,
            stats.active_matches()
        );
    }
}
