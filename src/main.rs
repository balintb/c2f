use arboard::Clipboard;
use c2f::{determine_action, load_config};
use clap::{Arg, Command};
use std::fs;
use std::io::{self, Write};

fn ask_confirmation(filename: &str, append: bool) -> bool {
    let action = determine_action(filename, append);
    print!("Are you sure you want to {action} '{filename}'? (y/n): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase() == "y"
}

fn main() {
    let matches = Command::new("c2f")
        .version("0.1.0")
        .about("Write clipboard contents to file")
        .arg(
            Arg::new("filename")
                .help("File to write to. Defaults to 'clipboard' if not provided")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("append")
                .short('a')
                .long("append")
                .help("Append to file instead of overwriting")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress all output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let filename = matches
        .get_one::<String>("filename")
        .map(|s| s.as_str())
        .unwrap_or("clipboard");
    let append = matches.get_flag("append");
    let quiet_flag = matches.get_flag("quiet");

    if filename == "clipboard"
        && !matches.contains_id("filename")
        && !append
        && std::path::Path::new("clipboard").exists()
    {
        eprintln!(
            "Default file 'clipboard' already exists. Use -a to append or specify a filename."
        );
        std::process::exit(1);
    }

    let config = load_config();
    let quiet = quiet_flag || config.quiet;

    // clipboard contents
    let mut clipboard = Clipboard::new().expect("Failed to initialize clipboard");
    let clipboard_contents = match clipboard.get_text() {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading clipboard: {e}");
            std::process::exit(1);
        }
    };

    if clipboard_contents.is_empty() {
        eprintln!("Clipboard empty - nothing to write.");
        std::process::exit(1);
    }

    if config.ask_confirmation && !ask_confirmation(filename, append) {
        if !quiet {
            println!("Cancelled.");
        }
        return;
    }

    let result = if append {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .and_then(|mut file| file.write_all(clipboard_contents.as_bytes()))
    } else {
        fs::write(filename, clipboard_contents.as_bytes())
    };

    match result {
        Ok(_) => {
            if !quiet {
                let action = if append { "appended to" } else { "written to" };
                println!("Successfully {action} '{filename}'");
            }
        }
        Err(e) => {
            eprintln!("Error writing to file: {e}");
            std::process::exit(1);
        }
    }
}
