mod declorations; pub use declorations::{condition_decl, cycle_decl, struct_decl};
mod expressions; pub use expressions::{Arithmetic, ArithmeticExpr, FnCallExpr, Expr, ObjectExpr, ValueExpr, VarCallExpr};
mod operators; pub use operators::{ArithmeticOperator, LogicalOperator, Operator};
mod statements; pub use statements::{GlobalConstStatmnt, EnumStatmnt, FnStatmnt, ModStatmnt, UseStatmnt, Statement};
mod types; pub use types::{HasType, PrimType, RefType};


pub struct IRFile {
  pub path: String,
  pub use_exprs: Vec<UseStatmnt>,
  pub mod_exprs: Vec<ModStatmnt>,
  pub pubstatement_exprs: Vec<Statement>,
}

