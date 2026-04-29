use crate::Expr;

#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub arg: Option<Box<Expr>>,
}

impl ReturnStmt {
    pub fn new(arg: Option<Expr>) -> Self {
        Self {
            arg: match arg {
                Some(e) => Some(Box::new(e)),
                None => None,
            },
        }
    }
}
