use crate::expr::SirExpr;

impl SirExpr {
    pub fn error() -> Self {
        SirExpr::Error { ty: None }
    }

    pub fn ty(&self) -> Option<crate::ids::SirTypeId> {
        match self {
            SirExpr::Error { ty }
            | SirExpr::Literal { ty, .. }
            | SirExpr::ValueRef { ty, .. }
            | SirExpr::TypeRef { ty, .. }
            | SirExpr::Call { ty, .. }
            | SirExpr::IntrinsicPrintln { ty, .. }
            | SirExpr::TemplateString { ty, .. }
            | SirExpr::Unary { ty, .. }
            | SirExpr::Binary { ty, .. }
            | SirExpr::Member { ty, .. }
            | SirExpr::StructLiteral { ty, .. }
            | SirExpr::New { ty, .. } => *ty,
        }
    }
}
