//! Ownership Module
//!
//! Implements AZC's ownership model inspired by Rust.
//! Core rules:
//! 1. Each value has a single owner
//! 2. When owner goes out of scope, value is dropped
//! 3. Values can be borrowed (immutable or mutable)
//! 4. Multiple immutable borrows OR one mutable borrow
//! 5. No dangling references

mod borrow;

pub use borrow::*;

use std::collections::{HashMap, HashSet};

/// Ownership information for a variable
#[derive(Debug, Clone)]
pub struct OwnershipInfo {
    /// Variable name
    pub name: String,

    /// Is this variable moved?
    pub moved: bool,

    /// Is this variable borrowed?
    pub borrowed: bool,

    /// Is the borrow mutable?
    pub mutable_borrow: bool,

    /// Current borrow count
    pub borrow_count: usize,

    /// Scope depth where this was declared
    pub scope_depth: usize,
}

impl OwnershipInfo {
    pub fn new(name: String, scope_depth: usize) -> Self {
        OwnershipInfo {
            name,
            moved: false,
            borrowed: false,
            mutable_borrow: false,
            borrow_count: 0,
            scope_depth,
        }
    }

    /// Check if this variable can be used (not moved or invalidly borrowed)
    pub fn is_valid(&self) -> bool {
        !self.moved
    }

    /// Check if this variable can be borrowed immutably
    pub fn can_borrow(&self) -> bool {
        !self.moved && !self.mutable_borrow
    }

    /// Check if this variable can be borrowed mutably
    pub fn can_borrow_mut(&self) -> bool {
        !self.moved && !self.borrowed && self.borrow_count == 0
    }
}

/// Borrow information
#[derive(Debug, Clone)]
pub struct BorrowInfo {
    /// Borrowed variable
    pub var: String,

    /// Is this a mutable borrow?
    pub mutable: bool,

    /// Scope depth where borrow was created
    pub scope_depth: usize,

    /// Is this borrow still active?
    pub active: bool,
}

/// Ownership analyzer
#[derive(Debug)]
pub struct OwnershipAnalyzer {
    /// Variable ownership information
    ownership: HashMap<String, OwnershipInfo>,

    /// Active borrows
    borrows: Vec<BorrowInfo>,

    /// Current scope depth
    scope_depth: usize,

    /// Variables that have been moved in current scope
    moved_vars: HashSet<String>,
}

