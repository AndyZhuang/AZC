//! AZC Distributed System
//!
//! Provides secure peer-to-peer agent communication with encryption
//! and authentication.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

pub type AgentEndpoint = String;
pub type SessionId = String;

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub bind_address: SocketAddr,
    pub max_connections: usize,
    pub timeout: Duration,
    pub enable_encryption: bool,
    pub enable_auth: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:0".parse().unwrap(),
            max_connections: 100,
            timeout: Duration::from_secs(30),
            enable_encryption: true,
            enable_auth: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub agent_id: AgentEndpoint,
    pub address: SocketAddr,
    pub connected_at: Instant,
    pub last_activity: Instant,
    pub encrypted: bool,
    pub authenticated: bool,
    pub messages_sent: u64,
    pub messages_received: u64,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Request,
    Response,
    Error,
    Heartbeat,
    AuthRequest,
    AuthResponse,
}

#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub id: String,
    pub session_id: SessionId,
    pub message_type: MessageType,
    pub from: AgentEndpoint,
    pub to: AgentEndpoint,
    pub method: String,
    pub payload: Vec<u8>,
    pub timestamp: Instant,
    pub encrypted: bool,
}

impl AgentMessage {
    pub fn new(from: &str, to: &str, method: &str, payload: Vec<u8>) -> Self {
        Self {
            id: uuid_v4(),
            session_id: uuid_v4(),
            message_type: MessageType::Request,
            from: from.to_string(),
            to: to.to_string(),
            method: method.to_string(),
            payload,
            timestamp: Instant::now(),
            encrypted: false,
        }
    }

    pub fn encrypt(&mut self, key: &[u8]) {
        if !self.encrypted {
            self.payload = simple_encrypt(&self.payload, key);
            self.encrypted = true;
        }
    }

    pub fn decrypt(&mut self, key: &[u8]) -> Result<(), DistributedError> {
        if self.encrypted {
            self.payload = simple_decrypt(&self.payload, key)
                .map_err(|_| DistributedError::DecryptionFailed)?;
            self.encrypted = false;
        }
        Ok(())
    }
}

pub struct Session {
    id: SessionId,
    client_agent: AgentEndpoint,
    server_agent: AgentEndpoint,
    established_at: Instant,
    last_activity: Instant,
    encryption_key: Option<Vec<u8>>,
    message_count: u64,
}

impl Session {
    pub fn new(client: &str, server: &str) -> Self {
        Self {
            id: uuid_v4(),
            client_agent: client.to_string(),
            server_agent: server.to_string(),
            established_at: Instant::now(),
            last_activity: Instant::now(),
            encryption_key: None,
            message_count: 0,
        }
    }

    pub fn set_encryption_key(&mut self, key: Vec<u8>) {
        self.encryption_key = Some(key);
    }

    pub fn encrypt_message(&self, msg: &mut AgentMessage) -> Result<(), DistributedError> {
        if let Some(key) = &self.encryption_key {
            msg.encrypt(key);
        }
        Ok(())
    }

    pub fn decrypt_message(&self, msg: &mut AgentMessage) -> Result<(), DistributedError> {
        if let Some(key) = &self.encryption_key {
            msg.decrypt(key)?;
        }
        Ok(())
    }

    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
        self.message_count += 1;
    }

    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }
}

pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<AgentEndpoint, ConnectionInfo>>>,
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, agent_id: &AgentEndpoint, address: SocketAddr) {
        let info = ConnectionInfo {
            agent_id: agent_id.clone(),
            address,
            connected_at: Instant::now(),
            last_activity: Instant::now(),
            encrypted: false,
            authenticated: false,
            messages_sent: 0,
            messages_received: 0,
        };

        self.agents.write().unwrap().insert(agent_id.clone(), info);
    }

    pub fn unregister(&self, agent_id: &AgentEndpoint) {
        self.agents.write().unwrap().remove(agent_id);

        let mut sessions = self.sessions.write().unwrap();
        sessions.retain(|_, s| s.client_agent != *agent_id && s.server_agent != *agent_id);
    }

    pub fn get(&self, agent_id: &AgentEndpoint) -> Option<ConnectionInfo> {
        self.agents.read().unwrap().get(agent_id).cloned()
    }

    pub fn find_by_capability(&self, _capability: &str) -> Vec<ConnectionInfo> {
        self.agents.read().unwrap().values().cloned().collect()
    }

    pub fn list_all(&self) -> Vec<ConnectionInfo> {
        self.agents.read().unwrap().values().cloned().collect()
    }

    pub fn create_session(&self, client: &str, server: &str) -> SessionId {
        let mut session = Session::new(client, server);
        let key = generate_key();
        session.set_encryption_key(key);

        let session_id = session.id.clone();

        self.sessions
            .write()
            .unwrap()
            .insert(session_id.clone(), session);

        session_id
    }

    pub fn get_session(&self, session_id: &SessionId) -> Option<Session> {
        self.sessions.read().unwrap().get(session_id).cloned()
    }

    pub fn remove_session(&self, session_id: &SessionId) {
        self.sessions.write().unwrap().remove(session_id);
    }

    pub fn cleanup_expired(&self, timeout: Duration) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.retain(|_, s| !s.is_expired(timeout));
    }

    pub fn update_activity(&self, agent_id: &AgentEndpoint) {
        if let Some(info) = self.agents.write().unwrap().get_mut(agent_id) {
            info.last_activity = Instant::now();
        }
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum DistributedError {
    ConnectionFailed(String),
    SendFailed(String),
    ReceiveFailed(String),
    EncryptionFailed,
    DecryptionFailed,
    AuthenticationFailed,
    SessionNotFound(String),
    AgentNotFound(String),
    Timeout,
}

impl std::fmt::Display for DistributedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistributedError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            DistributedError::SendFailed(msg) => write!(f, "Send failed: {}", msg),
            DistributedError::ReceiveFailed(msg) => write!(f, "Receive failed: {}", msg),
            DistributedError::EncryptionFailed => write!(f, "Encryption failed"),
            DistributedError::DecryptionFailed => write!(f, "Decryption failed"),
            DistributedError::AuthenticationFailed => write!(f, "Authentication failed"),
            DistributedError::SessionNotFound(id) => write!(f, "Session not found: {}", id),
            DistributedError::AgentNotFound(id) => write!(f, "Agent not found: {}", id),
            DistributedError::Timeout => write!(f, "Operation timed out"),
        }
    }
}

impl std::error::Error for DistributedError {}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}

fn generate_key() -> Vec<u8> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut key = Vec::new();
    for i in 0..32 {
        key.push(((timestamp >> i) & 0xff) as u8 ^ (i as u8 * 0x9e));
    }
    key
}

fn simple_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

fn simple_decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, ()> {
    Ok(simple_encrypt(data, key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_registry() {
        let registry = AgentRegistry::new();

        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        registry.register("agent1", addr);

        let info = registry.get("agent1").unwrap();
        assert_eq!(info.agent_id, "agent1");
    }

    #[test]
    fn test_session_encryption() {
        let mut session = Session::new("client", "server");
        let key = generate_key();
        session.set_encryption_key(key);

        let mut msg = AgentMessage::new("client", "server", "test", b"hello".to_vec());

        session.encrypt_message(&mut msg).unwrap();
        assert!(msg.encrypted);

        session.decrypt_message(&mut msg).unwrap();
        assert!(!msg.encrypted);
        assert_eq!(msg.payload, b"hello");
    }

    #[test]
    fn test_message_encryption() {
        let key = generate_key();

        let mut msg = AgentMessage::new("a", "b", "test", b"secret".to_vec());
        msg.encrypt(&key);

        assert!(msg.encrypted);

        msg.decrypt(&key).unwrap();
        assert_eq!(msg.payload, b"secret");
    }
}
