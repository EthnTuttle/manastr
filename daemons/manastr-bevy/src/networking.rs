use bevy::prelude::*;
use matchbox_socket::{WebRtcSocket, PeerId};
use tracing::{info, warn, error};
use std::collections::HashMap;

/// Networking Plugin - Hybrid Nostr + WebRTC coordination
pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NetworkingState>()
            .add_systems(Startup, initialize_networking)
            .add_systems(Update, (
                handle_webrtc_messages,
                send_realtime_updates,
                fallback_to_nostr,
            ));
    }
}

/// Hybrid networking state - WebRTC + Nostr fallback
#[derive(Resource, Default)]
pub struct NetworkingState {
    pub webrtc_socket: Option<WebRtcSocket>,
    pub connected_peers: HashMap<PeerId, PlayerConnection>,
    pub use_fallback: bool,
    pub signaling_server: String,
}

#[derive(Clone, Debug)]
pub struct PlayerConnection {
    pub peer_id: PeerId,
    pub player_name: String,
    pub connection_quality: ConnectionQuality,
    pub last_ping: std::time::Instant,
}

#[derive(Clone, Debug)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Disconnected,
}

/// Message types for real-time coordination
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum RealtimeMessage {
    // Visual updates (non-critical)
    CursorPosition { x: f32, y: f32 },
    CardHover { card_id: String },
    AnimationTrigger { animation: String },
    
    // Game state sync (important but not security-critical)
    PhaseTransition { new_phase: String },
    PlayerStatus { ready: bool },
    
    // Heartbeat for connection monitoring
    Ping { timestamp: u64 },
    Pong { timestamp: u64 },
}

/// Critical messages that MUST use Nostr (never WebRTC)
#[derive(Clone, Debug)]
pub enum CriticalMessage {
    TokenReveal { token_data: String },
    MoveCommitment { commitment_hash: String },
    MatchResult { result_signature: String },
    ChallengeCreation { challenge_id: String },
    ChallengeAcceptance { acceptance_signature: String },
}

fn initialize_networking(mut networking: ResMut<NetworkingState>) {
    info!("🔗 Initializing hybrid networking (WebRTC + Nostr fallback)");
    
    // Use Nostr relay as signaling server for WebRTC
    networking.signaling_server = "ws://localhost:7777".to_string();
    
    // For now, use Nostr fallback (WebRTC will be implemented later)
    networking.use_fallback = true;
    info!("🔄 Using Nostr fallback for all communication (WebRTC integration pending)");
}

async fn create_webrtc_socket(signaling_url: &str) -> Result<WebRtcSocket, Box<dyn std::error::Error>> {
    // Create WebRTC socket using matchbox with Nostr signaling
    // This would integrate with matchbox_nostr when available
    info!("Creating WebRTC socket with Nostr signaling: {}", signaling_url);
    
    // Placeholder for matchbox_socket integration
    // let socket = WebRtcSocket::new(signaling_url);
    // Ok(socket)
    
    // For now, return error to trigger fallback
    Err("WebRTC not yet implemented".into())
}

fn handle_webrtc_messages(mut networking: ResMut<NetworkingState>) {
    if let Some(ref mut socket) = networking.webrtc_socket {
        // Handle incoming WebRTC messages
        // Process real-time updates from peers
        // Update connection quality metrics
        
        // Placeholder for message handling
        // for (peer, message) in socket.receive_messages() {
        //     handle_realtime_message(peer, message);
        // }
    }
}

fn send_realtime_updates(networking: Res<NetworkingState>) {
    if networking.use_fallback {
        // Use Nostr events for all communication
        send_via_nostr_fallback();
    } else if let Some(ref socket) = networking.webrtc_socket {
        // Send non-critical updates via WebRTC for low latency
        // send_via_webrtc(socket, message);
    }
}

fn send_via_nostr_fallback() {
    // Implement pure Nostr communication
    // All messages go through Nostr events (higher latency but guaranteed delivery)
    info!("Using Nostr fallback for all communication");
}

fn fallback_to_nostr(mut networking: ResMut<NetworkingState>) {
    // Monitor WebRTC connection quality
    // Automatically fall back to Nostr if connections degrade
    
    if !networking.use_fallback {
        // Check connection health
        let should_fallback = check_connection_health(&networking);
        
        if should_fallback {
            warn!("🔄 Switching to Nostr fallback due to poor WebRTC connectivity");
            networking.use_fallback = true;
        }
    }
}

fn check_connection_health(networking: &NetworkingState) -> bool {
    // Check if we should fall back to Nostr
    // Based on connection quality, ping times, packet loss, etc.
    
    for connection in networking.connected_peers.values() {
        match connection.connection_quality {
            ConnectionQuality::Poor | ConnectionQuality::Disconnected => return true,
            _ => continue,
        }
    }
    
    false
}

/// Send critical game action via Nostr (never WebRTC)
pub fn send_critical_action(message: CriticalMessage) {
    info!("🔐 Sending critical action via Nostr: {:?}", message);
    
    // All security-critical actions MUST use Nostr events
    // This ensures cryptographic guarantees are maintained
    // WebRTC is only for visual/UX enhancements
    
    match message {
        CriticalMessage::TokenReveal { .. } => {
            // Publish KIND 31002 Nostr event
        }
        CriticalMessage::MoveCommitment { .. } => {
            // Publish KIND 31003 Nostr event
        }
        CriticalMessage::MatchResult { .. } => {
            // Publish KIND 31005 Nostr event
        }
        _ => {
            // Handle other critical message types
        }
    }
}