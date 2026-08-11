//! CLI adapter over the core crate.
//!
//! Thin by design: argument handling and I/O live here, logic lives in core.
//! Doubles as a scriptable test harness for the same code the web app runs.

use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: {{project-name}} <file>");
        return ExitCode::FAILURE;
    };
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("{path}: {err}");
            return ExitCode::FAILURE;
        }
    };
    let summary = {{crate_name}}_core::summarize(&bytes);
    println!("{summary:?}");
    ExitCode::SUCCESS
}
