use crate::{
    Atom,
    ast::{
        Module, 
        decl::{Decl, FnDecl, ClassDecl, GlobalConstDecl},
        Expr, Stmt, Typ,
        stmt::{ArgStmt, VarKind},
        operators::BinaryOperator,
    },
    lexer::{Token, Keyword}
};

/// Parser structure to maintain state during parsing
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        // Filter out whitespace and comments for easier parsing
        let tokens: Vec<Token> = tokens
            .into_iter()
            .filter(|t| !matches!(t, Token::Whitespace | Token::CommentLine))
            .collect();
        
        Self { tokens, position: 0 }
    }

    /// Get current token without consuming it
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Get token at offset from current position
    fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    /// Consume and return current token
    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Check if current token matches expected token
    fn check(&self, expected: &Token) -> bool {
        self.peek().map_or(false, |t| t == expected)
    }

    /// Check if current token is a keyword
    fn check_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.peek(), Some(Token::Keyword(k)) if *k == keyword)
    }

    /// Consume token if it matches expected, otherwise return error
    fn expect(&mut self, expected: Token) -> Result<Token, String> {
        match self.advance() {
            Some(token) if token == expected => Ok(token),
            Some(token) => Err(format!("Expected {:?}, found {:?}", expected, token)),
            None => Err(format!("Expected {:?}, found EOF", expected)),
        }
    }

    /// Check if we've reached end of tokens
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    /// Parse the entire module
    fn parse_module(&mut self) -> Result<Module, String> {
        let mut decls = Vec::new();

        while !self.is_at_end() {
            match self.parse_declaration() {
                Ok(decl) => decls.push(decl),
                Err(e) => {
                    eprintln!("Parse error: {}", e);
                    self.synchronize();
                }
            }
        }

        Ok(Module::new(decls))
    }

    /// Parse a top-level declaration
    fn parse_declaration(&mut self) -> Result<Decl, String> {
        // Check for visibility modifiers
        let is_public = if self.check_keyword(Keyword::Public) {
            self.advance();
            true
        } else {
            false
        };

        // Determine declaration type based on keyword
        match self.peek() {
            Some(Token::Keyword(Keyword::Function)) => {
                self.advance();
                self.parse_function_declaration(is_public)
            }
            Some(Token::Keyword(Keyword::Class)) => {
                self.advance();
                self.parse_class_declaration(is_public)
            }
            Some(Token::Keyword(Keyword::Const)) => {
                self.advance();
                self.parse_const_declaration()
            }
            Some(Token::Keyword(Keyword::Let)) => {
                Err("'let' declarations are not allowed at module level".to_string())
            }
            Some(token) => Err(format!("Unexpected token at module level: {:?}", token)),
            None => Err("Unexpected end of file".to_string()),
        }
    }

    /// Parse function declaration
    /// Example: function add(a: number, b: number): number { return a + b; }
    fn parse_function_declaration(&mut self, is_pub: bool) -> Result<Decl, String> {
        // Parse function name
        let ident = match self.advance() {
            Some(Token::Ident(name)) => name,
            other => return Err(format!("Expected function name, found {:?}", other)),
        };

        // Parse parameter list
        self.expect(Token::LeftParen)?;
        let args = self.parse_parameter_list()?;
        self.expect(Token::RightParen)?;

        // Parse optional return type annotation
        let returns = if self.check(&Token::Assign) && self.peek_ahead(1) == Some(&Token::GreaterThan) {
            // "=>" syntax for return type
            self.advance(); // consume "="
            self.advance(); // consume ">"
            self.parse_type()?
        } else {
            Typ::Void
        };

        // Parse function body
        self.expect(Token::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(Token::RightBrace)?;

        Ok(Decl::FnDecl(FnDecl::new(is_pub, ident, args, returns, body)))
    }

    /// Parse class declaration
    /// Example: class MyClass extends Parent { }
    fn parse_class_declaration(&mut self, is_pub: bool) -> Result<Decl, String> {
        // Parse class name
        let ident = match self.advance() {
            Some(Token::Ident(name)) => name,
            other => return Err(format!("Expected class name, found {:?}", other)),
        };

        // Optional extends clause
        let extends = if self.check_keyword(Keyword::Extends) {
            self.advance();
            match self.advance() {
                Some(Token::Ident(name)) => Some(name),
                other => return Err(format!("Expected parent class name, found {:?}", other)),
            }
        } else {
            None
        };

        // Optional implements clause
        let implements = if self.check_keyword(Keyword::Implements) {
            self.advance();
            let mut interfaces = Vec::new();
            loop {
                match self.advance() {
                    Some(Token::Ident(name)) => interfaces.push(name),
                    other => return Err(format!("Expected interface name, found {:?}", other)),
                }
                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
            Some(interfaces)
        } else {
            None
        };

        // Parse class body
        self.expect(Token::LeftBrace)?;
        let mut fields = Vec::new();
        
        // Parse class members (simplified - just collect field names)
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if let Some(Token::Ident(field_name)) = self.peek() {
                fields.push(field_name.clone());
            }
            self.advance();
        }
        self.expect(Token::RightBrace)?;

        Ok(Decl::ClassDecl(ClassDecl::new(is_pub, ident, extends, implements, fields)))
    }

    /// Parse const declaration
    /// Example: const PI = 3.14;
    fn parse_const_declaration(&mut self) -> Result<Decl, String> {
        // Parse constant name
        let _name = match self.advance() {
            Some(Token::Ident(name)) => name,
            other => return Err(format!("Expected constant name, found {:?}", other)),
        };

        // Expect assignment
        self.expect(Token::Assign)?;

        // Parse initializer expression
        let _value = self.parse_expression()?;

        // Expect semicolon
        self.expect(Token::Semicolon)?;

        Ok(Decl::GlobalConstDecl(GlobalConstDecl::new()))
    }

    /// Parse parameter list for functions
    /// Example: (a: number, b: string)
    fn parse_parameter_list(&mut self) -> Result<Vec<ArgStmt>, String> {
        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                // Parse parameter name
                let ident = match self.advance() {
                    Some(Token::Ident(name)) => name,
                    other => return Err(format!("Expected parameter name, found {:?}", other)),
                };

                // Parse optional type annotation
                let type_kind = if self.check(&Token::Assign) && self.peek_ahead(1) == Some(&Token::GreaterThan) {
                    // Skip "=>" for now, just parse type
                    self.advance(); // "="
                    self.advance(); // ">"
                    self.parse_type()?
                } else {
                    Typ::Void // Default type if not specified
                };

                params.push(ArgStmt::new(ident, type_kind));

                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance(); // consume comma
            }
        }

        Ok(params)
    }

    /// Parse a type annotation
    /// Example: number, string, MyClass
    fn parse_type(&mut self) -> Result<Typ, String> {
        match self.advance() {
            Some(Token::Ident(type_name)) => {
                // Map common type names to Typ enum
                // TODO: may be bottleneck
                match type_name.as_str().to_string().as_str() {
                    "void" => Ok(Typ::Void),
                    "boolean" | "bool" => Ok(Typ::Boolean),
                    "number" => Ok(Typ::Number),
                    "i8" => Ok(Typ::I8),
                    "i16" => Ok(Typ::I16),
                    "i32" => Ok(Typ::I32),
                    "i64" => Ok(Typ::I64),
                    "i128" => Ok(Typ::I128),
                    "u8" => Ok(Typ::U8),
                    "u16" => Ok(Typ::U16),
                    "u32" => Ok(Typ::U32),
                    "u64" => Ok(Typ::U64),
                    "u128" => Ok(Typ::U128),
                    "f16" => Ok(Typ::F16),
                    "f32" => Ok(Typ::F32),
                    "f64" => Ok(Typ::F64),
                    "f128" => Ok(Typ::F128),
                    "string" => Ok(Typ::String),
                    _ => Ok(Typ::TypeLiteral(type_name)),
                }
            }
            other => Err(format!("Expected type name, found {:?}", other)),
        }
    }

    /// Parse a block of statements
    fn parse_stmt_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    eprintln!("Statement parse error: {}", e);
                    self.advance(); // Skip problematic token
                }
            }
        }

        Ok(statements)
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Some(Token::Keyword(Keyword::Let)) => {
                self.advance();
                self.parse_var_declaration(VarKind::Let)
            }
            Some(Token::Keyword(Keyword::Const)) => {
                self.advance();
                self.parse_var_declaration(VarKind::Const)
            }
            Some(Token::Keyword(Keyword::Return)) => {
                self.advance();
                // Parse return expression
                let _expr = if !self.check(&Token::Semicolon) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                self.expect(Token::Semicolon)?;
                // For now, return a simple expression statement
                Ok(Stmt::ExprStmt(Expr::NumberLiteral(Atom::from("0"))))
            }
            _ => {
                // Expression statement
                let expr = self.parse_expression()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    /// Parse variable declaration
    /// Example: let x = 5;
    fn parse_var_declaration(&mut self, kind: VarKind) -> Result<Stmt, String> {
        let name = match self.advance() {
            Some(Token::Ident(name)) => name,
            other => return Err(format!("Expected variable name, found {:?}", other)),
        };

        let init = if self.check(&Token::Assign) {
            self.advance(); // consume "="
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(Token::Semicolon)?;

        Ok(Stmt::VarDecl { kind, name, init })
    }

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    /// Parse additive expressions (+ and -)
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;

        while matches!(self.peek(), Some(Token::Plus) | Some(Token::Minus)) {
            let op = match self.advance() {
                Some(Token::Plus) => BinaryOperator::Add,
                Some(Token::Minus) => BinaryOperator::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse multiplicative expressions (* and /)
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_call()?;

        while matches!(self.peek(), Some(Token::Multiply) | Some(Token::Divide)) {
            let op = match self.advance() {
                Some(Token::Multiply) => BinaryOperator::Mul,
                Some(Token::Divide) => BinaryOperator::Div,
                _ => unreachable!(),
            };
            let right = self.parse_call()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse call expressions
    /// Example: foo(1, 2, 3)
    fn parse_call(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_member()?;

        while self.check(&Token::LeftParen) {
            self.advance(); // consume "("
            let mut args = Vec::new();

            if !self.check(&Token::RightParen) {
                loop {
                    args.push(self.parse_expression()?);
                    if !self.check(&Token::Comma) {
                        break;
                    }
                    self.advance(); // consume comma
                }
            }

            self.expect(Token::RightParen)?;
            expr = Expr::Call {
                callee: Box::new(expr),
                args,
            };
        }

        Ok(expr)
    }

    /// Parse member access expressions
    /// Example: obj.prop
    fn parse_member(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        while self.check(&Token::Dot) {
            self.advance(); // consume "."
            let property = self.parse_primary()?;
            expr = Expr::Member {
                object: Box::new(expr),
                properties: Some(vec![property]),
            };
        }

        Ok(expr)
    }

    /// Parse primary expressions (literals, identifiers, parenthesized expressions)
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::NumberLiteral(val)) => {
                let val = val.clone();
                self.advance();
                Ok(Expr::NumberLiteral(val))
            }
            Some(Token::StringLiteral(val)) => {
                let val = val.clone();
                self.advance();
                Ok(Expr::StringLiteral(val))
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            Some(Token::LeftParen) => {
                self.advance(); // consume "("
                let expr = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok(expr)
            }
            Some(Token::LeftBrace) => {
                self.parse_object_literal()
            }
            Some(token) => Err(format!("Unexpected token in expression: {:?}", token)),
            None => Err("Unexpected end of file in expression".to_string()),
        }
    }

    /// Parse object literal
    /// Example: { x: 1, y: 2 }
    fn parse_object_literal(&mut self) -> Result<Expr, String> {
        self.expect(Token::LeftBrace)?;
        let mut properties = Vec::new();

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            // Parse key
            let key = match self.advance() {
                Some(Token::Ident(name)) => name,
                other => return Err(format!("Expected property key, found {:?}", other)),
            };

            // Expect colon (we'll use "=" as a substitute if needed)
            if !self.check(&Token::Assign) {
                return Err("Expected ':' or '=' after property key".to_string());
            }
            self.advance();

            // Parse value
            let value = self.parse_expression()?;

            properties.push(crate::ast::Property { key, value });

            if !self.check(&Token::Comma) {
                break;
            }
            self.advance(); // consume comma
        }

        self.expect(Token::RightBrace)?;
        Ok(Expr::ObjectLiteral(properties))
    }

    /// Synchronize parser after error (panic mode recovery)
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            // Look for statement/declaration boundaries
            if matches!(
                self.peek(),
                Some(Token::Keyword(Keyword::Function))
                    | Some(Token::Keyword(Keyword::Class))
                    | Some(Token::Keyword(Keyword::Const))
                    | Some(Token::Keyword(Keyword::Let))
                    | Some(Token::Keyword(Keyword::Return))
            ) {
                return;
            }

            self.advance();
        }
    }
}

/// Main entry point: parse a vector of tokens into a Module
pub fn parse_module(tokens: Vec<Token>) -> Module {
    let mut parser = Parser::new(tokens);
    
    match parser.parse_module() {
        Ok(module) => module,
        Err(e) => {
            eprintln!("Fatal parse error: {}", e);
            // Return empty module on error
            Module::new(Vec::new())
        }
    }
}