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
        "nsc_generic_type_args_{}_{}_{}.ns",
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
fn generic_call_type_arguments_are_not_parsed_as_comparisons() {
    let source = r#"
function main(): i32 {
    return init<i32>(2);
}

function init<T>(value: T): T {
    return value;
}
"#;

    let result = analyze_source(source);
    assert!(result.is_ok(), "expected success, got: {:#?}", result.err());
}

#[test]
fn generic_type_parameter_t_is_valid_type_in_checker() {
    let source = r#"
function identity<T>(value: T): T {
    let local: T = value;
    return local;
}
"#;

    let result = analyze_source(source);
    match result {
        Ok(_) => {}
        Err(diags) => {
            assert!(
                !has_error_code(&diags, "E0634"),
                "generic type parameter should not trigger E0634: {:#?}",
                diags
            );
            panic!("expected success, got diagnostics: {:#?}", diags);
        }
    }
}
