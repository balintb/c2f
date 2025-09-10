# `c2f` - Clipboard to file

Simple CLI tool to write clipboard contents to a file.

## Overview

`c2f` is a lightweight cross-platform command-line utility that saves or appends your clipboard contents to a file.

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

Requires Rust 1.70+

```bash
git clone https://github.com/balintb/c2f
cd c2f
cargo install --path .
```

### Pre-built Binaries

Download the latest release from the [releases page](https://github.com/balintb/c2f/releases).

## Usage

```bash
# write clipboard to default file ('clipboard')
c2f

# write clipboard to specific file
c2f myfile.txt

# append to file
c2f -a existing.txt

# quiet mode
c2f -q output.txt
```

### Options

- `-a, --append` - append to file instead of overwriting
- `-q, --quiet` - suppress all output  
- `-h, --help` - Help
- `-V, --version` - Version

## Configuration

c2f supports a configuration file at `~/.config/c2f/config.toml`:

```toml
ask_confirmation = false # ask for confirmation before writing (default false)
quiet = false            # quiet mode (default false)
```

## Examples

```bash
# copy text
echo "Hello World" | pbcopy  # macos
# then write it
c2f hello.txt

# append clipboard to file
c2f -a ~/notes.log

# save clipboard quietly
c2f -q snippet.js
```

## Platform Support

`c2f` is cross-platform:

- macOS
- Linux
- Windows

## License

MIT License - see [LICENSE](LICENSE) for details.
`c2f` Copyright 2025 [balintb](https://github.com/balintb)
