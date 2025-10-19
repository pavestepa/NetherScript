use crate::er::{
    statements::{fn_statmnt::FnStatmntArg, FnStatmntExpr},
    types::HasType,
};

pub struct TraitMember {
    name: String,
    args: Vec<FnStatmntArg>,
    return_type: HasType,
    default_impl: Option<Vec<FnStatmntExpr>>,
}
