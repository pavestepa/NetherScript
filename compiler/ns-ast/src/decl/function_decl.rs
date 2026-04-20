use crate::{Callable, StmtsBlock};

#[derive(Debug)]
pub struct FunctionDecl {
    pub signature: Callable,
    pub body: StmtsBlock,
}
