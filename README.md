# idgen

[![CI](https://github.com/nikbucher/idgen/actions/workflows/ci.yml/badge.svg)](https://github.com/nikbucher/idgen/actions/workflows/ci.yml)

A CLI tool for generating random identifiers with customizable alphabets and block formatting.

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

## Synopsis

```text
$ idgen --help
Generate random identifiers

Usage: idgen [OPTIONS]

Options:
  -a, --alphabet <ALPHABET>
          Alphabet to use for ID generation

          Possible values:
          - nanoid:                  NanoID / URL-safe Base64: _-0-9a-zA-Z
          - all:                     All alphanumeric: 0-9a-zA-Z
          - uppercase:               Ambiguity-safe uppercase: excludes B, D, I, O, S, Z
          - all-uppercase:           All uppercase: 0-9A-Z
          - lowercase:               Ambiguity-safe lowercase: excludes 1, l
          - all-lowercase:           All lowercase: 0-9a-z
          - uppercase-and-lowercase: Ambiguity-safe mixed case: excludes B, D, I, K, O, S, Z, k, l

          [default: lowercase]

  -b, --block-size <BLOCK_SIZE>
          Number of characters per block

          [default: 4]

  -d, --delimiter <DELIMITER>
          Delimiter between blocks

          [default: -]

  -s, --size <SIZE>
          Total number of characters in the generated ID

          [default: 20]

  -n, --count <COUNT>
          Number of IDs to generate

          [default: 1]

  -q, --quiet
          Suppress config output

      --completions <SHELL>
          Generate shell completion script and exit

          [possible values: bash, elvish, fish, powershell, zsh]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Alphabets

Alphabets marked as ambiguity-safe exclude characters that look alike when read on screen or in print — such as `0`/`O`, `1`/`l`, `8`/`B` — making generated IDs easier to read, type, and communicate verbally.

| Name                      | Characters                           | Description                                               |
|---------------------------|--------------------------------------|-----------------------------------------------------------|
| `nanoid`                  | `_-0-9a-zA-Z`                        | NanoID / URL-safe Base64 (64 chars)                       |
| `all`                     | `0-9a-zA-Z`                          | All alphanumeric (62 chars)                               |
| `uppercase`               | `34679ACEFGHJKLMNPQRTUVWXY`          | Ambiguity-safe uppercase (excludes 0, 1, 2, 5, 8, B, D, I, O, S, Z) |
| `all-uppercase`           | `0-9A-Z`                             | All uppercase (36 chars)                                  |
| `lowercase`               | `023456789abcdefghijkmnopqrstuvwxyz` | Ambiguity-safe lowercase (excludes 1, l)                  |
| `all-lowercase`           | `0-9a-z`                             | All lowercase (36 chars)                                  |
| `uppercase-and-lowercase` | `34679ACEFGHJLMNPQRTUVWXY...`        | Ambiguity-safe mixed (excludes 0, 1, 2, 5, 8, B, D, I, K, O, S, Z, k, l) |

## Shell Completions

Tab completion is supported for `bash`, `zsh`, `fish`, `powershell`, and `elvish`.

### Homebrew

Completions for bash, zsh, and fish are installed automatically.

### Manual

Generate and install the completion script for your shell:

```sh
# bash (Linux)
idgen --completions bash | sudo tee /etc/bash_completion.d/idgen

# bash (macOS with Homebrew-installed bash)
idgen --completions bash > "$(brew --prefix)/etc/bash_completion.d/idgen"

# zsh — place in any directory on your $fpath
idgen --completions zsh > "${fpath[1]}/_idgen"

# fish
idgen --completions fish > ~/.config/fish/completions/idgen.fish

# PowerShell — add to $PROFILE
idgen --completions powershell | Out-String | Invoke-Expression
```

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
