// Copyright (c) 2025 balintb - https://github.com/balintb/c2f
// Licensed under the MIT License

use crate::args::*;

fn parse_args(cmd: &str) -> Result<Args, ArgsError> {
    let args: Vec<String> = cmd.split_whitespace().map(String::from).collect();

    let mut parsed = Args {
        filename: None,
        append: false,
        quiet: false,
        detect: None,
        append_ext: false,
    };

    let mut i = 1; // skip 1st arg (self)
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
                // combined
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

#[test]
fn test_no_args() {
    let args = parse_args("c2f").unwrap();
    assert_eq!(args.filename, None);
    assert!(!args.append);
    assert!(!args.quiet);
    assert_eq!(args.detect, None);
    assert!(!args.append_ext);
}

#[test]
fn test_filename_only() {
    let args = parse_args("c2f output.txt").unwrap();
    assert_eq!(args.filename, Some("output.txt".to_string()));
    assert!(!args.append);
    assert!(!args.quiet);
}

#[test]
fn test_single_flags() {
    let args = parse_args("c2f -a file.txt").unwrap();
    assert!(args.append);
    assert!(!args.quiet);
    assert!(!args.append_ext);

    let args = parse_args("c2f -q file.txt").unwrap();
    assert!(!args.append);
    assert!(args.quiet);
    assert!(!args.append_ext);

    let args = parse_args("c2f -e file").unwrap();
    assert!(!args.append);
    assert!(!args.quiet);
    assert!(args.append_ext);
}

#[test]
fn test_long_flags() {
    let args = parse_args("c2f --append file.txt").unwrap();
    assert!(args.append);

    let args = parse_args("c2f --quiet file.txt").unwrap();
    assert!(args.quiet);

    let args = parse_args("c2f --append-ext file").unwrap();
    assert!(args.append_ext);
}

#[test]
fn test_combined_flags_two() {
    let args = parse_args("c2f -qa file.txt").unwrap();
    assert!(args.quiet);
    assert!(args.append);
    assert!(!args.append_ext);

    let args = parse_args("c2f -qe file").unwrap();
    assert!(args.quiet);
    assert!(!args.append);
    assert!(args.append_ext);

    let args = parse_args("c2f -ae file").unwrap();
    assert!(args.append);
    assert!(args.append_ext);
    assert!(!args.quiet);
}

#[test]
fn test_combined_flags_three() {
    let args = parse_args("c2f -qae file").unwrap();
    assert!(args.quiet);
    assert!(args.append);
    assert!(args.append_ext);

    let args = parse_args("c2f -aeq file").unwrap();
    assert!(args.append);
    assert!(args.append_ext);
    assert!(args.quiet);

    let args = parse_args("c2f -eqa file").unwrap();
    assert!(args.append_ext);
    assert!(args.quiet);
    assert!(args.append);
}

#[test]
fn test_detect_flag() {
    let args = parse_args("c2f --detect=true file").unwrap();
    assert_eq!(args.detect, Some(true));

    let args = parse_args("c2f --detect=false file").unwrap();
    assert_eq!(args.detect, Some(false));
}

#[test]
fn test_mixed_flags() {
    let args = parse_args("c2f -qa --detect=true file").unwrap();
    assert!(args.quiet);
    assert!(args.append);
    assert_eq!(args.detect, Some(true));

    let args = parse_args("c2f --detect=false -qe file").unwrap();
    assert!(args.quiet);
    assert!(args.append_ext);
    assert_eq!(args.detect, Some(false));
}

#[test]
fn test_help_flag() {
    assert!(matches!(parse_args("c2f -h"), Err(ArgsError::Help)));
    assert!(matches!(parse_args("c2f --help"), Err(ArgsError::Help)));
}

#[test]
fn test_version_flag() {
    assert!(matches!(parse_args("c2f -V"), Err(ArgsError::Version)));
    assert!(matches!(
        parse_args("c2f --version"),
        Err(ArgsError::Version)
    ));
}

#[test]
fn test_unknown_flag() {
    assert!(matches!(
        parse_args("c2f -x"),
        Err(ArgsError::UnknownFlag(_))
    ));
    assert!(matches!(
        parse_args("c2f --unknown"),
        Err(ArgsError::UnknownFlag(_))
    ));
}

#[test]
fn test_invalid_combined_flag() {
    // invalid char in combined flags
    assert!(matches!(
        parse_args("c2f -qax file"),
        Err(ArgsError::UnknownFlag(_))
    ));
}

#[test]
fn test_invalid_detect_value() {
    assert!(matches!(
        parse_args("c2f --detect=maybe"),
        Err(ArgsError::InvalidValue(_, _))
    ));
}

#[test]
fn test_too_many_args() {
    assert!(matches!(
        parse_args("c2f file1 file2"),
        Err(ArgsError::TooManyArgs)
    ));
}

#[test]
fn test_help_in_combined_flags() {
    // -h in combined flags should return help error
    assert!(matches!(parse_args("c2f -qh"), Err(ArgsError::Help)));
    assert!(matches!(parse_args("c2f -ahq"), Err(ArgsError::Help)));
}

#[test]
fn test_version_in_combined_flags() {
    // -V in combined flags should return version error
    assert!(matches!(parse_args("c2f -qV"), Err(ArgsError::Version)));
    assert!(matches!(parse_args("c2f -aVe"), Err(ArgsError::Version)));
}

#[test]
fn test_exit_codes() {
    assert_eq!(ArgsError::Help.exit_code(), 0);
    assert_eq!(ArgsError::Version.exit_code(), 0);
    assert_eq!(ArgsError::UnknownFlag("test".to_string()).exit_code(), 1);
    assert_eq!(ArgsError::TooManyArgs.exit_code(), 1);
    assert_eq!(
        ArgsError::InvalidValue("--detect".to_string(), "maybe".to_string()).exit_code(),
        1
    );
}

#[test]
fn test_filename_methods() {
    let mut args = Args {
        filename: None,
        append: false,
        quiet: false,
        detect: None,
        append_ext: false,
    };

    assert_eq!(args.filename(), "clipboard");
    assert!(!args.has_explicit_filename());

    args.filename = Some("test.txt".to_string());
    assert_eq!(args.filename(), "test.txt");
    assert!(args.has_explicit_filename());
}
