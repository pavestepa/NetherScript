use crate::{Callable, StmtsBlock};

#[derive(Debug)]
pub struct Function {
    pub signature: Callable,
    pub body: StmtsBlock,
}
