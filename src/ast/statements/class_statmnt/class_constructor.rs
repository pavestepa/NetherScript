use crate::ast::statements::fn_statmnt::FnStatmntArg;

pub struct ClassConstructor {
    pub args: Vec<FnStatmntArg>,
    pub body: ClassConstructorExprs,
}

pub enum ClassConstructorExprs {}
