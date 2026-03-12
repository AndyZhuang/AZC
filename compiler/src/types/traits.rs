//! AZC Trait System
//!
//! Implements traits for ad-hoc polymorphism.

use std::collections::HashMap;
use std::fmt;

use super::Type;

/// Trait definition
#[derive(Debug, Clone)]
pub struct Trait {
    /// Trait name
    pub name: String,
    /// Generic parameters
    pub type_params: Vec<String>,
    /// Required methods
    pub methods: HashMap<String, TraitMethod>,
    /// Associated types
    pub associated_types: Vec<String>,
    /// Super traits (inheritance)
    pub super_traits: Vec<String>,
}

impl Trait {
    pub fn new(name: impl Into<String>) -> Self {
        Trait {
            name: name.into(),
            type_params: Vec::new(),
            methods: HashMap::new(),
            associated_types: Vec::new(),
            super_traits: Vec::new(),
        }
    }

    pub fn with_type_param(mut self, param: impl Into<String>) -> Self {
        self.type_params.push(param.into());
        self
    }

    pub fn with_method(mut self, name: impl Into<String>, method: TraitMethod) -> Self {
        self.methods.insert(name.into(), method);
        self
    }

    pub fn with_associated_type(mut self, name: impl Into<String>) -> Self {
        self.associated_types.push(name.into());
        self
    }

    pub fn with_super_trait(mut self, name: impl Into<String>) -> Self {
        self.super_traits.push(name.into());
        self
    }

    pub fn get_method(&self, name: &str) -> Option<&TraitMethod> {
        self.methods.get(name)
    }

    pub fn has_method(&self, name: &str) -> bool {
        self.methods.contains_key(name)
    }
}

impl fmt::Display for Trait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "trait {}", self.name)?;

        if !self.type_params.is_empty() {
            write!(f, "<")?;
            for (i, param) in self.type_params.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", param)?;
            }
            write!(f, ">")?;
        }

        Ok(())
    }
}

/// Method signature in a trait
#[derive(Debug, Clone)]
pub struct TraitMethod {
    /// Method name
    pub name: String,
    /// Generic parameters
    pub type_params: Vec<String>,
    /// Parameters with types
    pub params: Vec<(String, Type)>,
    /// Self parameter kind
    pub self_kind: Option<SelfKind>,
    /// Return type
    pub return_type: Type,
}

impl TraitMethod {
    pub fn new(name: impl Into<String>) -> Self {
        TraitMethod {
            name: name.into(),
            type_params: Vec::new(),
            params: Vec::new(),
            self_kind: None,
            return_type: Type::Nil,
        }
    }

    pub fn with_param(mut self, name: impl Into<String>, ty: Type) -> Self {
        self.params.push((name.into(), ty));
        self
    }

    pub fn with_self(mut self, kind: SelfKind) -> Self {
        self.self_kind = Some(kind);
        self
    }

    pub fn with_return(mut self, ty: Type) -> Self {
        self.return_type = ty;
        self
    }
}

/// Kind of self parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfKind {
    /// &self
    Ref,
    /// &mut self
    RefMut,
    /// self (owned)
    Owned,
}

/// Trait implementation
#[derive(Debug, Clone)]
pub struct TraitImpl {
    /// Trait being implemented
    pub trait_name: String,
    /// Type parameters for trait
    pub trait_args: Vec<Type>,
    /// Implementing type
    pub for_type: Type,
    /// Method implementations
    pub methods: HashMap<String, MethodImpl>,
    /// Associated type bindings
    pub associated_types: HashMap<String, Type>,
}

impl TraitImpl {
    pub fn new(trait_name: impl Into<String>, for_type: Type) -> Self {
        TraitImpl {
            trait_name: trait_name.into(),
            trait_args: Vec::new(),
            for_type,
            methods: HashMap::new(),
            associated_types: HashMap::new(),
        }
    }

    pub fn with_trait_arg(mut self, arg: Type) -> Self {
        self.trait_args.push(arg);
        self
    }

    pub fn with_method(mut self, name: impl Into<String>, method: MethodImpl) -> Self {
        self.methods.insert(name.into(), method);
        self
    }

