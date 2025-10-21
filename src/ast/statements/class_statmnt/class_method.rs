use crate::ast::{
    statements::fn_statmnt::{FnStatmntArg, FnStatmntExpr},
    types::HasType,
};

pub struct ClassMethod {
    pub is_pub: bool,
    pub is_static: bool,
    pub name: String,
    pub args: Vec<FnStatmntArg>,
    pub return_type: HasType,
    pub body: Vec<FnStatmntExpr>,
}
