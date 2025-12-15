use crate::lexer::{Token, Keyword};
use crate::ast::Module;

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

        Self { tokens, position: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    pub fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    pub fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        if token.is_some() {
            self.position += 1;
        }
        token
    }

    pub fn check(&self, expected: &Token) -> bool {
        self.peek() == Some(expected)
    }

    pub fn check_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.peek(), Some(Token::Keyword(k)) if *k == keyword)
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.advance() {
            Some(t) if t == expected => Ok(()),
            Some(t) => Err(format!("Expected {:?}, found {:?}", expected, t)),
            None => Err(format!("Expected {:?}, found EOF", expected)),
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if matches!(
                self.peek(),
                Some(Token::Keyword(
                    Keyword::Function
                        | Keyword::Class
                        | Keyword::Const
                        | Keyword::Let
                        | Keyword::Return
                ))
            ) {
                return;
            }
            self.advance();
        }
    }
}

/// Public API
pub fn parse_module(tokens: Vec<Token>) -> Module {
    let mut parser = Parser::new(tokens);
    match parser.parse_module() {
        Ok(m) => m,
        Err(_) => Module::new(Vec::new()),
    }
}
