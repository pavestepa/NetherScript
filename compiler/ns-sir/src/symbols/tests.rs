use crate::symbols::{SirImportLink, unresolved_import_count};

#[test]
fn counts_unresolved_imports() {
    let imports = vec![SirImportLink {
        local_name: "a".to_string(),
        from_module_path: "m".to_string(),
        target: None,
    }];
    assert_eq!(unresolved_import_count(&imports), 1);
}
