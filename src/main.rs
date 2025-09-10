mod args;

use arboard::Clipboard;
use c2f::{determine_action, load_config};
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

    let filename = args.filename();
    let append = args.append;
    let quiet_flag = args.quiet;

    if filename == "clipboard"
        && !args.has_explicit_filename()
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
