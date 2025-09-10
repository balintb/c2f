// Copyright (c) 2025 balintb - https://github.com/balintb/c2f
// Licensed under the MIT License

mod args;
mod detect;

use arboard::Clipboard;
use c2f::{determine_action, load_config};
use detect::{detect_content, ClipboardContent, ContentType};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

fn ask_confirmation(filename: &str, append: bool) -> bool {
    let action = determine_action(filename, append);
    print!("Are you sure you want to {action} '{filename}'? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase() == "y"
}

fn main() {
    let args = match args::Args::parse() {
        Ok(args) => args,
        Err(err) => {
            match err {
                args::ArgsError::Help => args::print_help(),
                args::ArgsError::Version => args::print_version(),
                _ => args::print_error(&err),
            }
            std::process::exit(err.exit_code());
        }
    };

    let append = args.append;
    let quiet_flag = args.quiet;
    let config = load_config();
    let quiet = quiet_flag || config.quiet;

    // Get clipboard contents and detect type
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    // Use explicit --detect flag if provided, otherwise use config setting
    let should_detect = args.detect.unwrap_or(config.detect_type);

    let (content_type, clipboard_content) = if !should_detect {
        // Skip detection, treat as plain text
        let text = clipboard
            .get_text()
            .map_err(|e| format!("Error reading clipboard: {e}"))
            .unwrap_or_else(|e| {
                eprintln!("{e}");
                std::process::exit(1);
            });

        if text.is_empty() {
            eprintln!("Clipboard is empty");
            std::process::exit(1);
        }

        (ContentType::PlainText, ClipboardContent::Text(text))
    } else {
        match detect_content(&mut clipboard) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    };

    // Determine filename
    let filename = if args.has_explicit_filename() {
        let base_filename = args.filename().to_string();

        // If --append-ext is used and detection is enabled, append the extension
        if args.append_ext && should_detect {
            let extension = content_type.extension();
            // Only append extension if the filename doesn't already have it
            if !base_filename.ends_with(&format!(".{extension}")) {
                format!("{base_filename}.{extension}")
            } else {
                base_filename
            }
        } else {
            base_filename
        }
    } else {
        // Generate filename with detected extension
        let base_name = "clipboard";
        let extension = content_type.extension();
        let mut filename = format!("{base_name}.{extension}");

        // If file exists and not appending, add number suffix
        if !append && Path::new(&filename).exists() {
            let mut counter = 2;
            loop {
                filename = format!("{base_name}-{counter}.{extension}");
                if !Path::new(&filename).exists() {
                    break;
                }
                counter += 1;
            }
        }
        filename
    };

    if config.ask_confirmation && !ask_confirmation(&filename, append) {
        if !quiet {
            println!("Cancelled.");
        }
        return;
    }

    // Get bytes to write
    let bytes = match clipboard_content {
        ClipboardContent::Image(bytes) => bytes,
        ClipboardContent::Text(text) => text.into_bytes(),
    };

    let result = if append {
        // Only append for text files
        if matches!(content_type, ContentType::Image) {
            eprintln!("Cannot append to image files");
            std::process::exit(1);
        }
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&filename)
            .and_then(|mut file| file.write_all(&bytes))
    } else {
        fs::write(&filename, &bytes)
    };

    match result {
        Ok(_) => {
            if !quiet {
                let action = if append { "appended to" } else { "written to" };
                if should_detect && !args.has_explicit_filename() {
                    println!("Detected format: {content_type}");
                }
                println!("Successfully {action} '{filename}'");
            }
        }
        Err(e) => {
            eprintln!("Error writing to file: {e}");
            std::process::exit(1);
        }
    }
}
