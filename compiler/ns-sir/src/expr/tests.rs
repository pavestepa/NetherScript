use crate::expr::{SirExpr, contains_error_expr};

#[test]
fn detects_nested_error_expr() {
    let expr = SirExpr::Unary {
        op: "-".to_string(),
        value: Box::new(SirExpr::Error { ty: None }),
        ty: None,
    };
    assert!(contains_error_expr(&expr));
}
