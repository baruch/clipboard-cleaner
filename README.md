# Clipboard Cleaner

A cross-platform clipboard monitoring tool that automatically cleans clipboard entries by removing trailing suffixes and whitespace. Built with Rust for Linux, macOS, and Windows.

## Features

- **Real-time clipboard monitoring** - Automatically detects when clipboard content changes
- **Pattern-based cleaning** - Removes trailing suffixes like ` │································` (with any number of middle dots)
- **Whitespace trimming** - Strips trailing spaces and empty lines
- **Cross-platform** - Works on Linux (X11 and Wayland), macOS, and Windows
- **Dry-run mode** - Preview what would be cleaned without modifying clipboard
- **Configurable logging** - Verbose mode for debugging and monitoring

## Installation

### From Source

```bash
git clone <repository-url>
cd clipboard-cleaner
cargo build --release
```

The binary will be available at `target/release/clipboard-cleaner`.

### Usage

Run the clipboard cleaner in the background:

```bash
clipboard-cleaner
```

### Command Line Options

```
Usage: clipboard-cleaner [OPTIONS]

Options:
      --dry-run             Show what would be cleaned without modifying clipboard
  -v, --verbose             Enable verbose logging to see cleaning operations
      --pattern <PATTERN>   Custom regex pattern for cleaning (advanced users)
      --remove-empty-lines  Also remove trailing empty lines [default: true]
  -h, --help                Print help
  -V, --version             Print version
```

### Examples

**Basic usage:**
```bash
clipboard-cleaner
```

**Dry-run mode** (see what would be cleaned):
```bash
clipboard-cleaner --dry-run
```

**Verbose logging:**
```bash
clipboard-cleaner --verbose
```

**Custom pattern** (advanced users):
```bash
clipboard-cleaner --pattern "custom_regex_pattern"
```

## How It Works

The clipboard cleaner monitors your system clipboard in real-time. When new content is detected, it automatically:

1. **Removes trailing suffixes** matching the pattern ` │[·]+$` (space + vertical bar + middle dots at end of line)
2. **Trims trailing whitespace** from each line
3. **Removes trailing empty lines** (if enabled)
4. **Updates the clipboard** with the cleaned content

### Example Transformations

```
Before: "Hello world │··············"
After:  "Hello world"

Before: "Line 1 │·····\nLine 2   \nLine 3 │···\n\n"
After:  "Line 1\nLine 2\nLine 3"
```

## Platform Support

- **Linux**: Supports both X11 and Wayland display servers
- **macOS**: Native clipboard integration
- **Windows**: Windows API clipboard access

## Requirements

- Rust 1.70 or later (for building from source)
- On Linux: X11 libraries or Wayland support

## Building

```bash
cargo build --release
```

## Testing

Run the unit tests:

```bash
cargo test
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bug reports and feature requests.