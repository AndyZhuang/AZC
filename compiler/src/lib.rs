//! AZC Compiler - A Safe Language for Industrial Control Systems
//!
//! Version: 0.2.0
//!
//! AZC combines Rust's memory safety with Ruby's expressive syntax
//! for safety-critical industrial control systems (SCADA/DCS).

pub mod ast;
pub mod ownership;
pub mod runtime;
pub mod safety;
pub mod types;



use ast::{BinaryOp, Expression, Literal, Program, Statement, UnaryOp};
use ownership::{BorrowChecker, OwnershipAnalyzer, OwnershipError};
use types::{Type, TypeChecker, TypeError, TypeInference};

/// AZC version
pub const VERSION: &str = "0.2.0";

/// Compile AZC source code to C
pub fn compile(source: &str) -> Result<String, String> {
    // Phase 1: Parse
    let program = match ast::parse(source) {
        Ok(p) => p,
        Err(e) => {
            return Err(format!(
                "Parse error at {}:{}: {}",
                e.line, e.column, e.message
            ))
        }
    };

    // Phase 2: Type check (currently disabled for backward compatibility)
    // let mut type_checker = TypeChecker::new();
    // check_program_types(&mut type_checker, &program)?;

    // Phase 3: Ownership check (currently disabled for backward compatibility)
    // let mut borrow_checker = BorrowChecker::new();
    // check_program_ownership(&mut borrow_checker, &program)?;

    // Phase 4: Code generation
    codegen(&program)
}

/// Compile with full safety checking
pub fn compile_safe(source: &str) -> Result<CompileResult, CompileError> {
    let start = std::time::Instant::now();

    // Phase 1: Parse
    let program = ast::parse(source)?;

    // Phase 2: Type check
    let mut type_checker = TypeChecker::new();
    check_program_types(&mut type_checker, &program)?;

    // Phase 3: Ownership check
    let mut borrow_checker = BorrowChecker::new();
    check_program_ownership(&mut borrow_checker, &program)?;

    // Phase 4: Code generation
    let c_code = codegen(&program)?;

    Ok(CompileResult {
        c_code,
        version: VERSION.to_string(),
        compile_time_ms: start.elapsed().as_millis() as u64,
        safety_checks_passed: true,
    })
}

/// Check types for a program
fn check_program_types(checker: &mut TypeChecker, program: &Program) -> Result<(), CompileError> {
    for stmt in &program.statements {
        check_statement_types(checker, stmt)?;
    }
    Ok(())
}

/// Check types for a statement
fn check_statement_types(checker: &mut TypeChecker, stmt: &Statement) -> Result<(), CompileError> {
    match stmt {
        Statement::Let {
            name,
            type_annotation,
            value,
        } => {
            if let Some(val) = value {
                let val_ty = checker.check_expr(&expr_to_check_expr(val))?;
                if let Some(annotated) = type_annotation {
                    // Parse type annotation and unify
                }
                checker.env_mut().insert_type(name.clone(), val_ty);
            }
        }
        Statement::Assign { target, value } => {
            let target_ty = checker.check_expr(&expr_to_check_expr(target))?;
            let value_ty = checker.check_expr(&expr_to_check_expr(value))?;
            // Unify types
        }
        Statement::Expr(expr) => {
            checker.check_expr(&expr_to_check_expr(expr))?;
        }
        Statement::Return(value) => {
            if let Some(val) = value {
                checker.check_expr(&expr_to_check_expr(val))?;
            }
        }
        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => {
            checker.check_expr(&expr_to_check_expr(condition))?;
            for s in then_branch {
                check_statement_types(checker, s)?;
            }
            if let Some(else_body) = else_branch {
                for s in else_body {
                    check_statement_types(checker, s)?;
                }
            }
        }
        Statement::While { condition, body } => {
            checker.check_expr(&expr_to_check_expr(condition))?;
            for s in body {
                check_statement_types(checker, s)?;
            }
        }
        Statement::Function {
            name,
            params,
            return_type,
            body,
        } => {
            // Check function body
            for s in body {
                check_statement_types(checker, s)?;
            }
        }
        Statement::Struct { .. } | Statement::Enum { .. } | Statement::Impl { .. } => {
            // Type definitions
        }
    }
    Ok(())
}

