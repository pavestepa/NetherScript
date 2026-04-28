use std::fs;
use std::path::Path;
use std::process::Command;

mod cli;

use cli::{execute_mode, format_run_exit_code, print_help, Cli, Mode};
use ns_codegen::generate_rust;
use ns_parser::main_parse_with_diagnostics;
use ns_sir::lower::lower_ast_module;

const ENTRY_NS_PATH: &str = "./from/main.ns";
const RUST_OUT_DIR: &str = "./out/rust_app";

fn main() {
    let cli = Cli::parse(std::env::args().skip(1).collect());
    if cli.help {
        print_help();
        return;
    }

    let parsed = match main_parse_with_diagnostics(ENTRY_NS_PATH) {
        Ok(package) => package.into_merged_module(),
        Err(diagnostics) => {
            eprintln!("ns-parser: {} error(s)", diagnostics.len());
            for d in diagnostics {
                eprintln!("{}", d.render_pretty());
            }
            std::process::exit(1);
        }
    };

    let sir = match lower_ast_module(&parsed) {
        Ok(sir) => sir,
        Err(diagnostics) => {
            eprintln!("ns-sir: {} error(s)", diagnostics.len());
            for d in diagnostics {
                eprintln!("{}", d.render_pretty());
            }
            std::process::exit(1);
        }
    };

    let rust_source = generate_rust(&sir);
    if let Err(err) = write_rust_project(RUST_OUT_DIR, &rust_source) {
        eprintln!("nsc: failed to write Rust project: {err}");
        std::process::exit(1);
    }
    if let Err(err) = check_generated_project(RUST_OUT_DIR) {
        eprintln!("nsc: generated Rust crate failed cargo check: {err}");
        std::process::exit(1);
    }
    let run_exit_code = match execute_mode(&cli) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("nsc: post-generation command failed: {err}");
            std::process::exit(1);
        }
    };

    println!("ns-parser: ok");
    println!("ns-sir: ok");
    println!("ns-codegen: wrote Rust crate to {RUST_OUT_DIR}");
    println!("ns-codegen: cargo check ok");
    match cli.mode {
        Mode::Check => {}
        Mode::Build => println!("nsc: cargo build ok"),
        Mode::Run => println!("nsc: cargo run ok{}", format_run_exit_code(run_exit_code)),
        Mode::Dev => println!(
            "nsc: dev mode ok (build + run){}",
            format_run_exit_code(run_exit_code)
        ),
    }
}

fn write_rust_project(out_dir: &str, rust_main_source: &str) -> std::io::Result<()> {
    let out_path = Path::new(out_dir);
    let src_dir = out_path.join("src");
    fs::create_dir_all(&src_dir)?;

    let cargo_toml = r#"[package]
name = "netherscript_output"
version = "0.1.0"
edition = "2026"

[dependencies]
"#;
    fs::write(out_path.join("Cargo.toml"), cargo_toml)?;
    fs::write(src_dir.join("main.rs"), rust_main_source)?;
    Ok(())
}

fn check_generated_project(out_dir: &str) -> std::io::Result<()> {
    let status = Command::new("cargo")
        .arg("check")
        .current_dir(out_dir)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(
            "cargo check returned non-zero exit status",
        ))
    }
}
