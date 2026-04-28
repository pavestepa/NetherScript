use crate::decl::SirProgram;

impl SirProgram {
    pub fn new(types: Vec<crate::types::SirType>) -> Self {
        Self {
            decls: Vec::new(),
            values: Vec::new(),
            functions: Vec::new(),
            classes: Vec::new(),
            enums: Vec::new(),
            fields: Vec::new(),
            traits: Vec::new(),
            imports: Vec::new(),
            exports: Vec::new(),
            types,
        }
    }

    pub fn dump(&self) -> String {
        format!("{self:#?}")
    }
}
