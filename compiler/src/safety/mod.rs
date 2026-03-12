//! Full Borrow Checking Pipeline
//!
//! This module integrates the borrow checker with the type checker
//! to provide comprehensive safety analysis.

use crate::ast::{Expression, Literal, Program, Statement};
use crate::ownership::{BorrowChecker, OwnershipError};
use crate::types::{Type, TypeChecker, TypeError};

/// Safety analysis result
#[derive(Debug)]
pub struct SafetyAnalysis {
    pub type_errors: Vec<TypeError>,
    pub ownership_errors: Vec<OwnershipError>,
    pub warnings: Vec<String>,
    pub is_safe: bool,
}

impl SafetyAnalysis {
    pub fn new() -> Self {
        SafetyAnalysis {
            type_errors: Vec::new(),
            ownership_errors: Vec::new(),
            warnings: Vec::new(),
            is_safe: true,
        }
    }

    pub fn add_type_error(&mut self, error: TypeError) {
        self.type_errors.push(error);
        self.is_safe = false;
    }

    pub fn add_ownership_error(&mut self, error: OwnershipError) {
        self.ownership_errors.push(error);
        self.is_safe = false;
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for SafetyAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

/// Full safety analyzer
pub struct SafetyAnalyzer {
    type_checker: TypeChecker,
    borrow_checker: BorrowChecker,
}

impl SafetyAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = SafetyAnalyzer {
            type_checker: TypeChecker::new(),
            borrow_checker: BorrowChecker::new(),
        };
        // Register built-in functions
        analyzer.borrow_checker.declare("puts");
        analyzer.borrow_checker.declare("print");
        analyzer
    }


    /// Analyze a program for safety issues
    pub fn analyze(&mut self, program: &Program) -> SafetyAnalysis {
        let mut analysis = SafetyAnalysis::new();

        // Phase 1: Type checking
        for stmt in &program.statements {
            if let Err(e) = self.check_statement_types(stmt) {
                analysis.add_type_error(e);
            }
        }

        // Phase 2: Ownership checking
        for stmt in &program.statements {
            if let Err(e) = self.check_statement_ownership(stmt) {
                analysis.add_ownership_error(e);
            }
        }

        analysis
    }

