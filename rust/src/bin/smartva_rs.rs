//! Drop-in CLI binary (`smartva-rs`) compatible with upstream `smartva` (`va_cli.py`).
//!
//! Supports the exact CLI flags of upstream `SmartVA-Analyze` (`va_cli.py`):
//!
//! ```text
//! smartva-rs [OPTIONS] <INPUT_CSV> <OUTPUT_DIR>
//!   --country <CODE>          Country ISO-3 code (default: "Unknown")
//!   --hiv <BOOL>              Death from HIV possible (default: True)
//!   --malaria <BOOL>          Death from Malaria possible (default: True)
//!   --hce <BOOL>              Use Health Care Experience variables (default: True)
//!   --freetext <BOOL>         Use "open response" variables (default: True)
//!   --figures <BOOL>          Generate figures and graphs (default: True)
//!   --language <LANG>         english | chinese | spanish (default: english)
//!   --legacy-format           Output files in v1.1 directory format
//!   --list-countries          List all valid country codes and exit
//!   --version                 Print version and exit
//! ```

use std::env;
use std::path::PathBuf;
use std::process;

use smartva_rust::{SmartVaEngine, SmartVaRunOptions};

const VERSION_STR: &str = "SmartVA-Analyze 3.0.0-rust (smartva-rust 0.2.0)";

fn parse_bool_flag(name: &str, raw: &str) -> Result<bool, String> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => Ok(true),
        "false" | "f" | "no" | "n" | "0" => Ok(false),
        _ => Err(format!("Invalid boolean value for {name}: {raw:?}")),
    }
}

fn print_help(prog: &str) {
    println!(
        "Usage: {prog} [OPTIONS] [INPUT_CSV] [OUTPUT_DIR]\n\n\
         Drop-in Pure-Rust replacement for SmartVA-Analyze `smartva` CLI.\n\n\
         Options:\n  \
           --country <CODE>       Data origin country abbreviation (default: \"Unknown\")\n  \
           --hiv <BOOL>           Acquire data from an HIV region (default: True)\n  \
           --malaria <BOOL>       Acquire data from a Malaria region (default: True)\n  \
           --hce <BOOL>           Use Health Care Experience (HCE) variables (default: True)\n  \
           --freetext <BOOL>      Use \"open response\" variables (default: True)\n  \
           --figures <BOOL>       Generate figures and graphs for output (default: True)\n  \
           --language <LANG>      Translation language: english | chinese | spanish\n  \
           --legacy-format        Output files in a file structure similar to v1.1\n  \
           --list-countries       List all valid country codes and exit\n  \
           --version              Show program's version number and exit\n  \
           -h, --help             Show this help message and exit"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.first().map(|s| s.as_str()).unwrap_or("smartva-rs");

    let mut opts = SmartVaRunOptions {
        country: "Unknown".to_string(),
        hiv: true,
        malaria: true,
        hce: true,
        freetext: true,
        figures: true,
        language: "english".to_string(),
        legacy_format: false,
    };

    let mut list_countries = false;
    let mut positionals: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            print_help(prog);
            process::exit(0);
        } else if arg == "--version" {
            println!("{VERSION_STR}");
            process::exit(0);
        } else if arg == "--list-countries" {
            list_countries = true;
            i += 1;
        } else if arg == "--legacy-format" {
            opts.legacy_format = true;
            i += 1;
        } else if let Some((flag, val)) = arg.split_once('=') {
            apply_kv_flag(flag, val, &mut opts);
            i += 1;
        } else if matches!(
            arg.as_str(),
            "--country" | "--hiv" | "--malaria" | "--hce" | "--freetext" | "--figures" | "--language"
        ) {
            if i + 1 >= args.len() {
                eprintln!("Error: {arg} requires a value");
                process::exit(2);
            }
            apply_kv_flag(arg, &args[i + 1], &mut opts);
            i += 2;
        } else if arg.starts_with('-') {
            eprintln!("Error: unrecognized option {arg}");
            process::exit(2);
        } else {
            positionals.push(arg.clone());
            i += 1;
        }
    }

    let engine = SmartVaEngine::new();

    if list_countries {
        println!("\nAvailable country codes:");
        for c in engine.list_countries() {
            println!("- {c}");
        }
        process::exit(0);
    }

    if positionals.len() < 2 {
        eprintln!("Error: Both INPUT_CSV and OUTPUT_DIR must be specified.");
        print_help(prog);
        process::exit(2);
    }

    let input_csv = PathBuf::from(&positionals[0]);
    let output_dir = PathBuf::from(&positionals[1]);

    match engine.run_cli_directory(&input_csv, &output_dir, &opts) {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{err}");
            process::exit(2);
        }
    }
}

fn apply_kv_flag(flag: &str, val: &str, opts: &mut SmartVaRunOptions) {
    match flag {
        "--country" => opts.country = val.to_string(),
        "--hiv" => {
            opts.hiv = parse_bool_flag(flag, val).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                process::exit(2);
            })
        }
        "--malaria" => {
            opts.malaria = parse_bool_flag(flag, val).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                process::exit(2);
            })
        }
        "--hce" => {
            opts.hce = parse_bool_flag(flag, val).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                process::exit(2);
            })
        }
        "--freetext" => {
            opts.freetext = parse_bool_flag(flag, val).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                process::exit(2);
            })
        }
        "--figures" => {
            opts.figures = parse_bool_flag(flag, val).unwrap_or_else(|e| {
                eprintln!("Error: {e}");
                process::exit(2);
            })
        }
        "--language" => opts.language = val.to_ascii_lowercase(),
        _ => {
            eprintln!("Error: unrecognized option {flag}");
            process::exit(2);
        }
    }
}
