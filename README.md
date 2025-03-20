# LSGit

LSGit is a fast, concurrent Git repository finder that helps you locate Git repositories across your filesystem. Using Go's concurrency features, it efficiently scans directories to find all Git repositories, making it easy to manage multiple projects.

## Features

- **Fast Concurrent Scanning**: Utilizes Go's concurrency for efficient directory scanning
- **Cross-Platform**: Supports Windows and Linux (amd64 and arm64)
- **Flexible Search**: Configurable recursive search with adjustable depth
- **Path Options**: Choose between relative or absolute paths
- **Debug Mode**: Performance monitoring and statistics

## Installation

### Windows Installation

1. Download the Windows installer (`lsgit-setup.exe`) from the [latest release](https://github.com/mrmaveric/lsgit/releases/latest)
2. Run the installer with administrator privileges
3. The installer will:
   - Install LSGit to C:\tool by default
   - Add the installation directory to your system PATH
   - Support both x64 and ARM64 architectures automatically

### Linux Installation

1. Download the appropriate binary for your architecture (amd64 or arm64) from the [latest release](https://github.com/mrmaveric/lsgit/releases/latest)
2. Make the binary executable:
   ```bash
   chmod +x lsgit
   ```
3. Move the binary to a directory in your PATH:
   ```bash
   sudo mv lsgit /usr/local/bin/
   ```

## Building from Source

### Prerequisites

- Go 1.x or higher
- Git
- For Windows builds: Inno Setup (only needed for creating the installer)

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/mrmaveric/lsgit.git
   cd lsgit
   ```

2. Build the project:
   ```bash
   go build
   ```

3. (Optional) Create Windows installer:
   - Install Inno Setup
   - Run the build command with GoReleaser:
     ```bash
     goreleaser build
     ```
   - Compile the installer:
     ```bash
     iscc setup.iss
     ```

## Usage Guide

### Basic Commands

```bash
# Basic usage (scan current directory)
lsgit

# Scan specific directory
lsgit /path/to/directory

# Recursive search
lsgit -r

# Custom search depth
lsgit -r -depth 3

# Show absolute paths
lsgit -a

# Show debug information
lsgit -debug
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `-r` | Enable recursive search | false |
| `-depth <n>` | Set maximum recursion depth | 5 |
| `-a` | Print absolute paths | false |
| `-debug` | Show debug information | false |

### Notes

- Default depth for recursive search is 5 levels
- Debug mode shows buffer usage and Git project count
- When using relative paths (default), paths are shown relative to the starting directory
- The tool automatically detects Git repositories by checking for required Git files

## Author

James Bull

## License

[Add your chosen license here]
