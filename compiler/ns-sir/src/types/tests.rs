use crate::types::{SirType, is_error_type};

#[test]
fn recognizes_error_type() {
    assert!(is_error_type(&SirType::Error));
}
