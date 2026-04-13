# idgen

[![CI](https://github.com/nikbucher/idgen/actions/workflows/ci.yml/badge.svg)](https://github.com/nikbucher/idgen/actions/workflows/ci.yml)

A CLI tool for generating random [NanoID](https://github.com/ai/nanoid)-style identifiers.

## Install

### Homebrew

```sh
brew install nikbucher/tap/idgen
```

### Pre-built binaries

Download the latest binary for your platform from [GitHub Releases](https://github.com/nikbucher/idgen/releases):

| Platform | Architecture  | Download                                  |
|----------|---------------|-------------------------------------------|
| Linux    | x86_64        | `idgen-x86_64-unknown-linux-gnu.tar.gz`   |
| Linux    | aarch64       | `idgen-aarch64-unknown-linux-gnu.tar.gz`  |
| macOS    | Intel         | `idgen-x86_64-apple-darwin.tar.gz`        |
| macOS    | Apple Silicon | `idgen-aarch64-apple-darwin.tar.gz`       |
| Windows  | x86_64        | `idgen-x86_64-pc-windows-msvc.zip`        |
| Windows  | aarch64       | `idgen-aarch64-pc-windows-msvc.zip`       |

Extract the archive and place the `idgen` binary somewhere on your `PATH`.

### Cargo

```sh
cargo install idgen
```

### From source

```sh
git clone https://github.com/nikbucher/idgen.git
cd idgen
cargo install --path .
```

## Usage

```sh
# Generate a single ID with defaults (lowercase, 20 chars, blocks of 4)
idgen

# Generate 5 IDs
idgen -n 5

# Use uppercase ambiguity-safe alphabet
idgen -a uppercase

# Custom size, block size, and delimiter
idgen -s 16 -b 4 -d .

# Quiet mode (no config output, just the ID)
idgen -q

# Combine options
idgen -a all -s 32 -b 8 -d _ -n 3 -q
```

## Options

| Flag | Long           | Default     | Description                |
|------|----------------|-------------|----------------------------|
| `-a` | `--alphabet`   | `lowercase` | Alphabet to use            |
| `-b` | `--block-size` | `4`         | Characters per block       |
| `-d` | `--delimiter`  | `-`         | Delimiter between blocks   |
| `-s` | `--size`       | `20`        | Total number of characters |
| `-n` | `--count`      | `1`         | Number of IDs to generate  |
| `-q` | `--quiet`      | `false`     | Suppress config output     |

## Alphabets

Alphabets marked as ambiguity-safe exclude characters that look alike when read on screen or in print — such as `0`/`O`, `1`/`l`, `8`/`B` — making generated IDs easier to read, type, and communicate verbally.

| Name                      | Characters                           | Description                                               |
|---------------------------|--------------------------------------|-----------------------------------------------------------|
| `j-nano`                  | `_-0-9a-zA-Z`                        | NanoID default (64 chars)                                 |
| `all`                     | `0-9a-zA-Z`                          | All alphanumeric (62 chars)                               |
| `uppercase`               | `34679ACEFGHJKLMNPQRTUVWXY`          | Ambiguity-safe uppercase (excludes 0, 1, 2, 5, 8, B, D, I, O, S, Z) |
| `all-uppercase`           | `0-9A-Z`                             | All uppercase (36 chars)                                  |
| `lowercase`               | `023456789abcdefghijkmnopqrstuvwxyz` | Ambiguity-safe lowercase (excludes 1, l)                  |
| `all-lowercase`           | `0-9a-z`                             | All lowercase (36 chars)                                  |
| `uppercase-and-lowercase` | `34679ACEFGHJLMNPQRTUVWXY...`        | Ambiguity-safe mixed (excludes 0, 1, 2, 5, 8, B, D, I, K, O, S, Z, k, l) |

## Releasing

Releases are automated via GitHub Actions. To create a new release:

```sh
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

This triggers a cross-platform build and creates a GitHub Release with binaries for Linux, macOS, and Windows. The Homebrew formula is updated automatically.

## Documentation

- [Vision](docs/vision.md) — project goals and scope
- [Use cases](docs/use_cases/) — functional specifications

## License

[MIT](LICENSE)
