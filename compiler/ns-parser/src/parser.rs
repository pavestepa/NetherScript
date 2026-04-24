
use ns_lexer::{TextRange, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::CommentLine))
            .collect();

        Self { tokens, position: 0 }
    }

    pub fn current(&self) -> &Token {
        self.tokens
            .get(self.position)
            .unwrap_or_else(|| self.tokens.last().expect("token stream is empty"))
    }

    pub fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or_else(|| self.tokens.last().expect("token stream is empty"))
    }

    pub fn previous(&self) -> &Token {
        self.tokens
            .get(self.position.saturating_sub(1))
            .unwrap_or_else(|| self.tokens.first().expect("token stream is empty"))
    }

    pub fn parse(&mut self, token_kind: TokenKind) -> Token {
        let token = self.current().clone();
        if token_kind != token.kind {
            self.panic_at_current(format!(
                "Expected {:?}, but got {:?}",
                token_kind, token.kind
            ));
        }
        self.position += 1;
        token
    }

    pub fn is_not_end(&self) -> bool {
        self.position < self.tokens.len()
    }

    pub fn panic_at_current(&self, message: impl Into<String>) -> ! {
        let token = self.current();
        self.panic_at_range(token.range, message)
    }

    pub fn panic_at_range(&self, range: TextRange, message: impl Into<String>) -> ! {
        panic!(
            "Parser panic at {}..{}: {}",
            range.start,
            range.end,
            message.into()
        )
    }

    pub fn consume_newlines(&mut self) {
        while self.is_not_end() && self.current().kind == TokenKind::Newline {
            self.position += 1;
        }
    }

    pub fn parse_optional_stmt_delimiter(&mut self) {
        if !self.is_not_end() {
            return;
        }

        if self.current().kind == TokenKind::Semicolon {
            self.parse(TokenKind::Semicolon);
        }
        self.consume_newlines();
    }

    pub fn parse_required_after_linebreaks(&mut self, token_kind: TokenKind, context: &str) {
        self.consume_newlines();
        if self.current().kind != token_kind {
            self.panic_at_current(format!("expected {:?} {}", token_kind, context));
        }
        self.parse(token_kind);
    }

    pub fn peek_non_newline_kind(&self, mut offset: usize) -> TokenKind {
        while self.peek(offset).kind == TokenKind::Newline {
            offset += 1;
        }
        self.peek(offset).kind
    }

    pub fn looks_like_type_modifier_decl_start(&self) -> bool {
        let mut offset = 0usize;
        match self.peek(offset).kind {
            TokenKind::Ident(_) => {
                offset += 1;
                if self.peek(offset).kind == TokenKind::Less {
                    let mut depth = 0usize;
                    loop {
                        match self.peek(offset).kind {
                            TokenKind::Less => depth += 1,
                            TokenKind::Greater => {
                                depth = depth.saturating_sub(1);
                                if depth == 0 {
                                    offset += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                        offset += 1;
                    }
                }
            }
            TokenKind::Ampersand => {
                offset += 1;
                if self.peek(offset).kind == TokenKind::Keyword(ns_lexer::Keyword::Mut) {
                    offset += 1;
                }
                if self.peek(offset).kind != TokenKind::Keyword(ns_lexer::Keyword::Dynamic) {
                    return false;
                }
                offset += 1;
                if !matches!(self.peek(offset).kind, TokenKind::Ident(_)) {
                    return false;
                }
                offset += 1;
            }
            TokenKind::Keyword(ns_lexer::Keyword::Ref | ns_lexer::Keyword::Mut) => {
                offset += 1;
                if self.peek(offset).kind != TokenKind::Keyword(ns_lexer::Keyword::Dynamic) {
                    return false;
                }
                offset += 1;
                if !matches!(self.peek(offset).kind, TokenKind::Ident(_)) {
                    return false;
                }
                offset += 1;
            }
            TokenKind::Keyword(ns_lexer::Keyword::Dynamic) => {
                offset += 1;
                if !matches!(self.peek(offset).kind, TokenKind::Ident(_)) {
                    return false;
                }
                offset += 1;
            }
            _ => return false,
        }

        matches!(
            self.peek_non_newline_kind(offset),
            TokenKind::Keyword(ns_lexer::Keyword::Extends | ns_lexer::Keyword::Implements)
        )
    }
}
