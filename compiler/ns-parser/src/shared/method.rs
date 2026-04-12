use crate::{Atom, atom, syntax::{ast::{Binding, Method, RefKind, This, TypeNode, ast::Ast}, lexer::{Keyword, TokenKind}, parser::Parser}};

impl Parser {
    pub fn parse_method(&mut self) -> Ast<Method> {
        println!("[STARTED] parse FunctionDecl");
        /*  parsing of method name Ident */
        let ident = match self.parse_ident() {
            Ok(v) => v,
            Err(v) => {
                return Ast::Error(v);
            }
        };

        /* parsing arguments of function like: "(a: i32, s: String)" */
        let args = match self.parse_this_arguments_with_parens() {
            Ok(v) => v,
            Err(v) => {
                return Ast::Error(v);
            }
        };

        
    }
    fn parse_this_arguments_with_parens(&mut self) -> Result<(This, Vec<Ast<Binding>>), String> {
        /* check for "(" existing */
        self.parse(TokenKind::LeftParen);

        /* check for existing 'this' and type kind of */
        let this = self.parse_this();

        /* check for Ident of argument parse starting or is closing with ")" */
        match self.current().kind {
            TokenKind::RightParen => {
                self.parse(TokenKind::RightParen);
                return Ok((this, vec![]));
            }
            TokenKind::Ident(_) => {
                let args = self.parse_method_arguments();

                if args.is_ok() {
                    return Ok((this, args.unwrap()));
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

    fn parse_this(&mut self) -> This {
        match self.current().kind {
            TokenKind::Ident(ident) => {
                if ident == atom("this") {
                    
                }
            }
            TokenKind::Keyword(keyword) => {
                let ref_kind = self.try_parse_ref_kind();
                
            }
            }
        }
    }

    fn parse_method_arguments(&mut self) -> Result<Vec<Ast<Binding>>, String> {

        
        let typed_bindings = self.parse_bindings();
        match self.current().kind {
            TokenKind::RightParen => {
                self.parse(TokenKind::RightParen);
                return Ok(typed_bindings);
            }
            e => {
                self.error(format!("expected ')' but found {:?}", e));
                return Err(format!("expected ')' but found {:?}", e));
            }
        }
    }

    fn parse_method_returns(&mut self) -> Ast<TypeNode> {
        println!("started parse fn returns");
        self.parse(TokenKind::Colon);
        self.parse_type_node()
    }
}