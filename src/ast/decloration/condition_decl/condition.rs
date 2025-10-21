use crate::ast::{expressions::Expr, operators::LogicalOperator};

pub struct Condition {
    pub left: ConditionExpr,
    pub logical_operator: LogicalOperator,
    pub right: ConditionExpr,
}
pub enum ConditionExpr {
    Condition(Box<Condition>),
    Expr(Box<Expr>),
}
