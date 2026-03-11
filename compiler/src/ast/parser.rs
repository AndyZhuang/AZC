//! AST Parser
//!
//! Parses AZC source code into an AST.

use super::nodes::*;

/// Parse AZC source code
pub fn parse(source: &str) -> Result<Program, ParseError> {
    let mut parser = Parser::new(source);
    parser.parse_program()
}

/// Parse error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}

/// Parser state
struct Parser {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Parser {
    fn new(source: &str) -> Self {
        Parser {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::new();

        while !self.is_at_end() {
            self.skip_whitespace_and_comments();

            if self.is_at_end() {
                break;
            }

            if let Some(stmt) = self.parse_statement()? {
                program.statements.push(stmt);
            }
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Ok(None);
        }

        let keyword = self.peek_word();

        match keyword.as_str() {
            "let" => self.parse_let(),
            "def" => self.parse_function(),
            "if" => self.parse_if_statement(),
            "while" => self.parse_while(),
            "return" => self.parse_return(),
            "class" | "struct" => self.parse_struct(),
            "enum" => self.parse_enum(),
            "impl" => self.parse_impl(),
            "end" => Ok(None),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("let")?;
        self.skip_whitespace();

        let name = self.parse_identifier()?;
        self.skip_whitespace();

        let type_annotation = if self.check(':') {
            self.advance();
            self.skip_whitespace();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.skip_whitespace();

        let value = if self.check('=') {
            self.advance();
            self.skip_whitespace();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Some(Statement::Let {
            name,
            type_annotation,
            value,
        }))
    }

    fn parse_function(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("def")?;
        self.skip_whitespace();

        let name = self.parse_identifier()?;
        self.skip_whitespace();
        
        let params = if self.check('(') {
            self.advance();
            let p = self.parse_params()?;
            self.expect(')')?;
            p
        } else {
            Vec::new()
        };
        self.skip_whitespace();

        let return_type = if self.check_str("->") {
            self.advance();
            self.advance();
            self.skip_whitespace();
            Some(self.parse_type()?)
        } else {
            None
        };

        let mut body = Vec::new();

        while !self.is_at_end() && !self.check_word("end") {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        if !self.is_at_end() {
            self.consume_word("end")?;
        }

        Ok(Some(Statement::Function {
            name,
            params,
            return_type,
            body,
        }))
    }

    fn parse_if_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("if")?;
        self.skip_whitespace();

        let condition = self.parse_expression()?;
        self.skip_whitespace();

        let mut then_branch = Vec::new();
        let mut else_branch = None;

        while !self.is_at_end() && !self.check_word("else") && !self.check_word("end") {
            if let Some(stmt) = self.parse_statement()? {
                then_branch.push(stmt);
            }
        }

        if self.check_word("else") {
            self.consume_word("else")?;
            let mut else_body = Vec::new();

            while !self.is_at_end() && !self.check_word("end") {
                if let Some(stmt) = self.parse_statement()? {
                    else_body.push(stmt);
                }
            }

            else_branch = Some(else_body);
        }

        if !self.is_at_end() {
            self.consume_word("end")?;
        }

        Ok(Some(Statement::If {
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn parse_while(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("while")?;
        self.skip_whitespace();

        let condition = self.parse_expression()?;
        self.skip_whitespace();

        let mut body = Vec::new();

        while !self.is_at_end() && !self.check_word("end") {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        if !self.is_at_end() {
            self.consume_word("end")?;
        }

        Ok(Some(Statement::While { condition, body }))
    }

    fn parse_return(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("return")?;
        self.skip_whitespace();

        let value = if self.is_at_end() || self.check('\n') {
            None
        } else {
            Some(self.parse_expression()?)
        };

        Ok(Some(Statement::Return(value)))
    }

    fn parse_struct(&mut self) -> Result<Option<Statement>, ParseError> {
        let keyword = self.peek_word();
        self.consume_word(&keyword)?;
        self.skip_whitespace();

        let name = self.parse_identifier()?;
        self.skip_whitespace();

        let mut fields = Vec::new();

        while !self.is_at_end() && !self.check_word("end") {
            self.skip_whitespace_and_comments();
            if self.check_word("end") {
                break;
            }

            let field_name = self.parse_identifier()?;
            self.skip_whitespace();
            self.expect(':')?;
            self.skip_whitespace();
            let field_type = self.parse_type()?;

            fields.push((field_name, field_type));
            self.skip_whitespace();
        }

        if !self.is_at_end() {
            self.consume_word("end")?;
        }

        Ok(Some(Statement::Struct { name, fields }))
    }

    fn parse_enum(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("enum")?;
        self.skip_whitespace();

        let name = self.parse_identifier()?;
        self.skip_whitespace();

        let mut variants = Vec::new();

        while !self.is_at_end() && !self.check_word("end") {
            self.skip_whitespace_and_comments();
            if self.check_word("end") {
                break;
            }

            let variant_name = self.parse_identifier()?;
            self.skip_whitespace();

            let fields = if self.check('(') {
                self.advance();
                let mut types = Vec::new();

                while !self.check(')') {
                    self.skip_whitespace();
                    types.push(self.parse_type()?);
                    self.skip_whitespace();

                    if self.check(',') {
                        self.advance();
                    }
                }

                self.expect(')')?;
                Some(types)
            } else {
                None
            };

            variants.push((variant_name, fields));
            self.skip_whitespace();
        }

        if !self.is_at_end() {
            self.consume_word("end")?;
        }

        Ok(Some(Statement::Enum { name, variants }))
    }

    fn parse_impl(&mut self) -> Result<Option<Statement>, ParseError> {
        self.consume_word("impl")?;
        self.skip_whitespace();

        let target = self.parse_identifier()?;
        self.skip_whitespace();

        let mut methods = Vec::new();

        while !self.is_at_end() && !self.check_word("end") {
            if let Some(stmt) = self.parse_statement()? {
                methods.push(stmt);
            }
        }

        if !self.is_at_end() {
            self.consume_word("end")?;
        }

        Ok(Some(Statement::Impl { target, methods }))
    }

    fn parse_expr_statement(&mut self) -> Result<Option<Statement>, ParseError> {
        let expr = self.parse_expression()?;

        if self.check('=') && !self.check_str("==") {
            self.advance();
            self.skip_whitespace();
            let value = self.parse_expression()?;
            Ok(Some(Statement::Assign {
                target: expr,
                value,
            }))
        } else {
            Ok(Some(Statement::Expr(expr)))
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_and()?;

        while self.check_word("or") {
            self.consume_word("or")?;
            self.skip_whitespace();
            let right = self.parse_and()?;
            left = Expression::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_comparison()?;

        while self.check_word("and") {
            self.consume_word("and")?;
            self.skip_whitespace();
            let right = self.parse_comparison()?;
            left = Expression::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_additive()?;

        loop {
            let op = if self.check_str("==") {
                self.advance();
                self.advance();
                BinaryOp::Eq
            } else if self.check_str("!=") {
                self.advance();
                self.advance();
                BinaryOp::Ne
            } else if self.check_str("<=") {
                self.advance();
                self.advance();
                BinaryOp::Le
            } else if self.check_str(">=") {
                self.advance();
                self.advance();
                BinaryOp::Ge
            } else if self.check('<') {
                self.advance();
                BinaryOp::Lt
            } else if self.check('>') {
                self.advance();
                BinaryOp::Gt
            } else {
                break;
            };

            self.skip_whitespace();
            let right = self.parse_additive()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = if self.check('+') {
                self.advance();
                BinaryOp::Add
            } else if self.check('-') {
                self.advance();
                BinaryOp::Sub
            } else {
                break;
            };

            self.skip_whitespace();
            let right = self.parse_multiplicative()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_unary()?;

        loop {
            let op = if self.check('*') && !self.peek_is('*') {
                self.advance();
                BinaryOp::Mul
            } else if self.check('/') {
                self.advance();
                BinaryOp::Div
            } else if self.check('%') {
                self.advance();
                BinaryOp::Mod
            } else {
                break;
            };

            self.skip_whitespace();
            let right = self.parse_unary()?;
            left = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.check('-') {
            self.advance();
            self.skip_whitespace();
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            });
        }

        if self.check_word("not") {
            self.consume_word("not")?;
            self.skip_whitespace();
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            });
        }

        if self.check_str("&mut") {
            self.consume_str("&mut")?;
            self.skip_whitespace();
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::RefMut,
                operand: Box::new(operand),
            });
        }

        if self.check('&') {
            self.advance();
            self.skip_whitespace();
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::Ref,
                operand: Box::new(operand),
            });
        }

        if self.check('*') && !self.peek_is('*') {
            self.advance();
            self.skip_whitespace();
            let operand = self.parse_unary()?;
            return Ok(Expression::Unary {
                op: UnaryOp::Deref,
                operand: Box::new(operand),
            });
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            self.skip_whitespace();

            if self.check('(') {
                self.advance();
                let args = self.parse_args()?;
                self.expect(')')?;
                expr = Expression::Call {
                    func: Box::new(expr),
                    args,
                };
            } else if self.check('.') {
                self.advance();
                self.skip_whitespace();
                let field = self.parse_identifier()?;

                if self.check('(') {
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(')')?;
                    expr = Expression::MethodCall {
                        object: Box::new(expr),
                        method: field,
                        args,
                    };
                } else {
                    expr = Expression::Field {
                        object: Box::new(expr),
                        field,
                    };
                }
            } else if self.check('[') {
                self.advance();
                self.skip_whitespace();
                let index = self.parse_expression()?;
                self.skip_whitespace();
                self.expect(']')?;
                expr = Expression::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        self.skip_whitespace();

        if self.check('"') {
            return self.parse_string();
        }

        if self.check('\'') {
            return self.parse_char();
        }

        if self.check_digit() {
            return self.parse_number();
        }

        if self.check_word("true") {
            self.consume_word("true")?;
            return Ok(Expression::Literal(Literal::Bool(true)));
        }

        if self.check_word("false") {
            self.consume_word("false")?;
            return Ok(Expression::Literal(Literal::Bool(false)));
        }

        if self.check_word("nil") {
            self.consume_word("nil")?;
            return Ok(Expression::Literal(Literal::Nil));
        }

        if self.check('[') {
            return self.parse_array();
        }

        if self.check('(') {
            return self.parse_paren();
        }

        if self.check('{') {
            return self.parse_block();
        }

        let word = self.peek_word();
        if !word.is_empty() {
            self.consume_word(&word)?;
            return Ok(Expression::Variable(word));
        }

        Err(self.error("Expected expression"))
    }

    fn parse_number(&mut self) -> Result<Expression, ParseError> {
        let mut num_str = String::new();

        while self.check_digit() {
            num_str.push(self.advance());
        }

        if self.check('.') {
            num_str.push(self.advance());

            while self.check_digit() {
                num_str.push(self.advance());
            }

            let value = num_str
                .parse::<f64>()
                .map_err(|_| self.error("Invalid float literal"))?;
            return Ok(Expression::Literal(Literal::Float(value)));
        }

        let value = num_str
            .parse::<i64>()
            .map_err(|_| self.error("Invalid integer literal"))?;
        Ok(Expression::Literal(Literal::Int(value)))
    }

    fn parse_string(&mut self) -> Result<Expression, ParseError> {
        self.expect('"')?;
        let mut s = String::new();

        while !self.check('"') && !self.is_at_end() {
            if self.check('\\') {
                self.advance();
                if let Some(c) = self.escape_char()? {
                    s.push(c);
                }
            } else {
                s.push(self.advance());
            }
        }

        self.expect('"')?;
        Ok(Expression::Literal(Literal::String(s)))
    }

    fn parse_char(&mut self) -> Result<Expression, ParseError> {
        self.expect('\'')?;

        let c = if self.check('\\') {
            self.advance();
            self.escape_char()?.unwrap_or(' ')
        } else {
            self.advance()
        };

        self.expect('\'')?;
        Ok(Expression::Literal(Literal::Char(c)))
    }

    fn escape_char(&mut self) -> Result<Option<char>, ParseError> {
        let c = self.advance();
        Ok(Some(match c {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            '0' => '\0',
            _ => return Err(self.error("Invalid escape sequence")),
        }))
    }

    fn parse_array(&mut self) -> Result<Expression, ParseError> {
        self.expect('[')?;
        self.skip_whitespace();

        let mut elements = Vec::new();

        if !self.check(']') {
            elements.push(self.parse_expression()?);

            while self.check(',') {
                self.advance();
                self.skip_whitespace();
                if self.check(']') {
                    break;
                }
                elements.push(self.parse_expression()?);
            }
        }

        self.expect(']')?;
        Ok(Expression::Array(elements))
    }

    fn parse_paren(&mut self) -> Result<Expression, ParseError> {
        self.expect('(')?;
        self.skip_whitespace();

        let expr = self.parse_expression()?;
        self.skip_whitespace();

        self.expect(')')?;
        Ok(expr)
    }

    fn parse_block(&mut self) -> Result<Expression, ParseError> {
        self.expect('{')?;
        self.skip_whitespace();

        let mut statements = Vec::new();
        let mut value = None;

        while !self.check('}') && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
            self.skip_whitespace();
        }

        self.expect('}')?;

        Ok(Expression::Block { statements, value })
    }

    fn parse_params(&mut self) -> Result<Vec<(String, Option<String>)>, ParseError> {
        let mut params = Vec::new();

        self.skip_whitespace();

        if self.check(')') {
            return Ok(params);
        }

        loop {
            self.skip_whitespace();
            let name = self.parse_identifier()?;
            self.skip_whitespace();

            let type_ann = if self.check(':') {
                self.advance();
                self.skip_whitespace();
                Some(self.parse_type()?)
            } else {
                None
            };

            params.push((name, type_ann));

            self.skip_whitespace();
            if !self.check(',') {
                break;
            }
            self.advance();
        }

        Ok(params)
    }

    fn parse_args(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();

        self.skip_whitespace();

        if self.check(')') {
            return Ok(args);
        }

        loop {
            self.skip_whitespace();
            args.push(self.parse_expression()?);
            self.skip_whitespace();

            if !self.check(',') {
                break;
            }
            self.advance();
        }

        Ok(args)
    }

    fn parse_type(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace();
        let mut ty = String::new();

        if self.check('&') {
            ty.push(self.advance());
            if self.check_str("mut") {
                ty.push_str("mut ");
                self.consume_str("mut")?;
            }
        }

        let name = self.parse_identifier()?;
        ty.push_str(&name);

        if self.check('<') {
            ty.push(self.advance());
            ty.push_str(&self.parse_type()?);
            while self.check(',') {
                ty.push(self.advance());
                ty.push_str(&self.parse_type()?);
            }
            self.expect('>')?;
            ty.push('>');
        }

        Ok(ty)
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let mut ident = String::new();

        while let Some(&c) = self.source.get(self.pos) {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if ident.is_empty() {
            return Err(self.error("Expected identifier"));
        }

        Ok(ident)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();

            if self.check('#') {
                while !self.check('\n') && !self.is_at_end() {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.get(self.pos) {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn peek_word(&self) -> String {
        let mut word = String::new();
        let mut pos = self.pos;

        while let Some(&c) = self.source.get(pos) {
            if c.is_alphanumeric() || c == '_' {
                word.push(c);
                pos += 1;
            } else {
                break;
            }
        }

        word
    }

    fn consume_word(&mut self, word: &str) -> Result<(), ParseError> {
        for c in word.chars() {
            if self.source.get(self.pos) != Some(&c) {
                return Err(self.error(&format!("Expected '{}'", c)));
            }
            self.advance();
        }
        Ok(())
    }

    fn consume_str(&mut self, s: &str) -> Result<(), ParseError> {
        for c in s.chars() {
            if self.source.get(self.pos) != Some(&c) {
                return Err(self.error(&format!("Expected '{}'", s)));
            }
            self.advance();
        }
        Ok(())
    }

    fn check(&self, c: char) -> bool {
        self.source.get(self.pos) == Some(&c)
    }

    fn check_str(&self, s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if self.source.get(self.pos + i) != Some(&c) {
                return false;
            }
        }
        true
    }

    fn check_word(&self, word: &str) -> bool {
        let peeked = self.peek_word();
        peeked == word
    }

    fn check_digit(&self) -> bool {
        self.source
            .get(self.pos)
            .map_or(false, |c| c.is_ascii_digit())
    }

    fn peek_is(&self, c: char) -> bool {
        self.source.get(self.pos).map_or(false, |&ch| ch == c)
    }

    fn advance(&mut self) -> char {
        let c = self.source.get(self.pos).copied().unwrap_or('\0');
        self.pos += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn expect(&mut self, c: char) -> Result<(), ParseError> {
        if self.check(c) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("Expected '{}'", c)))
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            line: self.line,
            column: self.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
        let program = parse("42").unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_let() {
        let program = parse("let x = 10").unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let program = parse("def foo()\n  puts \"hello\"\nend").unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_if() {
        let program = parse("if true\n  puts \"yes\"\nend").unwrap();
        assert_eq!(program.statements.len(), 1);
    }
}
