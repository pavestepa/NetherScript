use crate::{
    ast::{ast::Ast, VarKind, VarStmt},
    lexer::TokenKind,
    parser::Parser,
};

impl Parser {
    pub fn parse_var_stmt_let(&mut self) -> Ast<VarStmt> {
        self.parse_var_stmt(true)
    }
    pub fn parse_var_stmt_const(&mut self) -> Ast<VarStmt> {
        self.parse_var_stmt(false)
    }

    fn parse_var_stmt(&mut self, is_let: bool) -> Ast<VarStmt> {
        /*  parsing var kind. let or const */
        let kind = if is_let { VarKind::Let } else { VarKind::Const };

        let typed_binding = self.parse_typed_binding();

        let semicolon = self.parse_semicolon();
        if semicolon.is_err() {
            self.error("expected ';' after export declaration");
        }

        Ast::Parsed(VarStmt {
            kind: kind,
            typed_binding: typed_binding,
        })
    }
}
