use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use ns_lexer::lexer;
use ns_parser::Parser;

use crate::decl::{SirDecl, SirImplTarget};
use crate::lower::lower_ast_module;
use crate::stmt::SirStmt;
use crate::verify::{verify, verify_typed_sir};

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn parse_source(source: &str) -> ns_ast::Module {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock drift")
        .as_nanos();
    let seq = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    path.push(format!(
        "ns_sir_pipeline_test_{}_{}_{}.ns",
        std::process::id(),
        nanos,
        seq
    ));

    fs::write(&path, source).expect("write temp source");
    let path_str = path.to_str().expect("utf8 temp path");
    let tokens = lexer(path_str);
    let mut parser = Parser::new(tokens);
    let module = parser.parse_module();
    let _ = fs::remove_file(path);
    module
}

#[test]
fn pipeline_lowers_class_inheritance_with_trait_delegation() {
    let module = parse_source(
        r#"
class A {
    x: i32 = 1;
    getX(): i32 {
        return this.x;
    }
}

class B extends A {
    y: i32 = 2;
    getY(): i32 {
        return this.y;
    }
}
"#,
    );
    let sir = lower_ast_module(&module).expect("pipeline should succeed");
    let b = sir.classes.iter().find(|c| c.name == "B").expect("class B");
    assert!(b.inherited_class.is_some());
    assert!(!b.implemented_traits.is_empty());
    assert!(b
        .delegated_trait_impls
        .iter()
        .any(|d| d.via_member == "inherited_class"));
}

#[test]
fn pipeline_lowers_enum_methods_into_impl_block() {
    let module = parse_source(
        r#"
enum Flag {
    A,
    B,
    isA(): bool {
        return true;
    }
}
"#,
    );
    let sir = lower_ast_module(&module).expect("pipeline should succeed");
    assert_eq!(sir.enums.len(), 1);
    assert!(sir.decls.iter().any(
        |d| matches!(d, SirDecl::Impl { target: SirImplTarget::Enum(_), methods } if !methods.is_empty())
    ));
}

#[test]
fn pipeline_canonicalizes_while_to_loop_if_break() {
    let module = parse_source(
        r#"
function main(): i32 {
    let i: i32 = 0;
    while (true) {
        break;
    }
    return i;
}
"#,
    );
    let sir = lower_ast_module(&module).expect("pipeline should succeed");
    let main_fn = sir.functions.iter().find(|f| f.name == "main").expect("main symbol");
    let body = sir
        .decls
        .iter()
        .find_map(|d| match d {
            SirDecl::Function { id, body } if *id == main_fn.id => Some(body),
            _ => None,
        })
        .expect("main body");
    assert!(body.iter().any(|stmt| match stmt {
        SirStmt::Loop { body } => matches!(
            body.first(),
            Some(SirStmt::If { then_body, .. }) if matches!(then_body.first(), Some(SirStmt::Break))
        ),
        _ => false,
    }));
}

#[test]
fn pipeline_rejects_unresolved_links() {
    let module = parse_source(
        r#"
import missing from a;
export missing;
"#,
    );
    let err = lower_ast_module(&module).expect_err("pipeline must fail unresolved links");
    assert!(err.iter().any(|d| {
        d.code
            .as_deref()
            .map(|c| c == "SIR0010" || c == "SIR0011")
            .unwrap_or(false)
    }));
}

#[test]
fn pipeline_rejects_inheritance_cycle() {
    let module = parse_source(
        r#"
class A extends B {}
class B extends A {}
"#,
    );
    let err = lower_ast_module(&module).expect_err("pipeline must fail cycle");
    assert!(err
        .iter()
        .any(|d| d.code.as_deref().map(|c| c == "SIR3002").unwrap_or(false)));
}

#[test]
fn pipeline_produces_verified_typed_sir() {
    let module = parse_source(
        r#"
class A {
    x: i32 = 1;
    getX(): i32 {
        return this.x;
    }
}
"#,
    );
    let sir = lower_ast_module(&module).expect("pipeline should succeed");
    verify(&sir).expect("structural verify should pass");
    verify_typed_sir(&sir).expect("typed verify should pass after fill pass");
}
