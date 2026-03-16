//! AZC Plugin System
//!
//! Provides dynamic plugin loading and management for extending AZC functionality.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub type PluginId = String;

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub id: PluginId,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub entry_point: String,
}

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub key: String,
    pub value: serde_json::Value,
}

pub trait Plugin: Send + Sync {
    fn info(&self) -> &PluginInfo;

    fn init(&self) -> Result<(), PluginError>;

    fn deinit(&self) -> Result<(), PluginError>;

    fn configure(&self, configs: Vec<PluginConfig>) -> Result<(), PluginError>;

    fn get_capabilities(&self) -> Vec<String>;
}

pub struct PluginManager {
    plugins: Arc<RwLock<HashMap<PluginId, Arc<dyn Plugin>>>>,
    configs: Arc<RwLock<HashMap<PluginId, Vec<PluginConfig>>>>,
    plugin_paths: Vec<PathBuf>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
            plugin_paths: Vec::new(),
        }
    }

    pub fn add_plugin_path(&mut self, path: PathBuf) {
        self.plugin_paths.push(path);
    }

    pub fn register_plugin(&self, plugin: Arc<dyn Plugin>) -> Result<PluginId, PluginError> {
        let info = plugin.info();

        if self.plugins.read().unwrap().contains_key(&info.id) {
            return Err(PluginError::AlreadyLoaded(info.id.clone()));
        }

        plugin.init()?;

        self.plugins
            .write()
            .unwrap()
            .insert(info.id.clone(), plugin);

        if let Ok(configs) = self.configs.read() {
            if let Some(saved_configs) = configs.get(&info.id) {
                let _ = plugin.configure(saved_configs.clone());
            }
        }

        Ok(info.id.clone())
    }

    pub fn unregister_plugin(&self, id: &PluginId) -> Result<(), PluginError> {
        let plugin = self
            .plugins
            .write()
            .unwrap()
            .remove(id)
            .ok_or(PluginError::NotFound(id.clone()))?;

        plugin.deinit()
    }

    pub fn get_plugin(&self, id: &PluginId) -> Option<Arc<dyn Plugin>> {
        self.plugins.read().unwrap().get(id).cloned()
    }

    pub fn configure_plugin(
        &self,
        id: &PluginId,
        configs: Vec<PluginConfig>,
    ) -> Result<(), PluginError> {
        self.configs
            .write()
            .unwrap()
            .insert(id.clone(), configs.clone());

        if let Some(plugin) = self.get_plugin(id) {
            plugin.configure(configs)
        } else {
            Err(PluginError::NotFound(id.clone()))
        }
    }

    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.plugins
            .read()
            .unwrap()
            .values()
            .map(|p| p.info().clone())
            .collect()
    }

    pub fn find_plugins_with_capability(&self, capability: &str) -> Vec<PluginInfo> {
        self.plugins
            .read()
            .unwrap()
            .values()
            .filter(|p| p.get_capabilities().contains(&capability.to_string()))
            .map(|p| p.info().clone())
            .collect()
    }

    pub fn load_plugin_from_file(&self, path: &PathBuf) -> Result<PluginId, PluginError> {
        let metadata = std::fs::metadata(path).map_err(|e| PluginError::IoError(e.to_string()))?;

        if !metadata.is_file() {
            return Err(PluginError::InvalidPath("not a file".to_string()));
        }

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match extension {
            "so" | "dylib" | "dll" => self.load_native_plugin(path),
            "azc" => self.load_azc_plugin(path),
            _ => Err(PluginError::UnsupportedPluginType(extension.to_string())),
        }
    }

    fn load_native_plugin(&self, path: &PathBuf) -> Result<PluginId, PluginError> {
        Err(PluginError::UnsupportedPluginType("native".to_string()))
    }

    fn load_azc_plugin(&self, path: &PathBuf) -> Result<PluginId, PluginError> {
        let content =
            std::fs::read_to_string(path).map_err(|e| PluginError::IoError(e.to_string()))?;

        let info: PluginInfo = serde_json::from_str(&content)
            .map_err(|e| PluginError::InvalidFormat(e.to_string()))?;

        let plugin = Arc::new(AzcPlugin {
            info: info.clone(),
            content,
        });

        self.register_plugin(plugin)
    }

    pub fn get_plugin_config(&self, id: &PluginId) -> Option<Vec<PluginConfig>> {
        self.configs.read().unwrap().get(id).cloned()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

struct AzcPlugin {
    info: PluginInfo,
    content: String,
}

impl Plugin for AzcPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }

    fn init(&self) -> Result<(), PluginError> {
        Ok(())
    }

    fn deinit(&self) -> Result<(), PluginError> {
        Ok(())
    }

    fn configure(&self, _configs: Vec<PluginConfig>) -> Result<(), PluginError> {
        Ok(())
    }

    fn get_capabilities(&self) -> Vec<String> {
        vec!["azc_extension".to_string()]
    }
}

#[derive(Debug, Clone)]
pub enum PluginError {
    NotFound(String),
    AlreadyLoaded(String),
    InvalidPath(String),
    UnsupportedPluginType(String),
    IoError(String),
    InvalidFormat(String),
    InitFailed(String),
    DeinitFailed(String),
    DependencyMissing(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::NotFound(id) => write!(f, "Plugin not found: {}", id),
            PluginError::AlreadyLoaded(id) => write!(f, "Plugin already loaded: {}", id),
            PluginError::InvalidPath(msg) => write!(f, "Invalid plugin path: {}", msg),
            PluginError::UnsupportedPluginType(t) => write!(f, "Unsupported plugin type: {}", t),
            PluginError::IoError(msg) => write!(f, "IO error: {}", msg),
            PluginError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            PluginError::InitFailed(msg) => write!(f, "Plugin init failed: {}", msg),
            PluginError::DeinitFailed(msg) => write!(f, "Plugin deinit failed: {}", msg),
            PluginError::DependencyMissing(dep) => write!(f, "Missing dependency: {}", dep),
        }
    }
}

impl std::error::Error for PluginError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager() {
        let manager = PluginManager::new();

        let plugins = manager.list_plugins();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_azc_plugin() {
        let plugin = AzcPlugin {
            info: PluginInfo {
                id: "test".to_string(),
                name: "Test Plugin".to_string(),
                version: "1.0.0".to_string(),
                author: "Test".to_string(),
                description: "A test plugin".to_string(),
                dependencies: Vec::new(),
                entry_point: "main".to_string(),
            },
            content: "{}".to_string(),
        };

        assert_eq!(plugin.info().name, "Test Plugin");
        assert!(plugin.init().is_ok());
    }
}
