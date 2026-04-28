use crate::types::SirType;

pub fn is_error_type(ty: &SirType) -> bool {
    matches!(ty, SirType::Error)
}
