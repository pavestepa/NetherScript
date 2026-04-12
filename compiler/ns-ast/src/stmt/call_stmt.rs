use crate::syntax::ast::expr::FunctionCall;

#[derive(Debug, Clone)]
pub struct CallStmt(pub FunctionCall);
