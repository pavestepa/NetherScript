use crate::Expr;

#[derive(Debug, Clone)]
pub enum TemplateStringPart {
    Text(String),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct TemplateStringExpr {
    pub parts: Vec<TemplateStringPart>,
}