    pub fn with_associated_type(mut self, name: impl Into<String>, ty: Type) -> Self {
        self.associated_types.insert(name.into(), ty);
        self
    }
}

/// Method implementation
#[derive(Debug, Clone)]
pub struct MethodImpl {
    /// Method name
    pub name: String,
    /// Parameter names
    pub params: Vec<String>,
    /// Method body (as code string for now)
    pub body: String,
}

impl MethodImpl {
    pub fn new(name: impl Into<String>) -> Self {
        MethodImpl {
            name: name.into(),
            params: Vec::new(),
            body: String::new(),
        }
    }

    pub fn with_param(mut self, name: impl Into<String>) -> Self {
        self.params.push(name.into());
        self
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }
}

/// Trait registry for type checking
#[derive(Debug, Default)]
pub struct TraitRegistry {
    /// All defined traits
    traits: HashMap<String, Trait>,
    /// All trait implementations
    implementations: Vec<TraitImpl>,
}

impl TraitRegistry {
    pub fn new() -> Self {
        TraitRegistry {
            traits: HashMap::new(),
            implementations: Vec::new(),
        }
    }

    /// Register a new trait
    pub fn register_trait(&mut self, tr: Trait) {
        self.traits.insert(tr.name.clone(), tr);
    }

    /// Get a trait by name
    pub fn get_trait(&self, name: &str) -> Option<&Trait> {
        self.traits.get(name)
    }

    /// Register a trait implementation
    pub fn register_impl(&mut self, impl_: TraitImpl) {
        self.implementations.push(impl_);
    }

    /// Check if a type implements a trait
    pub fn implements(&self, ty: &Type, trait_name: &str) -> bool {
        self.implementations
            .iter()
            .any(|i| i.trait_name == trait_name && i.for_type == *ty)
    }

    /// Get all implementations for a type
    pub fn get_impls_for_type(&self, ty: &Type) -> Vec<&TraitImpl> {
        self.implementations
            .iter()
            .filter(|i| i.for_type == *ty)
            .collect()
    }

    /// Get implementation of a specific trait for a type
    pub fn get_impl(&self, ty: &Type, trait_name: &str) -> Option<&TraitImpl> {
        self.implementations
            .iter()
            .find(|i| i.for_type == *ty && i.trait_name == trait_name)
    }

    /// Get method from trait implementation
    pub fn get_method(
        &self,
        ty: &Type,
        trait_name: &str,
        method_name: &str,
    ) -> Option<&MethodImpl> {
        self.get_impl(ty, trait_name)
            .and_then(|i| i.methods.get(method_name))
    }
}

