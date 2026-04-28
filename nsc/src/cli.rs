use std::process::Command;

const DEFAULT_OUT_DIR: &str = "./out/rust_app";

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Check,
    Build,
    Run,
    Dev,
}

#[derive(Debug, Clone, Copy)]
pub struct Cli {
    pub mode: Mode,
    pub release: bool,
    pub help: bool,
}

impl Cli {
    pub fn parse(args: Vec<String>) -> Self {
        let mut mode = Mode::Check;
        let mut release = false;
        let mut help = false;

        for arg in args {
            match arg.as_str() {
                "--build" => mode = Mode::Build,
                "--run" => mode = Mode::Run,
                "--dev" => mode = Mode::Dev,
                "--release" => release = true,
                "--help" | "-h" => help = true,
                _ => {}
            }
        }

        Self {
            mode,
            release,
            help,
        }
    }
}

pub fn execute_mode(cli: &Cli) -> std::io::Result<Option<i32>> {
    match cli.mode {
        Mode::Check => Ok(None),
        Mode::Build => {
            cargo_build(DEFAULT_OUT_DIR, cli.release)?;
            Ok(None)
        }
        Mode::Run => cargo_run(DEFAULT_OUT_DIR, cli.release),
        Mode::Dev => {
            cargo_build(DEFAULT_OUT_DIR, false)?;
            cargo_run(DEFAULT_OUT_DIR, false)
        }
    }
}

pub fn print_help() {
    println!("nsc - NetherScript compiler");
    println!();
    println!("Usage:");
    println!("  nsc [--build | --run | --dev] [--release]");
    println!();
    println!("Modes:");
    println!("  (default)   generate Rust crate + cargo check");
    println!("  --build     generate + check + cargo build");
    println!("  --run       generate + check + cargo run");
    println!("  --dev       generate + check + cargo build + cargo run");
    println!();
    println!("Flags:");
    println!("  --release   use release profile for --build/--run");
    println!("  -h, --help  show this help");
}

pub fn format_run_exit_code(code: Option<i32>) -> String {
    match code {
        Some(exit) if exit != 0 => format!(" (program exit code {exit})"),
        _ => String::new(),
    }
}

fn cargo_build(out_dir: &str, release: bool) -> std::io::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build").current_dir(out_dir);
    if release {
        cmd.arg("--release");
    }
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("cargo build returned non-zero exit status"))
    }
}

fn cargo_run(out_dir: &str, release: bool) -> std::io::Result<Option<i32>> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").current_dir(out_dir);
    if release {
        cmd.arg("--release");
    }
    let status = cmd.status()?;
    if status.success() {
        Ok(Some(0))
    } else {
        // Non-zero run status can be a valid program result (e.g. NS main(): i32).
        Ok(status.code())
    }
}
