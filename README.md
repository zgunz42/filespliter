# File Splitter

[![CI](https://github.com/zgunz42/filesplitter/actions/workflows/ci.yml/badge.svg)](https://github.com/zgunz42/filesplitter/actions/workflows/ci.yml)
[![Release](https://github.com/zgunz42/filesplitter/actions/workflows/release.yml/badge.svg)](https://github.com/zgunz42/filesplitter/actions/workflows/release.yml)
[![Security Audit](https://github.com/zgunz42/filesplitter/actions/workflows/security.yml/badge.svg)](https://github.com/zgunz42/filesplitter/actions/workflows/security.yml)

A fast, efficient file splitter and joiner written in Rust. Designed for splitting large files (like RAR archives) into smaller parts and reassembling them.

**Available as both CLI and GUI applications!**

## Features

### Core Features
- **Fast I/O**: Uses 8MB buffered reading/writing for optimal performance
- **Large File Support**: Handles files up to 18 exabytes using u64 data types (tested with multi-GB files)
- **Real-time Progress Bars**: Visual progress bars with ETA for both split and join operations
- **Smart Progress Tracking**: Shows current part, percentage, bytes processed, and estimated time remaining
- **Automatic Part Detection**: Automatically finds and joins all sequential parts
- **Memory Efficient**: Streams data in chunks without loading entire file into memory
- **Human-Readable Sizes**: Displays file sizes in MB/GB format automatically

### GUI Application (`filesplitter-gui`)
- **Cross-platform GUI**: Works on Windows, macOS, and Linux
- **Intuitive Interface**: Simple tab-based design for Split and Join operations
- **File Pickers**: Native file dialogs for easy file selection
- **Live Progress**: Real-time progress bars with ETA calculation
- **Quick Size Selection**: One-click buttons for common part sizes (10MB, 100MB, 1GB, 4GB)
- **Non-blocking Operations**: File operations run in background threads
- **Status Display**: Shows current operation, bytes processed, and estimated completion time

### CLI Application (`filesplitter`)
- **Colored Output**: Beautiful colored terminal output for better readability
- **CLI Interface**: User-friendly command-line interface using clap
- **Error Handling**: Comprehensive error handling with colored error messages
- **Scriptable**: Perfect for automation and batch processing

## Installation

### Pre-built Binaries (Recommended)

Download pre-built binaries from the [Releases](../../releases) page:

#### CLI Application

**Linux (x86_64):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-linux-amd64.tar.gz
tar xzf filesplitter-linux-amd64.tar.gz
chmod +x filesplitter
sudo mv filesplitter /usr/local/bin/
```

**Linux (ARM64):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-linux-arm64.tar.gz
tar xzf filesplitter-linux-arm64.tar.gz
chmod +x filesplitter
sudo mv filesplitter /usr/local/bin/
```

**macOS (Intel):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-macos-amd64.tar.gz
tar xzf filesplitter-macos-amd64.tar.gz
chmod +x filesplitter
sudo mv filesplitter /usr/local/bin/
```

**macOS (Apple Silicon):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-macos-arm64.tar.gz
tar xzf filesplitter-macos-arm64.tar.gz
chmod +x filesplitter
sudo mv filesplitter /usr/local/bin/
```

**Windows:**
1. Download `filesplitter-windows-amd64.exe.zip` from [Releases](../../releases)
2. Extract the zip file
3. Add the directory to your PATH or run directly

#### GUI Application

**Linux (x86_64):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-gui-linux-amd64.tar.gz
tar xzf filesplitter-gui-linux-amd64.tar.gz
chmod +x filesplitter-gui
sudo mv filesplitter-gui /usr/local/bin/
```

**Linux (ARM64):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-gui-linux-arm64.tar.gz
tar xzf filesplitter-gui-linux-arm64.tar.gz
chmod +x filesplitter-gui
sudo mv filesplitter-gui /usr/local/bin/
```

**macOS (Intel):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-gui-macos-amd64.tar.gz
tar xzf filesplitter-gui-macos-amd64.tar.gz
chmod +x filesplitter-gui
sudo mv filesplitter-gui /usr/local/bin/
```

**macOS (Apple Silicon):**
```bash
wget https://github.com/zgunz42/filesplitter/releases/latest/download/filesplitter-gui-macos-arm64.tar.gz
tar xzf filesplitter-gui-macos-arm64.tar.gz
chmod +x filesplitter-gui
sudo mv filesplitter-gui /usr/local/bin/
```

**Windows:**
1. Download `filesplitter-gui-windows-amd64.exe.zip` from [Releases](../../releases)
2. Extract the zip file
3. Run `filesplitter-gui.exe` directly or add to your PATH

### Building from Source

**Requirements:**
- Rust 1.70 or later
- Cargo (comes with Rust)

**Build steps:**
```bash
# Clone the repository
git clone https://github.com/zgunz42/filesplitter.git
cd filesplitter

# Build both CLI and GUI in release mode
cargo build --release --all

# The binaries will be at:
# - target/release/filesplitter (CLI)
# - target/release/filesplitter-gui (GUI)
# On Windows: .exe extension

# Or build individually:
cargo build --release --bin filesplitter        # CLI only
cargo build --release --bin filesplitter-gui    # GUI only
```

**Run tests:**
```bash
cargo test --all-features
```

**Install locally:**
```bash
# Install CLI
cargo install --path . --bin filesplitter

# Install GUI
cargo install --path . --bin filesplitter-gui

# Install both
cargo install --path . --bins
```

## Usage

### GUI Application

The GUI application provides an easy-to-use interface for both split and join operations.

**Running the GUI:**
```bash
# From binary
./target/release/filesplitter-gui

# Or if installed
filesplitter-gui
```

**Split Files:**
1. Click the "Split" tab
2. Click "Choose File..." to select your file
3. Either enter a part size in bytes or use the quick selection buttons:
   - **10 MB** - Small parts for email attachments
   - **100 MB** - Medium parts for cloud storage
   - **1 GB** - Large parts for USB transfers
   - **4 GB** - Maximum for FAT32 filesystems
4. Click "Split File" to start
5. Watch the real-time progress bar with ETA
6. Parts will be created in the same directory as the input file

**Join Files:**
1. Click the "Join" tab
2. Click "Choose First Part..." and select the `.part001` file
3. The application will automatically detect all sequential parts
4. Click "Choose Output Location..." to select where to save the joined file
5. Click "Join Files" to start
6. Monitor progress with the live progress bar and ETA

### CLI Application (Command Line)

The command-line version is perfect for scripts and automation.

#### Split a File

Split a file into parts of specified size:

```bash
./target/release/filesplitter split --input file.rar --size 10485760
```

This splits `file.rar` into parts of 10MB (10485760 bytes) each. Parts will be named:
- `file.rar.part001`
- `file.rar.part002`
- `file.rar.part003`
- etc.

### Join Parts

Reassemble parts back into the original file:

```bash
./target/release/filesplitter join --input file.rar.part001 --output file.rar
```

The program automatically detects and joins all sequential parts (part001, part002, etc.)

## Examples

### Split a 100MB file into 25MB parts

```bash
./target/release/filesplitter split --input archive.rar --size 26214400
```

### Split a 38GB file into 4GB parts

```bash
./target/release/filesplitter split --input large_archive.rar --size 4294967296
```

This will create 10 parts (9 full 4GB parts + 1 smaller final part).

### Join the parts back

```bash
./target/release/filesplitter join --input archive.rar.part001 --output archive_restored.rar
```

## Output Features

The program provides a rich visual experience:

**Colored Output:**
- Cyan headers and borders
- Green labels for file information
- Yellow for file sizes
- Magenta for join operations
- Red for errors
- Progress bars with gradient colors

**Progress Bar Display:**
- Spinner animation during processing
- Real-time progress bar (50 characters wide)
- Bytes processed / Total bytes
- Percentage complete
- Current operation message (e.g., "Splitting part 5/10")
- **ETA (Estimated Time Remaining)** - dynamically calculated
- Elapsed time

**Example Progress Bar:**
```
⠋ [00:00:12] [████████████████░░░░] 42.5 MB/100 MB (42%) Splitting part 3/10 ETA: 00:00:16
```

## Common Part Sizes

- 10 MB: 10485760 bytes
- 25 MB: 26214400 bytes
- 50 MB: 52428800 bytes
- 100 MB: 104857600 bytes
- 500 MB: 524288000 bytes
- 1 GB: 1073741824 bytes
- 4 GB: 4294967296 bytes

## Performance

The program uses:
- 8MB buffer size for optimal I/O performance
- Buffered readers and writers to minimize system calls
- Efficient byte-by-byte streaming without loading entire file into memory

Benchmark results (100MB file, 10MB parts):
- Split time: ~0.15 seconds
- Join time: ~0.11 seconds
- Memory usage: ~16MB (constant, regardless of file size)

## Large File Support

The program fully supports very large files:

**Technical Capabilities:**
- Uses `u64` for all size calculations (max: 18 exabytes)
- Tested with 100MB+ files
- Designed to handle files of any practical size (including 38GB+ files)

**Example: 38GB file split into 4GB parts**
- File size: 40,802,189,312 bytes
- Part size: 4,294,967,296 bytes
- Number of parts: 10
- Command: `./target/release/filesplitter split --input file.rar --size 4294967296`

**Progress Tracking:**
Both split and join operations show real-time progress percentages, essential for monitoring long-running operations on large files.

## Architecture

```
src/
├── main.rs       # CLI interface and command handling
├── splitter.rs   # File splitting logic
└── joiner.rs     # File joining logic
```

### Code Patterns

- **Separation of Concerns**: Each module has a single responsibility
- **Error Propagation**: Uses `anyhow::Result` for clean error handling
- **Builder Pattern**: Constructors validate inputs before creating instances
- **Buffered I/O**: All file operations use buffered readers/writers
- **Resource Management**: Automatic cleanup via Rust's RAII

## Error Handling

The program validates:
- Input files exist before processing
- Part size is greater than 0
- All part files exist before joining
- Sequential part files are present (part001, part002, etc.)

## CI/CD and Releases

This project uses GitHub Actions for continuous integration and automated releases.

### Continuous Integration

The CI pipeline runs on every push and pull request:

**Test Suite** (`ci.yml`):
- Tests on Linux, macOS, and Windows
- Runs tests in both debug and release mode
- Code formatting check with `rustfmt`
- Linting with `clippy`
- Builds artifacts for all platforms

**Security Audit** (`security.yml`):
- Daily security audits using `cargo-audit`
- Dependency checking with `cargo-deny`
- Runs on every push to main branch

### Automated Releases

The release pipeline is triggered when a version tag is pushed:

```bash
git tag v1.0.0
git push origin v1.0.0
```

This will automatically:
1. Create a GitHub release
2. Build binaries for all supported platforms:
   - Linux (x86_64, ARM64)
   - macOS (Intel, Apple Silicon)
   - Windows (x86_64)
3. Upload compressed and raw binaries to the release
4. Generate release notes

**Supported Targets:**
- `x86_64-unknown-linux-gnu` - Linux AMD64
- `aarch64-unknown-linux-gnu` - Linux ARM64
- `x86_64-apple-darwin` - macOS Intel
- `aarch64-apple-darwin` - macOS Apple Silicon
- `x86_64-pc-windows-msvc` - Windows AMD64

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Run formatting (`cargo fmt`)
6. Run clippy (`cargo clippy`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

The CI pipeline will automatically run tests on your PR.

## License

MIT
