# 🦀 crab

A lightweight, universally secure command-line text analyzer and forensic file monitor written in Rust.

## Description

**crab** is a highly efficient utility built for parsing bulk data dumps, log files, and text streams without risking encoding-related panics or character boundary crashes.

### Core Specifications:
* **Universal Character Safety:** Fully immune to string index out-of-bounds errors and byte-length bugs. It safely and accurately processes **absolutely any global character set**, multi-byte scripts, complex symbols, and emojis using exact po-symbol `.chars()` vector allocation.
* **Streamlined Forensic Tracking:** Features multi-threaded non-blocking monitoring for live target logs and structural identity analysis.

## CLI Usage

```text
Usage: crab <COMMAND> [ARGS]

Commands:
  read     <FILE>                Print entire file contents
  line     <FILE>                Print file lines with numeric prefixes
  slice    <FILE> <START-END>    Extract text segment by character range

  find     <FILE> <PATTERN>      Search lines for matching pattern (case-insensitive)
  count    <FILE> <PATTERN>      Count total occurrences of a specific pattern
  reverse  <FILE>                Reverse character order of every line in file

  replace  <FILE> <OLD> <NEW>    Replace matching patterns globally within file
  mask     <FILE> <PATTERN>      Censor target pattern with masks matching length
  rewrite  <FILE> <TEXT>         Overwrite entire file with a single text line

  len      <FILE>                Count total Unicode characters (UTF-8 safe)
  lenline  <FILE>                Count total lines based on trailing line feeds

  watch    <FILE>                Real-time multi-threaded log file monitoring
  diff     <FILE1> <FILE2>       Line-by-line structural divergence verification
  compare  <FILE1> <FILE2>       Isolated unique content matching matrix
```

## Examples

```bash
crab read data.log
crab slice context.db 10-25
crab mask server.conf secret_token
crab watch production.stream
crab compare legacy.csv snapshot.csv
```

## Installation

This utility is distributed exclusively through the **w1n** environment manager. To deploy the binary directly into your local path environment, execute:

```bash
w1n -S crab
```

## License

This project is licensed under the **GNU General Public License v3.0 (GPL-3.0)**. Permanent copyleft reciprocity applies to all derivative works.
