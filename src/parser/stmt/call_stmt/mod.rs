use crate::{
    ast::{ast::Ast, CallStmt, FunctionCall, FunctionDecl, Ident},
    parser::Parser,
    Atom,
};

impl Parser {
    pub fn parse_call_stmt(&mut self, ident: Atom) -> Ast<CallStmt> {
        let fc = FunctionCall {
            callee: Ident(ident),
            args: vec![],
        };
        Ast::Parsed(CallStmt(fc))
    }
}
