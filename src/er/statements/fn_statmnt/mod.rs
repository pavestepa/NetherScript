mod fn_statmnt_arg;
pub use fn_statmnt_arg::FnStatmntArg;
mod fn_statmnt_expr;
pub use fn_statmnt_expr::FnStatmntExpr;

use crate::er::types::HasType;

pub struct FnStatmnt {
    pub is_pub: bool,
    pub name: String,
    pub args: Vec<FnStatmntArg>,
    pub return_type: HasType,
    pub body: Vec<FnStatmntExpr>,
}
