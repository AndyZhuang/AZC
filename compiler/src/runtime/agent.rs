//! AZC Agent Runtime System
//!
//! Provides native support for AI Agent sandbox, hot reload, plugins,
//! and distributed secure transmission.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub type AgentId = String;
pub type ToolName = String;
pub type MessageId = String;

#[derive(Debug, Clone)]
pub enum AgentState {
    Idle,
    Running,
    Waiting,
    Error(String),
    Terminated,
}

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub id: AgentId,
    pub memory_limit: usize,
    pub cpu_quota: Duration,
    pub network_enabled: bool,
    pub timeout: Duration,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            id: uuid_v4(),
            memory_limit: 64 * 1024 * 1024,
            cpu_quota: Duration::from_secs(30),
            network_enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: ToolName,
    pub handler: Arc<Mutex<dyn Fn(Vec<Value>) -> Value + Send + Sync>>,
    pub description: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Binary(Vec<u8>),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

pub struct Agent {
    id: AgentId,
    state: Arc<Mutex<AgentState>>,
    memory: Arc<Mutex<HashMap<String, Value>>>,
    tools: Arc<Mutex<HashMap<ToolName, Tool>>>,
    config: AgentConfig,
    created_at: Instant,
    message_log: Arc<Mutex<Vec<AgentMessage>>>,
}

struct AgentMessage {
    id: MessageId,
    timestamp: Instant,
    direction: MessageDirection,
    content: String,
}

enum MessageDirection {
    Incoming,
    Outgoing,
}

impl Agent {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            state: Arc::new(Mutex::new(AgentState::Idle)),
            memory: Arc::new(Mutex::new(HashMap::new())),
            tools: Arc::new(Mutex::new(HashMap::new())),
            config: AgentConfig::default(),
            created_at: Instant::now(),
            message_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn with_config(id: &str, config: AgentConfig) -> Self {
        Self {
            id: id.to_string(),
            state: Arc::new(Mutex::new(AgentState::Idle)),
            memory: Arc::new(Mutex::new(HashMap::new())),
            tools: Arc::new(Mutex::new(HashMap::new())),
            config,
            created_at: Instant::now(),
            message_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> AgentState {
        self.state.lock().unwrap().clone()
    }

    pub fn register_tool(
        &self,
        name: &str,
        handler: impl Fn(Vec<Value>) -> Value + Send + Sync + 'static,
        description: &str,
    ) {
        let tool = Tool {
            name: name.to_string(),
            handler: Arc::new(Mutex::new(handler)),
            description: description.to_string(),
            parameters: Vec::new(),
        };
        self.tools.lock().unwrap().insert(name.to_string(), tool);
    }

    pub fn call_tool(&self, name: &str, args: Vec<Value>) -> Result<Value, AgentError> {
        let tools = self.tools.lock().unwrap();
        let tool = tools
            .get(name)
            .ok_or(AgentError::ToolNotFound(name.to_string()))?;

        let handler = tool.handler.lock().unwrap();
        Ok(handler(args))
    }

    pub fn set_state(&self, key: &str, value: Value) {
        self.memory.lock().unwrap().insert(key.to_string(), value);
    }

    pub fn get_state(&self, key: &str) -> Option<Value> {
        self.memory.lock().unwrap().get(key).cloned()
    }

    pub fn log_message(&self, direction: MessageDirection, content: &str) {
        let msg = AgentMessage {
            id: uuid_v4(),
            timestamp: Instant::now(),
            direction,
            content: content.to_string(),
        };
        self.message_log.lock().unwrap().push(msg);
    }

    pub fn get_message_log(&self) -> Vec<(MessageId, String, String)> {
        self.message_log
            .lock()
            .unwrap()
            .iter()
            .map(|m| {
                let dir = match m.direction {
                    MessageDirection::Incoming => "incoming",
                    MessageDirection::Outgoing => "outgoing",
                };
                (m.id.clone(), dir.to_string(), m.content.clone())
            })
            .collect()
    }

    pub fn reset(&self) {
        self.memory.lock().unwrap().clear();
        self.message_log.lock().unwrap().clear();
        *self.state.lock().unwrap() = AgentState::Idle;
    }

    pub fn export_state(&self) -> HashMap<String, Value> {
        self.memory.lock().unwrap().clone()
    }

    pub fn import_state(&self, state: HashMap<String, Value>) {
        *self.memory.lock().unwrap() = state;
    }
}

#[derive(Debug, Clone)]
pub enum AgentError {
    ToolNotFound(String),
    InvalidArgument(String),
    Timeout,
    MemoryLimitExceeded,
    NetworkError(String),
    SandboxViolation(String),
    PluginError(String),
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentError::ToolNotFound(name) => write!(f, "Tool not found: {}", name),
            AgentError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            AgentError::Timeout => write!(f, "Agent operation timed out"),
            AgentError::MemoryLimitExceeded => write!(f, "Agent memory limit exceeded"),
            AgentError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AgentError::SandboxViolation(msg) => write!(f, "Sandbox violation: {}", msg),
            AgentError::PluginError(msg) => write!(f, "Plugin error: {}", msg),
        }
    }
}

impl std::error::Error for AgentError {}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!(
        "{:x}-{:x}-4{:x}-{:x}-{:x}",
        (timestamp >> 96) & 0xffffffff,
        (timestamp >> 64) & 0xffff,
        (timestamp >> 48) & 0xfff,
        (timestamp >> 32) & 0x3fff | 0x8000,
        timestamp & 0xffffffffffff
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new("test_agent");
        assert_eq!(agent.id(), "test_agent");
        assert!(matches!(agent.state(), AgentState::Idle));
    }

    #[test]
    fn test_agent_tools() {
        let agent = Agent::new("test");

        agent.register_tool(
            "add",
            |args| {
                if let (Value::Number(a), Value::Number(b)) = (&args[0], &args[1]) {
                    Value::Number(a + b)
                } else {
                    Value::Null
                }
            },
            "Add two numbers",
        );

        let result = agent.call_tool("add", vec![Value::Number(2.0), Value::Number(3.0)]);
        assert!(matches!(result, Ok(Value::Number(5.0))));
    }

    #[test]
    fn test_agent_state() {
        let agent = Agent::new("test");
        agent.set_state("name", Value::String("AZC".to_string()));

        let name = agent.get_state("name");
        assert!(matches!(name, Some(Value::String(s)) if s == "AZC"));
    }
}
