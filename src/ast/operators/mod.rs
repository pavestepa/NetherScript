mod arithmetic_operator; pub use arithmetic_operator::ArithmeticOperator;
mod logical_operator; pub use logical_operator::LogicalOperator;
pub enum Operator {
  ArithmeticOperator(ArithmeticOperator),
  LogicalOperator(LogicalOperator)
}