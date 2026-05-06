# minigrep

A minimal CLI search tool written in Rust, inspired by `grep`. Searches a file for lines matching a given query and prints them to stdout.

---


## Prerequisites

- Rust
- Cargo *(included with Rust)*

---

## Installation

```bash
git clone <your-repo-url>
cd minigrep
```

---

## Usage

```bash
cargo run <query> <filepath>
```

| Argument | Description |
|----------|-------------|
| `<query>` | The string to search for |
| `<filepath>` | Path to the file to search in |

---

## Examples

**Case-sensitive search:**
```bash
cargo run hello poem.txt
```

**Case-insensitive search** — set the `IGNORE_CASE` environment variable:

```bash
# Linux / macOS
IGNORE_CASE=1 cargo run hello poem.txt

# Windows PowerShell
$env:IGNORE_CASE=1; cargo run hello poem.txt
```

---

## How It Works

`minigrep` reads the entire file into memory, then iterates over each line checking for the query string. Matching lines are collected and printed one per line.

When `IGNORE_CASE` is set (to any value), both the query and each line are lowercased before comparison, making the search case-insensitive without modifying the original output.

---


