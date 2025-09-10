// Copyright (c) 2025 balintb - https://github.com/balintb/c2f
// Licensed under the MIT License

use std::env;

pub struct Args {
    pub filename: Option<String>,
    pub append: bool,
    pub quiet: bool,
    pub detect: Option<bool>, // None means use config default
    pub append_ext: bool,
}

impl Args {
    pub fn parse() -> Result<Self, ArgsError> {
        let args: Vec<String> = env::args().collect();
        let mut parsed = Args {
            filename: None,
            append: false,
            quiet: false,
            detect: None,
            append_ext: false,
        };

        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "-h" | "--help" => return Err(ArgsError::Help),
                "-V" | "--version" => return Err(ArgsError::Version),
                "-a" | "--append" => parsed.append = true,
                "-q" | "--quiet" => parsed.quiet = true,
                "-e" | "--append-ext" => parsed.append_ext = true,
                arg if arg.starts_with("--detect=") => {
                    let value = arg.strip_prefix("--detect=").unwrap();
                    match value {
                        "true" => parsed.detect = Some(true),
                        "false" => parsed.detect = Some(false),
                        _ => {
                            return Err(ArgsError::InvalidValue(
                                "--detect".to_string(),
                                value.to_string(),
                            ))
                        }
                    }
                }
                arg if arg.starts_with("-") && !arg.starts_with("--") && arg.len() > 2 => {
                    // Handle combined short flags like -qa, -qe, -aqe
                    for ch in arg[1..].chars() {
                        match ch {
                            'a' => parsed.append = true,
                            'q' => parsed.quiet = true,
                            'e' => parsed.append_ext = true,
                            'h' => return Err(ArgsError::Help),
                            'V' => return Err(ArgsError::Version),
                            _ => return Err(ArgsError::UnknownFlag(format!("-{ch}"))),
                        }
                    }
                }
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
    InvalidValue(String, String),
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
    println!(
        "  [filename]  File to write to. Defaults to 'clipboard' with auto-detected extension"
    );
    println!();
    println!("Options:");
    println!("  -a, --append          Append to file instead of overwriting");
    println!("  -q, --quiet           Suppress all output");
    println!("  -e, --append-ext      Add detected extension to specified filename");
    println!("      --detect=<bool>   Override content detection (true/false)");
    println!("  -h, --help            Print help");
    println!("  -V, --version         Print version");
}

pub fn print_version() {
    println!("c2f {}", env!("CARGO_PKG_VERSION"));
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
        ArgsError::InvalidValue(flag, value) => {
            eprintln!("error: invalid value '{value}' for '{flag}'");
            eprintln!("  [possible values: true, false]");
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