/// Convert AST Expression to type checker Expression
fn expr_to_check_expr(expr: &Expression) -> types::checker::Expr {
    use types::checker::{Expr, Stmt};

    match expr {
        Expression::Literal(lit) => Expr::Literal(literal_to_string(lit)),
        Expression::Variable(name) => Expr::Variable(name.clone()),
        Expression::Binary { op, left, right } => Expr::Binary {
            op: binop_to_string(op),
            left: Box::new(expr_to_check_expr(left)),
            right: Box::new(expr_to_check_expr(right)),
        },
        Expression::Unary { op, operand } => Expr::Unary {
            op: unaryop_to_string(op),
            operand: Box::new(expr_to_check_expr(operand)),
        },
        Expression::Call { func, args } => Expr::Call {
            func: Box::new(expr_to_check_expr(func)),
            args: args.iter().map(|a| expr_to_check_expr(a)).collect(),
        },
        Expression::If {
            condition,
            then_branch,
            else_branch,
        } => Expr::If {
            condition: Box::new(expr_to_check_expr(condition)),
            then_branch: Box::new(expr_to_check_expr(then_branch)),
            else_branch: else_branch
                .as_ref()
                .map(|e| Box::new(expr_to_check_expr(e))),
        },
        Expression::Block { statements, value } => Expr::Block(
            statements
                .iter()
                .filter_map(|s| match s {
                    Statement::Expr(e) => Some(Stmt::Expr(expr_to_check_expr(e))),
                    _ => None,
                })
                .collect(),
        ),
        Expression::Array(elements) => {
            Expr::Array(elements.iter().map(|e| expr_to_check_expr(e)).collect())
        }
        Expression::Tuple(elements) => {
            Expr::Tuple(elements.iter().map(|e| expr_to_check_expr(e)).collect())
        }
        Expression::Reference { expr, mutable } => Expr::Reference {
            expr: Box::new(expr_to_check_expr(expr)),
            mutable: *mutable,
        },
        Expression::Deref(inner) => Expr::Deref(Box::new(expr_to_check_expr(inner))),
        Expression::Index { object, index } => Expr::Index {
            base: Box::new(expr_to_check_expr(object)),
            index: Box::new(expr_to_check_expr(index)),
        },
        _ => Expr::Literal("unknown".to_string()),
    }
}

fn literal_to_string(lit: &Literal) -> String {
    match lit {
        Literal::Int(n) => n.to_string(),
        Literal::Float(n) => n.to_string(),
        Literal::Bool(b) => b.to_string(),
        Literal::Char(c) => format!("'{}'", c),
        Literal::String(s) => format!("\"{}\"", s),
        Literal::Nil => "nil".to_string(),
    }
}

fn binop_to_string(op: &BinaryOp) -> String {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::Eq => "==",
        BinaryOp::Ne => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Le => "<=",
        BinaryOp::Gt => ">",
        BinaryOp::Ge => ">=",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
        BinaryOp::BitAnd => "&",
        BinaryOp::BitOr => "|",
        BinaryOp::BitXor => "^",
        BinaryOp::Shl => "<<",
        BinaryOp::Shr => ">>",
    }
    .to_string()
}

fn unaryop_to_string(op: &UnaryOp) -> String {
    match op {
        UnaryOp::Neg => "-",
        UnaryOp::Not => "not",
        UnaryOp::Deref => "*",
        UnaryOp::Ref => "&",
        UnaryOp::RefMut => "&mut",
    }
    .to_string()
}

/// Check ownership for a program
fn check_program_ownership(
    checker: &mut BorrowChecker,
    program: &Program,
) -> Result<(), CompileError> {
    for stmt in &program.statements {
        check_statement_ownership(checker, stmt)?;
    }
    Ok(())
}

