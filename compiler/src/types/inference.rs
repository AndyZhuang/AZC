//! Type Inference
//!
//! Implements Hindley-Milner type inference with extensions for AZC.

use super::ast::{FloatSize, IntSize, Type, TypeId};
use super::env::{TypeEnv, TypeScheme};
use std::collections::HashMap;

/// Type inference context
#[derive(Debug)]
pub struct TypeInference {
    /// Type variable substitutions
    substitutions: HashMap<TypeId, Type>,

    /// Counter for generating fresh type variables
    counter: TypeId,
}

impl TypeInference {
    /// Create a new inference context
    pub fn new() -> Self {
        TypeInference {
            substitutions: HashMap::new(),
            counter: 0,
        }
    }

    /// Generate a fresh type variable
    pub fn fresh(&mut self) -> Type {
        let id = self.counter;
        self.counter += 1;
        Type::var(id)
    }

    /// Apply current substitutions to a type
    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(id) => {
                if let Some(substituted) = self.substitutions.get(id) {
                    self.apply(substituted)
                } else {
                    ty.clone()
                }
            }
            Type::Array { elem, size } => Type::Array {
                elem: Box::new(self.apply(elem)),
                size: *size,
            },
            Type::Tuple(elements) => Type::Tuple(elements.iter().map(|t| self.apply(t)).collect()),
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
                params: params.iter().map(|t| self.apply(t)).collect(),
                ret: Box::new(self.apply(ret)),
            },
            Type::Option(inner) => Type::Option(Box::new(self.apply(inner))),
            Type::Result { ok, err } => Type::Result {
                ok: Box::new(self.apply(ok)),
                err: Box::new(self.apply(err)),
            },
            Type::Alias(name, inner) => Type::Alias(name.clone(), Box::new(self.apply(inner))),
            _ => ty.clone(),
        }
    }

    /// Unify two types
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), TypeError> {
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        match (&t1, &t2) {
            // Same type - done
            (Type::Int(s1), Type::Int(s2)) if s1 == s2 => Ok(()),
            (Type::UInt(s1), Type::UInt(s2)) if s1 == s2 => Ok(()),
            (Type::Float(s1), Type::Float(s2)) if s1 == s2 => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Char, Type::Char) => Ok(()),
            (Type::String, Type::String) => Ok(()),
            (Type::Nil, Type::Nil) => Ok(()),
            (Type::Never, Type::Never) => Ok(()),

            // Type variable - bind
            (Type::Var(id), other) | (other, Type::Var(id)) => {
                // Occurs check - prevent infinite types
                if self.occurs(*id, other) {
                    return Err(TypeError::InfiniteType(*id, other.clone()));
                }
                self.substitutions.insert(*id, other.clone());
                Ok(())
            }

            // Numeric coercion (int <-> float)
            (Type::Int(_), Type::Float(_)) | (Type::Float(_), Type::Int(_)) => {
                // Promote to float
                Ok(())
            }

            // Different integer sizes - use larger
            (Type::Int(s1), Type::Int(s2)) => {
                let result_size = match (s1, s2) {
                    (IntSize::I128, _) | (_, IntSize::I128) => IntSize::I128,
                    (IntSize::I64, _) | (_, IntSize::I64) => IntSize::I64,
                    (IntSize::I32, _) | (_, IntSize::I32) => IntSize::I32,
                    (IntSize::I16, _) | (_, IntSize::I16) => IntSize::I16,
                    (IntSize::I8, _) | (_, IntSize::I8) => IntSize::I8,
                    (IntSize::ISize, _) | (_, IntSize::ISize) => IntSize::ISize,
                };
                // We could update the type, but for now just accept
                Ok(())
            }

            // Arrays
            (Type::Array { elem: e1, size: s1 }, Type::Array { elem: e2, size: s2 }) => {
                if s1 != s2 {
                    return Err(TypeError::Mismatch(t1, t2));
                }
                self.unify(e1, e2)
            }

            // Tuples
            (Type::Tuple(e1), Type::Tuple(e2)) => {
                if e1.len() != e2.len() {
                    return Err(TypeError::Mismatch(t1, t2));
                }
                for (a, b) in e1.iter().zip(e2.iter()) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // References
            (
                Type::Ref {
                    inner: i1,
                    mutable: m1,
                },
                Type::Ref {
                    inner: i2,
                    mutable: m2,
                },
            ) => {
                if m1 != m2 {
                    return Err(TypeError::MutabilityMismatch);
                }
                self.unify(i1, i2)
            }

            // Functions
            (
                Type::Function {
                    params: p1,
                    ret: r1,
                },
                Type::Function {
                    params: p2,
                    ret: r2,
                },
            ) => {
                if p1.len() != p2.len() {
                    return Err(TypeError::ArgCountMismatch(p1.len(), p2.len()));
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b)?;
                }
                self.unify(r1, r2)
            }

            // Option
            (Type::Option(i1), Type::Option(i2)) => self.unify(i1, i2),

            // Result
            (Type::Result { ok: o1, err: e1 }, Type::Result { ok: o2, err: e2 }) => {
                self.unify(o1, o2)?;
                self.unify(e1, e2)
            }

            // Type mismatch
            _ => Err(TypeError::Mismatch(t1, t2)),
        }
    }

    /// Check if a type variable occurs in a type (prevents infinite types)
    fn occurs(&self, id: TypeId, ty: &Type) -> bool {
        match ty {
            Type::Var(vid) => {
                if *vid == id {
                    return true;
                }
                if let Some(substituted) = self.substitutions.get(vid) {
                    self.occurs(id, substituted)
                } else {
                    false
                }
            }
            Type::Array { elem, .. } => self.occurs(id, elem),
            Type::Tuple(elements) => elements.iter().any(|t| self.occurs(id, t)),
            Type::Ref { inner, .. } => self.occurs(id, inner),
            Type::Box { inner } => self.occurs(id, inner),
            Type::Rc { inner } => self.occurs(id, inner),
            Type::Function { params, ret } => {
                params.iter().any(|t| self.occurs(id, t)) || self.occurs(id, ret)
            }
            Type::Option(inner) => self.occurs(id, inner),
            Type::Result { ok, err } => self.occurs(id, ok) || self.occurs(id, err),
            Type::Alias(_, inner) => self.occurs(id, inner),
            _ => false,
        }
    }

    /// Infer type of a literal
    pub fn infer_literal(&mut self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::default_int(),
            Literal::Float(_) => Type::default_float(),
            Literal::Bool(_) => Type::Bool,
            Literal::Char(_) => Type::Char,
            Literal::String(_) => Type::String,
            Literal::Nil => Type::Nil,
        }
    }

    /// Infer type of a binary expression
    pub fn infer_binary(
        &mut self,
        op: &BinOp,
        left: &Type,
        right: &Type,
    ) -> Result<Type, TypeError> {
        // Unify operand types
        self.unify(left, right)?;

        match op {
            // Arithmetic operators
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                let result = self.apply(left);
                if result.is_numeric() {
                    Ok(result)
                } else {
                    Err(TypeError::NonNumericOp(result, op.clone()))
                }
            }

            // Comparison operators
            BinOp::Eq | BinOp::Ne => Ok(Type::Bool),

            BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                let result = self.apply(left);
                if result.is_numeric() {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError::NonNumericOp(result, op.clone()))
                }
            }

            // Logical operators
            BinOp::And | BinOp::Or => {
                self.unify(left, &Type::Bool)?;
                self.unify(right, &Type::Bool)?;
                Ok(Type::Bool)
            }
        }
    }

    /// Infer type of a unary expression
    pub fn infer_unary(&mut self, op: &UnaryOp, operand: &Type) -> Result<Type, TypeError> {
        match op {
            UnaryOp::Neg => {
                if operand.is_numeric() {
                    Ok(operand.clone())
                } else {
                    Err(TypeError::NonNumericOp(operand.clone(), BinOp::Sub))
                }
            }
            UnaryOp::Not => {
                self.unify(operand, &Type::Bool)?;
                Ok(Type::Bool)
            }
            UnaryOp::Deref => match operand {
                Type::Ref { inner, .. } | Type::Box { inner } | Type::Rc { inner } => {
                    Ok((**inner).clone())
                }
                _ => Err(TypeError::NotDereferencable(operand.clone())),
            },
            UnaryOp::Ref => Ok(Type::Ref {
                inner: Box::new(operand.clone()),
                mutable: false,
            }),
            UnaryOp::RefMut => Ok(Type::Ref {
                inner: Box::new(operand.clone()),
                mutable: true,
            }),
        }
    }

    /// Get all substitutions
    pub fn substitutions(&self) -> &HashMap<TypeId, Type> {
        &self.substitutions
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Nil,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    Ref,
    RefMut,
}

