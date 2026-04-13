# Use Case: Generate Random Identifier

## Overview

**Use Case ID:** UC-001   
**Use Case Name:** Generate Random Identifier   
**Primary Actor:** Developer   
**Goal:** Obtain one or more random identifiers with a controllable character set and output format, suitable for use in scripts, configuration, filenames, or test data   
**Status:** Done

## Preconditions

- `idgen` is installed and available on the system PATH
- The developer has a terminal or script context in which to run the command

## Main Success Scenario

1. Developer invokes `idgen` without arguments.
2. System selects the default alphabet (ambiguity-safe lowercase), default size (20 characters), default block size (4), and default delimiter (`-`).
3. System generates a random identifier of the configured length using characters drawn uniformly at random from the selected alphabet.
4. System formats the identifier by splitting it into blocks separated by the delimiter.
5. System prints the configuration summary to the error stream.
6. System prints the formatted identifier to the output stream.
7. Developer receives one identifier and uses it as needed.

## Alternative Flows

### A1: Select a specific alphabet

**Trigger:** Developer specifies `-a <alphabet>` (step 1)
**Flow:**

1. System uses the specified alphabet in place of the default.
2. Use case continues at step 3.

### A2: Customize identifier size

**Trigger:** Developer specifies `-s <n>` (step 1)
**Flow:**

1. System uses the specified total character count in place of the default (20).
2. Use case continues at step 3.

### A3: Customize block formatting

**Trigger:** Developer specifies `-b <n>` and/or `-d <delimiter>` (step 1)
**Flow:**

1. System uses the specified block size and/or delimiter in place of the defaults (4 and `-`).
2. If block size is 0, system skips block splitting and outputs the identifier as a continuous string.
3. Use case continues at step 3.

### A4: Generate multiple identifiers

**Trigger:** Developer specifies `-n <count>` (step 1)
**Flow:**

1. System repeats steps 3–4 exactly `count` times.
2. System prints each formatted identifier on a separate line.
3. Use case continues at step 5.

### A5: Suppress configuration summary for pipeline use

**Trigger:** Developer specifies `-q` (step 1)
**Flow:**

1. System skips step 5 (no configuration summary is written to the error stream).
2. Use case continues at step 6.
3. Developer's pipeline or script receives only the identifier lines and can process them without filtering metadata.

### A6: Combined options

**Trigger:** Developer specifies any combination of `-a`, `-s`, `-b`, `-d`, `-n`, `-q` (step 1)
**Flow:**

1. System applies all specified options simultaneously.
2. Use case continues at step 3 with all overrides in effect.

## Postconditions

### Success Postconditions

- One or more identifiers have been written to the output stream, one per line.
- Each identifier contains exactly the specified number of characters (before delimiter insertion).
- Every character in each identifier belongs to the selected alphabet.
- When quiet mode is not active, a configuration summary has been written to the error stream.
- No files are created or modified on the filesystem.

### Failure Postconditions

- No identifier is produced.
- System writes an error message to the error stream describing the invalid argument.
- Exit code is non-zero.

## Business Rules

### BR-001: Ambiguity-safe alphabets exclude visually confusable characters

Alphabets designated as ambiguity-safe must not include characters that are routinely confused by humans when reading printed or on-screen output. When one character of a confusable pair is excluded, both are excluded.

| Pair  | `uppercase` | `lowercase` | `uppercase-and-lowercase` |
|-------|-------------|-------------|---------------------------|
| 0 ↔ O | both out    | both kept   | both out                  |
| 0 ↔ D | both out    | n/a         | both out                  |
| 1 ↔ I | both out    | n/a         | both out                  |
| 1 ↔ l | n/a         | both out    | both out                  |
| 2 ↔ Z | both out    | n/a         | both out                  |
| 5 ↔ S | both out    | n/a         | both out                  |
| 8 ↔ B | both out    | n/a         | both out                  |
| K ↔ k | n/a         | n/a         | both out                  |

Pairs marked n/a do not apply because one or both characters are absent from that alphabet's case range.

### BR-002: Characters are selected with uniform probability

Each character position in the generated identifier must be independently and uniformly distributed across the chosen alphabet. No character may be systematically over- or under-represented.

### BR-003: Block size of 0 disables block formatting

When block size is set to 0, the identifier is output as a single unbroken string regardless of the delimiter setting. This is the mechanism for producing identifiers that must not contain any separator character.

### BR-004: Configuration summary is written to the error stream, identifiers to the output stream

To allow clean pipeline composition, the two output streams are strictly separated. The configuration summary (alphabet, block-size, delimiter, size, count) goes to stderr; the identifier lines go to stdout. Quiet mode suppresses only the configuration summary; it never suppresses the identifiers.

### BR-005: All alphabet character sets are ASCII

Every alphabet must consist exclusively of printable ASCII characters. This ensures compatibility with the broadest range of terminals, shells, file systems, and downstream tools without encoding concerns.
