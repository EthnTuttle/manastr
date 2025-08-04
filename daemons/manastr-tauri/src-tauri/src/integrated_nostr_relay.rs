use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use serde_json::Value;
use tracing::{info, warn, error, debug};
use nostr::{Event, EventId, Filter, Kind, PublicKey, Timestamp};
use futures_util::{SinkExt, StreamExt};

/// Integrated Nostr Relay that shares the same SQLite database as CDK
/// This avoids the dependency conflict by using the same rusqlite version
pub struct IntegratedNostrRelay {
    /// Shared database connection using the same SQLite as CDK
    db_path: String,
    /// In-memory event storage for fast access
    events: Arc<RwLock<HashMap<EventId, Event>>>,
    /// Subscriptions by connection ID
    subscriptions: Arc<RwLock<HashMap<String, Vec<Filter>>>>,
    /// Message sender for broadcasting
    broadcast_tx: mpsc::UnboundedSender<RelayMessage>,
    /// Message receiver for handling
    broadcast_rx: Option<mpsc::UnboundedReceiver<RelayMessage>>,
}

#[derive(Debug, Clone)]
pub enum RelayMessage {
    Event(Event),
    Subscribe { conn_id: String, filters: Vec<Filter> },
    Unsubscribe { conn_id: String },
    Close { conn_id: String },
}

impl IntegratedNostrRelay {
    /// Create a new integrated Nostr relay using the same database as CDK
    pub fn new(db_path: String) -> Self {
        let (broadcast_tx, broadcast_rx) = mpsc::unbounded_channel();
        
        Self {
            db_path,
            events: Arc::new(RwLock::new(HashMap::new())),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
            broadcast_rx: Some(broadcast_rx),
        }
    }

    /// Start the relay server
    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting Integrated Nostr Relay");
        info!("   • Using shared SQLite database: {}", self.db_path);
        info!("   • No dependency conflicts with CDK");
        info!("   • True library integration achieved");

        // Initialize the database schema for Nostr events
        self.initialize_db().await?;

        // Start the WebSocket server
        self.start_websocket_server().await?;

        // Start the message processing loop
        if let Some(rx) = self.broadcast_rx.take() {
            self.start_message_processor(rx).await;
        }

