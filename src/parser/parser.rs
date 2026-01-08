use crate::ast::Module;
use crate::lexer::{Keyword, Token};
use crate::Atom;

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t, Token::Whitespace | Token::CommentLine))
            .collect();

        Self {
            tokens,
            position: 0,
        }
    }

    // get current position token
    pub fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        println!("[LOG] peek: {:?}", token);
        token
    }

    // get with jumped position token
    pub fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    // get current token and +1 to position
    pub fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        if token.is_some() {
            self.position += 1;
        }
        token
    }

    // get next token and +1 to position
    pub fn next(&mut self) -> Option<Token> {
        self.position += 1;
        let token = self.tokens.get(self.position).cloned();
        println!("[LOG] next: {:?}", token);
        if token.is_some() {
            return token;
        }
        None
    }

    // is current token equal with expected
    pub fn check(&self, expected: Token) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            return Ok(());
        }
        Err(format!(
            "[loc] expected {:?}, but found {:?}",
            expected,
            *self.peek().unwrap()
        ))
    }

    // is current token keyword equal with expected
    pub fn check_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.peek(), Some(Token::Keyword(k)) if *k == keyword)
    }

    // is current token equal or send Err
    pub fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.advance() {
            Some(t) if t == expected => Ok(()),
            Some(t) => Err(format!("Expected {:?}, found {:?}", expected, t)),
            None => Err(format!("Expected {:?}, found EOF", expected)),
        }
    }

    // is position more than tokens length
    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    // find first equal token and set to this position or none
    pub fn first_finded(&mut self, _token: Token) -> Option<Token> {
        let token = Some(&_token);
        let old_position = self.position;
        self.advance();
        while !self.is_at_end() {
            if self.peek() == token {
                return Some(_token);
            }
            self.advance();
        }
        self.position = old_position;
        return None;
    }

    pub fn ident(&mut self) -> Result<Atom, String> {
        if let Some(&Token::Ident(value)) = self.peek() {
            return Ok(value.clone());
        }
        Err(format!(
            "expected Ident, but found {:?}",
            *self.peek().unwrap()
        ))
    }

    pub fn keyword(&mut self) -> Result<Keyword, String> {
        if let Some(&Token::Keyword(value)) = self.peek() {
            return Ok(value.clone());
        }
        Err(format!(
            "expected Ident, but found {:?}",
            *self.peek().unwrap()
        ))
    }

    fn consume(&mut self, expected: &Token) {
        let tok = self.next();
        if tok.clone().unwrap() != *expected {
            panic!("expected {:?}, got {:?}", expected, tok);
        }
    }

    pub fn is(&self, token: Token) -> bool {
        self.peek().unwrap() == &token
    }

    pub fn is_not(&self, token: Token) -> bool {
        self.peek().unwrap() != &token
    }
}
