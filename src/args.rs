// Copyright (c) 2025 balintb - https://github.com/balintb/c2f
// Licensed under the MIT License

use std::env;

pub struct Args {
    pub filename: Option<String>,
    pub append: bool,
    pub quiet: bool,
}

impl Args {
    pub fn parse() -> Result<Self, ArgsError> {
        let args: Vec<String> = env::args().collect();
        let mut parsed = Args {
            filename: None,
            append: false,
            quiet: false,
        };

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "-h" | "--help" => return Err(ArgsError::Help),
                "-V" | "--version" => return Err(ArgsError::Version),
                "-a" | "--append" => parsed.append = true,
                "-q" | "--quiet" => parsed.quiet = true,
                arg if arg.starts_with('-') => return Err(ArgsError::UnknownFlag(arg.to_string())),
                _ => {
                    if parsed.filename.is_none() {
                        parsed.filename = Some(arg.to_string());
                    } else {
                        return Err(ArgsError::TooManyArgs);
                    }
                }
            }
            i += 1;
        }

        Ok(parsed)
    }

    pub fn filename(&self) -> &str {
        self.filename.as_deref().unwrap_or("clipboard")
    }

    pub fn has_explicit_filename(&self) -> bool {
        self.filename.is_some()
    }
}

pub enum ArgsError {
    Help,
    Version,
    UnknownFlag(String),
    TooManyArgs,
}

impl ArgsError {
    pub fn exit_code(&self) -> i32 {
        match self {
            ArgsError::Help | ArgsError::Version => 0,
            _ => 1,
        }
    }
}

pub fn print_help() {
    print_version();
    println!("Write clipboard contents to file");
    println!();
    println!("Usage: c2f [OPTIONS] [filename]");
    println!();
    println!("Arguments:");
    println!("  [filename]  File to write to. Defaults to 'clipboard' if not provided");
    println!();
    println!("Options:");
    println!("  -a, --append   Append to file instead of overwriting");
    println!("  -q, --quiet    Suppress all output");
    println!("  -h, --help     Print help");
    println!("  -V, --version  Print version");
}

pub fn print_version() {
    println!("c2f 0.0.5");
}

pub fn print_error(err: &ArgsError) {
    match err {
        ArgsError::UnknownFlag(flag) => {
            eprintln!("error: unexpected argument '{flag}' found");
            eprintln!();
            eprintln!("Usage: c2f [OPTIONS] [filename]");
            eprintln!();
            eprintln!("For more information, try '--help'.");
        }
        ArgsError::TooManyArgs => {
            eprintln!("error: unexpected argument found");
            eprintln!();
            eprintln!("Usage: c2f [OPTIONS] [filename]");
            eprintln!();
            eprintln!("For more information, try '--help'.");
        }
        _ => {}
    }
}