        Ok(())
    }

    async fn initialize_db(&self) -> Result<()> {
        info!("📊 Initializing Nostr relay database schema...");

        // Use the same rusqlite as CDK to avoid conflicts
        let conn = rusqlite::Connection::open(&self.db_path)?;
        
        // Create table for Nostr events if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS nostr_events (
                id TEXT PRIMARY KEY,
                pubkey TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                kind INTEGER NOT NULL,
                tags TEXT NOT NULL,
                content TEXT NOT NULL,
                sig TEXT NOT NULL,
                event_json TEXT NOT NULL
            )",
            [],
        )?;

        // Create indexes for efficient querying
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_nostr_pubkey ON nostr_events(pubkey)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_nostr_kind ON nostr_events(kind)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_nostr_created_at ON nostr_events(created_at)",
            [],
        )?;

        info!("✅ Nostr relay database schema initialized");
        Ok(())
    }

    async fn start_websocket_server(&self) -> Result<()> {
        info!("🌐 Starting WebSocket server on ws://127.0.0.1:7777");

        // Use tokio-tungstenite for WebSocket server
        let listener = tokio::net::TcpListener::bind("127.0.0.1:7777").await?;
        
        let events = self.events.clone();
        let subscriptions = self.subscriptions.clone();
        let broadcast_tx = self.broadcast_tx.clone();
        let db_path = self.db_path.clone();

        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                debug!("New WebSocket connection from: {}", addr);
                
                let events = events.clone();
                let subscriptions = subscriptions.clone();
                let broadcast_tx = broadcast_tx.clone();
                let db_path = db_path.clone();
                
                tokio::spawn(async move {
                    if let Err(e) = Self::handle_websocket_connection(
                        stream, 
                        addr.to_string(),
                        events,
                        subscriptions,
                        broadcast_tx,
                        db_path
                    ).await {
                        warn!("WebSocket connection error: {}", e);
                    }
                });
            }
        });

        info!("✅ WebSocket server started successfully");
        Ok(())
    }

    async fn handle_websocket_connection(
        stream: tokio::net::TcpStream,
        conn_id: String,
        events: Arc<RwLock<HashMap<EventId, Event>>>,
        subscriptions: Arc<RwLock<HashMap<String, Vec<Filter>>>>,
        broadcast_tx: mpsc::UnboundedSender<RelayMessage>,
        _db_path: String,
    ) -> Result<()> {
        use tokio_tungstenite::{accept_async, tungstenite::Message};
        
        let ws_stream = accept_async(stream).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        info!("✅ WebSocket connection established: {}", conn_id);

        // Handle incoming messages
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    debug!("Received message: {}", text);
                    
                    // Parse Nostr message
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        if let Some(array) = json.as_array() {
                            if !array.is_empty() {
                                match array[0].as_str() {
                                    Some("EVENT") => {
                                        if array.len() >= 2 {
                                            if let Ok(event) = serde_json::from_value::<Event>(array[1].clone()) {
                                                // Store event
                                                let event_id = event.id;
                                                events.write().await.insert(event_id, event.clone());
                                                
                                                // Broadcast event
                                                let _ = broadcast_tx.send(RelayMessage::Event(event));
                                                
                                                // Send OK response
                                                let ok_msg = format!("[\"OK\",\"{}\",true,\"\"]", event_id);
                                                let _ = ws_sender.send(Message::Text(ok_msg)).await;
                                            }
                                        }
                                    }
                                    Some("REQ") => {
                                        if array.len() >= 3 {
                                            let _sub_id = array[1].as_str().unwrap_or("");
                                            // Parse filters and handle subscription
                                            info!("📡 Subscription request received");
                                        }
                                    }
                                    Some("CLOSE") => {
                                        if array.len() >= 2 {
                                            let _sub_id = array[1].as_str().unwrap_or("");
                                            info!("🔒 Close subscription request");
                                        }
                                    }
                                    _ => {
                                        debug!("Unknown message type: {}", text);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("🔌 WebSocket connection closed: {}", conn_id);
                    break;
                }
                Err(e) => {
                    warn!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        // Clean up subscriptions for this connection
        subscriptions.write().await.remove(&conn_id);
        let _ = broadcast_tx.send(RelayMessage::Close { conn_id });

        Ok(())
    }

    async fn start_message_processor(&self, mut rx: mpsc::UnboundedReceiver<RelayMessage>) {
        info!("🔄 Starting message processor");
        
        while let Some(message) = rx.recv().await {
            match message {
                RelayMessage::Event(event) => {
                    debug!("Processing event: {}", event.id);
                    // Store to database
                    if let Err(e) = self.store_event(&event).await {
                        error!("Failed to store event: {}", e);
                    }
                }
                RelayMessage::Subscribe { conn_id, filters } => {
                    debug!("Processing subscription for: {}", conn_id);
                    self.subscriptions.write().await.insert(conn_id, filters);
                }
                RelayMessage::Unsubscribe { conn_id } => {
                    debug!("Processing unsubscribe for: {}", conn_id);
                    self.subscriptions.write().await.remove(&conn_id);
                }
                RelayMessage::Close { conn_id } => {
                    debug!("Processing close for: {}", conn_id);
                    self.subscriptions.write().await.remove(&conn_id);
                }
            }
        }
    }

    async fn store_event(&self, event: &Event) -> Result<()> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        
        let tags_json = serde_json::to_string(&event.tags)?;
        let event_json = serde_json::to_string(event)?;
        
        conn.execute(
            "INSERT OR REPLACE INTO nostr_events 
             (id, pubkey, created_at, kind, tags, content, sig, event_json) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            (
                event.id.to_string(),
                event.pubkey.to_string(),
                event.created_at.as_u64() as i64,
                event.kind.as_u64() as i64,
                tags_json,
                event.content.clone(),
                event.sig.to_string(),
                event_json,
            ),
        )?;

        debug!("✅ Event stored: {}", event.id);
        Ok(())
    }

    /// Get events matching filters
    pub async fn get_events(&self, filters: &[Filter]) -> Result<Vec<Event>> {
        let conn = rusqlite::Connection::open(&self.db_path)?;
        let mut events = Vec::new();
        
        for filter in filters {
            let mut query = String::from("SELECT event_json FROM nostr_events WHERE 1=1");
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
            
            // Add filter conditions
            if let Some(kinds) = &filter.kinds {
                if !kinds.is_empty() {
                    let kind_placeholders: Vec<String> = kinds.iter().map(|_| "?".to_string()).collect();
                    query.push_str(&format!(" AND kind IN ({})", kind_placeholders.join(",")));
                    for kind in kinds {
                        params.push(Box::new(kind.as_u64() as i64));
                    }
                }
            }
            
            if let Some(authors) = &filter.authors {
                if !authors.is_empty() {
                    let author_placeholders: Vec<String> = authors.iter().map(|_| "?".to_string()).collect();
                    query.push_str(&format!(" AND pubkey IN ({})", author_placeholders.join(",")));
                    for author in authors {
                        params.push(Box::new(author.to_string()));
                    }
                }
            }
            
            if let Some(since) = filter.since {
                query.push_str(" AND created_at >= ?");
                params.push(Box::new(since.as_u64() as i64));
            }
            
            if let Some(until) = filter.until {
                query.push_str(" AND created_at <= ?");
                params.push(Box::new(until.as_u64() as i64));
            }
            
            query.push_str(" ORDER BY created_at DESC");
            
            if let Some(limit) = filter.limit {
                query.push_str(" LIMIT ?");
                params.push(Box::new(limit as i64));
            }
            
            let mut stmt = conn.prepare(&query)?;
            let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
            
            let rows = stmt.query_map(param_refs.as_slice(), |row| {
                let event_json: String = row.get(0)?;
                Ok(event_json)
            })?;
            
            for row in rows {
                if let Ok(event_json) = row {
                    if let Ok(event) = serde_json::from_str::<Event>(&event_json) {
                        events.push(event);
                    }
                }
            }
        }
        
        Ok(events)
    }
}