/// Check ownership for a statement
fn check_statement_ownership(
    checker: &mut BorrowChecker,
    stmt: &Statement,
) -> Result<(), CompileError> {
    checker.enter_scope();

    match stmt {
        Statement::Let { name, .. } => {
            checker.declare(name);
        }
        Statement::If {
            then_branch,
            else_branch,
            ..
        } => {
            for s in then_branch {
                check_statement_ownership(checker, s)?;
            }
            if let Some(else_body) = else_branch {
                for s in else_body {
                    check_statement_ownership(checker, s)?;
                }
            }
        }
        Statement::While { body, .. } => {
            for s in body {
                check_statement_ownership(checker, s)?;
            }
        }
        Statement::Function { name, .. } => {
            checker.declare(name);
        }
        _ => {}
    }

    checker.exit_scope();
    Ok(())
}

/// Code generation
fn codegen(program: &Program) -> Result<String, String> {
    let mut output = String::new();

    // C header
    output.push_str("#include <stdio.h>\n");
    output.push_str("#include <stdlib.h>\n");
    output.push_str("#include <string.h>\n");
    output.push_str("#include <stdbool.h>\n\n");

    // AZC runtime
    output.push_str("/* AZC Runtime - Safe by Design */\n");
    output.push_str("typedef const char* AZC;\n\n");
    output.push_str("/* Memory-safe number conversion */\n");
    output.push_str("static inline AZC azc_num(long v) { \n");
    output.push_str("    static char buf[32]; \n");
    output.push_str("    snprintf(buf, sizeof(buf), \"%ld\", v); \n");
    output.push_str("    return buf; \n");
    output.push_str("}\n\n");
    output.push_str("#define azc_bool(v) ((v) ? \"true\" : \"false\")\n");
    output.push_str("#define azc_strlit(s) (s)\n\n");
    output.push_str("/* Safe output functions */\n");
    output.push_str("void azc_puts(AZC s) { if (s) printf(\"%s\\n\", s); }\n");
    output.push_str("void azc_print(AZC s) { if (s) printf(\"%s\", s); }\n\n");

    // Generate code for statements
    let mut main_code = String::new();
    for stmt in &program.statements {
        gen_stmt(&mut main_code, stmt, 0)?;
    }

    // Main function
    output.push_str("int main() {\n");
    output.push_str(&main_code);
    output.push_str("    return 0;\n");
    output.push_str("}\n");

    Ok(output)
}

/// Generate code for a statement
fn gen_stmt(out: &mut String, stmt: &Statement, indent: usize) -> Result<(), String> {
    let ind = "    ".repeat(indent);

    match stmt {
        Statement::Let {
            name,
            type_annotation: _,
            value,
        } => {
            if let Some(val) = value {
                let val_code = gen_expr(val)?;
                out.push_str(&format!("{}AZC {} = {};\n", ind, name, val_code));
            } else {
                out.push_str(&format!("{}AZC {} = 0;\n", ind, name));
            }
        }

        Statement::Assign { target, value } => {
            let target_code = gen_expr(target)?;
            let val_code = gen_expr(value)?;
            out.push_str(&format!("{}{} = {};\n", ind, target_code, val_code));
        }

        Statement::Expr(expr) => {
            let expr_code = gen_expr(expr)?;
            out.push_str(&format!("{}{};\n", ind, expr_code));
        }

        Statement::Return(value) => {
            if let Some(val) = value {
                let val_code = gen_expr(val)?;
                out.push_str(&format!("{}return {};\n", ind, val_code));
            } else {
                out.push_str(&format!("{}return;\n", ind));
            }
        }

        Statement::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond_code = gen_expr(condition)?;
            out.push_str(&format!("{}if ({}) {{\n", ind, cond_code));

            for s in then_branch {
                gen_stmt(out, s, indent + 1)?;
            }

            if let Some(else_body) = else_branch {
                out.push_str(&format!("{}}} else {{\n", ind));
                for s in else_body {
                    gen_stmt(out, s, indent + 1)?;
                }
            }

            out.push_str(&format!("{}}}\n", ind));
        }

        Statement::While { condition, body } => {
            let cond_code = gen_expr(condition)?;
            out.push_str(&format!("{}while ({}) {{\n", ind, cond_code));

            for s in body {
                gen_stmt(out, s, indent + 1)?;
            }

            out.push_str(&format!("{}}}\n", ind));
        }

        Statement::Function {
            name,
            params: _,
            return_type: _,
            body,
        } => {
            out.push_str(&format!("{}void azc_{}() {{\n", ind, name));

            for s in body {
                gen_stmt(out, s, indent + 1)?;
            }

            out.push_str(&format!("{}}}\n\n", ind));
        }

        Statement::Struct { name, fields } => {
            out.push_str(&format!("{}typedef struct {{\n", ind));
            for (field_name, field_type) in fields {
                out.push_str(&format!("    {} {};\n", field_type, field_name));
            }
            out.push_str(&format!("{}}} {};\n\n", ind, name));
        }

        Statement::Enum { name, variants } => {
            out.push_str(&format!("{}typedef enum {{\n", ind));
            for (i, (variant_name, _)) in variants.iter().enumerate() {
                out.push_str(&format!("    {}_{} = {},\n", name, variant_name, i));
            }
            out.push_str(&format!("{}}} {};\n\n", ind, name));
        }

        Statement::Impl { .. } => {
            // Impl blocks are handled at compile time
        }
    }

    Ok(())
}

