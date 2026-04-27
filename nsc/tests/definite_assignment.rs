use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use ns_lexer::lexer;
use ns_parser::Parser;
use ns_sema::analyze;

fn analyze_source(source: &str) -> Result<ns_sema::SemaContext, Vec<ns_sema::Diagnostic>> {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    path.push(format!("nsc_definite_assignment_{nanos}.ns"));

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
fn return_uninitialized_local_is_error() {
    let source = r#"
function test(): i32 {
    let a: i32;
    return a;
}
"#;

    let result = analyze_source(source);
    let diags = result.expect_err("expected use-before-init to fail");
    assert!(
        has_error_code(&diags, "E0637"),
        "expected E0637, got: {:#?}",
        diags
    );
}

#[test]
fn assign_then_return_is_ok() {
    let source = r#"
function test(): i32 {
    let a: i32;
    a = 1;
    return a;
}
"#;

    let result = analyze_source(source);
    assert!(result.is_ok(), "expected success, got: {:#?}", result.err());
}

#[test]
fn if_without_else_does_not_definitely_initialize() {
    let source = r#"
function test(c: boolean): i32 {
    let a: i32;
    if (c) {
        a = 1;
    }
    return a;
}
"#;

    let result = analyze_source(source);
    let diags = result.expect_err("expected use-before-init to fail");
    assert!(
        has_error_code(&diags, "E0637"),
        "expected E0637, got: {:#?}",
        diags
    );
}

#[test]
fn if_else_with_both_branches_initialized_is_ok() {
    let source = r#"
function test(c: boolean): i32 {
    let a: i32;
    if (c) {
        a = 1;
    } else {
        a = 2;
    }
    return a;
}
"#;

    let result = analyze_source(source);
    assert!(result.is_ok(), "expected success, got: {:#?}", result.err());
}

#[test]
fn while_init_is_not_propagated_outside_loop() {
    let source = r#"
function test(c: boolean): i32 {
    let a: i32;
    while (c) {
        a = 1;
    }
    return a;
}
"#;

    let result = analyze_source(source);
    let diags = result.expect_err("expected use-before-init to fail");
    assert!(
        has_error_code(&diags, "E0637"),
        "expected E0637, got: {:#?}",
        diags
    );
}

#[test]
fn loop_init_is_not_propagated_for_mvp() {
    let source = r#"
function test(): i32 {
    let a: i32;
    loop {
        a = 1;
        break;
    }
    return a;
}
"#;

    let result = analyze_source(source);
    let diags = result.expect_err("expected use-before-init to fail in MVP");
    assert!(
        has_error_code(&diags, "E0637"),
        "expected E0637, got: {:#?}",
        diags
    );
}
