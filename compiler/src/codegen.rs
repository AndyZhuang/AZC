//! AZC Fast Compilation System
//!
//! Provides incremental compilation, caching, and parallel processing
//! for extremely fast compilation times (<100ms for small changes).

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::ast::{Program, Statement};

pub type SourceHash = String;
pub type ModuleCacheKey = String;

#[derive(Debug, Clone)]
pub struct CompilationCache {
    source_hash: SourceHash,
    ast_hash: SourceHash,
    codegen_hash: SourceHash,
    output: String,
    timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct CompilationResult {
    pub output: String,
    pub success: bool,
    pub duration: Duration,
    pub cache_hit: bool,
    pub warnings: Vec<String>,
}

pub struct FastCompiler {
    cache: Arc<RwLock<HashMap<ModuleCacheKey, CompilationCache>>>,
    incremental: bool,
    parallel: bool,
    max_cache_size: usize,
}

impl FastCompiler {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            incremental: true,
            parallel: true,
            max_cache_size: 1000,
        }
    }

    pub fn with_incremental(mut self, enable: bool) -> Self {
        self.incremental = enable;
        self
    }

    pub fn with_parallel(mut self, enable: bool) -> Self {
        self.parallel = enable;
        self
    }

    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.max_cache_size = size;
        self
    }

    pub fn compile(&self, source: &str, path: Option<&str>) -> CompilationResult {
        let start = Instant::now();
        let source_hash = compute_hash(source);
        let cache_key = path
            .map(|p| p.to_string())
            .unwrap_or_else(|| "default".to_string());

        let cache_hit = if self.incremental {
            if let Some(cached) = self.get_cached(&cache_key, &source_hash) {
                return CompilationResult {
                    output: cached.output,
                    success: true,
                    duration: start.elapsed(),
                    cache_hit: true,
                    warnings: Vec::new(),
                };
            }
            false
        } else {
            false
        };

        match crate::compile(source) {
            Ok(output) => {
                let output_hash = compute_hash(&output);

                if self.incremental {
                    self.cache_output(&cache_key, &source_hash, &output_hash, &output);
                }

                CompilationResult {
                    output,
                    success: true,
                    duration: start.elapsed(),
                    cache_hit,
                    warnings: Vec::new(),
                }
            }
            Err(e) => CompilationResult {
                output: e,
                success: false,
                duration: start.elapsed(),
                cache_hit,
                warnings: Vec::new(),
            },
        }
    }

    fn get_cached(&self, key: &str, source_hash: &str) -> Option<String> {
        let cache = self.cache.read().unwrap();
        if let Some(cached) = cache.get(key) {
            if cached.source_hash == *source_hash {
                return Some(cached.output.clone());
            }
        }
        None
    }

    fn cache_output(&self, key: &str, source_hash: &str, _output_hash: &str, output: &str) {
        let mut cache = self.cache.write().unwrap();

        if cache.len() >= self.max_cache_size {
            let oldest = cache
                .iter()
                .min_by_key(|(_, v)| v.timestamp)
                .map(|(k, _)| k.clone());

            if let Some(oldest_key) = oldest {
                cache.remove(&oldest_key);
            }
        }

        cache.insert(
            key.to_string(),
            CompilationCache {
                source_hash: source_hash.to_string(),
                ast_hash: String::new(),
                codegen_hash: String::new(),
                output: output.to_string(),
                timestamp: Instant::now(),
            },
        );
    }

    pub fn clear_cache(&self) {
        self.cache.write().unwrap().clear();
    }

    pub fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.read().unwrap();
        CacheStats {
            entries: cache.len(),
            max_entries: self.max_cache_size,
        }
    }

    pub fn preload_modules(&self, paths: &[PathBuf]) -> Result<(), String> {
        for path in paths {
            let source = fs::read_to_string(path)
                .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

            let module_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            self.compile(&source, Some(module_name));
        }
        Ok(())
    }
}

impl Default for FastCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub max_entries: usize,
}

fn compute_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub struct ParallelProcessor {
    max_workers: usize,
}

impl ParallelProcessor {
    pub fn new(max_workers: usize) -> Self {
        Self {
            max_workers: max_workers.max(1),
        }
    }

    pub fn process_statements<F, R>(&self, statements: &[Statement], f: F) -> Vec<R>
    where
        F: Fn(&Statement) -> R + Send + Sync,
        R: Send,
    {
        if self.max_workers == 1 {
            statements.iter().map(f).collect()
        } else {
            statements.par_iter().map(f).collect()
        }
    }
}

impl Default for ParallelProcessor {
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

#[cfg(feature = "parallel")]
mod parallel_impl {
    use rayon::prelude::*;

    pub fn par_map<T, R, F>(items: &[T], f: F) -> Vec<R>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&T) -> R + Sync,
    {
        items.par_iter().map(f).collect()
    }
}

#[cfg(not(feature = "parallel"))]
mod parallel_impl {
    pub fn par_map<T, R, F>(items: &[T], f: F) -> Vec<R>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&T) -> R + Sync,
    {
        items.iter().map(f).collect()
    }
}

pub use parallel_impl::par_map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_compiler() {
        let compiler = FastCompiler::new();

        let source = r#"
            let x = 10
            puts x
        "#;

        let result = compiler.compile(source, None);
        assert!(result.success);
        assert!(result.duration.as_millis() < 1000);
    }

    #[test]
    fn test_cache() {
        let compiler = FastCompiler::new();

        let source = "let x = 10";

        let result1 = compiler.compile(source, Some("test"));
        assert!(!result1.cache_hit);

        let result2 = compiler.compile(source, Some("test"));
        assert!(result2.cache_hit);
    }

    #[test]
    fn test_cache_stats() {
        let compiler = FastCompiler::new();
        compiler.compile("let x = 10", Some("test"));

        let stats = compiler.cache_stats();
        assert!(stats.entries > 0);
    }
}
