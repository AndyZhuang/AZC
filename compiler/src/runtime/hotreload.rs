//! AZC Hot Reload System
//!
//! Provides live code reloading for Agents without stopping execution.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};

pub type ModuleId = String;
pub type Version = u64;

#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    pub watch_dirs: Vec<PathBuf>,
    pub debounce_ms: u64,
    pub enable_auto_reload: bool,
    pub max_versions: usize,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            watch_dirs: Vec::new(),
            debounce_ms: 100,
            enable_auto_reload: true,
            max_versions: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct ModuleVersion {
    pub version: Version,
    pub content: String,
    pub checksum: String,
    pub timestamp: Instant,
    pub source_path: Option<PathBuf>,
}

pub struct Module {
    id: ModuleId,
    current_version: Version,
    versions: Vec<ModuleVersion>,
    watchers: Arc<Mutex<Vec<Box<dyn Fn(ModuleId, ChangeType, Version) + Send + Sync>>>>,
}

impl Module {
    pub fn new(id: &str, content: &str) -> Self {
        let checksum = compute_checksum(content);
        let version = ModuleVersion {
            version: 1,
            content: content.to_string(),
            checksum,
            timestamp: Instant::now(),
            source_path: None,
        };

        Self {
            id: id.to_string(),
            current_version: 1,
            versions: vec![version],
            watchers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn current_version(&self) -> Version {
        self.current_version
    }

    pub fn get_version(&self, version: Version) -> Option<&ModuleVersion> {
        self.versions.iter().find(|v| v.version == version)
    }

    pub fn get_current(&self) -> &ModuleVersion {
        self.get_version(self.current_version).unwrap()
    }

    pub fn update(&self, new_content: &str) -> Result<Version, HotReloadError> {
        let new_checksum = compute_checksum(new_content);

        if let Some(current) = self.get_current() {
            if current.checksum == new_checksum {
                return Err(HotReloadError::NoChanges);
            }
        }

        let new_version = self.current_version + 1;
        let module_version = ModuleVersion {
            version: new_version,
            content: new_content.to_string(),
            checksum: new_checksum,
            timestamp: Instant::now(),
            source_path: None,
        };

        {
            let mut versions = Vec::new();
            for v in &self.versions {
                if self.versions.len() < 10 || v.version > self.current_version - 5 {
                    versions.push(v.clone());
                }
            }
            versions.push(module_version);
        }

        self.current_version = new_version;

        for watcher in self.watchers.lock().unwrap().iter() {
            watcher(self.id.clone(), ChangeType::Modified, new_version);
        }

        Ok(new_version)
    }

    pub fn rollback(&self, version: Version) -> Result<Version, HotReloadError> {
        if !self.versions.iter().any(|v| v.version == version) {
            return Err(HotReloadError::VersionNotFound(version));
        }

        let old_version = self.current_version;
        self.current_version = version;

        for watcher in self.watchers.lock().unwrap().iter() {
            watcher(self.id.clone(), ChangeType::Modified, version);
        }

        Ok(old_version)
    }

    pub fn subscribe(
        &self,
        watcher: impl Fn(ModuleId, ChangeType, Version) + Send + Sync + 'static,
    ) {
        self.watchers.lock().unwrap().push(Box::new(watcher));
    }

    pub fn diff(&self, v1: Version, v2: Version) -> Option<String> {
        let content1 = self.get_version(v1)?.content.clone();
        let content2 = self.get_version(v2)?.content.clone();

        Some(compute_diff(&content1, &content2))
    }
}

pub struct HotReloadManager {
    modules: Arc<RwLock<HashMap<ModuleId, Module>>>,
    config: HotReloadConfig,
    last_reload: Arc<Mutex<Instant>>,
}

impl HotReloadManager {
    pub fn new(config: HotReloadConfig) -> Self {
        Self {
            modules: Arc::new(RwLock::new(HashMap::new())),
            config,
            last_reload: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn register_module(&self, id: &str, content: &str) -> Result<ModuleId, HotReloadError> {
        let mut modules = self.modules.write().unwrap();

        if modules.contains_key(id) {
            return Err(HotReloadError::ModuleExists(id.to_string()));
        }

        let module = Module::new(id, content);
        let module_id = module.id().to_string();
        modules.insert(module_id.clone(), module);

        Ok(module_id)
    }

    pub fn update_module(&self, id: &str, content: &str) -> Result<Version, HotReloadError> {
        let modules = self.modules.read().unwrap();
        let module = modules
            .get(id)
            .ok_or(HotReloadError::ModuleNotFound(id.to_string()))?;

        module.update(content)
    }

    pub fn get_module(&self, id: &str) -> Option<String> {
        let modules = self.modules.read().unwrap();
        modules.get(id).map(|m| m.get_current().content.clone())
    }

    pub fn get_version(&self, id: &str, version: Version) -> Option<ModuleVersion> {
        let modules = self.modules.read().unwrap();
        modules
            .get(id)
            .and_then(|m| m.get_version(version).cloned())
    }

    pub fn list_modules(&self) -> Vec<(ModuleId, Version)> {
        let modules = self.modules.read().unwrap();
        modules
            .iter()
            .map(|(id, m)| (id.clone(), m.current_version()))
            .collect()
    }

    pub fn can_reload(&self) -> bool {
        let last = *self.last_reload.lock().unwrap();
        let debounce = Duration::from_millis(self.config.debounce_ms);
        last.elapsed() > debounce
    }

    pub fn mark_reloaded(&self) {
        *self.last_reload.lock().unwrap() = Instant::now();
    }
}

#[derive(Debug, Clone)]
pub enum HotReloadError {
    ModuleNotFound(String),
    ModuleExists(String),
    VersionNotFound(Version),
    NoChanges,
    ReloadDenied,
}

impl std::fmt::Display for HotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HotReloadError::ModuleNotFound(id) => write!(f, "Module not found: {}", id),
            HotReloadError::ModuleExists(id) => write!(f, "Module already exists: {}", id),
            HotReloadError::VersionNotFound(v) => write!(f, "Version not found: {}", v),
            HotReloadError::NoChanges => write!(f, "No changes to module"),
            HotReloadError::ReloadDenied => write!(f, "Reload denied (too soon)"),
        }
    }
}

impl std::error::Error for HotReloadError {}

fn compute_checksum(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn compute_diff(old: &str, new: &str) -> String {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let mut diff = String::new();

    let max_lines = old_lines.len().max(new_lines.len());

    for i in 0..max_lines {
        let old_line = old_lines.get(i).copied();
        let new_line = new_lines.get(i).copied();

        match (old_line, new_line) {
            (Some(a), Some(b)) if a == b => {
                diff.push_str(&format!("  {}\n", b));
            }
            (Some(a), Some(b)) => {
                diff.push_str(&format!("- {}\n", a));
                diff.push_str(&format!("+ {}\n", b));
            }
            (Some(a), None) => {
                diff.push_str(&format!("- {}\n", a));
            }
            (None, Some(b)) => {
                diff.push_str(&format!("+ {}\n", b));
            }
            _ => {}
        }
    }

    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = Module::new("test", "hello world");
        assert_eq!(module.id(), "test");
        assert_eq!(module.current_version(), 1);
    }

    #[test]
    fn test_module_update() {
        let module = Module::new("test", "hello");
        let new_version = module.update("hello world").unwrap();
        assert_eq!(new_version, 2);
        assert_eq!(module.get_current().content, "hello world");
    }

    #[test]
    fn test_module_no_changes() {
        let module = Module::new("test", "hello");
        let result = module.update("hello");
        assert!(matches!(result, Err(HotReloadError::NoChanges)));
    }

    #[test]
    fn test_hotreload_manager() {
        let manager = HotReloadManager::new(HotReloadConfig::default());

        manager.register_module("test", "content").unwrap();
        let content = manager.get_module("test").unwrap();
        assert_eq!(content, "content");

        manager.update_module("test", "new content").unwrap();
        let content = manager.get_module("test").unwrap();
        assert_eq!(content, "new content");
    }
}
