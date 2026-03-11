//! Type Checker
//!
//! Performs type checking on the AZC AST.

use super::ast::{IntSize, Type, UIntSize};
use super::env::TypeEnv;
use super::inference::{BinOp, Literal, TypeError, TypeInference, UnaryOp};

/// Type check result
pub type TypeResult<T> = Result<T, TypeError>;

/// Type checker
pub struct TypeChecker {
    inference: TypeInference,
    env: TypeEnv,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        TypeChecker {
            inference: TypeInference::new(),
            env: TypeEnv::with_builtins(),
        }
    }

    /// Create a type checker with a custom environment
    pub fn with_env(env: TypeEnv) -> Self {
        TypeChecker {
            inference: TypeInference::new(),
            env,
        }
    }

    /// Check an expression and return its type
    pub fn check_expr(&mut self, expr: &Expr) -> TypeResult<Type> {
        match expr {
            Expr::Literal(lit) => Ok(self.inference.infer_literal(&self.convert_literal(lit))),

            Expr::Variable(name) => self
                .env
                .lookup(name)
                .map(|scheme| self.inference.apply(&scheme.ty))
                .ok_or_else(|| TypeError::UnboundVariable(name.clone())),

            Expr::Binary { op, left, right } => {
                let left_ty = self.check_expr(left)?;
                let right_ty = self.check_expr(right)?;
                self.inference
                    .infer_binary(&self.convert_binop(op), &left_ty, &right_ty)
            }

            Expr::Unary { op, operand } => {
                let operand_ty = self.check_expr(operand)?;
                self.inference
                    .infer_unary(&self.convert_unaryop(op), &operand_ty)
            }

            Expr::Call { func, args } => {
                let func_ty = self.check_expr(func)?;

                match func_ty {
                    Type::Function { params, ret } => {
                        if args.len() != params.len() {
                            return Err(TypeError::ArgCountMismatch(params.len(), args.len()));
                        }

                        for (arg, param_ty) in args.iter().zip(params.iter()) {
                            let arg_ty = self.check_expr(arg)?;
                            self.inference.unify(&arg_ty, param_ty)?;
                        }

                        Ok(self.inference.apply(&ret))
                    }
                    _ => Err(TypeError::NotCallable(func_ty)),
                }
            }

            Expr::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_ty = self.check_expr(condition)?;
                self.inference.unify(&cond_ty, &Type::Bool)?;

                let then_ty = self.check_expr(then_branch)?;

                if let Some(else_expr) = else_branch {
                    let else_ty = self.check_expr(else_expr)?;
                    self.inference.unify(&then_ty, &else_ty)?;
                    Ok(then_ty)
                } else {
                    Ok(Type::Nil)
                }
            }

            Expr::Block(stmts) => {
                let mut result = Type::Nil;
                for stmt in stmts {
                    result = self.check_stmt(stmt)?;
                }
                Ok(result)
            }

            Expr::Array(elements) => {
                if elements.is_empty() {
                    let elem_ty = self.inference.fresh();
                    Ok(Type::Array {
                        elem: Box::new(elem_ty),
                        size: Some(0),
                    })
                } else {
                    let first_ty = self.check_expr(&elements[0])?;
                    for elem in &elements[1..] {
                        let elem_ty = self.check_expr(elem)?;
                        self.inference.unify(&first_ty, &elem_ty)?;
                    }
                    Ok(Type::Array {
                        elem: Box::new(first_ty),
                        size: Some(elements.len()),
                    })
                }
            }

            Expr::Tuple(elements) => {
                let types: Result<Vec<_>, _> =
                    elements.iter().map(|e| self.check_expr(e)).collect();
                Ok(Type::Tuple(types?))
            }

            Expr::Reference { expr, mutable } => {
                let inner_ty = self.check_expr(expr)?;
                Ok(Type::Ref {
                    inner: Box::new(inner_ty),
                    mutable: *mutable,
                })
            }

            Expr::Deref(expr) => {
                let ref_ty = self.check_expr(expr)?;
                match ref_ty {
                    Type::Ref { inner, .. } | Type::Box { inner } | Type::Rc { inner } => {
                        Ok(*inner)
                    }
                    _ => Err(TypeError::NotDereferencable(ref_ty)),
                }
            }

            Expr::Index { base, index } => {
                let base_ty = self.check_expr(base)?;
                let index_ty = self.check_expr(index)?;

                self.inference
                    .unify(&index_ty, &Type::UInt(UIntSize::USize))?;

                match base_ty {
                    Type::Array { elem, .. } => Ok(*elem),
                    _ => Err(TypeError::CannotInfer(
                        "indexing non-array type".to_string(),
                    )),
                }
            }
        }
    }

    /// Check a statement
    pub fn check_stmt(&mut self, stmt: &Stmt) -> TypeResult<Type> {
        match stmt {
            Stmt::Let { name, ty, value } => {
                if let Some(value) = value {
                    let value_ty = self.check_expr(value)?;

                    if let Some(annotated_ty) = ty {
                        self.inference.unify(&value_ty, annotated_ty)?;
                        self.env.insert_type(name.clone(), annotated_ty.clone());
                    } else {
                        self.env.insert_type(name.clone(), value_ty);
                    }
                } else {
                    let inferred_ty = ty.clone().unwrap_or_else(|| self.inference.fresh());
                    self.env.insert_type(name.clone(), inferred_ty);
                }

                Ok(Type::Nil)
            }

            Stmt::Assign { target, value } => {
                let target_ty = self.check_expr(target)?;
                let value_ty = self.check_expr(value)?;
                self.inference.unify(&target_ty, &value_ty)?;
                Ok(Type::Nil)
            }

            Stmt::Expr(expr) => self.check_expr(expr),

            Stmt::Return(value) => {
                if let Some(expr) = value {
                    self.check_expr(expr)
                } else {
                    Ok(Type::Nil)
                }
            }

            Stmt::While { condition, body } => {
                let cond_ty = self.check_expr(condition)?;
                self.inference.unify(&cond_ty, &Type::Bool)?;
                self.check_expr(body)?;
                Ok(Type::Nil)
            }

            Stmt::Function {
                name,
                params,
                ret_ty,
                body,
            } => {
                let mut param_types = Vec::new();
                let mut child_env = self.env.child();

                for (param_name, param_ty) in params {
                    let ty = param_ty.clone().unwrap_or_else(|| self.inference.fresh());
                    param_types.push(ty.clone());
                    child_env.insert_type(param_name.clone(), ty);
                }

                let return_ty = ret_ty.clone().unwrap_or_else(|| self.inference.fresh());

                let mut checker = TypeChecker::with_env(child_env);
                let body_ty = checker.check_expr(body)?;
                checker.inference.unify(&body_ty, &return_ty)?;

                let func_ty = Type::Function {
                    params: param_types,
                    ret: Box::new(return_ty),
                };

                self.env.insert_type(name.clone(), func_ty.clone());

                Ok(Type::Nil)
            }
        }
    }

    /// Convert a literal string to the internal representation
    fn convert_literal(&self, lit: &str) -> Literal {
        if lit == "true" {
            Literal::Bool(true)
        } else if lit == "false" {
            Literal::Bool(false)
        } else if lit == "nil" {
            Literal::Nil
        } else if lit.starts_with('"') && lit.ends_with('"') {
            Literal::String(lit[1..lit.len() - 1].to_string())
        } else if lit.starts_with('\'') && lit.ends_with('\'') {
            let ch = lit[1..lit.len() - 1].chars().next().unwrap();
            Literal::Char(ch)
        } else if lit.contains('.') {
            Literal::Float(lit.parse().unwrap_or(0.0))
        } else {
            Literal::Int(lit.parse().unwrap_or(0))
        }
    }

    /// Convert binary operator string to enum
    fn convert_binop(&self, op: &str) -> BinOp {
        match op {
            "+" => BinOp::Add,
            "-" => BinOp::Sub,
            "*" => BinOp::Mul,
            "/" => BinOp::Div,
            "%" => BinOp::Mod,
            "==" => BinOp::Eq,
            "!=" => BinOp::Ne,
            "<" => BinOp::Lt,
            "<=" => BinOp::Le,
            ">" => BinOp::Gt,
            ">=" => BinOp::Ge,
            "and" => BinOp::And,
            "or" => BinOp::Or,
            _ => BinOp::Eq,
        }
    }

    /// Convert unary operator string to enum
    fn convert_unaryop(&self, op: &str) -> UnaryOp {
        match op {
            "-" => UnaryOp::Neg,
            "not" => UnaryOp::Not,
            "*" => UnaryOp::Deref,
            "&" => UnaryOp::Ref,
            "&mut" => UnaryOp::RefMut,
            _ => UnaryOp::Not,
        }
    }

    /// Get the current environment
    pub fn env(&self) -> &TypeEnv {
        &self.env
    }

    /// Get mutable access to the environment
    pub fn env_mut(&mut self) -> &mut TypeEnv {
        &mut self.env
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Expression AST (simplified for type checking)
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Variable(String),
    Binary {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: String,
        operand: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Block(Vec<Stmt>),
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    Reference {
        expr: Box<Expr>,
        mutable: bool,
    },
    Deref(Box<Expr>),
    Index {
        base: Box<Expr>,
        index: Box<Expr>,
    },
}

/// Statement AST (simplified for type checking)
#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<Type>,
        value: Option<Expr>,
    },
    Assign {
        target: Expr,
        value: Expr,
    },
    Expr(Expr),
    Return(Option<Expr>),
    While {
        condition: Expr,
        body: Expr,
    },
    Function {
        name: String,
        params: Vec<(String, Option<Type>)>,
        ret_ty: Option<Type>,
        body: Expr,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_int() {
        let mut checker = TypeChecker::new();
        let expr = Expr::Literal("42".to_string());
        let ty = checker.check_expr(&expr).unwrap();
        assert_eq!(ty, Type::default_int());
    }

    #[test]
    fn test_literal_bool() {
        let mut checker = TypeChecker::new();
        let expr = Expr::Literal("true".to_string());
        let ty = checker.check_expr(&expr).unwrap();
        assert_eq!(ty, Type::Bool);
    }

    #[test]
    fn test_binary_add() {
        let mut checker = TypeChecker::new();
        let expr = Expr::Binary {
            op: "+".to_string(),
            left: Box::new(Expr::Literal("1".to_string())),
            right: Box::new(Expr::Literal("2".to_string())),
        };
        let ty = checker.check_expr(&expr).unwrap();
        assert_eq!(ty, Type::default_int());
    }

    #[test]
    fn test_let_statement() {
        let mut checker = TypeChecker::new();
        let stmt = Stmt::Let {
            name: "x".to_string(),
            ty: None,
            value: Some(Expr::Literal("42".to_string())),
        };
        checker.check_stmt(&stmt).unwrap();
        assert!(checker.env().contains("x"));
    }

    #[test]
    fn test_function() {
        let mut checker = TypeChecker::new();
        let stmt = Stmt::Function {
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), Some(Type::default_int())),
                ("b".to_string(), Some(Type::default_int())),
            ],
            ret_ty: Some(Type::default_int()),
            body: Expr::Binary {
                op: "+".to_string(),
                left: Box::new(Expr::Variable("a".to_string())),
                right: Box::new(Expr::Variable("b".to_string())),
            },
        };
        checker.check_stmt(&stmt).unwrap();
        assert!(checker.env().contains("add"));
    }
}