    fn check_statement_types(&mut self, stmt: &Statement) -> Result<(), TypeError> {
        match stmt {
            Statement::Let {
                name,
                type_annotation,
                value,
                mutable: _, // Ignore mutability for type checking
            } => {
                if let Some(val) = value {
                    let _ = self.check_expression_types(val)?;
                }
                Ok(())
            }
            Statement::Assign { target, value } => {
                let _ = self.check_expression_types(target)?;
                let _ = self.check_expression_types(value)?;
                Ok(())
            }
            Statement::Expr(expr) => {
                let _ = self.check_expression_types(expr)?;
                Ok(())
            }
            Statement::Return(value) => {
                if let Some(val) = value {
                    let _ = self.check_expression_types(val)?;
                }
                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let _ = self.check_expression_types(condition)?;
                for s in then_branch {
                    self.check_statement_types(s)?;
                }
                if let Some(else_body) = else_branch {
                    for s in else_body {
                        self.check_statement_types(s)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                let _ = self.check_expression_types(condition)?;
                for s in body {
                    self.check_statement_types(s)?;
                }
                Ok(())
            }
            Statement::Function { body, .. } => {
                for s in body {
                    self.check_statement_types(s)?;
                }
                Ok(())
            }
            Statement::Struct { .. } | Statement::Enum { .. } | Statement::Impl { .. } => Ok(()),
            // Handle all other statement types
            _ => Ok(()),
        }
    }

    fn check_expression_types(&mut self, expr: &Expression) -> Result<Type, TypeError> {
        match expr {
            Expression::Literal(lit) => Ok(self.infer_literal_type(lit)),
            Expression::Variable(name) => {
                // For now, return a default type
                Ok(Type::default_int())
            }
            Expression::Binary { left, right, .. } => {
                let _ = self.check_expression_types(left)?;
                let _ = self.check_expression_types(right)?;
                Ok(Type::default_int())
            }
            Expression::Unary { operand, .. } => {
                let _ = self.check_expression_types(operand)?;
                Ok(Type::default_int())
            }
            Expression::Call { func, args, type_args: _ } => {
                let _ = self.check_expression_types(func)?;
                for arg in args {
                    let _ = self.check_expression_types(arg)?;
                }
                Ok(Type::Nil)
            }
            Expression::MethodCall { object, args, .. } => {
                let _ = self.check_expression_types(object)?;
                for arg in args {
                    let _ = self.check_expression_types(arg)?;
                }
                Ok(Type::Nil)
            }
            Expression::Field { object, .. } => {
                let _ = self.check_expression_types(object)?;
                Ok(Type::default_int())
            }
            Expression::Index { object, index } => {
                let _ = self.check_expression_types(object)?;
                let _ = self.check_expression_types(index)?;
                Ok(Type::default_int())
            }
            Expression::Array(elements) => {
                for elem in elements {
                    let _ = self.check_expression_types(elem)?;
                }
                Ok(Type::Array {
                    elem: Box::new(Type::default_int()),
                    size: None,
                })
            }
            Expression::Tuple(elements) => {
                let mut types = Vec::new();
                for elem in elements {
                    types.push(self.check_expression_types(elem)?);
                }
                Ok(Type::Tuple(types))
            }
            Expression::Reference { expr, .. } => {
                let inner = self.check_expression_types(expr)?;
                Ok(Type::Ref {
                    inner: Box::new(inner),
                    mutable: false,
                })
            }
            Expression::Deref(inner) => {
                let _ = self.check_expression_types(inner)?;
                Ok(Type::default_int())
            }
            Expression::Block { statements, value } => {
                for s in statements {
                    self.check_statement_types(s)?;
                }
                if let Some(v) = value {
                    self.check_expression_types(v)
                } else {
                    Ok(Type::Nil)
                }
            }
            Expression::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let _ = self.check_expression_types(condition)?;
                let then_ty = self.check_expression_types(then_branch)?;
                if let Some(else_expr) = else_branch {
                    let _ = self.check_expression_types(else_expr)?;
                    Ok(then_ty)
                } else {
                    Ok(Type::Nil)
                }
            }
            Expression::Match { value, arms } => {
                let _ = self.check_expression_types(value)?;
                for arm in arms {
                    let _ = self.check_expression_types(&arm.body)?;
                }
                Ok(Type::Nil)
            }
            Expression::Lambda { body, .. } => {
                let _ = self.check_expression_types(body)?;
                Ok(Type::Nil)
            }
            Expression::StructInstantiation { fields, .. } => {
                for (_, value) in fields {
                    let _ = self.check_expression_types(value)?;
                }
                Ok(Type::Nil)
            }
            // Handle all other expression types
            _ => Ok(Type::Nil),
        }
    }

    fn infer_literal_type(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Int(_) => Type::default_int(),
            Literal::Float(_) => Type::default_float(),
            Literal::Bool(_) => Type::Bool,
            Literal::Char(_) => Type::Char,
            Literal::String(_) => Type::String,
            Literal::Nil => Type::Nil,
        }
    }

    fn check_statement_ownership(&mut self, stmt: &Statement) -> Result<(), OwnershipError> {
        match stmt {
            Statement::Let { name, .. } => {
                self.borrow_checker.declare(name);
                Ok(())
            }
            Statement::Assign { target, value } => {
                self.check_expression_ownership(target)?;
                self.check_expression_ownership(value)?;
                Ok(())
            }
            Statement::Expr(expr) => self.check_expression_ownership(expr),
            Statement::Return(value) => {
                if let Some(val) = value {
                    self.check_expression_ownership(val)?;
                }
                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.check_expression_ownership(condition)?;

                self.borrow_checker.enter_scope();
                for s in then_branch {
                    self.check_statement_ownership(s)?;
                }
                self.borrow_checker.exit_scope();

                if let Some(else_body) = else_branch {
                    self.borrow_checker.enter_scope();
                    for s in else_body {
                        self.check_statement_ownership(s)?;
                    }
                    self.borrow_checker.exit_scope();
                }

                Ok(())
            }
            Statement::While { condition, body } => {
                self.check_expression_ownership(condition)?;

                self.borrow_checker.enter_scope();
                for s in body {
                    self.check_statement_ownership(s)?;
                }
                self.borrow_checker.exit_scope();

                Ok(())
            }
            Statement::Function { name, body, .. } => {
                self.borrow_checker.declare(name);

                self.borrow_checker.enter_scope();
                for s in body {
                    self.check_statement_ownership(s)?;
                }
                self.borrow_checker.exit_scope();

                Ok(())
            }
            Statement::Struct { .. } | Statement::Enum { .. } | Statement::Impl { .. } => Ok(()),
            // Handle all other statement types
            _ => Ok(()),
        }
    }

