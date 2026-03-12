//! AZC Type System
//!
//! This module implements AZC's type system including:
//! - Type definitions (primitives, compounds, references)
//! - Type inference (Hindley-Milner)
//! - Type checking
//! - Type error reporting

pub mod ast;
pub mod async_;
pub mod checker;
pub mod env;
pub mod generics;
pub mod inference;
pub mod patterns;
pub mod traits;

pub use ast::*;
pub use async_::*;
pub use checker::*;
pub use env::*;
pub use generics::*;
pub use inference::*;
pub use patterns::*;
pub use traits::*;
