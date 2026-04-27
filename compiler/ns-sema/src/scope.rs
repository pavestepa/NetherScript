use std::collections::HashMap;

use crate::symbol::SymbolId;

#[derive(Debug, Default)]
pub struct Scope {
    pub parent: Option<usize>,
    pub type_names: HashMap<String, SymbolId>,
    pub value_names: HashMap<String, SymbolId>,
}
