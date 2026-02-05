use crate::{
    ast::{ast::Ast, BindingStmt, LetOrVar},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_binding_stmt_let(&mut self) -> Ast<BindingStmt> {
        self.parse_binding_stmt(true)
    }
    pub fn parse_binding_stmt_const(&mut self) -> Ast<BindingStmt> {
        self.parse_binding_stmt(false)
    }

    fn parse_binding_stmt(&mut self, is_let: bool) -> Ast<BindingStmt> {
        /*  parsing binding kind. let or var */
        let kind = if is_let { LetOrVar::Let } else { LetOrVar::Var };

        let typed_binding = self.parse_typed_binding();

        self.consume(TokenKind::Semicolon);

        Ast::Parsed(BindingStmt {
            kind: kind,
            typed_binding: typed_binding,
        })
    }
}
