use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use ns_parser::main_parse_with_diagnostics;
use ns_sema::analyze;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[test]
fn index_module_decls_are_available_in_merged_analysis_module() {
    let mut root = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let seq = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    root.push(format!(
        "nsc_package_index_imports_{}_{}_{}",
        std::process::id(),
        nanos,
        seq
    ));
    fs::create_dir_all(&root).expect("create temp root");

    let entry = root.join("main.ns");
    let module_a = root.join("a.ns");

    fs::write(
        &entry,
        r#"
index a;
import num from a;

function main(): i32 {
    return num;
}
"#,
    )
    .expect("write entry");

    fs::write(&module_a, "export const num: i32 = 32;\n").expect("write module a");

    let package = main_parse_with_diagnostics(entry.to_str().expect("utf8 path"))
        .expect("package parse should succeed");
    let merged = package.into_merged_module();

    let result = analyze(&merged);
    assert!(result.is_ok(), "expected success, got: {:#?}", result.err());
}
