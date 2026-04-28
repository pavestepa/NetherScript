use crate::symbols::SirImportLink;

impl SirImportLink {
    pub fn unresolved(local_name: String, from_module_path: String) -> Self {
        Self {
            local_name,
            from_module_path,
            target: None,
        }
    }
}
