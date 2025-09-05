# rustafari - hardware monitor client built with rust

A comprehensive system information tool that displays hardware, software, and network details about your machine.

## Prerequisites

### Install Rust and Cargo

1. **Install Rust via rustup (recommended)**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Follow the prompts** and restart your terminal or run:
   ```bash
   source $HOME/.cargo/env
   ```

3. **Verify installation**:
   ```bash
   rustc --version
   cargo --version
   ```

### System Requirements

- **Linux**: Requires `GLIBC_2.34` or higher
- **Check your glibc version**:
  ```bash
  ldd --version
  ```

## Build and Run

### Clone and Build
```bash
git clone <repository-url>
cd rustafari
cargo build --release
```

### Run the Application
```bash
# Run directly with cargo
cargo run

# Or run the compiled binary
./target/release/rustafari
```

## What it displays

- **Hardware**: CPU model/cores/cache, RAM, disk space, hostname, battery life
- **Software**: OS type, name, kernel version, architecture
- **Network**: MAC address, local IP, external IP, device serial number
