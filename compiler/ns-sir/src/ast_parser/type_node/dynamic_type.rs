use ns_ast::DynamicType;

use crate::types::SirType;

pub(crate) fn lower(d: &DynamicType) -> SirType {
    SirType::Dynamic {
        interface: d.interface.clone().into_simple().as_str().to_string(),
    }
}
