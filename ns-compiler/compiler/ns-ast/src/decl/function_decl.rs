use crate::{Callable, StmtsBlock};

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub signature: Callable,
    pub body: StmtsBlock,
}
