//! AST Node Definitions

use std::fmt;

/// A complete AZC program
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

/// Top-level statements
#[derive(Debug, Clone)]
pub enum Statement {
    /// Variable declaration
    Let {
        name: String,
        type_annotation: Option<String>,
        value: Option<Expression>,
        mutable: bool,
    },

    /// Assignment
    Assign {
        target: Expression,
        value: Expression,
    },

    /// Expression statement
    Expr(Expression),

    /// Function definition
    Function {
        name: String,
        type_params: Vec<String>,
        params: Vec<(String, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Statement>,
    },

    /// Return statement
    Return(Option<Expression>),

    /// If statement
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    /// While loop
    While {
        condition: Expression,
        body: Vec<Statement>,
    },

    /// For loop
    For {
        var: String,
        iterable: Expression,
        body: Vec<Statement>,
    },

    /// Match expression/statement
    Match {
        value: Expression,
        arms: Vec<MatchArm>,
    },

    /// Struct definition
    Struct {
        name: String,
        type_params: Vec<String>,
        fields: Vec<(String, String)>,
    },

    /// Enum definition
    Enum {
        name: String,
        type_params: Vec<String>,
        variants: Vec<EnumVariant>,
    },

    /// Trait definition
    Trait {
        name: String,
        type_params: Vec<String>,
        methods: Vec<TraitMethod>,
    },

    /// Impl block
    Impl {
        trait_name: Option<String>,
        trait_args: Vec<String>,
        target: String,
        type_params: Vec<String>,
        methods: Vec<Statement>,
    },

    /// Type alias
    TypeAlias {
        name: String,
        type_params: Vec<String>,
        target: String,
    },

    // ===== v0.4.0 Advanced Features =====

    /// Async function definition
    AsyncFunction {
        name: String,
        type_params: Vec<String>,
        params: Vec<(String, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Statement>,
    },

    /// Unsafe block
    Unsafe {
        body: Vec<Statement>,
        reason: Option<String>,
    },

    /// Macro definition
    Macro {
        name: String,
        params: Vec<MacroParam>,
        body: Vec<Statement>,
    },

    /// Extern block for FFI
    Extern {
        abi: String,
        declarations: Vec<ExternDecl>,
    },

    /// Use statement
    Use { path: String, alias: Option<String> },
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<String>>,
}

/// Trait method
#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub name: String,
    pub params: Vec<(String, Option<String>)>,
    pub return_type: Option<String>,
    pub body: Option<Vec<Statement>>,
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
}

/// Patterns for matching
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Variable(String),
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Array {
        elements: Vec<Pattern>,
        rest: Option<Box<Pattern>>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Pattern)>,
    },
    Enum {
        name: String,
        variant: String,
        args: Option<Vec<Pattern>>,
    },
    Range {
        start: Box<Pattern>,
        end: Box<Pattern>,
        inclusive: bool,
    },
    Or(Vec<Pattern>),
}

// ===== v0.4.0 Supporting Types =====

/// Macro parameter
#[derive(Debug, Clone)]
pub struct MacroParam {
    pub name: String,
    pub kind: MacroParamKind,
}

/// Kind of macro parameter
#[derive(Debug, Clone)]
pub enum MacroParamKind {
    Expr,
    Ident,
    Type,
    Block,
    Stmt,
}

/// External declaration for FFI
#[derive(Debug, Clone)]
pub struct ExternDecl {
    pub name: String,
    pub params: Vec<(String, Option<String>)>,
    pub return_type: Option<String>,
    pub is_variadic: bool,
}
/// Expressions
#[derive(Debug, Clone)]
pub enum Expression {
    /// Literal value
    Literal(Literal),

    /// Variable reference
    Variable(String),

    /// Path (e.g., Option::Some)
    Path(Vec<String>),

