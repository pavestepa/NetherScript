use super::Expr;

#[derive(Debug)]
pub struct LogicalExpr {
    left: Box<Expr>,
    op: LogicalOperator,
    right: Box<Expr>,
}
impl LogicalExpr {
    pub fn new(left: Expr, op: LogicalOperator, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op: op,
            right: Box::new(right),
        }
    }
}
#[derive(Debug)]
pub enum LogicalOperator {
    EqualTo,
}
