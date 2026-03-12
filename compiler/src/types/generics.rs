//! AZC Generics System
//!
//! Implements generic type parameters for functions, structs, and enums.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Generic type parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParam {
    /// Parameter name (e.g., 'T', 'E')
    pub name: String,
    /// Optional trait bounds
    pub bounds: Vec<TraitBound>,
    /// Default type (for optional type parameters)
    pub default: Option<super::Type>,
}

impl TypeParam {
    pub fn new(name: impl Into<String>) -> Self {
        TypeParam {
            name: name.into(),
            bounds: Vec::new(),
            default: None,
        }
    }

    pub fn with_bound(mut self, bound: TraitBound) -> Self {
        self.bounds.push(bound);
        self
    }

    pub fn with_default(mut self, ty: super::Type) -> Self {
        self.default = Some(ty);
        self
    }
}

/// Trait bound on a type parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitBound {
    /// Trait name
    pub trait_name: String,
    /// Associated type bindings (e.g., Iterator<Item = String>)
    pub associated_types: HashMap<String, super::Type>,
}

impl TraitBound {
    pub fn new(name: impl Into<String>) -> Self {
        TraitBound {
            trait_name: name.into(),
            associated_types: HashMap::new(),
        }
    }

    pub fn with_associated_type(mut self, name: String, ty: super::Type) -> Self {
        self.associated_types.insert(name, ty);
        self
    }
}

impl fmt::Display for TraitBound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.trait_name)?;
        if !self.associated_types.is_empty() {
            write!(f, "<")?;
            for (i, (name, ty)) in self.associated_types.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} = {}", name, ty)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// Generic type parameters for a declaration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParams {
    /// Type parameters in declaration order
    pub params: Vec<TypeParam>,
}

impl GenericParams {
    pub fn new() -> Self {
        GenericParams { params: Vec::new() }
    }

    pub fn from_names(names: &[&str]) -> Self {
        GenericParams {
            params: names.iter().map(|n| TypeParam::new(*n)).collect(),
        }
    }

    pub fn add(&mut self, param: TypeParam) {
        self.params.push(param);
    }

    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    pub fn names(&self) -> Vec<&str> {
        self.params.iter().map(|p| p.name.as_str()).collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.params.iter().any(|p| p.name == name)
    }

    pub fn get(&self, name: &str) -> Option<&TypeParam> {
        self.params.iter().find(|p| p.name == name)
    }
}

impl Default for GenericParams {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GenericParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.params.is_empty() {
            return Ok(());
        }
        write!(f, "<")?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param.name)?;
            if !param.bounds.is_empty() {
                write!(f, ": ")?;
                for (j, bound) in param.bounds.iter().enumerate() {
                    if j > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{}", bound)?;
                }
            }
        }
        write!(f, ">")
    }
}

/// Generic type arguments (instantiation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericArgs {
    /// Type arguments
    pub args: Vec<super::Type>,
}

impl GenericArgs {
    pub fn new() -> Self {
        GenericArgs { args: Vec::new() }
    }

    pub fn from_types(types: Vec<super::Type>) -> Self {
        GenericArgs { args: types }
    }

    pub fn add(&mut self, ty: super::Type) {
        self.args.push(ty);
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }
}

impl Default for GenericArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GenericArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            return Ok(());
        }
        write!(f, "<")?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ">")
    }
}

/// Type substitution for generic instantiation
#[derive(Debug, Clone)]
pub struct TypeSubst {
    /// Mapping from type parameter names to concrete types
    mapping: HashMap<String, super::Type>,
}

impl TypeSubst {
    pub fn new() -> Self {
        TypeSubst {
            mapping: HashMap::new(),
        }
    }

    pub fn from_params(params: &GenericParams, args: &GenericArgs) -> Result<Self, String> {
        if params.len() != args.len() {
            return Err(format!(
                "Expected {} type arguments, got {}",
                params.len(),
                args.len()
            ));
        }

        let mut subst = TypeSubst::new();
        for (param, arg) in params.params.iter().zip(args.args.iter()) {
            subst.insert(&param.name, arg.clone());
        }

        Ok(subst)
    }

    pub fn insert(&mut self, name: &str, ty: super::Type) {
        self.mapping.insert(name.to_string(), ty);
    }