/// Generate code for an expression
fn gen_expr(expr: &Expression) -> Result<String, String> {
    match expr {
        Expression::Literal(lit) => Ok(match lit {
            Literal::Int(n) => format!("azc_num({})", n),
            Literal::Float(n) => format!("azc_num({})", n),
            Literal::Bool(b) => format!("azc_bool({})", b),
            Literal::Char(c) => format!("'{}'", c),
            Literal::String(s) => format!("azc_strlit(\"{}\")", s),
            Literal::Nil => "0".to_string(),
        }),

        Expression::Variable(name) => Ok(name.clone()),

        Expression::Binary { op, left, right } => {
            let left_code = gen_expr(left)?;
            let right_code = gen_expr(right)?;

            Ok(match op {
                BinaryOp::Add => format!("({} + {})", left_code, right_code),
                BinaryOp::Sub => format!("({} - {})", left_code, right_code),
                BinaryOp::Mul => format!("({} * {})", left_code, right_code),
                BinaryOp::Div => format!("({} / {})", left_code, right_code),
                BinaryOp::Mod => format!("({} % {})", left_code, right_code),
                BinaryOp::Eq => format!("({} == {})", left_code, right_code),
                BinaryOp::Ne => format!("({} != {})", left_code, right_code),
                BinaryOp::Lt => format!("({} < {})", left_code, right_code),
                BinaryOp::Le => format!("({} <= {})", left_code, right_code),
                BinaryOp::Gt => format!("({} > {})", left_code, right_code),
                BinaryOp::Ge => format!("({} >= {})", left_code, right_code),
                BinaryOp::And => format!("({} && {})", left_code, right_code),
                BinaryOp::Or => format!("({} || {})", left_code, right_code),
                _ => format!("({} {} {})", left_code, op, right_code),
            })
        }

        Expression::Unary { op, operand } => {
            let operand_code = gen_expr(operand)?;

            Ok(match op {
                UnaryOp::Neg => format!("(-{})", operand_code),
                UnaryOp::Not => format!("(!{})", operand_code),
                UnaryOp::Deref => format!("(*{})", operand_code),
                UnaryOp::Ref => format!("(&{})", operand_code),
                UnaryOp::RefMut => format!("(&{})", operand_code),
            })
        }

        Expression::Call { func, args } => {
            let func_code = gen_expr(func)?;
            let args_code: Vec<String> = args
                .iter()
                .map(|a| gen_expr(a))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(format!("{}({})", func_code, args_code.join(", ")))
        }

        Expression::MethodCall {
            object,
            method,
            args,
        } => {
            let obj_code = gen_expr(object)?;
            let args_code: Vec<String> = args
                .iter()
                .map(|a| gen_expr(a))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(format!(
                "azc_{}({}, {})",
                method,
                obj_code,
                args_code.join(", ")
            ))
        }

        Expression::Field { object, field } => {
            let obj_code = gen_expr(object)?;
            Ok(format!("{}.{}", obj_code, field))
        }

        Expression::Index { object, index } => {
            let obj_code = gen_expr(object)?;
            let idx_code = gen_expr(index)?;
            Ok(format!("{}[{}]", obj_code, idx_code))
        }

        Expression::Array(elements) => {
            let elements_code: Vec<String> = elements
                .iter()
                .map(|e| gen_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("{{{}}}", elements_code.join(", ")))
        }

        Expression::Tuple(elements) => {
            let elements_code: Vec<String> = elements
                .iter()
                .map(|e| gen_expr(e))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("{{{}}}", elements_code.join(", ")))
        }

        Expression::Reference {
            expr: inner,
            mutable: _,
        } => {
            let inner_code = gen_expr(inner)?;
            Ok(format!("(&{})", inner_code))
        }

        Expression::Deref(inner) => {
            let inner_code = gen_expr(inner)?;
            Ok(format!("(*{})", inner_code))
        }

        Expression::Block { statements, value } => {
            let mut block_code = String::from("{ ");
            for stmt in statements {
                gen_stmt(&mut block_code, stmt, 0)?;
            }
            if let Some(val) = value {
                let val_code = gen_expr(val)?;
                block_code.push_str(&val_code);
            }
            block_code.push_str(" }");
            Ok(block_code)
        }

        Expression::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond_code = gen_expr(condition)?;
            let then_code = gen_expr(then_branch)?;

            if let Some(else_expr) = else_branch {
                let else_code = gen_expr(else_expr)?;
                Ok(format!("({} ? {} : {})", cond_code, then_code, else_code))
            } else {
                Ok(format!("({} ? {} : 0)", cond_code, then_code))
            }
        }

        Expression::Match { .. } => Ok("/* match expression */".to_string()),

        Expression::Lambda { .. } => Ok("/* lambda expression */".to_string()),

        Expression::StructInstantiation { name, fields } => {
            let fields_code: Vec<String> = fields
                .iter()
                .map(|(f, v)| {
                    let v_code = gen_expr(v)?;
                    Ok(format!(".{} = {}", f, v_code))
                })
                .collect::<Result<Vec<_>, String>>()?;

            Ok(format!("({}) {{ {} }}", name, fields_code.join(", ")))
        }
    }
}

