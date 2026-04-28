use std::collections::{HashSet, VecDeque};
use std::io;
use std::path::{Path, PathBuf};

use ns_ast::{Package, PackageModule};
use ns_diagnostics::{Diagnostic, SourceSpan};

use crate::parse_module_with_diagnostics;

pub fn main_parse(entry_module_path: &str) -> io::Result<Package> {
    match main_parse_with_diagnostics(entry_module_path) {
        Ok(pkg) => Ok(pkg),
        Err(diags) => {
            let kind = match diags.first().and_then(|d| d.code.as_deref()) {
                Some("P2001") => io::ErrorKind::NotFound,
                _ => io::ErrorKind::Other,
            };
            Err(io::Error::new(
                kind,
                diags
                    .first()
                    .map(Diagnostic::render_pretty)
                    .unwrap_or_else(|| "unknown parser error".to_string()),
            ))
        }
    }
}

pub fn main_parse_with_diagnostics(entry_module_path: &str) -> Result<Package, Vec<Diagnostic>> {
    let entry = std::fs::canonicalize(entry_module_path).map_err(|err| vec![
        Diagnostic::error_with_code(
            "P2002",
            format!("failed to open entry module `{entry_module_path}`: {err}"),
        )
        .with_span(SourceSpan {
            file: Some(entry_module_path.to_string()),
            line: None,
            column: None,
            start: None,
            end: None,
            label: Some("entry module I/O error".to_string()),
        }),
    ])?;

    let mut queue = VecDeque::new();
    queue.push_back(entry.clone());

    let mut visited = HashSet::new();
    let mut modules = Vec::new();

    while let Some(path) = queue.pop_front() {
        let canonical = std::fs::canonicalize(&path).map_err(|err| vec![
            Diagnostic::error_with_code(
                "P2002",
                format!("failed to canonicalize module `{}`: {err}", path.display()),
            )
            .with_span(SourceSpan {
                file: Some(path.to_string_lossy().to_string()),
                line: None,
                column: None,
                start: None,
                end: None,
                label: Some("module I/O error".to_string()),
            }),
        ])?;
        if !visited.insert(canonical.clone()) {
            continue;
        }

        let module = parse_single_module(&canonical)?;
        let module_dir = canonical.parent().ok_or_else(|| vec![
            Diagnostic::error_with_code(
                "P2003",
                format!("module path has no parent directory: {}", canonical.display()),
            )
            .with_span(SourceSpan {
                file: Some(canonical_path_string(&canonical)),
                line: None,
                column: None,
                start: None,
                end: None,
                label: Some("invalid module path".to_string()),
            }),
        ])?;

        for index_ident in module.index() {
            let name = index_ident.clone().into_simple().as_str().to_string();
            let index_path = module_dir.join(&name).join("index.ns");
            if !index_path.exists() {
                return Err(vec![
                    Diagnostic::error_with_code(
                        "P2001",
                        format!(
                            "index module `{name}` referenced from `{}` not found at `{}`",
                            canonical.display(),
                            index_path.display()
                        ),
                    )
                    .with_span(SourceSpan {
                        file: Some(canonical_path_string(&canonical)),
                        line: None,
                        column: None,
                        start: None,
                        end: None,
                        label: Some("missing index module".to_string()),
                    }),
                ]);
            }
            queue.push_back(index_path);
        }

        modules.push(PackageModule::new(canonical_path_string(&canonical), module));
    }

    Ok(Package::new(canonical_path_string(&entry), modules))
}

fn parse_single_module(path: &Path) -> Result<ns_ast::Module, Vec<Diagnostic>> {
    let path_str = path.to_str().ok_or_else(|| {
        vec![
            Diagnostic::error_with_code(
                "P2000",
                format!("module path is not valid UTF-8: {}", path.display()),
            )
            .with_span(SourceSpan {
                file: None,
                line: None,
                column: None,
                start: None,
                end: None,
                label: Some("invalid module path".to_string()),
            }),
        ]
    })?;
    parse_module_with_diagnostics(path_str)
}

fn canonical_path_string(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{main_parse, main_parse_with_diagnostics};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn main_parse_loads_entry_and_index_modules_as_package() {
        let root = temp_case_dir("package_parse_ok");
        let entry = root.join("main.ns");
        let child_dir = root.join("util");
        let child_index = child_dir.join("index.ns");

        fs::create_dir_all(&child_dir).expect("create child dir");
        fs::write(
            &entry,
            r#"
index util;

function main(): i32 {
    return 1;
}
"#,
        )
        .expect("write entry");
        fs::write(
            &child_index,
            r#"
function helper(): i32 {
    return 2;
}
"#,
        )
        .expect("write child index");

        let pkg = main_parse(entry.to_str().expect("utf8 path")).expect("main_parse should succeed");
        assert_eq!(pkg.modules().len(), 2, "expected entry + one index module");
        assert_eq!(
            pkg.entry_module().expect("entry module should exist").index().len(),
            1
        );
    }

    #[test]
    fn main_parse_errors_when_index_module_is_missing() {
        let root = temp_case_dir("package_parse_missing");
        let entry = root.join("main.ns");
        fs::write(
            &entry,
            r#"
index missing;
"#,
        )
        .expect("write entry");

        let err = main_parse(entry.to_str().expect("utf8 path")).expect_err("must fail");
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn main_parse_reports_syntax_error_as_diagnostic() {
        let root = temp_case_dir("package_parse_syntax");
        let entry = root.join("main.ns");
        fs::write(&entry, "function main( {").expect("write entry");

        let diags = main_parse_with_diagnostics(entry.to_str().expect("utf8 path"))
            .expect_err("must fail with diagnostics");
        assert!(!diags.is_empty());
        assert_eq!(diags[0].severity, ns_diagnostics::Severity::Error);
        assert!(diags[0].render_pretty().contains("main.ns"));
    }

    fn temp_case_dir(label: &str) -> PathBuf {
        let mut dir = std::env::temp_dir();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock drift")
            .as_nanos();
        let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        dir.push(format!(
            "ns_parser_{label}_{}_{}_{}",
            std::process::id(),
            nanos,
            seq
        ));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }
}
