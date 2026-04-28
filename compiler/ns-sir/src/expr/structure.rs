use crate::ids::{SirFnId, SirTypeId, SirValueId};

#[derive(Debug, Clone)]
pub enum SirLiteral {
    Number(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum SirTemplatePart {
    Text(String),
    Expr(Box<SirExpr>),
}

#[derive(Debug, Clone)]
pub enum SirExpr {
    Error {
        ty: Option<SirTypeId>,
    },
    Literal {
        lit: SirLiteral,
        ty: Option<SirTypeId>,
    },
    ValueRef {
        value: SirValueId,
        ty: Option<SirTypeId>,
    },
    TypeRef {
        type_id: SirTypeId,
        ty: Option<SirTypeId>,
    },
    Call {
        callee: Option<SirFnId>,
        type_args: Vec<SirTypeId>,
        args: Vec<SirExpr>,
        ty: Option<SirTypeId>,
    },
    IntrinsicPrintln {
        args: Vec<SirExpr>,
        ty: Option<SirTypeId>,
    },
    TemplateString {
        parts: Vec<SirTemplatePart>,
        ty: Option<SirTypeId>,
    },
    Unary {
        op: String,
        value: Box<SirExpr>,
        ty: Option<SirTypeId>,
    },
    Binary {
        op: String,
        left: Box<SirExpr>,
        right: Box<SirExpr>,
        ty: Option<SirTypeId>,
    },
    Member {
        object: Box<SirExpr>,
        property: String,
        ty: Option<SirTypeId>,
    },
    StructLiteral {
        type_name: String,
        fields: Vec<(String, SirExpr)>,
        ty: Option<SirTypeId>,
    },
    New {
        class_name: String,
        args: Vec<SirExpr>,
        ty: Option<SirTypeId>,
    },
}
