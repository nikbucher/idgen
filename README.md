# idgen

A CLI tool for generating random NanoID-style identifiers.

## Install

### Homebrew

```sh
brew install nikbucher/tap/idgen
```

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

| Name                      | Characters                           | Description                                               |
|---------------------------|--------------------------------------|-----------------------------------------------------------|
| `j-nano`                  | `_-0-9a-zA-Z`                        | NanoID default (64 chars)                                 |
| `all`                     | `0-9a-zA-Z`                          | All alphanumeric (62 chars)                               |
| `uppercase`               | `34679ACEFGHJKLMNPQRTUVWXY`          | Ambiguity-safe uppercase (excludes B, D, I, O, S, Z)      |
| `all-uppercase`           | `0-9A-Z`                             | All uppercase (36 chars)                                  |
| `lowercase`               | `023456789abcdefghijkmnopqrstuvwxyz` | Ambiguity-safe lowercase (excludes 1, l)                  |
| `all-lowercase`           | `0-9a-z`                             | All lowercase (36 chars)                                  |
| `uppercase-and-lowercase` | `34679ACEFGHJLMNPQRTUVWXY...`        | Ambiguity-safe mixed (excludes B, D, I, K, O, S, Z, k, l) |

## License

MIT