/// Built-in traits
pub fn builtin_traits() -> Vec<Trait> {
    vec![
        // Clone trait
        Trait::new("Clone").with_method(
            "clone",
            TraitMethod::new("clone")
                .with_self(SelfKind::Ref)
                .with_return(Type::Generic("Self".to_string())),
        ),
        // Copy trait (marker)
        Trait::new("Copy"),
        // Debug trait
        Trait::new("Debug").with_method(
            "fmt",
            TraitMethod::new("fmt")
                .with_self(SelfKind::Ref)
                .with_return(Type::String),
        ),
        // Display trait
        Trait::new("Display").with_method(
            "to_string",
            TraitMethod::new("to_string")
                .with_self(SelfKind::Ref)
                .with_return(Type::String),
        ),
        // Eq trait
        Trait::new("Eq").with_method(
            "eq",
            TraitMethod::new("eq")
                .with_self(SelfKind::Ref)
                .with_param(
                    "other",
                    Type::Ref {
                        inner: Box::new(Type::Generic("Self".to_string())),
                        mutable: false,
                    },
                )
                .with_return(Type::Bool),
        ),
        // Ord trait
        Trait::new("Ord").with_method(
            "cmp",
            TraitMethod::new("cmp")
                .with_self(SelfKind::Ref)
                .with_param(
                    "other",
                    Type::Ref {
                        inner: Box::new(Type::Generic("Self".to_string())),
                        mutable: false,
                    },
                )
                .with_return(Type::Enum("Ordering".to_string())),
        ),
        // Hash trait
        Trait::new("Hash").with_method(
            "hash",
            TraitMethod::new("hash")
                .with_self(SelfKind::Ref)
                .with_return(Type::UInt(super::UIntSize::U64)),
        ),
        // Default trait
        Trait::new("Default").with_method(
            "default",
            TraitMethod::new("default").with_return(Type::Generic("Self".to_string())),
        ),
        // Iterator trait
        Trait::new("Iterator").with_type_param("Item").with_method(
            "next",
            TraitMethod::new("next")
                .with_self(SelfKind::RefMut)
                .with_return(Type::Option(Box::new(Type::Generic("Item".to_string())))),
        ),
        // Add trait
        Trait::new("Add")
            .with_type_param("Rhs")
            .with_associated_type("Output")
            .with_method(
                "add",
                TraitMethod::new("add")
                    .with_self(SelfKind::Ref)
                    .with_param("rhs", Type::Generic("Rhs".to_string()))
                    .with_return(Type::Generic("Output".to_string())),
            ),
        // Sub trait
        Trait::new("Sub")
            .with_type_param("Rhs")
            .with_associated_type("Output")
            .with_method(
                "sub",
                TraitMethod::new("sub")
                    .with_self(SelfKind::Ref)
                    .with_param("rhs", Type::Generic("Rhs".to_string()))
                    .with_return(Type::Generic("Output".to_string())),
            ),
        // Mul trait
        Trait::new("Mul")
            .with_type_param("Rhs")
            .with_associated_type("Output")
            .with_method(
                "mul",
                TraitMethod::new("mul")
                    .with_self(SelfKind::Ref)
                    .with_param("rhs", Type::Generic("Rhs".to_string()))
                    .with_return(Type::Generic("Output".to_string())),
            ),
        // Div trait
        Trait::new("Div")
            .with_type_param("Rhs")
            .with_associated_type("Output")
            .with_method(
                "div",
                TraitMethod::new("div")
                    .with_self(SelfKind::Ref)
                    .with_param("rhs", Type::Generic("Rhs".to_string()))
                    .with_return(Type::Generic("Output".to_string())),
            ),
        // Read trait
        Trait::new("Read").with_method(
            "read",
            TraitMethod::new("read")
                .with_self(SelfKind::RefMut)
                .with_param(
                    "buf",
                    Type::Array {
                        elem: Box::new(Type::UInt(super::UIntSize::U8)),
                        size: None,
                    },
                )
                .with_return(Type::Result {
                    ok: Box::new(Type::UInt(super::UIntSize::USize)),
                    err: Box::new(Type::String),
                }),
        ),
        // Write trait
        Trait::new("Write").with_method(
            "write",
            TraitMethod::new("write")
                .with_self(SelfKind::RefMut)
                .with_param(
                    "buf",
                    Type::Ref {
                        inner: Box::new(Type::Array {
                            elem: Box::new(Type::UInt(super::UIntSize::U8)),
                            size: None,
                        }),
                        mutable: false,
                    },
                )
                .with_return(Type::Result {
                    ok: Box::new(Type::UInt(super::UIntSize::USize)),
                    err: Box::new(Type::String),
                }),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_creation() {
        let tr = Trait::new("Display").with_method(
            "to_string",
            TraitMethod::new("to_string")
                .with_self(SelfKind::Ref)
                .with_return(Type::String),
        );

        assert_eq!(tr.name, "Display");
        assert!(tr.has_method("to_string"));
    }

    #[test]
    fn test_trait_registry() {
        let mut registry = TraitRegistry::new();

        let tr = Trait::new("Clone");
        registry.register_trait(tr);

        assert!(registry.get_trait("Clone").is_some());
    }

    #[test]
    fn test_impl_checking() {
        let mut registry = TraitRegistry::new();

        let tr = Trait::new("Display");
        registry.register_trait(tr);

        let impl_ = TraitImpl::new("Display", Type::default_int());
        registry.register_impl(impl_);

        assert!(registry.implements(&Type::default_int(), "Display"));
    }

    // ===== Comprehensive v0.3.0 Tests =====

    #[test]
    fn test_trait_multiple_methods() {
        let tr = Trait::new("Collection")
            .with_method(
                "len",
                TraitMethod::new("len")
                    .with_self(SelfKind::Ref)
                    .with_return(Type::Int(super::super::IntSize::ISize)),
            )
            .with_method(
                "is_empty",
                TraitMethod::new("is_empty")
                    .with_self(SelfKind::Ref)
                    .with_return(Type::Bool),
            );

        assert_eq!(tr.name, "Collection");
        assert!(tr.has_method("len"));
        assert!(tr.has_method("is_empty"));
        assert!(!tr.has_method("push"));
    }

    #[test]
    fn test_trait_with_params() {
        let tr = Trait::new("Iterator")
            .with_type_param("Item")
            .with_method(
                "next",
                TraitMethod::new("next")
                    .with_self(SelfKind::RefMut)
                    .with_return(Type::Generic("Item".to_string())),
            );

        assert_eq!(tr.type_params.len(), 1);
        assert_eq!(tr.type_params[0], "Item");
    }

    #[test]
    fn test_trait_method_with_params() {
        let method = TraitMethod::new("insert")
            .with_self(SelfKind::RefMut)
            .with_param("key", Type::String)
            .with_param("value", Type::default_int())
            .with_return(Type::Bool);

        assert_eq!(method.params.len(), 2);
        assert!(method.self_kind.is_some());
    }

    #[test]
    fn test_trait_impl_with_methods() {
        let impl_ = TraitImpl::new("Display", Type::String)
            .with_method(
                "to_string",
                MethodImpl::new("to_string"),
            );

        assert_eq!(impl_.trait_name, "Display");
        assert_eq!(impl_.for_type, Type::String);
        assert!(impl_.methods.contains_key("to_string"));
    }

    #[test]
    fn test_multiple_impls_same_type() {
        let mut registry = TraitRegistry::new();

        registry.register_trait(Trait::new("Clone"));
        registry.register_trait(Trait::new("Debug"));

        registry.register_impl(TraitImpl::new("Clone", Type::default_int()));
        registry.register_impl(TraitImpl::new("Debug", Type::default_int()));

        assert!(registry.implements(&Type::default_int(), "Clone"));
        assert!(registry.implements(&Type::default_int(), "Debug"));
        assert!(!registry.implements(&Type::default_int(), "Display"));
    }

    #[test]
    fn test_impl_for_different_types() {
        let mut registry = TraitRegistry::new();

        registry.register_trait(Trait::new("Clone"));

        registry.register_impl(TraitImpl::new("Clone", Type::default_int()));
        registry.register_impl(TraitImpl::new("Clone", Type::String));
        registry.register_impl(TraitImpl::new("Clone", Type::Bool));

        assert!(registry.implements(&Type::default_int(), "Clone"));
        assert!(registry.implements(&Type::String, "Clone"));
        assert!(registry.implements(&Type::Bool, "Clone"));
    }

    #[test]
    fn test_registry_trait_lookup() {
        let mut registry = TraitRegistry::new();

        let tr = Trait::new("Iterator").with_type_param("Item");
        registry.register_trait(tr);

        let found = registry.get_trait("Iterator").unwrap();
        assert_eq!(found.name, "Iterator");
        assert_eq!(found.type_params.len(), 1);
    }

    #[test]
    fn test_registry_empty() {
        let registry = TraitRegistry::new();

        assert!(registry.get_trait("NonExistent").is_none());
        assert!(!registry.implements(&Type::default_int(), "Clone"));
    }

    #[test]
    fn test_self_kind_variants() {
        // Test all SelfKind variants
        assert!(matches!(SelfKind::Owned, SelfKind::Owned));
        assert!(matches!(SelfKind::Ref, SelfKind::Ref));
        assert!(matches!(SelfKind::RefMut, SelfKind::RefMut));
    }

    #[test]
    fn test_trait_display() {
        let tr = Trait::new("Display");
        let display = format!("{}", tr);
        assert!(display.contains("Display"));
    }

    #[test]
    fn test_trait_impl_debug() {
        let impl_ = TraitImpl::new("Clone", Type::Int(super::super::IntSize::I32));
        // TraitImpl implements Debug
        let debug = format!("{:?}", impl_);
        assert!(debug.contains("Clone"));
    }
}
