//! AZC Type AST Definitions
//!
//! Defines all types in AZC including primitives, compounds, and references.

use std::fmt;

/// Unique identifier for type variables
pub type TypeId = u32;

/// AZC Types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    Int(IntSize),
    UInt(UIntSize),
    Float(FloatSize),
    Bool,
    Char,
    String,
    Nil,

    // Never type (for functions that don't return)
    Never,

    // Type variable (for inference)
    Var(TypeId),

    // Compound types
    Array {
        elem: Box<Type>,
        size: Option<usize>, // None for dynamic arrays
    },
    Tuple(Vec<Type>),

    // Reference types
    Ref {
        inner: Box<Type>,
        mutable: bool,
    },
    Box {
        inner: Box<Type>,
    },
    Rc {
        inner: Box<Type>,
    },

    // Function type
    Function {
        params: Vec<Type>,
        ret: Box<Type>,
    },

    // User-defined types (to be implemented)
    Struct(String),
    Enum(String),
    Alias(String, Box<Type>),

    // Generic type parameter
    Generic(String),

    // Special types
    Option(Box<Type>),
    Result {
        ok: Box<Type>,
        err: Box<Type>,
    },
}

/// Integer sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
}

/// Unsigned integer sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UIntSize {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

/// Float sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatSize {
    F32,
    F64,
}

impl Type {
    /// Create a new type variable
    pub fn var(id: TypeId) -> Self {
        Type::Var(id)
    }

    /// Check if this is a primitive type
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Int(_)
                | Type::UInt(_)
                | Type::Float(_)
                | Type::Bool
                | Type::Char
                | Type::String
                | Type::Nil
        )
    }

    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int(_) | Type::UInt(_) | Type::Float(_))
    }

    /// Check if this is a reference type
    pub fn is_reference(&self) -> bool {
        matches!(self, Type::Ref { .. } | Type::Box { .. } | Type::Rc { .. })
    }

    /// Get the default type for integer literals
    pub fn default_int() -> Self {
        Type::Int(IntSize::I32)
    }

    /// Get the default type for float literals
    pub fn default_float() -> Self {
        Type::Float(FloatSize::F64)
    }

    /// Check if type is Copy (can be implicitly copied)
    pub fn is_copy(&self) -> bool {
        match self {
            // All primitives are Copy
            Type::Int(_) | Type::UInt(_) | Type::Float(_) | Type::Bool | Type::Char => true,

            // Tuples are Copy if all elements are Copy
            Type::Tuple(elements) => elements.iter().all(|t| t.is_copy()),

            // Arrays are Copy if element is Copy and size is known
            Type::Array { elem, size } => size.is_some() && elem.is_copy(),

            // References are Copy
            Type::Ref { .. } => true,

            // Everything else is not Copy by default
            _ => false,
        }
    }

    /// Get the size of this type in bytes (approximate)
    pub fn size(&self) -> usize {
        match self {
            Type::Int(size) => match size {
                IntSize::I8 => 1,
                IntSize::I16 => 2,
                IntSize::I32 => 4,
                IntSize::I64 => 8,
                IntSize::I128 => 16,
                IntSize::ISize => 8, // Assume 64-bit
            },
            Type::UInt(size) => match size {
                UIntSize::U8 => 1,
                UIntSize::U16 => 2,
                UIntSize::U32 => 4,
                UIntSize::U64 => 8,
                UIntSize::U128 => 16,
                UIntSize::USize => 8,
            },
            Type::Float(size) => match size {
                FloatSize::F32 => 4,
                FloatSize::F64 => 8,
            },
            Type::Bool => 1,
            Type::Char => 4,       // UTF-32
            Type::String => 24,    // Fat pointer (ptr + len + cap)
            Type::Ref { .. } => 8, // Pointer
            Type::Box { .. } => 8,
            Type::Rc { .. } => 8,
            Type::Array { elem, size } => elem.size() * size.unwrap_or(0),
            Type::Tuple(elements) => elements.iter().map(|t| t.size()).sum(),
            Type::Nil => 0,
            Type::Never => 0,
            Type::Var(_) => 8,          // Unknown, assume pointer size
            Type::Function { .. } => 8, // Function pointer
            Type::Struct(_) => 0,       // Unknown without definition
            Type::Enum(_) => 0,
            Type::Alias(_, inner) => inner.size(),
            Type::Generic(_) => 8,
            Type::Option(inner) => inner.size() + 1, // Tag byte
            Type::Result { ok, err } => std::cmp::max(ok.size(), err.size()) + 1,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int(size) => write!(
                f,
                "{}",
                match size {
                    IntSize::I8 => "i8",
                    IntSize::I16 => "i16",
                    IntSize::I32 => "i32",
                    IntSize::I64 => "i64",
                    IntSize::I128 => "i128",
                    IntSize::ISize => "isize",
                }
            ),
            Type::UInt(size) => write!(
                f,
                "{}",
                match size {
                    UIntSize::U8 => "u8",
                    UIntSize::U16 => "u16",
                    UIntSize::U32 => "u32",
                    UIntSize::U64 => "u64",
                    UIntSize::U128 => "u128",
                    UIntSize::USize => "usize",
                }
            ),
            Type::Float(size) => write!(
                f,
                "{}",
                match size {
                    FloatSize::F32 => "f32",
                    FloatSize::F64 => "f64",
                }
            ),
            Type::Bool => write!(f, "Bool"),
            Type::Char => write!(f, "Char"),
            Type::String => write!(f, "String"),
            Type::Nil => write!(f, "Nil"),
            Type::Never => write!(f, "Never"),
            Type::Var(id) => write!(f, "_{}", id),
            Type::Array { elem, size } => {
                if let Some(s) = size {
                    write!(f, "[{}; {}]", elem, s)
                } else {
                    write!(f, "[{}]", elem)
                }
            }
            Type::Tuple(elements) => {
                write!(f, "(")?;
                for (i, t) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Ref { inner, mutable } => {
                if *mutable {
                    write!(f, "&mut {}", inner)
                } else {
                    write!(f, "&{}", inner)
                }
            }
            Type::Box { inner } => write!(f, "Box<{}>", inner),
            Type::Rc { inner } => write!(f, "Rc<{}>", inner),
            Type::Function { params, ret } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Struct(name) => write!(f, "{}", name),
            Type::Enum(name) => write!(f, "{}", name),
            Type::Alias(name, _) => write!(f, "{}", name),
            Type::Generic(name) => write!(f, "{}", name),
            Type::Option(inner) => write!(f, "Option<{}>", inner),
            Type::Result { ok, err } => write!(f, "Result<{}, {}>", ok, err),
        }
    }
}