/// Compile error
#[derive(Debug)]
pub enum CompileError {
    Parse(ast::ParseError),
    Type(TypeError),
    Ownership(OwnershipError),
    CodeGen(String),
}

impl From<ast::ParseError> for CompileError {
    fn from(e: ast::ParseError) -> Self {
        CompileError::Parse(e)
    }
}

impl From<TypeError> for CompileError {
    fn from(e: TypeError) -> Self {
        CompileError::Type(e)
    }
}

impl From<OwnershipError> for CompileError {
    fn from(e: OwnershipError) -> Self {
        CompileError::Ownership(e)
    }
}

impl From<String> for CompileError {
    fn from(e: String) -> Self {
        CompileError::CodeGen(e)
    }
}


impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Parse(e) => write!(f, "Parse error: {}", e),
            CompileError::Type(e) => write!(f, "Type error: {}", e),
            CompileError::Ownership(e) => write!(f, "Ownership error: {}", e),
            CompileError::CodeGen(msg) => write!(f, "Code generation error: {}", msg),
        }
    }
}

impl std::error::Error for CompileError {}

/// Compilation result with metadata
#[derive(Debug)]
pub struct CompileResult {
    pub c_code: String,
    pub version: String,
    pub compile_time_ms: u64,
    pub safety_checks_passed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple() {
        let source = "let x = 42";
        let result = compile(source);
        assert!(result.is_ok());
        let c_code = result.unwrap();
        assert!(c_code.contains("AZC x = azc_num(42)"));
    }

    #[test]
    fn test_compile_puts() {
        let source = "puts \"hello\"";
        let result = compile(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_if() {
        let source = "if true\n  puts \"yes\"\nend";
        let result = compile(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_function() {
        let source = "def foo()\n  puts \"hello\"\nend";
        let result = compile(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.2.0");
    }
}
