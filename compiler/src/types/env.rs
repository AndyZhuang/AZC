//! Type Environment
//!
//! Manages type bindings and scope for type inference.

use super::ast::{Type, TypeId, UIntSize};
use std::collections::HashMap;

/// Type scheme for polymorphic types
#[derive(Debug, Clone)]
pub struct TypeScheme {
    /// Bound type variables
    pub vars: Vec<TypeId>,
    /// The actual type
    pub ty: Type,
}

impl TypeScheme {
    /// Create a monomorphic type scheme
    pub fn mono(ty: Type) -> Self {
        TypeScheme {
            vars: Vec::new(),
            ty,
        }
    }

    /// Create a polymorphic type scheme
    pub fn poly(vars: Vec<TypeId>, ty: Type) -> Self {
        TypeScheme { vars, ty }
    }
}

/// Type environment
#[derive(Debug, Clone)]
pub struct TypeEnv {
    /// Variable bindings
    bindings: HashMap<String, TypeScheme>,

    /// Parent scope (for nested scopes)
    parent: Option<Box<TypeEnv>>,

    /// Type variable counter
    var_counter: TypeId,

    /// Type definitions (structs, enums, type aliases)
    type_defs: HashMap<String, Type>,
}

impl TypeEnv {
    /// Create a new empty type environment
    pub fn new() -> Self {
        TypeEnv {
            bindings: HashMap::new(),
            parent: None,
            var_counter: 0,
            type_defs: HashMap::new(),
        }
    }

    /// Create a child scope
    pub fn child(&self) -> Self {
        TypeEnv {
            bindings: HashMap::new(),
            parent: Some(Box::new(self.clone())),
            var_counter: self.var_counter,
            type_defs: self.type_defs.clone(),
        }
    }

    /// Generate a fresh type variable
    pub fn fresh_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::var(id)
    }

    /// Insert a binding
    pub fn insert(&mut self, name: String, scheme: TypeScheme) {
        self.bindings.insert(name, scheme);
    }

    /// Insert a simple type binding
    pub fn insert_type(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, TypeScheme::mono(ty));
    }

    /// Look up a binding
    pub fn lookup(&self, name: &str) -> Option<&TypeScheme> {
        if let Some(scheme) = self.bindings.get(name) {
            Some(scheme)
        } else if let Some(ref parent) = self.parent {
            parent.lookup(name)
        } else {
            None
        }
    }

    /// Check if a name is bound
    pub fn contains(&self, name: &str) -> bool {
        self.bindings.contains_key(name) || self.parent.as_ref().map_or(false, |p| p.contains(name))
    }

    /// Insert a type definition
    pub fn insert_type_def(&mut self, name: String, ty: Type) {
        self.type_defs.insert(name, ty);
    }

    /// Look up a type definition
    pub fn lookup_type_def(&self, name: &str) -> Option<&Type> {
        self.type_defs
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|p| p.lookup_type_def(name)))
    }

    /// Get all bound names
    pub fn bound_names(&self) -> Vec<&String> {
        let mut names: Vec<&String> = self.bindings.keys().collect();
        if let Some(ref parent) = self.parent {
            names.extend(parent.bound_names());
        }
        names
    }

    /// Create environment with built-in types
    pub fn with_builtins() -> Self {
        let mut env = Self::new();

        // Built-in functions
        env.insert_type(
            "puts".to_string(),
            Type::Function {
                params: vec![Type::String],
                ret: Box::new(Type::Nil),
            },
        );

        env.insert_type(
            "print".to_string(),
            Type::Function {
                params: vec![Type::String],
                ret: Box::new(Type::Nil),
            },
        );

        // Type conversions
        env.insert_type(
            "to_i".to_string(),
            Type::Function {
                params: vec![Type::String],
                ret: Box::new(Type::default_int()),
            },
        );

        env.insert_type(
            "to_f".to_string(),
            Type::Function {
                params: vec![Type::String],
                ret: Box::new(Type::default_float()),
            },
        );

        // Array operations
        let elem_ty = env.fresh_var();
        env.insert(
            "len".to_string(),
            TypeScheme::poly(
                vec![0],
                Type::Function {
                    params: vec![Type::Array {
                        elem: Box::new(elem_ty.clone()),
                        size: None,
                    }],
                    ret: Box::new(Type::UInt(UIntSize::USize)),
                },
            ),
        );

        env
    }

    /// Get current type variable counter
    pub fn var_count(&self) -> TypeId {
        self.var_counter
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::IntSize;


    #[test]
    fn test_env_basic() {
        let mut env = TypeEnv::new();
        env.insert_type("x".to_string(), Type::Int(IntSize::I32));

        assert!(env.contains("x"));
        assert!(!env.contains("y"));
    }

    #[test]
    fn test_env_scope() {
        let mut parent = TypeEnv::new();
        parent.insert_type("x".to_string(), Type::Int(IntSize::I32));

        let child = parent.child();
        assert!(child.contains("x"));
    }

    #[test]
    fn test_fresh_var() {
        let mut env = TypeEnv::new();
        let v1 = env.fresh_var();
        let v2 = env.fresh_var();

        assert_ne!(v1, v2);
    }
}
