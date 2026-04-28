use crate::symbols::SirImportLink;

pub fn unresolved_import_count(imports: &[SirImportLink]) -> usize {
    imports.iter().filter(|i| i.target.is_none()).count()
}
