# env-init

A cross-platform CLI tool to bootstrap your development environment in seconds.

Supports **Linux** (apt), **Windows** (MSYS2/pacman), **macOS** (Homebrew) and **Android** (Termux).

## Installation

```bash
cargo install --git https://github.com/ahaoboy/env-init

curl -fsSL https://raw.githubusercontent.com/ahaoboy/env-init/main/install.sh | sh
```

Or download a prebuilt binary from [Releases](https://github.com/ahaoboy/env-init/releases).

## Usage

```bash
env-init [OPTIONS] <COMMAND>
```

### Global Options

| Option          | Description                                           |
| --------------- | ----------------------------------------------------- |
| `--dry-run`     | Preview what would be done without making any changes |
| `-v, --verbose` | Enable verbose (debug-level) output                   |
| `--version`     | Print version                                         |
| `-h, --help`    | Print help                                            |

### Commands

| Command      | Description                                                                                                            |
| ------------ | ---------------------------------------------------------------------------------------------------------------------- |
| `install`    | Install platform-specific development tools (gcc, cmake, git, fish, etc.)                                              |
| `install-ei` | Install CLI tools via [easy-install](https://github.com/easy-install/easy-install) (starship, hyperfine, zoxide, etc.) |
| `ssh`        | Configure SSH server with predefined `sshd_config`                                                                     |
| `init`       | Extract embedded configuration files to a local directory                                                              |
| `reset`      | Restore modified system configuration files from backup                                                                |
| `rust`       | Install Rust toolchain via rustup                                                                                      |
| `node`       | Install Node.js and pnpm                                                                                               |
| `shell`      | Configure fish shell and starship prompt                                                                               |
| `git`        | Configure git settings and aliases                                                                                     |
| `root`       | Configure root auto-login (Linux)                                                                                      |
| `windows`    | Configure Windows/MSYS2 path and shell settings (Windows only)                                                         |
| `ei`         | Pass-through to easy-install with custom arguments                                                                     |

### Examples

```bash
# Preview all actions without executing
env-init --dry-run install

# Full environment setup
env-init install
env-init rust
env-init node
env-init shell
env-init git

# Windows-specific setup (run in MSYS2 terminal)
env-init windows
# restart terminal after this

# Install CLI tools
env-init install-ei

# Configure SSH server (Linux, requires root)
env-init ssh
```

### Operation Log

All operations are recorded to `~/.env-init/operations.log` for auditing.
