use crate::{
    ast::{BlockStmt, FunctionDecl, TypeRef},
    lexer::{Keyword, Token},
    parser::{Parse, Parser},
};

impl Parser {
    pub fn parse_function_decl(&mut self) -> Parse<FunctionDecl> {
        // Check is public Function Declaration
        if fn_and_pub.is_err() {
            return Err(fn_and_pub.err().unwrap());
        }
        // now we have "public function" or "function"
        let is_pub = fn_and_pub.unwrap();
        self.next();

        // Check identificator of function
        let parse_ident = self.ident();
        if parse_ident.is_err() {
            return Err(parse_ident.err().unwrap());
        }
        // now we have name of function
        let ident = parse_ident.unwrap();
        self.next();

        // Check is "("
        let left_paren = self.check(Token::LeftParen);
        if left_paren.is_err() {
            return Err(left_paren.err().unwrap());
        }
        // now we have "("
        self.next();

        // check if is arguments and ")"
        let mut args = vec![];
        loop {
            if self.peek().unwrap() == &Token::RightParen {
                break;
            }
            let arg = parse_fn_arg(self)?;
            args.push(arg);
        }

        // Check if now is not ")"
        let right_paren = self.check(Token::RightParen);
        if right_paren.is_err() {
            return Err(left_paren.err().unwrap());
        }
        self.next();

        // Check to returns type
        let returns_type;

        match *self.peek().unwrap() {
            Token::LeftBrace => {
                returns_type = Typ::Void;
            }
            Token::Colon => {
                let type_token = self.next().unwrap();
                match type_token {
                    Token::Ident(_) => {
                        returns_type = self.parse_type()?;
                        self.next();
                    }
                    _ => {
                        return Err(format!(
                            "expected Ident, but found {:?}",
                            self.peek().unwrap()
                        ));
                    }
                }
            }
            _ => {
                return Err(format!(
                    "expected Colon, but found {:?}",
                    self.peek().unwrap()
                ));
            }
        }
        println!("");
        println!("FUNCTION DECL BOILERPALETE: {:?}", ident);
        println!("");

        let block_stmt: BlockStmt = self.parse_block_stmt()?;

        Ok(FunctionDecl::new(
            is_pub,
            ident,
            args,
            returns_type,
            block_stmt,
        ))
    }
}
