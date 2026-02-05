use crate::{
    ast::{ast::Ast, Binding, FunctionDecl, TypeRef},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_function_decl(&mut self) -> Ast<FunctionDecl> {
        println!("started parsing of function_decl");
        /*  parsing of function name Ident */
        let ident = self.parse_ident();
        if ident.is_err() {
            self.error(ident.clone().err().unwrap());
            return Ast::Error(ident.err().unwrap());
        }
        println!("fn ident parsed");

        /* parsing arguments of function like: "(a: i32, s: String)" */
        let args = self.parse_arguments_with_parens();
        if args.is_err() {
            return Ast::Error(args.err().unwrap());
        }
        println!("fn args parsed");
        /* parsing for function return type */
        let returns = self.parse_returns();
        if returns.is_err() {
            return Ast::Error(returns.err().unwrap());
        }

        println!("fn returns type parsed");
        let body = self.parse_block_stmt();

        Ast::Parsed(FunctionDecl::new(
            ident.unwrap(),
            args.unwrap(),
            returns.unwrap(),
            body,
        ))
    }

    fn parse_arguments_with_parens(&mut self) -> Result<Vec<Ast<Binding>>, String> {
        /* check for "(" existing */
        self.consume(TokenKind::LeftParen);

        /* check for Ident of argument parse starting or is closing with ")" */
        match self.current().kind {
            TokenKind::RightParen => {
                self.consume(TokenKind::RightParen);
                return Ok(vec![]);
            }
            TokenKind::Ident(_) => {
                let args = self.parse_arguments();

                if args.is_ok() {
                    return Ok(args.unwrap());
                } else {
                    return Err(args.err().unwrap());
                }
            }
            e => {
                self.error(format!(
                    "can not use {:?} in function arguments declaration",
                    e
                ));
                return Err(format!(
                    "can not use {:?} in function arguments declaration",
                    e
                ));
            }
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Ast<Binding>>, String> {
        let typed_bindings = self.parse_typed_bindings();
        match self.current().kind {
            TokenKind::RightParen => {
                self.consume(TokenKind::RightParen);
                return Ok(typed_bindings);
            }
            e => {
                self.error(format!("expected ')' but found {:?}", e));
                return Err(format!("expected ')' but found {:?}", e));
            }
        }
    }

    fn parse_returns(&mut self) -> Result<TypeRef, String> {
        println!("started parse fn returns");
        match self.current().kind {
            TokenKind::Colon => {
                self.consume(TokenKind::Colon);
                println!("Colon parsed");
            }
            e => {
                self.error(format!(
                    "expected ':' for declare return type of function, but found {:?}",
                    e
                ));
                return Err(format!(
                    "expected ':' for declare return type of function, but found {:?}",
                    e
                ));
            }
        }

        self.parse_type_ref()
    }
}
