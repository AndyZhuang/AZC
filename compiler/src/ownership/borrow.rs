//! Borrow Checker
//!
//! Validates borrow rules at compile time.

use super::{OwnershipAnalyzer, OwnershipError};
use std::collections::{HashMap, HashSet};

/// Borrow checker
#[derive(Debug)]
pub struct BorrowChecker {
    analyzer: OwnershipAnalyzer,
}

impl BorrowChecker {
    pub fn new() -> Self {
        BorrowChecker {
            analyzer: OwnershipAnalyzer::new(),
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.analyzer.enter_scope();
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) -> Vec<String> {
        self.analyzer.exit_scope()
    }

    /// Declare a new variable
    pub fn declare(&mut self, name: &str) {
        self.analyzer.declare(name);
    }

    /// Check a use of a variable
    pub fn check_use(&mut self, name: &str) -> Result<(), OwnershipError> {
        self.analyzer.can_use(name)
    }

    /// Check an assignment (potential move or copy)
    pub fn check_assign(
        &mut self,
        target: &str,
        source: &str,
        is_copy: bool,
    ) -> Result<(), OwnershipError> {
        if target == source {
            return Ok(());
        }

        if is_copy {
            self.analyzer.copy_value(source, target)
        } else {
            self.analyzer.move_value(source, target)
        }
    }

    /// Check an immutable borrow
    pub fn check_borrow(&mut self, var: &str) -> Result<(), OwnershipError> {
        self.analyzer.borrow(var)
    }

    /// Check a mutable borrow
    pub fn check_borrow_mut(&mut self, var: &str) -> Result<(), OwnershipError> {
        self.analyzer.borrow_mut(var)
    }

    /// End a borrow
    pub fn end_borrow(&mut self, var: &str) {
        self.analyzer.end_borrow(var);
    }

    /// Validate that all borrows are valid
    pub fn validate(&self) -> Result<Vec<String>, OwnershipError> {
        self.analyzer.check_active_borrows()?;

        let warnings: Vec<String> = self
            .analyzer
            .active_borrows()
            .iter()
            .map(|b| format!("Warning: Borrow of '{}' is still active", b.var))
            .collect();

        Ok(warnings)
    }

    /// Get the underlying analyzer
    pub fn analyzer(&self) -> &OwnershipAnalyzer {
        &self.analyzer
    }
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Lifetime annotation (for future implementation)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime {
    pub name: String,
}

impl Lifetime {
    pub fn new(name: &str) -> Self {
        Lifetime {
            name: name.to_string(),
        }
    }

    pub fn anonymous() -> Self {
        Lifetime {
            name: "_".to_string(),
        }
    }
}

/// Lifetime checker (for future implementation)
#[derive(Debug)]
pub struct LifetimeChecker {
    scopes: Vec<HashMap<String, Lifetime>>,
    current: Lifetime,
}

impl LifetimeChecker {
    pub fn new() -> Self {
        LifetimeChecker {
            scopes: vec![HashMap::new()],
            current: Lifetime::anonymous(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn bind(&mut self, var: &str, lifetime: Lifetime) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(var.to_string(), lifetime);
        }
    }

    pub fn get(&self, var: &str) -> Option<&Lifetime> {
        for scope in self.scopes.iter().rev() {
            if let Some(lifetime) = scope.get(var) {
                return Some(lifetime);
            }
        }
        None
    }
}

impl Default for LifetimeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_checker_basic() {
        let mut checker = BorrowChecker::new();
        checker.declare("x");
        assert!(checker.check_use("x").is_ok());
    }

    #[test]
    fn test_borrow_checker_move() {
        let mut checker = BorrowChecker::new();
        checker.declare("x");
        checker.check_assign("y", "x", false).unwrap();
        assert!(checker.check_use("x").is_err());
        assert!(checker.check_use("y").is_ok());
    }

    #[test]
    fn test_borrow_checker_copy() {
        let mut checker = BorrowChecker::new();
        checker.declare("x");
        checker.check_assign("y", "x", true).unwrap();
        assert!(checker.check_use("x").is_ok());
        assert!(checker.check_use("y").is_ok());
    }

    #[test]
    fn test_lifetime_checker() {
        let mut checker = LifetimeChecker::new();
        let lt = Lifetime::new("'a");
        checker.bind("x", lt);
        assert!(checker.get("x").is_some());
    }
}
