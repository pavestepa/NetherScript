use crate::ast::SyntaxError;
use crate::lexer::{Keyword, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    errors: Vec<SyntaxError>,
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

    pub fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }

    pub fn previous(&self) -> &Token {
        self.tokens.get(self.position - 1).unwrap()
    }

    pub fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    pub fn consume(&mut self, token_kind: TokenKind) -> Token {
        let token = self.current().clone();
        if token_kind != token.kind {
            self.error(format!(
                "Expected {:?}, but got {:?}",
                token_kind, token.kind
            ));
        }
        self.position += 1;
        return token;
    } // TODO: consume ident way

    pub fn is_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    pub fn is_not_end(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn error(&mut self, message: impl Into<String>) {
        let token = self.current();
        let syntax_error = SyntaxError {
            message: message.into(),
            range: token.range,
        };
        self.errors.push(syntax_error.clone());
        self.synchronize();
    }

    pub fn get_errors(&self) -> Vec<SyntaxError> {
        self.errors.clone()
    }

    fn synchronize(&mut self) {
        self.position += 1;

        while self.is_not_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }
            match self.current().kind {
                TokenKind::Keyword(keyword) => match keyword {
                    Keyword::Let
                    | Keyword::Loop
                    | Keyword::If
                    | Keyword::Function
                    | Keyword::Const
                    | Keyword::Enum
                    | Keyword::Import
                    | Keyword::Index
                    | Keyword::Struct
                    | Keyword::Type => {
                        return;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
