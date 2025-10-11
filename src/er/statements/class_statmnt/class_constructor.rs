use crate::er::statements::fn_statmnt::FnArg;

pub struct ClassConstructor {
  pub args: Vec<FnArg>,
  pub body: ClassConstructorExprs,
}

pub enum ClassConstructorExprs {

}