use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use ns_lexer::lexer;
use ns_parser::Parser;
use ns_sema::analyze;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn analyze_source(source: &str) -> Result<ns_sema::SemaContext, Vec<ns_sema::Diagnostic>> {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let seq = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.push(format!(
        "nsc_return_paths_{}_{}_{}.ns",
        std::process::id(),
        nanos,
        seq
    ));

    fs::write(&path, source).expect("write temp source");
    let path_str = path.to_str().expect("utf8 temp path");
    let tokens = lexer(path_str);
    let mut parser = Parser::new(tokens);
    let module = parser.parse_module();
    let result = analyze(&module);
    let _ = fs::remove_file(path);
    result
}

fn has_error_code(diags: &[ns_sema::Diagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code.as_deref() == Some(code))
}

#[test]
fn non_void_function_without_return_is_error() {
    let source = r#"
function main(): i32 {
    let a = 1;
}
"#;

    let result = analyze_source(source);
    let diags = result.expect_err("expected missing-return failure");
    assert!(
        has_error_code(&diags, "E0641"),
        "expected E0641, got: {:#?}",
        diags
    );
}

#[test]
fn if_else_both_return_satisfies_non_void_function() {
    let source = r#"
function main(c: boolean): i32 {
    if (c) {
        return 1;
    } else {
        return 2;
    }
}
"#;

    let result = analyze_source(source);
    assert!(result.is_ok(), "expected success, got: {:#?}", result.err());
}