impl OwnershipAnalyzer {
    pub fn new() -> Self {
        OwnershipAnalyzer {
            ownership: HashMap::new(),
            borrows: Vec::new(),
            scope_depth: 0,
            moved_vars: HashSet::new(),
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// Exit current scope and drop owned values
    pub fn exit_scope(&mut self) -> Vec<String> {
        let mut dropped = Vec::new();

        // Collect variables to drop
        let vars_to_remove: Vec<String> = self
            .ownership
            .iter()
            .filter(|(_, info)| info.scope_depth == self.scope_depth)
            .map(|(name, _)| name.clone())
            .collect();

        // Drop variables
        for var in &vars_to_remove {
            self.ownership.remove(var);
            dropped.push(var.clone());

            // Invalidate borrows of this variable
            for borrow in &mut self.borrows {
                if &borrow.var == var {
                    borrow.active = false;
                }
            }
        }

        // Remove inactive borrows from this scope
        self.borrows
            .retain(|b| b.active && b.scope_depth < self.scope_depth);

        self.scope_depth -= 1;
        dropped
    }

    /// Declare a new variable
    pub fn declare(&mut self, name: &str) {
        self.ownership.insert(
            name.to_string(),
            OwnershipInfo::new(name.to_string(), self.scope_depth),
        );
    }

    /// Check if a variable can be used
    pub fn can_use(&self, name: &str) -> Result<(), OwnershipError> {
        if let Some(info) = self.ownership.get(name) {
            if info.moved {
                return Err(OwnershipError::UseAfterMove(name.to_string()));
            }
            Ok(())
        } else {
            Err(OwnershipError::UnboundVariable(name.to_string()))
        }
    }

    /// Move a value from one variable to another
    pub fn move_value(&mut self, from: &str, to: &str) -> Result<(), OwnershipError> {
        // Check if source is valid
        self.can_use(from)?;

        // Check if source is borrowed
        if let Some(info) = self.ownership.get(from) {
            if info.borrowed {
                return Err(OwnershipError::MoveOfBorrowed(from.to_string()));
            }
        }

        // Mark source as moved
        if let Some(info) = self.ownership.get_mut(from) {
            info.moved = true;
        }

        // Create new ownership for target
        self.declare(to);

        Ok(())
    }

    /// Copy a value (for Copy types)
    pub fn copy_value(&mut self, from: &str, to: &str) -> Result<(), OwnershipError> {
        self.can_use(from)?;
        self.declare(to);
        Ok(())
    }

    /// Create an immutable borrow
    pub fn borrow(&mut self, var: &str) -> Result<(), OwnershipError> {
        self.can_use(var)?;

        if let Some(info) = self.ownership.get_mut(var) {
            if !info.can_borrow() {
                return Err(OwnershipError::CannotBorrow(var.to_string()));
            }

            info.borrowed = true;
            info.borrow_count += 1;
        }

        self.borrows.push(BorrowInfo {
            var: var.to_string(),
            mutable: false,
            scope_depth: self.scope_depth,
            active: true,
        });

        Ok(())
    }

    /// Create a mutable borrow
    pub fn borrow_mut(&mut self, var: &str) -> Result<(), OwnershipError> {
        self.can_use(var)?;

        if let Some(info) = self.ownership.get_mut(var) {
            if !info.can_borrow_mut() {
                return Err(OwnershipError::CannotBorrowMut(var.to_string()));
            }

            info.mutable_borrow = true;
            info.borrow_count += 1;
        }

        self.borrows.push(BorrowInfo {
            var: var.to_string(),
            mutable: true,
            scope_depth: self.scope_depth,
            active: true,
        });

        Ok(())
    }

    /// End a borrow
    pub fn end_borrow(&mut self, var: &str) {
        if let Some(info) = self.ownership.get_mut(var) {
            if info.borrow_count > 0 {
                info.borrow_count -= 1;
            }

            if info.borrow_count == 0 {
                info.borrowed = false;
                info.mutable_borrow = false;
            }
        }

        // Remove the borrow
        if let Some(pos) = self.borrows.iter().position(|b| &b.var == var && b.active) {
            self.borrows.remove(pos);
        }
    }

    /// Check if any borrows are still active
    pub fn check_active_borrows(&self) -> Result<(), OwnershipError> {
        for borrow in &self.borrows {
            if borrow.active {
                return Err(OwnershipError::BorrowStillActive(borrow.var.clone()));
            }
        }
        Ok(())
    }

    /// Get ownership info for a variable
    pub fn get_info(&self, name: &str) -> Option<&OwnershipInfo> {
        self.ownership.get(name)
    }

    /// Get all active borrows
    pub fn active_borrows(&self) -> Vec<&BorrowInfo> {
        self.borrows.iter().filter(|b| b.active).collect()
    }
}

impl Default for OwnershipAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Ownership errors
#[derive(Debug, Clone)]
pub enum OwnershipError {
    /// Use of moved value
    UseAfterMove(String),

    /// Move of borrowed value
    MoveOfBorrowed(String),

    /// Cannot borrow immutably
    CannotBorrow(String),

    /// Cannot borrow mutably
    CannotBorrowMut(String),

    /// Borrow still active
    BorrowStillActive(String),

    /// Mutable and immutable borrow conflict
    BorrowConflict(String),

    /// Unbound variable
    UnboundVariable(String),

    /// Dangling reference
    DanglingReference(String),
}

impl std::fmt::Display for OwnershipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnershipError::UseAfterMove(var) => {
                write!(f, "Use of moved value: '{}'", var)
            }
            OwnershipError::MoveOfBorrowed(var) => {
                write!(f, "Cannot move borrowed value: '{}'", var)
            }
            OwnershipError::CannotBorrow(var) => {
                write!(
                    f,
                    "Cannot borrow '{}' as immutable because it is already borrowed as mutable",
                    var
                )
            }
            OwnershipError::CannotBorrowMut(var) => {
                write!(
                    f,
                    "Cannot borrow '{}' as mutable because it is already borrowed",
                    var
                )
            }
            OwnershipError::BorrowStillActive(var) => {
                write!(f, "Borrow of '{}' is still active", var)
            }
            OwnershipError::BorrowConflict(var) => {
                write!(
                    f,
                    "Cannot have mutable and immutable borrows of '{}' at the same time",
                    var
                )
            }
            OwnershipError::UnboundVariable(var) => {
                write!(f, "Unbound variable: '{}'", var)
            }
            OwnershipError::DanglingReference(var) => {
                write!(f, "Dangling reference: '{}'", var)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declare() {
        let mut analyzer = OwnershipAnalyzer::new();
        analyzer.declare("x");
        assert!(analyzer.can_use("x").is_ok());
    }

    #[test]
    fn test_move() {
        let mut analyzer = OwnershipAnalyzer::new();
        analyzer.declare("x");
        analyzer.move_value("x", "y").unwrap();
        assert!(analyzer.can_use("x").is_err());
        assert!(analyzer.can_use("y").is_ok());
    }

    #[test]
    fn test_borrow() {
        let mut analyzer = OwnershipAnalyzer::new();
        analyzer.declare("x");
        analyzer.borrow("x").unwrap();
        assert!(analyzer.can_use("x").is_ok());
    }

    #[test]
    fn test_borrow_conflict() {
        let mut analyzer = OwnershipAnalyzer::new();
        analyzer.declare("x");
        analyzer.borrow_mut("x").unwrap();
        assert!(analyzer.borrow("x").is_err());
    }

    #[test]
    fn test_scope() {
        let mut analyzer = OwnershipAnalyzer::new();
        analyzer.declare("x");
        analyzer.enter_scope();
        analyzer.declare("y");
        let dropped = analyzer.exit_scope();
        assert!(dropped.contains(&"y".to_string()));
        assert!(!dropped.contains(&"x".to_string()));
    }
}
