use crate::expr::CallExpr;

#[derive(Debug, Clone)]
pub struct CallStmt(pub CallExpr);
