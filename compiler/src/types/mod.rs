//! AZC Type System
//!
//! This module implements AZC's type system including:
//! - Type definitions (primitives, compounds, references)
//! - Type inference (Hindley-Milner)
//! - Type checking
//! - Type error reporting

pub mod ast;
pub mod checker;
pub mod env;
pub mod inference;

pub use ast::*;
pub use checker::*;
pub use env::*;
pub use inference::*;