    /// Binary operation
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },

    /// Function call
    Call {
        func: Box<Expression>,
        type_args: Vec<String>,
        args: Vec<Expression>,
    },

    /// Method call
    MethodCall {
        object: Box<Expression>,
        method: String,
        type_args: Vec<String>,
        args: Vec<Expression>,
    },

    /// Field access
    Field {
        object: Box<Expression>,
        field: String,
    },

    /// Index operation
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },

    /// Array literal
    Array(Vec<Expression>),

    /// Tuple literal
    Tuple(Vec<Expression>),

    /// Hash/Map literal
    Hash(Vec<(Expression, Expression)>),

    /// Reference
    Reference {
        expr: Box<Expression>,
        mutable: bool,
    },

    /// Dereference
    Deref(Box<Expression>),

    /// Block expression
    Block {
        statements: Vec<Statement>,
        value: Option<Box<Expression>>,
    },

    /// If expression
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },

    /// Match expression
    Match {
        value: Box<Expression>,
        arms: Vec<MatchArm>,
    },

    /// Lambda/closure
    Lambda {
        params: Vec<(String, Option<String>)>,
        body: Box<Expression>,
    },

    /// Struct instantiation
    StructInstantiation {
        name: String,
        type_args: Vec<String>,
        fields: Vec<(String, Expression)>,
    },

    /// Enum instantiation
    EnumInstantiation {
        name: String,
        variant: String,
        args: Vec<Expression>,
    },

    /// Try expression (error propagation)
    Try(Box<Expression>),

    /// Range expression
    Range {
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
        inclusive: bool,
    },

    /// Cast expression
    Cast {
        expr: Box<Expression>,
        target: String,
    },

    /// Type annotation
    TypeAnnotation { expr: Box<Expression>, ty: String },

    // ===== v0.4.0 Advanced Features =====

    /// Await expression
    Await(Box<Expression>),

    /// Async block
    AsyncBlock(Vec<Statement>),

    /// Macro call
    MacroCall {
        name: String,
        args: Vec<Expression>,
    },

    /// Unsafe expression
    UnsafeExpr(Box<Expression>),

    /// Foreign function call (FFI)
    ForeignCall {
        abi: String,
        name: String,
        args: Vec<Expression>,
    },
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

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Char(c) => write!(f, "'{}'", c),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
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

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,

    // Range
    Range,
    RangeInclusive,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Ne => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Ge => write!(f, ">="),
            BinaryOp::And => write!(f, "and"),
            BinaryOp::Or => write!(f, "or"),
            BinaryOp::BitAnd => write!(f, "&"),
            BinaryOp::BitOr => write!(f, "|"),
            BinaryOp::BitXor => write!(f, "^"),
            BinaryOp::Shl => write!(f, "<<"),
            BinaryOp::Shr => write!(f, ">>"),
            BinaryOp::Range => write!(f, ".."),
            BinaryOp::RangeInclusive => write!(f, "..="),
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    Deref,
    Ref,
    RefMut,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "not"),
            UnaryOp::Deref => write!(f, "*"),
            UnaryOp::Ref => write!(f, "&"),
            UnaryOp::RefMut => write!(f, "&mut"),
        }
    }
}

/// Type representation in AST
#[derive(Debug, Clone)]
pub enum TypeNode {
    Named(String),
    Generic {
        name: String,
        args: Vec<TypeNode>,
    },
    Tuple(Vec<TypeNode>),
    Array {
        elem: Box<TypeNode>,
        size: Option<usize>,
    },
    Function {
        params: Vec<TypeNode>,
        ret: Box<TypeNode>,
    },
    Reference {
        inner: Box<TypeNode>,
        mutable: bool,
    },
    Optional(Box<TypeNode>),
}

impl fmt::Display for TypeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeNode::Named(name) => write!(f, "{}", name),
            TypeNode::Generic { name, args } => {
                write!(f, "{}<", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ">")
            }
            TypeNode::Tuple(elements) => {
                write!(f, "(")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, ")")
            }
            TypeNode::Array { elem, size } => {
                if let Some(s) = size {
                    write!(f, "[{}; {}]", elem, s)
                } else {
                    write!(f, "[{}]", elem)
                }
            }
            TypeNode::Function { params, ret } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            TypeNode::Reference { inner, mutable } => {
                if *mutable {
                    write!(f, "&mut {}", inner)
                } else {
                    write!(f, "&{}", inner)
                }
            }
            TypeNode::Optional(inner) => {
                write!(f, "{}?", inner)
            }
        }
    }
}
