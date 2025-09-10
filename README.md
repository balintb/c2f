# `c2f` - Clipboard to file

Simple CLI tool to write clipboard contents to a file with content format detection.

## Overview

`c2f` is a lightweight cross-platform command-line utility that saves or appends your clipboard contents to a file. It can automatically detect content types and add an appropriate file extension.

![GitHub top language](https://img.shields.io/github/languages/top/balintb/c2f)
![Crates.io License](https://img.shields.io/crates/l/c2f)

![Crates.io Version](https://img.shields.io/crates/v/c2f)
![Crates.io Size](https://img.shields.io/crates/size/c2f)
![Crates.io MSRV](https://img.shields.io/crates/msrv/c2f?logo=rust)

## Installation

### Homebrew (Recommended)

```bash
brew install balintb/tap/c2f
```

### Cargo

```bash
cargo install c2f
```

### From Source

Requires Rust 1.60+

```bash
git clone https://github.com/balintb/c2f
cd c2f
cargo install --path .
```

### Pre-built Binaries

Download the latest release from the [releases page](https://github.com/balintb/c2f/releases).

## Usage

```bash
# write clipboard with auto-detected extension
c2f
# creates clipboard.json / clipboard.py / clipboard.md / etc.

# write clipboard to specific file
c2f myfile.txt

# append to file
c2f -a existing.txt

# quiet mode (no output)
c2f -q

# override content detection
c2f --detect=false  # always use .txt extension
c2f --detect=true   # force detection even if disabled in config
```

### Options

- `-a, --append` - Append to file instead of overwriting
- `-q, --quiet` - Suppress all output
- `-e, --append-ext` - Add detected extension to specified filename
- `--detect=<bool>` - Override content detection (true/false)
- `-h, --help` - Print help
- `-V, --version` - Print version

## Configuration

c2f supports a configuration file at `~/.config/c2f/config.toml`:

```toml
ask_confirmation = false  # Ask for confirmation before writing (default: false)
quiet = false             # Suppress all output (default: false)
detect_type = true        # Enable content type detection (default: true)
```

Command-line flags override config file settings.

## Examples

### Basic Usage

```bash
# copy JSON and save with autodetected extension
echo '{"name": "test"}' | pbcopy  # macOS
c2f  # creates clipboard.json

# copy Python and save
echo 'import sys' | pbcopy
c2f  # creates clipboard.py

# copy MD and save
echo '# Title' | pbcopy
c2f  # creates clipboard.md
```

### Specific Filename

```bash
# save to specific file
c2f output.txt

# append to existing file
c2f -a notes.log

# quiet mode (no output)
c2f -q data.json
```

### Detection Control

```bash
# disable detection (always use .txt)
c2f --detect=false

# force detection even if disabled in config
c2f --detect=true

# append detected extension to specified filename
echo '{"data": true}' | pbcopy
c2f -e myfile  # creates myfile.json
c2f --append-ext output  # creates output.json
```

### More Examples

```bash
# quietly append JSON to log file with explicit detection
echo '{"timestamp": "2025-01-10", "event": "test"}' | pbcopy
c2f -qa --detect=true events.log

# save code snippet with auto-extension in quiet mode
echo 'def hello(): print("world")' | pbcopy
c2f -qe snippet  # creates snippet.py silently

# override config to force detection and append extension
c2f --detect=true --append-ext report  # creates report.{ext} based on content

# copy SQL query and append to daily log
c2f -a ~/logs/queries-$(date +%Y%m%d).sql

# copy code from browser/IDE, then:
c2f -qe ~/snippets/solution  # auto-detects language, adds extension

# append API responses to a file
c2f -qa api-responses.jsonl  # append mode for JSON Lines format

# note-taking: save formatted content with proper extensions
c2f -e ~/notes/$(date +%Y%m%d)-meeting  # creates dated .md file
```

## Supported Formats

`c2f` can automatically detect and assign appropriate extensions for:

**Data Formats**: JSON (`.json`), XML (`.xml`), YAML (`.yaml`), TOML (`.toml`), CSV (`.csv`), SQL (`.sql`)

**Programming Languages**:

- Rust (`.rs`), Python (`.py`), JavaScript (`.js`), TypeScript (`.ts`)
- Go (`.go`), Java (`.java`), C# (`.cs`), C++ (`.cpp`), C (`.c`)
- Shell (`.sh`), PowerShell (`.ps1`), Ruby (`.rb`), PHP (`.php`)
- Swift (`.swift`), Kotlin (`.kt`)

**Markup & Style**: HTML (`.html`), Markdown (`.md`), LaTeX (`.tex`), CSS (`.css`), SCSS (`.scss`)

**Config Files**: `Dockerfile`, `.gitignore`, `Makefile`, `.env`, INI (`.ini`)

**Images**: PNG images from clipboard

**Fallback**: Plain text (`.txt`) for unrecognized content

## Platform Support

`c2f` is cross-platform:

- macOS
- Linux
- Windows

## License

MIT License - see [LICENSE](LICENSE) for details.

Copyright 2025 [balintb](https://github.com/balintb)
