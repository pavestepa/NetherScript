use std::collections::{HashSet, VecDeque};
use std::io;
use std::path::{Path, PathBuf};

use ns_ast::{Package, PackageModule};
use ns_lexer::lexer;

use crate::Parser;

pub fn main_parse(entry_module_path: &str) -> io::Result<Package> {
    let entry = std::fs::canonicalize(entry_module_path)?;

    let mut queue = VecDeque::new();
    queue.push_back(entry.clone());

    let mut visited = HashSet::new();
    let mut modules = Vec::new();

    while let Some(path) = queue.pop_front() {
        let canonical = std::fs::canonicalize(&path)?;
        if !visited.insert(canonical.clone()) {
            continue;
        }

        let module = parse_single_module(&canonical)?;
        let module_dir = canonical.parent().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("module path has no parent directory: {}", canonical.display()),
            )
        })?;

        for index_ident in module.index() {
            let name = index_ident.clone().into_simple().as_str().to_string();
            let index_path = module_dir.join(&name).join("index.ns");
            if !index_path.exists() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "index module `{name}` referenced from `{}` not found at `{}`",
                        canonical.display(),
                        index_path.display()
                    ),
                ));
            }
            queue.push_back(index_path);
        }

        modules.push(PackageModule::new(canonical_path_string(&canonical), module));
    }

    Ok(Package::new(canonical_path_string(&entry), modules))
}

fn parse_single_module(path: &Path) -> io::Result<ns_ast::Module> {
    let path_str = path.to_str().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("module path is not valid UTF-8: {}", path.display()),
        )
    })?;
    let tokens = lexer(path_str);
    let mut parser = Parser::new(tokens);
    Ok(parser.parse_module())
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

    use super::main_parse;

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