/// Type errors
#[derive(Debug, Clone)]
pub enum TypeError {
    /// Type mismatch
    Mismatch(Type, Type),

    /// Unbound variable
    UnboundVariable(String),

    /// Infinite type (occurs check failed)
    InfiniteType(TypeId, Type),

    /// Argument count mismatch
    ArgCountMismatch(usize, usize),

    /// Non-numeric operation
    NonNumericOp(Type, BinOp),

    /// Mutability mismatch
    MutabilityMismatch,

    /// Not dereferencable
    NotDereferencable(Type),

    /// Not callable
    NotCallable(Type),

    /// Cannot infer type
    CannotInfer(String),
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::Mismatch(t1, t2) => {
                write!(f, "Type mismatch: expected '{}', found '{}'", t1, t2)
            }
            TypeError::UnboundVariable(name) => {
                write!(f, "Unbound variable: {}", name)
            }
            TypeError::InfiniteType(id, ty) => {
                write!(f, "Infinite type: _{} occurs in {}", id, ty)
            }
            TypeError::ArgCountMismatch(expected, found) => {
                write!(
                    f,
                    "Argument count mismatch: expected {}, found {}",
                    expected, found
                )
            }
            TypeError::NonNumericOp(ty, op) => {
                write!(f, "Cannot apply '{}' to non-numeric type '{}'", op, ty)
            }
            TypeError::MutabilityMismatch => {
                write!(f, "Mutability mismatch: cannot borrow as mutable")
            }
            TypeError::NotDereferencable(ty) => {
                write!(f, "Cannot dereference type '{}'", ty)
            }
            TypeError::NotCallable(ty) => {
                write!(f, "Cannot call type '{}'", ty)
            }
            TypeError::CannotInfer(context) => {
                write!(f, "Cannot infer type: {}", context)
            }
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Mod => write!(f, "%"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Ne => write!(f, "!="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Le => write!(f, "<="),
            BinOp::Gt => write!(f, ">"),
            BinOp::Ge => write!(f, ">="),
            BinOp::And => write!(f, "and"),
            BinOp::Or => write!(f, "or"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_var() {
        let mut inf = TypeInference::new();
        let v1 = inf.fresh();
        let v2 = inf.fresh();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_unify_int() {
        let mut inf = TypeInference::new();
        let t1 = Type::Int(IntSize::I32);
        let t2 = Type::Int(IntSize::I32);
        assert!(inf.unify(&t1, &t2).is_ok());
    }

    #[test]
    fn test_unify_var() {
        let mut inf = TypeInference::new();
        let v = inf.fresh();
        let t = Type::Int(IntSize::I32);
        assert!(inf.unify(&v, &t).is_ok());
        assert_eq!(inf.apply(&v), t);
    }

    #[test]
    fn test_literal_inference() {
        let mut inf = TypeInference::new();
        assert_eq!(inf.infer_literal(&Literal::Int(42)), Type::default_int());
        assert_eq!(
            inf.infer_literal(&Literal::Float(3.14)),
            Type::default_float()
        );
        assert_eq!(inf.infer_literal(&Literal::Bool(true)), Type::Bool);
    }

    #[test]
    fn test_binary_inference() {
        let mut inf = TypeInference::new();
        let result = inf.infer_binary(
            &BinOp::Add,
            &Type::Int(IntSize::I32),
            &Type::Int(IntSize::I32),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Type::Int(IntSize::I32));
    }
}
