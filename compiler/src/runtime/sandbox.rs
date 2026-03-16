//! AZC Sandbox System
//!
//! Provides secure isolation for Agent execution with resource limits
//! and capability-based access control.

use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

pub type Capability = String;

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_bytes: usize,
    pub max_cpu_seconds: f64,
    pub max_network_bytes: usize,
    pub max_files: usize,
    pub max_execution_time: Duration,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 64 * 1024 * 1024,
            max_cpu_seconds: 30.0,
            max_network_bytes: 10 * 1024 * 1024,
            max_files: 100,
            max_execution_time: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub id: String,
    pub limits: ResourceLimits,
    pub capabilities: HashSet<Capability>,
    pub enable_network: bool,
    pub enable_filesystem: bool,
    pub enable_subprocess: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            id: uuid_v4(),
            limits: ResourceLimits::default(),
            capabilities: HashSet::new(),
            enable_network: true,
            enable_filesystem: false,
            enable_subprocess: false,
        }
    }
}

pub struct ResourceTracker {
    memory_used: Arc<Mutex<usize>>,
    cpu_used: Arc<Mutex<f64>>,
    network_used: Arc<Mutex<usize>>,
    started_at: Arc<RwLock<Instant>>,
    limits: ResourceLimits,
}

impl ResourceTracker {
    pub fn new(limits: ResourceLimits) -> Self {
        Self {
            memory_used: Arc::new(Mutex::new(0)),
            cpu_used: Arc::new(Mutex::new(0.0)),
            network_used: Arc::new(Mutex::new(0)),
            started_at: Arc::new(RwLock::new(Instant::now())),
            limits,
        }
    }

    pub fn alloc_memory(&self, size: usize) -> Result<(), SandboxError> {
        let mut used = self.memory_used.lock().unwrap();
        if *used + size > self.limits.max_memory_bytes {
            return Err(SandboxError::MemoryLimitExceeded);
        }
        *used += size;
        Ok(())
    }

    pub fn free_memory(&self, size: usize) {
        let mut used = self.memory_used.lock().unwrap();
        if *used >= size {
            *used -= size;
        }
    }

    pub fn add_cpu_time(&self, seconds: f64) -> Result<(), SandboxError> {
        let mut used = self.cpu_used.lock().unwrap();
        *used += seconds;
        if *used > self.limits.max_cpu_seconds {
            return Err(SandboxError::CPULimitExceeded);
        }
        Ok(())
    }

    pub fn add_network_bytes(&self, bytes: usize) -> Result<(), SandboxError> {
        let mut used = self.network_used.lock().unwrap();
        if *used + bytes > self.limits.max_network_bytes {
            return Err(SandboxError::NetworkLimitExceeded);
        }
        *used += bytes;
        Ok(())
    }

    pub fn check_timeout(&self) -> Result<(), SandboxError> {
        let start = self.started_at.read().unwrap();
        if start.elapsed() > self.limits.max_execution_time {
            return Err(SandboxError::ExecutionTimeout);
        }
        Ok(())
    }

    pub fn get_stats(&self) -> ResourceStats {
        ResourceStats {
            memory_used: *self.memory_used.lock().unwrap(),
            cpu_used: *self.cpu_used.lock().unwrap(),
            network_used: *self.network_used.lock().unwrap(),
            elapsed_time: self.started_at.read().unwrap().elapsed(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResourceStats {
    pub memory_used: usize,
    pub cpu_used: f64,
    pub network_used: usize,
    pub elapsed_time: Duration,
}

pub struct Sandbox {
    config: SandboxConfig,
    resources: ResourceTracker,
    allowed_paths: HashSet<String>,
    allowed_hosts: HashSet<String>,
}

impl Sandbox {
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            config: config.clone(),
            resources: ResourceTracker::new(config.limits),
            allowed_paths: HashSet::new(),
            allowed_hosts: HashSet::new(),
        }
    }

    pub fn add_allowed_path(&mut self, path: &str) {
        self.allowed_paths.insert(path.to_string());
    }

    pub fn add_allowed_host(&mut self, host: &str) {
        self.allowed_hosts.insert(host.to_string());
    }

    pub fn check_capability(&self, cap: &Capability) -> bool {
        self.config.capabilities.contains(cap)
    }

    pub fn check_path(&self, path: &str) -> Result<(), SandboxError> {
        if !self.config.enable_filesystem {
            return Err(SandboxError::FilesystemDisabled);
        }

        for allowed in &self.allowed_paths {
            if path.starts_with(allowed) {
                return Ok(());
            }
        }
        Err(SandboxError::PathNotAllowed(path.to_string()))
    }

    pub fn check_network(&self, host: &str) -> Result<(), SandboxError> {
        if !self.config.enable_network {
            return Err(SandboxError::NetworkDisabled);
        }

        if self.allowed_hosts.is_empty() {
            return Ok(());
        }

        for allowed in &self.allowed_hosts {
            if host == allowed || host.ends_with(&format!(".{}", allowed)) {
                return Ok(());
            }
        }

        Err(SandboxError::HostNotAllowed(host.to_string()))
    }

    pub fn resources(&self) -> &ResourceTracker {
        &self.resources
    }

    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub enum SandboxError {
    MemoryLimitExceeded,
    CPULimitExceeded,
    NetworkLimitExceeded,
    ExecutionTimeout,
    FilesystemDisabled,
    NetworkDisabled,
    PathNotAllowed(String),
    HostNotAllowed(String),
    CapabilityDenied(String),
}

impl std::fmt::Display for SandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxError::MemoryLimitExceeded => write!(f, "Memory limit exceeded"),
            SandboxError::CPULimitExceeded => write!(f, "CPU limit exceeded"),
            SandboxError::NetworkLimitExceeded => write!(f, "Network limit exceeded"),
            SandboxError::ExecutionTimeout => write!(f, "Execution timeout"),
            SandboxError::FilesystemDisabled => write!(f, "Filesystem access disabled"),
            SandboxError::NetworkDisabled => write!(f, "Network access disabled"),
            SandboxError::PathNotAllowed(p) => write!(f, "Path not allowed: {}", p),
            SandboxError::HostNotAllowed(h) => write!(f, "Host not allowed: {}", h),
            SandboxError::CapabilityDenied(c) => write!(f, "Capability denied: {}", c),
        }
    }
}

impl std::error::Error for SandboxError {}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", timestamp)
}
