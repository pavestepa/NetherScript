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
    fn err_expected(&self, token: Token) -> Result<(), String> {
        Err(format!("expected '{:?}' but is '{:?}'", token, self.peek()))
    }

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
        let parse_ident = self.check_token_ident();
        if parse_ident.is_err() {
            return Err(parse_ident.err().unwrap());
        }
        // now we have name of function
        let ident = parse_ident.unwrap();
        self.next();

        // Check is "("
        let left_colon = self.check_token_left_paren();
        if left_colon.is_err() {
            return Err(left_colon.err().unwrap());
        }
        // now we have "("
        self.next();

        // check if is arguments and ")"
        let mut args = vec![];
        loop {
            if self.peek().unwrap() == &Token::RightParen {
                break;
            }
            let arg = parse_fn_arg(self);
            if arg.is_ok() {
                args.push(arg.unwrap());
            } else {
                if self.peek().unwrap() == &Token::Comma {
                    self.next();
                } else {
                    return Err(format!(
                        "expected Comma, but found {:?}",
                        self.peek().unwrap()
                    ));
                }
            }
        }

        // Check if now is not ")"
        if self.peek().unwrap() != &Token::RightParen {
            return Err(format!(
                "expected RightParen, but found {:?}",
                self.peek().unwrap()
            ));
        }
        self.next();
        println!("{:?}", self.peek().unwrap());
        // Check to returns type
        let returns_type;
        if self.peek().unwrap() == &Token::LeftBrace {
            returns_type = Typ::Void;
            self.next(); // return type is void
        } else if self.peek().unwrap() == &Token::Colon {
            let typ_token = self.next();
            if let Token::Ident(value) = self.peek().unwrap() {
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
            if self.check(&fn_token) {
                return Ok(is_pub);
            } else {
                // is not "function"
                return Err(format!(
                    "expected {:?}, but found {:?}",
                    fn_token,
                    self.peek().unwrap()
                ));
            }
        } else {
            // we have "public" before
            self.next();
            // get next token after "public"
            if self.peek().is_some() {
                if self.peek().unwrap() == &fn_token {
                    return Ok(is_pub);
                } else {
                    return Err(format!(
                        "expected {:?}, but found {:?}",
                        fn_token,
                        self.peek().unwrap()
                    ));
                }
            } else {
                return Err(format!(
                    "expected {:?}, but found {:?}",
                    fn_token,
                    self.peek().unwrap()
                ));
            }
        }
    }

    fn check_token_ident(&mut self) -> Result<Atom, String> {
        if self.peek().is_some() {
            if let Token::Ident(value) = self.peek().unwrap() {
                return Ok(*value);
            } else {
                return Err(format!(
                    "expected {:?}, but found {:?}",
                    Token::Ident(atom("".to_string())),
                    self.peek().unwrap()
                ));
            }
        } else {
            return Err("Token not found".to_string());
        }
    }

    fn check_token_left_paren(&mut self) -> Result<(), String> {
        if self.peek().is_some() {
            if *self.peek().unwrap() == Token::LeftParen {
                return Ok(());
            } else {
                return self.err_expected(Token::LeftParen);
            }
        } else {
            return Err("Token not found".to_string());
        }
    }
    fn check_token_right_paren(&mut self) -> Result<(), String> {
        if self.peek().is_some() {
            if *self.peek().unwrap() == Token::RightParen {
                return Ok(());
            } else {
                return self.err_expected(Token::RightParen);
            }
        } else {
            return Err("Token not found".to_string());
        }
    }
}
