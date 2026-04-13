# Vision: idgen

## Goal

Provide developers with a fast, flexible command-line tool for generating random, human-friendly identifiers suitable for use in scripts, configuration files, and development workflows.

## Users

- **Developer**: Generates unique IDs on the command line for use in scripts, config values, filenames, or test data
- **DevOps Engineer**: Uses idgen in shell pipelines and automation scripts to produce collision-resistant identifiers

## Core Features

- **Alphabet selection**: Choose from multiple predefined alphabets (NanoID / URL-safe Base64, alphanumeric, ambiguity-safe uppercase/lowercase/mixed)
- **Flexible formatting**: Control ID length, block size, and delimiter to match any naming convention
- **Batch generation**: Generate multiple IDs in a single invocation
- **Quiet mode**: Suppress metadata output for clean pipeline integration

## Key Workflows

1. Run `idgen` to instantly get a single random ID with sensible defaults
2. Combine flags (`-a`, `-s`, `-b`, `-d`, `-n`) to generate IDs matching a specific format
3. Pipe output from quiet mode (`-q`) directly into scripts or other tools

## Success Criteria

- Single-command install via Homebrew or Cargo
- Generates IDs with no ambiguous characters when using safe alphabets
- Zero external runtime dependencies; executes in milliseconds
- Works reliably on macOS, Linux, and Windows