    fn check_expression_ownership(&mut self, expr: &Expression) -> Result<(), OwnershipError> {
        match expr {
            Expression::Literal(_) => Ok(()),
            Expression::Variable(name) => {
                self.borrow_checker.check_use(name)?;
                Ok(())
            }
            Expression::Binary { left, right, .. } => {
                self.check_expression_ownership(left)?;
                self.check_expression_ownership(right)?;
                Ok(())
            }
            Expression::Unary { operand, op } => {
                match op {
                    crate::ast::UnaryOp::Ref => {
                        if let Expression::Variable(ref name) = **operand {
                            self.borrow_checker.check_borrow(name)?;
                        } else {
                            self.check_expression_ownership(operand)?;
                        }
                    }
                    crate::ast::UnaryOp::RefMut => {
                        if let Expression::Variable(ref name) = **operand {
                            self.borrow_checker.check_borrow_mut(name)?;
                        } else {
                            self.check_expression_ownership(operand)?;
                        }
                    }
                    _ => {
                        self.check_expression_ownership(operand)?;
                    }
                }
                Ok(())
            }

            Expression::Call { func, args, type_args: _ } => {
                self.check_expression_ownership(func)?;
                for arg in args {
                    self.check_expression_ownership(arg)?;
                }
                Ok(())
            }
            Expression::MethodCall { object, args, .. } => {
                self.check_expression_ownership(object)?;
                for arg in args {
                    self.check_expression_ownership(arg)?;
                }
                Ok(())
            }
            Expression::Field { object, .. } => {
                self.check_expression_ownership(object)?;
                Ok(())
            }
            Expression::Index { object, index } => {
                self.check_expression_ownership(object)?;
                self.check_expression_ownership(index)?;
                Ok(())
            }
            Expression::Array(elements) => {
                for elem in elements {
                    self.check_expression_ownership(elem)?;
                }
                Ok(())
            }
            Expression::Tuple(elements) => {
                for elem in elements {
                    self.check_expression_ownership(elem)?;
                }
                Ok(())
            }
            Expression::Reference { expr, mutable } => {
                if let Expression::Variable(ref name) = **expr {
                    if *mutable {
                        self.borrow_checker.check_borrow_mut(name)?;
                    } else {
                        self.borrow_checker.check_borrow(name)?;
                    }
                } else {
                    self.check_expression_ownership(expr)?;
                }
                Ok(())
            }

            Expression::Deref(inner) => {
                self.check_expression_ownership(inner)?;
                Ok(())
            }
            Expression::Block { statements, value } => {
                for s in statements {
                    self.check_statement_ownership(s)?;
                }
                if let Some(v) = value {
                    self.check_expression_ownership(v)?;
                }
                Ok(())
            }
            Expression::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.check_expression_ownership(condition)?;
                self.check_expression_ownership(then_branch)?;
                if let Some(else_expr) = else_branch {
                    self.check_expression_ownership(else_expr)?;
                }
                Ok(())
            }
            Expression::Match { value, arms } => {
                self.check_expression_ownership(value)?;
                for arm in arms {
                    self.check_expression_ownership(&arm.body)?;
                }
                Ok(())
            }
            Expression::Lambda { body, .. } => {
                self.check_expression_ownership(body)?;
                Ok(())
            }
            Expression::StructInstantiation { fields, .. } => {
                for (_, value) in fields {
                    self.check_expression_ownership(value)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl Default for SafetyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::parse;

    #[test]
    fn test_safe_program() {
        let source = "let x = 10";
        let program = parse(source).unwrap();
        let mut analyzer = SafetyAnalyzer::new();
        let result = analyzer.analyze(&program);
        if !result.is_safe {
            println!("Type errors: {:?}", result.type_errors);
            println!("Ownership errors: {:?}", result.ownership_errors);
        }
        assert!(result.is_safe);
    }

    #[test]
    fn test_function_definition() {
        let source = "def foo()\n  puts \"hello\"\nend";
        let program = parse(source).unwrap();
        let mut analyzer = SafetyAnalyzer::new();
        let result = analyzer.analyze(&program);
        if !result.is_safe {
            println!("Type errors: {:?}", result.type_errors);
            println!("Ownership errors: {:?}", result.ownership_errors);
        }
        assert!(result.is_safe);
    }
}

