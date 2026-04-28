use crate::expr::SirExpr;
use crate::ids::{SirClassId, SirFieldId, SirValueId};

#[derive(Debug, Clone)]
pub enum SirStmt {
    Error,
    InitParent {
        class_id: SirClassId,
    },
    InitField {
        field_id: SirFieldId,
    },
    Expr(SirExpr),
    Let {
        value: SirValueId,
        is_mut: bool,
        init: Option<SirExpr>,
    },
    Assign {
        target: SirExpr,
        value: SirExpr,
    },
    Return(Option<SirExpr>),
    If {
        test: SirExpr,
        then_body: Vec<SirStmt>,
        else_body: Vec<SirStmt>,
    },
    Loop {
        body: Vec<SirStmt>,
    },
    Break,
}
