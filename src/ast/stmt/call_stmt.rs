use crate::ast::expr::call_expr::FunctionCall;

#[derive(Debug, Clone)]
pub struct CallStmt(pub FunctionCall);
