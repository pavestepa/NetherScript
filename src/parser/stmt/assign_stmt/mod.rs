use crate::{
    ast::{ast::Ast, AssignStmt, Ident},
    lexer::TokenKind,
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_assign_stmt(&mut self, ident: Atom) -> Ast<AssignStmt> {
        let assign = self.parse_expr(0);
        self.parse(TokenKind::Semicolon);
        Ast::Parsed(AssignStmt {
            ident: Ident(ident),
            assign: Box::from(assign),
        })
    }
}