    pub fn apply(&self, ty: &super::Type) -> super::Type {
        use super::Type;

        match ty {
            Type::Generic(name) => self
                .mapping
                .get(name)
                .cloned()
                .unwrap_or_else(|| ty.clone()),
            Type::Array { elem, size } => Type::Array {
                elem: Box::new(self.apply(elem)),
                size: *size,
            },
            Type::Tuple(elements) => Type::Tuple(elements.iter().map(|e| self.apply(e)).collect()),
            Type::Ref { inner, mutable } => Type::Ref {
                inner: Box::new(self.apply(inner)),
                mutable: *mutable,
            },
            Type::Box { inner } => Type::Box {
                inner: Box::new(self.apply(inner)),
            },
            Type::Rc { inner } => Type::Rc {
                inner: Box::new(self.apply(inner)),
            },
            Type::Function { params, ret } => Type::Function {
                params: params.iter().map(|p| self.apply(p)).collect(),
                ret: Box::new(self.apply(ret)),
            },
            Type::Option(inner) => Type::Option(Box::new(self.apply(inner))),
            Type::Result { ok, err } => Type::Result {
                ok: Box::new(self.apply(ok)),
                err: Box::new(self.apply(err)),
            },
            _ => ty.clone(),
        }
    }
}

impl Default for TypeSubst {
    fn default() -> Self {
        Self::new()
    }
}

/// Monomorphization context
#[derive(Debug)]
pub struct Monomorphizer {
    /// Track instantiated types to avoid duplicates
    instantiated: HashSet<String>,
}

impl Monomorphizer {
    pub fn new() -> Self {
        Monomorphizer {
            instantiated: HashSet::new(),
        }
    }

    /// Generate monomorphized name for a generic type
    pub fn mangle_name(base: &str, args: &GenericArgs) -> String {
        if args.is_empty() {
            return base.to_string();
        }

        let mut name = base.to_string();
        for arg in &args.args {
            name.push('_');
            name.push_str(&format!("{}", arg));
        }
        name
    }

    /// Check if a type instantiation already exists
    pub fn has_instantiation(&self, name: &str) -> bool {
        self.instantiated.contains(name)
    }

    /// Register a new instantiation
    pub fn register(&mut self, name: String) {
        self.instantiated.insert(name);
    }
}

