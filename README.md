# ArchiveUndo

A simple, fast CLI tool written in Rust to snapshot and restore your codebase.

## Features

- **`archive [name]`**: Creates a snapshot of your current directory (excluding `.git`) in `~/.archiveundo/snapshots/[name]`.
- **`undo [name]`**: Restores your current directory to the state of the specified snapshot. **Warning: This overwrites current files.**

## Installation

### Prerequisites
- Rust and Cargo installed.
- A C compiler (like `gcc` or `build-essential` on Linux).

### Manual Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/the32bitpringle/ArchiveUndo.git
   cd ArchiveUndo
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Move the binaries to your path (e.g., `~/.local/bin`):
   ```bash
   mkdir -p ~/.local/bin
   cp target/release/archive ~/.local/bin/
   cp target/release/undo ~/.local/bin/
   ```

## Usage

### Create a Snapshot
To save the current state of your project:
```bash
archive milestone-1
```

### Restore a Snapshot
To roll back your project to a previous state:
```bash
undo milestone-1
```
*Note: The tool will prompt for confirmation before overwriting your files.*

## Storage
Snapshots are stored in:
`~/.archiveundo/snapshots/`

## License
MIT
