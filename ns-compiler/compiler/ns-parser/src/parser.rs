use ns_lexer::{TextRange, Token, TokenKind};
use ns_diagnostics::{Diagnostic, SourceSpan};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    source_path: Option<String>,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsePanic {
    pub message: String,
    pub range: TextRange,
    pub file: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self::new_with_path(tokens, None)
    }

    pub fn new_with_path(tokens: Vec<Token>, source_path: Option<String>) -> Self {
        let tokens = tokens
            .into_iter()
            .filter(|t| !matches!(t.kind, TokenKind::Whitespace | TokenKind::CommentLine))
            .collect();

        Self {
            tokens,
            position: 0,
            source_path,
            diagnostics: Vec::new(),
        }
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
            self.report_syntax_error(token.range, format!(
                "Expected {:?}, but got {:?}",
                token_kind, token.kind
            ));
            if self.is_not_end() {
                self.position += 1;
            }
            return token;
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
        std::panic::panic_any(ParsePanic {
            message: message.into(),
            range,
            file: self.source_path.clone(),
        })
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

    pub fn source_path(&self) -> Option<&str> {
        self.source_path.as_deref()
    }

    pub fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.diagnostics)
    }

    pub(crate) fn report_syntax_error(&mut self, range: TextRange, message: impl Into<String>) {
        let message = message.into();
        if let Some(last) = self.diagnostics.last() {
            if last.code.as_deref() == Some("P1000")
                && last.message == message
                && last
                    .span
                    .as_ref()
                    .map(|s| (s.start, s.end))
                    == Some((Some(range.start), Some(range.end)))
            {
                return;
            }
        }
        self.diagnostics.push(
            Diagnostic::error_with_code("P1000", message)
                .with_span(SourceSpan {
                    file: self.source_path.clone(),
                    line: None,
                    column: None,
                    start: Some(range.start),
                    end: Some(range.end),
                    label: Some("syntax error".to_string()),
                })
                .with_note("parser recovered and continued from next boundary"),
        );
    }

    pub(crate) fn parse_decl_recover(&mut self) -> ns_ast::Decl {
        let start_pos = self.position;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.parse_decl()));
        match result {
            Ok(decl) => decl,
            Err(payload) => {
                self.capture_panic_as_diag(&payload);
                self.synchronize_decl();
                self.ensure_progress(start_pos);
                ns_ast::Decl::Error(ns_ast::ErrorDecl {
                    message: "failed to parse declaration".to_string(),
                })
            }
        }
    }

    pub(crate) fn parse_stmt_recover(&mut self) -> ns_ast::Stmt {
        let start_pos = self.position;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.parse_stmt()));
        match result {
            Ok(stmt) => stmt,
            Err(payload) => {
                self.capture_panic_as_diag(&payload);
                self.synchronize_stmt();
                self.ensure_progress(start_pos);
                ns_ast::Stmt::Error(ns_ast::ErrorStmt {
                    message: "failed to parse statement".to_string(),
                })
            }
        }
    }

    pub(crate) fn parse_expr_recover(&mut self) -> ns_ast::Expr {
        let start_pos = self.position;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.parse_expr(0)));
        match result {
            Ok(expr) => expr,
            Err(payload) => {
                let panic_range = payload
                    .downcast_ref::<ParsePanic>()
                    .map(|p| p.range);
                self.capture_panic_as_diag(&payload);
                self.synchronize_expr();
                if let Some(range) = panic_range {
                    if self.is_not_end() && self.current().range == range {
                        self.position += 1;
                    }
                }
                self.ensure_progress(start_pos);
                ns_ast::Expr::Error(ns_ast::ErrorExpr {
                    message: "failed to parse expression".to_string(),
                })
            }
        }
    }

    pub(crate) fn capture_panic_as_diag(&mut self, payload: &Box<dyn std::any::Any + Send>) {
        if let Some(parse) = payload.downcast_ref::<ParsePanic>() {
            self.report_syntax_error(parse.range, parse.message.clone());
            return;
        }
        let token = self.current().clone();
        self.report_syntax_error(token.range, "unexpected parser failure");
    }

    pub(crate) fn synchronize_decl(&mut self) {
        while self.is_not_end() {
            match self.current().kind {
                TokenKind::Keyword(
                    ns_lexer::Keyword::Class
                    | ns_lexer::Keyword::Function
                    | ns_lexer::Keyword::Enum
                    | ns_lexer::Keyword::Interface
                    | ns_lexer::Keyword::Const
                    | ns_lexer::Keyword::Type
                    | ns_lexer::Keyword::Import
                    | ns_lexer::Keyword::Export
                    | ns_lexer::Keyword::Index,
                ) => return,
                TokenKind::RightBrace => return,
                _ => self.position += 1,
            }
        }
    }

    pub(crate) fn synchronize_stmt(&mut self) {
        while self.is_not_end() {
            match self.current().kind {
                TokenKind::Semicolon | TokenKind::Newline | TokenKind::RightBrace => {
                    if self.current().kind != TokenKind::RightBrace {
                        self.position += 1;
                    }
                    return;
                }
                _ => self.position += 1,
            }
        }
    }

    pub(crate) fn synchronize_expr(&mut self) {
        while self.is_not_end() {
            match self.current().kind {
                TokenKind::Comma
                | TokenKind::Semicolon
                | TokenKind::Newline
                | TokenKind::RightParen
                | TokenKind::RightBrace => return,
                _ => self.position += 1,
            }
        }
    }

    fn ensure_progress(&mut self, start_pos: usize) {
        if self.position == start_pos && self.is_not_end() {
            self.position += 1;
        }
    }
}
