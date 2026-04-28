use crate::ids::SirTypeId;

#[derive(Debug, Clone)]
pub enum SirType {
    Error,
    Named { name: String, args: Vec<SirTypeId> },
    Dynamic { interface: String },
    TypeParam { name: String },
}
