<!-- PROJECT LOGO / BANNER -->
<p align="center">
  <img src="assets/README-header.png" alt="gol" width="100%">
</p>

<p align="center">
  <img src="assets/icon-128.png" alt="gol icon" width="64">
</p>

<p align="center">
  <strong>gol</strong> -- Conway's Game of Life in a Bevy window
  <br>
  <a href="https://crates.io/crates/gol"><img src="https://img.shields.io/crates/v/gol" alt="crates.io"></a>
  <a href="https://github.com/mrdwarf7/gol.rs/actions/workflows/build.yml"><img src="https://github.com/mrdwarf7/gol.rs/actions/workflows/build.yml/badge.svg" alt="build"></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT-blue" alt="license"></a>
  <a href="https://github.com/mrdwarf7/gol.rs/releases"><img src="https://img.shields.io/github/v/release/mrdwarf7/gol.rs" alt="release"></a>
</p>

<!-- TAGLINE + DESCRIPTION -->
## gol

Conway's Game of Life rendered in a Bevy window. A fixed 80x60 grid is
spawned at startup and stepped with B3/S23 rules.

```bash
cargo run
```

## Features

- Bevy window with a 2D camera
- Static 80x60 cell grid created on init
- Classic B3/S23 rules, toroidal wrap
- Seeded with a Gosper glider gun plus a glider, blinker, toad, and beacon
- Linux, macOS, Windows

## Screenshots

<!-- Add screenshots/GIFs for TUIs/GUIs -->
<!-- ![Demo](assets/demo.gif) -->

## Installation

### Cargo (Recommended)

```bash
cargo install gol
```

### One-liner (Linux/macOS)

```bash
curl -fsSL https://github.com/mrdwarf7/gol.rs/raw/main/build/install.sh | sh
```

Installs to `/usr/local/bin` (or `~/.local/bin` if not writable). Set `GOL_VERSION=vX.Y.Z` to pin a version.

### System Packages

| OS | Command |
|----|---------|
| Arch | `pacman -S gol` |
| macOS | `brew install gol` |
| Fedora | `dnf copr enable mrdwarf7/gol && dnf install gol` |
| NixOS | `nix-shell -p gol` |
| Windows | `winget install gol` |

### Release Archives

Download from [Releases](https://github.com/mrdwarf7/gol.rs/releases/latest).

Each archive contains:

```
gol-<target>-<tag>.zip
  gol[.exe]
  README.md
  LICENSE-MIT
  LICENSE-APACHE
  THIRD_PARTY_NOTICES.md
```

### Supported Targets

| OS | Arch | Triple |
|----|------|--------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | arm64 | `aarch64-unknown-linux-gnu` |
| macOS | Intel | `x86_64-apple-darwin` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |

## Usage

```bash
gol [OPTIONS] <INPUT>
```

### Examples

```bash
# Basic usage
gol input.txt

# With output file
gol input.txt -o output.txt

# Dry run (show what would happen)
gol --dry-run input.txt

# Verbose logging
RUST_LOG=debug gol input.txt
```

### Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--help` | `-h` | Show help |
| `--version` | `-V` | Print version |
| `--output` | `-o` | Output file path |
| `--dry-run` | | Show actions without executing |
| `--quiet` | `-q` | Suppress non-error output |
| `--verbose` | `-v` | Verbose output (repeat for more) |

Logging: `RUST_LOG=debug gol ...` (default: `info`)

## Dependencies

| Tool | Minimum Version | Install |
|------|-----------------|---------|
| ffmpeg | 4.0+ | `pacman -S ffmpeg` / `brew install ffmpeg` |
| libpcap | 1.9+ | `pacman -S libpcap` / `apt install libpcap-dev` |

## Configuration

Config file at `~/.config/gol/config.toml`:

```toml
option = "value"
```

Environment variables:
- `GOL_LOG=debug` — Enable debug logging

## How It Works

1. **Step 1** — Description
2. **Step 2** — Description
3. **Step 3** — Description

## Build

```bash
# Release binary
make build          # or: cargo build --release

# Run tests
make test           # or: cargo test

# Run locally
make run            # or: cargo run -- <args>

# Fetch bundled dependencies (ffmpeg, etc.)
./build/fetch-deps.sh <target> <out-dir>
```

## Comparison

| Feature | gol | Competitor A | Competitor B |
|---------|-----------------|--------------|--------------|
| Feature 1 | ✅ | ✅ | ❌ |
| Feature 2 | ✅ | ❌ | ✅ |
| Feature 3 | ✅ | ✅ | ✅ |

## Links

- [Documentation](https://docs.example.com)
- [Changelog](CHANGELOG.md)
- [Discord](https://discord.gg/XXX)
- [Issues](https://github.com/mrdwarf7/gol.rs/issues)

## License

MIT OR Apache-2.0 — see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).