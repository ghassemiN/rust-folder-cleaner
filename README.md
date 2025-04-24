# Cleaner

Cleaner is a small CLI tool written in Rust that deletes files and empty directories older than a specified age inside a given directory.

## Features

- Deletes files and empty directories older than a given duration
- Accepts input in days, hours, and minutes
- Supports a `--dry-run` mode to simulate without deleting anything
- Recursively scans subdirectories

## Usage

### Run with default settings:

```bash
cargo run -- --path /path/to/folder --days 3
```

### Example

```bash
cargo run -- --path /path/to/folder --days 0 --hours 2 --minutes 30
```

### SUGGESTION

To keep temporary folders clean without manual effort, it's better to run this script via a cronjob or a scheduled task.