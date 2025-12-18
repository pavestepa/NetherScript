mod fn_arg;

use crate::{
    ast::{
        decl::{FnArg, FnDecl},
        Typ,
    },
    atom,
    lexer::{Keyword, Token},
    parser::{decl::fn_decl::fn_arg::parse_fn_arg, Parser},
    Atom,
};

impl Parser {
    pub fn parse_fn_decl(&mut self) -> Result<FnDecl, String> {
        // Check is public Function Declaration
        let fn_and_pub = self.check_token_function_or_public_function();
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
        println!("{:?}", self.peek().unwrap());

        // Check to returns type
        let returns_type;
        if self.peek().unwrap() == &Token::LeftBrace {
            returns_type = Typ::Void;
            self.next(); // return type is void
        } else if self.peek().unwrap() == &Token::Colon {
            let typ_token = self.next().unwrap();
            if let Token::Ident(value) = typ_token {
                returns_type = self.parse_type()?;
                self.next();
                // Check to "{" after ":T"
                if self.peek().unwrap() == &Token::LeftBrace {
                    self.next();
                } else {
                    return Err(format!(
                        "expected LeftBrace, but found {:?}",
                        self.peek().unwrap()
                    ));
                }
            } else {
                return Err(format!(
                    "expected Ident, but found {:?}",
                    self.peek().unwrap()
                ));
            }
        } else {
            return Err(format!(
                "expected Colon, but found {:?}",
                self.peek().unwrap()
            ));
        }
        println!("{:?}", self.peek().unwrap());

        Ok(FnDecl::new(is_pub, ident, args, returns_type, vec![]))
    }

    fn check_token_function_or_public_function(&mut self) -> Result<bool, String> {
        let fn_token = Token::Keyword(Keyword::Function);
        let is_pub = self.check_keyword(Keyword::Public);

        // Check if is "function" keyword
        if is_pub == false {
            let is_fn = self.check(fn_token);
            if is_fn.is_ok() {
                return Ok(is_pub);
            } else {
                return Err(is_fn.err().unwrap());
            }
        } else {
            // we have "public" before
            self.next();
            // get next token after "public"
            let is_fn = self.check(fn_token);
            if is_fn.is_ok() {
                return Ok(is_pub);
            } else {
                return Err(is_fn.err().unwrap());
            }
        }
    }
}
