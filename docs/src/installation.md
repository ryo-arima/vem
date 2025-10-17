# Installation

VEM can be installed in several ways. Choose the method that works best for your setup.

## Prerequisites

- **Operating System**: Linux, macOS, or Windows
- **Vim**: Any recent version of Vim or Neovim

## Method 1: Using Cargo (Recommended)

If you have Rust and Cargo installed:

```bash
cargo install vem
```

This will download, compile, and install the latest version of VEM.

### Installing Rust and Cargo

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Method 2: Download Pre-built Binaries

Download the latest release for your platform from the [GitHub Releases page](https://github.com/ryo-arima/vem/releases).

### Linux/macOS
```bash
# Download and extract (replace with latest version)
wget https://github.com/ryo-arima/vem/releases/latest/download/vem-linux-x86_64
chmod +x vem-linux-x86_64
sudo mv vem-linux-x86_64 /usr/local/bin/vem
```

### Windows
Download the `.exe` file and place it in a directory that's in your `PATH`.

## Method 3: Build from Source

```bash
git clone https://github.com/ryo-arima/vem.git
cd vem
cargo build --release
```

The binary will be available at `target/release/vem`.

## Verification

Verify the installation by checking the version:

```bash
vem --version
```

You should see output similar to:
```
vem 0.1.0
```

## Next Steps

Now that VEM is installed, check out the [Quick Start](./quick-start.md) guide to create your first Vim environment!