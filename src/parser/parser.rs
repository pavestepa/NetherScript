use crate::ast::SyntaxError;
use crate::lexer::{Keyword, Token, TokenKind};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
    pub errors: Vec<SyntaxError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::CommentLine))
            .collect();

        Self {
            tokens,
            position: 0,
            errors: vec![],
        }
    }

    pub fn token(&self) -> TokenKind {
        self.peek().unwrap().kind
    }

    // get current position token
    pub fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
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
        if token.is_some() {
            return token;
        }
        None
    }

    pub fn error(&mut self, message: impl Into<String>) -> SyntaxError {
        let token = self.peek().unwrap();
        let syntax_error = SyntaxError {
            message: message.into(),
            range: token.range,
        };
        self.errors.push(syntax_error.clone());
        syntax_error
    }

    pub fn go_to_next_decl(&mut self) {
        loop {
            println!("see");
            if self.peek().is_some() {
                match self.token() {
                    TokenKind::Keyword(keyword) => match keyword {
                        Keyword::Const
                        | Keyword::Enum
                        | Keyword::Export
                        | Keyword::Function
                        | Keyword::Import
                        | Keyword::Index
                        | Keyword::Struct
                        | Keyword::Type => {
                            break;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            break;
        }
    }
}
