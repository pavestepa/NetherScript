use crate::{
    ast::{ast::Ast, FunctionDecl, Ident, TypeRef, TypedBinding},
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

        /* parsing arguments of function like: "(a: i32, s: String)" */
        let args = self.parse_arguments_with_parens();
        if args.is_err() {
            return Ast::Error(args.err().unwrap());
        }

        /* parsing for function return type */
        let returns = self.parse_returns();
        if returns.is_err() {
            return Ast::Error(returns.err().unwrap());
        }

        let body = self.parse_block_stmt();

        Ast::Parsed(FunctionDecl::new(
            ident.unwrap(),
            args.unwrap(),
            returns.unwrap(),
            body,
        ))
    }

    fn parse_arguments_with_parens(&mut self) -> Result<Vec<Ast<TypedBinding>>, String> {
        /* check for "(" existing */
        match self.token() {
            TokenKind::LeftParen => {
                self.next();
            }
            e => {
                self.error(format!("expected '(' but found {:?}", e));
                return Err(format!("expected '(' but found {:?}", e));
            }
        };

        /* check for Ident of argument parse starting or is closing with ")" */
        match self.token() {
            TokenKind::RightParen => {
                self.next();
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

    fn parse_arguments(&mut self) -> Result<Vec<Ast<TypedBinding>>, String> {
        let typed_bindings = self.parse_typed_bindings();
        match self.token() {
            TokenKind::RightParen => {
                self.next();
                return Ok(typed_bindings);
            }
            e => {
                self.error(format!("expected ')' but found {:?}", e));
                return Err(format!("expected ')' but found {:?}", e));
            }
        }
    }

    fn parse_returns(&mut self) -> Result<TypeRef, String> {
        match self.token() {
            TokenKind::Colon => {
                self.next();
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
