use crate::ast::Expr;

use super::parser::Parser;

impl Parser {
  pub fn parse_expression(&mut self) -> Result<Expr, String> {
      self.parse_additive()
  }
}