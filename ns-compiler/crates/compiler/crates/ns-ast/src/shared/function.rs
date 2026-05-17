use crate::{Callable, StmtsBlock};

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: Callable,
    pub body: StmtsBlock,
}

/// Class method: same shape as [`Function`] — [`Callable`] signature (including [`crate::This`]) plus body.
pub type Method = Function;
