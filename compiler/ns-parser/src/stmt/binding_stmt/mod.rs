use crate::syntax::{
    ast::{ast::Ast, BindingStmt, LetOrVar},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_binding_stmt_let(&mut self) -> Ast<BindingStmt> {
        self.parse_binding_stmt(true)
    }
    pub fn parse_binding_stmt_var(&mut self) -> Ast<BindingStmt> {
        self.parse_binding_stmt(false)
    }

    fn parse_binding_stmt(&mut self, is_let: bool) -> Ast<BindingStmt> {
        println!("[STARTED] parse BindingStmt");
        /*  parsing binding kind. let or var */
        let kind = if is_let { LetOrVar::Let } else { LetOrVar::Var };

        let typed_binding = self.parse_binding();

        match self.current().kind {
            TokenKind::Assign => {
                self.parse(TokenKind::Assign);
                let expr = self.parse_expr(0);
                self.parse(TokenKind::Semicolon);
                return Ast::Parsed(BindingStmt {
                    kind: kind,
                    typed_binding: typed_binding,
                    assign: Some(Box::new(expr)),
                });
            }
            TokenKind::Semicolon => {
                self.parse(TokenKind::Semicolon);
                return Ast::Parsed(BindingStmt {
                    kind: kind,
                    typed_binding: typed_binding,
                    assign: None,
                });
            }
            e => {
                let err = format!("Expected '=' for assigning of inited binding, or ';' for and statement. Found {:?}", e);
                self.error(err.clone());
                return Ast::Error(err);
            }
        }
    }
}