impl Default for Monomorphizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Type;
    use super::*;

    #[test]
    fn test_type_param() {
        let param = TypeParam::new("T");
        assert_eq!(param.name, "T");
        assert!(param.bounds.is_empty());
    }

    #[test]
    fn test_generic_params() {
        let params = GenericParams::from_names(&["T", "E"]);
        assert_eq!(params.len(), 2);
        assert!(params.contains("T"));
        assert!(params.contains("E"));
    }

    #[test]
    fn test_type_subst() {
        let params = GenericParams::from_names(&["T"]);
        let args = GenericArgs::from_types(vec![Type::default_int()]);

        let subst = TypeSubst::from_params(&params, &args).unwrap();

        let generic = Type::Generic("T".to_string());
        let concrete = subst.apply(&generic);

        assert_eq!(concrete, Type::default_int());
    }

    #[test]
    fn test_mangle_name() {
        let args = GenericArgs::from_types(vec![Type::default_int()]);
        let name = Monomorphizer::mangle_name("Vec", &args);
        assert!(name.contains("Vec"));
        assert!(name.contains("i32"));
    }

    // ===== Comprehensive v0.3.0 Tests =====

    #[test]
    fn test_type_param_with_bounds() {
        let param = TypeParam::new("T")
            .with_bound(TraitBound::new("Clone"))
            .with_bound(TraitBound::new("Debug"));
        
        assert_eq!(param.name, "T");
        assert_eq!(param.bounds.len(), 2);
        assert_eq!(param.bounds[0].trait_name, "Clone");
        assert_eq!(param.bounds[1].trait_name, "Debug");
    }

    #[test]
    fn test_type_param_with_default() {
        let param = TypeParam::new("T")
            .with_default(Type::default_int());
        
        assert_eq!(param.name, "T");
        assert!(param.default.is_some());
    }

    #[test]
    fn test_generic_params_multiple() {
        let params = GenericParams::from_names(&["T", "U", "V", "W"]);
        assert_eq!(params.len(), 4);
        assert!(params.contains("T"));
        assert!(params.contains("U"));
        assert!(params.contains("V"));
        assert!(params.contains("W"));
        assert!(!params.contains("X"));
    }

    #[test]
    fn test_generic_params_empty() {
        let params = GenericParams::new();
        assert_eq!(params.len(), 0);
        assert!(!params.contains("T"));
    }

    #[test]
    fn test_generic_args_from_types() {
        let args = GenericArgs::from_types(vec![
            Type::Int(super::super::IntSize::I32),
            Type::Float(super::super::FloatSize::F64),
        ]);
        
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn test_type_subst_multiple() {
        let params = GenericParams::from_names(&["T", "U"]);
        let args = GenericArgs::from_types(vec![
            Type::Int(super::super::IntSize::I32),
            Type::Bool,
        ]);

        let subst = TypeSubst::from_params(&params, &args).unwrap();

        let generic_t = Type::Generic("T".to_string());
        let generic_u = Type::Generic("U".to_string());
        
        assert_eq!(subst.apply(&generic_t), Type::Int(super::super::IntSize::I32));
        assert_eq!(subst.apply(&generic_u), Type::Bool);
    }

    #[test]
    fn test_type_subst_no_match() {
        let params = GenericParams::from_names(&["T"]);
        let args = GenericArgs::from_types(vec![Type::Bool]);

        let subst = TypeSubst::from_params(&params, &args).unwrap();

        // Applying to a non-generic type should return it unchanged
        let concrete = Type::Int(super::super::IntSize::I64);
        assert_eq!(subst.apply(&concrete), concrete);
    }

    #[test]
    fn test_type_subst_chain() {
        let mut subst = TypeSubst::new();
        subst.insert("T", Type::Int(super::super::IntSize::I32));
        subst.insert("U", Type::String);

        let generic_t = Type::Generic("T".to_string());
        assert_eq!(subst.apply(&generic_t), Type::Int(super::super::IntSize::I32));
    }

    #[test]
    fn test_monomorphizer_basic() {
        let mut mono = Monomorphizer::new();
        
        let args = GenericArgs::from_types(vec![Type::Int(super::super::IntSize::I32)]);
        
        let name = Monomorphizer::mangle_name("Vec", &args);
        assert!(name.starts_with("Vec"));
        mono.register(name.clone());
        assert!(mono.has_instantiation(&name));
    }

    #[test]
    fn test_monomorphizer_multiple_instantiations() {
        let mut mono = Monomorphizer::new();
        
        // Vec<i32>
        let args1 = GenericArgs::from_types(vec![Type::Int(super::super::IntSize::I32)]);
        let name1 = Monomorphizer::mangle_name("Vec", &args1);
        mono.register(name1.clone());
        
        // Vec<f64>
        let args2 = GenericArgs::from_types(vec![Type::Float(super::super::FloatSize::F64)]);
        let name2 = Monomorphizer::mangle_name("Vec", &args2);
        mono.register(name2.clone());
        
        assert_ne!(name1, name2);
        assert!(mono.has_instantiation(&name1));
        assert!(mono.has_instantiation(&name2));
    }

    #[test]
    fn test_mangle_name_multiple_args() {
        let args = GenericArgs::from_types(vec![
            Type::Int(super::super::IntSize::I32),
            Type::Bool,
            Type::String,
        ]);
        let name = Monomorphizer::mangle_name("HashMap", &args);
        assert!(name.starts_with("HashMap"));
        assert!(name.contains("i32"));
    }

    #[test]
    fn test_mangle_name_empty_args() {
        let args = GenericArgs::new();
        let name = Monomorphizer::mangle_name("Simple", &args);
        assert_eq!(name, "Simple");
    }

    #[test]
    fn test_trait_bound() {
        let bound = TraitBound::new("Clone")
            .with_associated_type("Item".to_string(), Type::default_int());
        
        assert_eq!(bound.trait_name, "Clone");
        assert!(bound.associated_types.contains_key("Item"));
    }

    #[test]
    fn test_instantiation_tracking() {
        let mut mono = Monomorphizer::new();
        
        assert!(!mono.has_instantiation("Vec_i32"));
        mono.register("Vec_i32".to_string());
        assert!(mono.has_instantiation("Vec_i32"));
    }
}