impl fmt::Display for IntSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntSize::I8 => write!(f, "i8"),
            IntSize::I16 => write!(f, "i16"),
            IntSize::I32 => write!(f, "i32"),
            IntSize::I64 => write!(f, "i64"),
            IntSize::I128 => write!(f, "i128"),
            IntSize::ISize => write!(f, "isize"),
        }
    }
}

impl fmt::Display for UIntSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UIntSize::U8 => write!(f, "u8"),
            UIntSize::U16 => write!(f, "u16"),
            UIntSize::U32 => write!(f, "u32"),
            UIntSize::U64 => write!(f, "u64"),
            UIntSize::U128 => write!(f, "u128"),
            UIntSize::USize => write!(f, "usize"),
        }
    }
}

impl fmt::Display for FloatSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FloatSize::F32 => write!(f, "f32"),
            FloatSize::F64 => write!(f, "f64"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Int(IntSize::I32).to_string(), "i32");
        assert_eq!(Type::Bool.to_string(), "Bool");
        assert_eq!(Type::String.to_string(), "String");
    }

    #[test]
    fn test_is_primitive() {
        assert!(Type::Int(IntSize::I32).is_primitive());
        assert!(Type::Bool.is_primitive());
        assert!(!Type::var(0).is_primitive());
    }

    #[test]
    fn test_is_copy() {
        assert!(Type::Int(IntSize::I32).is_copy());
        assert!(Type::Bool.is_copy());
        assert!(!Type::String.is_copy());
    }

    #[test]
    fn test_type_size() {
        assert_eq!(Type::Int(IntSize::I32).size(), 4);
        assert_eq!(Type::Int(IntSize::I64).size(), 8);
        assert_eq!(Type::Bool.size(), 1);
    }
}
