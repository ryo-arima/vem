# VEM (Vim Environment Manager)

> [!WARNING]
> **🚧 UNDER DEVELOPMENT 🚧**
> 
> This project is currently in active development and is not yet ready for production use.
> Features, APIs, and commands may change without notice.

VEM is a command-line tool written in Rust for efficiently managing multiple Vim environments. Switch between different `.vim` configurations easily based on your needs and preferences.

## Special Thanks

VEM aims to integrate with various amazing tools and plugins in the Vim/Neovim ecosystem. We would like to express our gratitude to the following projects:

### Plugin Managers
- [vim-plug](https://github.com/junegunn/vim-plug) - Minimalist Vim Plugin Manager
- [packer.nvim](https://github.com/wbthomason/packer.nvim) - A use-package inspired plugin manager for Neovim
- [lazy.nvim](https://github.com/folke/lazy.nvim) - Modern plugin manager for Neovim
- [dein.vim](https://github.com/Shougo/dein.vim) - Dark powered Vim/Neovim plugin manager
- [paq-nvim](https://github.com/savq/paq-nvim) - Neovim package manager
- [jetpack.vim](https://github.com/tani/vim-jetpack) - Lightning-fast plugin manager for Vim/Neovim

### AI Tools
- [GitHub Copilot](https://github.com/features/copilot) - AI pair programmer
- [ChatGPT](https://chat.openai.com/) - Conversational AI assistant
- [Codeium](https://codeium.com/) - Free AI code completion tool
- [Tabnine](https://www.tabnine.com/) - AI code completion assistant
- [Amazon CodeWhisperer](https://aws.amazon.com/codewhisperer/) - AI-powered code suggestions
- [Claude Code](https://claude.ai/) - Anthropic's AI assistant
- [Gemini Code Assist](https://cloud.google.com/gemini/docs/codeassist/overview) - Google's AI coding assistant
- [Cursor](https://cursor.sh/) - AI-powered code editor
- [Continue](https://continue.dev/) - Open-source AI code assistant
- [Aider](https://aider.chat/) - AI pair programming in terminal
- [Sourcegraph Cody](https://sourcegraph.com/cody) - AI coding assistant

### Color Schemes
- [Gruvbox](https://github.com/morhetz/gruvbox) - Retro groove color scheme
- [Nord](https://www.nordtheme.com/) - Arctic, north-bluish color palette
- [Dracula](https://draculatheme.com/) - Dark theme for many applications
- [TokyoNight](https://github.com/folke/tokyonight.nvim) - Clean, dark Neovim theme
- [Catppuccin](https://github.com/catppuccin/catppuccin) - Soothing pastel theme
- [OneDark](https://github.com/joshdick/onedark.vim) - Atom's iconic One Dark theme
- [Solarized](https://ethanschoonover.com/solarized/) - Precision colors for machines and people
- [Monokai](https://monokai.pro/) - Iconic color scheme for developers

### Development Tools
- [Exuberant Ctags](https://ctags.sourceforge.net/) - Programming language indexing tool
- [Universal Ctags](https://ctags.io/) - Modern maintained version of Ctags
- [LSP (Language Server Protocol)](https://microsoft.github.io/language-server-protocol/) - Language intelligence protocol
- [CoC.nvim](https://github.com/neoclide/coc.nvim) - Intellisense engine for Vim/Neovim

And many more themes and plugins that enhance the Vim/Neovim experience!

## Features

- 🚀 **Fast**: Lightweight and fast environment switching powered by Rust
- 🔧 **Flexible**: Manage multiple Vim configuration profiles
- 📁 **Organized**: Keep each environment isolated to prevent configuration conflicts
- 🎯 **Simple**: Intuitive command-line interface

## Documentation

📖 **[Read the full documentation](https://ryo-arima.github.io/vem/)**

- [Quick Start Guide](https://ryo-arima.github.io/vem/quick-start.html)
- [Installation Instructions](https://ryo-arima.github.io/vem/installation.html)
- [Command Reference](https://ryo-arima.github.io/vem/commands.html)
- [Architecture Overview](https://ryo-arima.github.io/vem/architecture.html)

## Installation

### From Pre-built Packages

Download the latest release from [GitHub Releases](https://github.com/ryo-arima/vem/releases).

Package names follow the format: `vem-<version>-<date>-<arch>.<ext>`
- Example: `vem-0.1.0-20251020-amd64.deb`
- Date is in UTC format (YYYYMMDD)

**Note:** Replace the package names below with the actual latest version from the [Releases page](https://github.com/ryo-arima/vem/releases/latest).

#### Debian/Ubuntu (deb)

```bash
# Check the latest release and download the appropriate package for your architecture
# For amd64:
wget https://github.com/ryo-arima/vem/releases/download/latest/vem_0.1.0-20251020_amd64.deb
sudo dpkg -i vem_0.1.0-20251020_amd64.deb

# For arm64:
wget https://github.com/ryo-arima/vem/releases/download/latest/vem_0.1.0-20251020_arm64.deb
sudo dpkg -i vem_0.1.0-20251020_arm64.deb
```

#### Red Hat/Fedora/CentOS (rpm)

```bash
# Check the latest release and download the appropriate package for your architecture
# For x86_64:
wget https://github.com/ryo-arima/vem/releases/download/latest/vem-0.1.0-20251020.x86_64.rpm
sudo rpm -i vem-0.1.0-20251020.x86_64.rpm

# For aarch64:
wget https://github.com/ryo-arima/vem/releases/download/latest/vem-0.1.0-20251020.aarch64.rpm
sudo rpm -i vem-0.1.0-20251020.aarch64.rpm
```

#### macOS (Homebrew)

```bash
# Download the Homebrew formula from the release
wget https://github.com/ryo-arima/vem/releases/download/latest/vem.rb
brew install ./vem.rb
```

#### Binary Archives (tar.gz/zip)

```bash
# Linux x86_64
wget https://github.com/ryo-arima/vem/releases/download/latest/vem-0.1.0-20251020-x86_64.tar.gz
tar -xzf vem-0.1.0-20251020-x86_64.tar.gz
sudo mv vem/vem /usr/local/bin/

# Linux aarch64
wget https://github.com/ryo-arima/vem/releases/download/latest/vem-0.1.0-20251020-aarch64.tar.gz
tar -xzf vem-0.1.0-20251020-aarch64.tar.gz
sudo mv vem/vem /usr/local/bin/

# macOS x86_64 (Intel)
wget https://github.com/ryo-arima/vem/releases/download/latest/vem-0.1.0-20251020-x86_64.tar.gz
tar -xzf vem-0.1.0-20251020-x86_64.tar.gz
sudo mv vem/vem /usr/local/bin/

# macOS arm64 (Apple Silicon)
wget https://github.com/ryo-arima/vem/releases/download/latest/vem-0.1.0-20251020-arm64.tar.gz
tar -xzf vem-0.1.0-20251020-arm64.tar.gz
sudo mv vem/vem /usr/local/bin/
```

### Using Cargo

```bash
cargo install vem
```

### Build from Source

```bash
git clone https://github.com/ryo-arima/vem.git
cd vem
cargo build --release
sudo cp target/release/vem /usr/local/bin/
```

## Usage

### Basic Commands

```bash
# Create a new environment
vem create <environment-name>

# List all environments
vem list

# Switch to an environment
vem switch <environment-name>

# Show current environment
vem current

# Remove an environment
vem remove <environment-name>
```

### Examples

```bash
# Create a development environment
vem create development

# Create a writing environment
vem create writing

# List all environments
vem list
# development
# writing

# Switch to development environment
vem switch development

# Check current environment
vem current
# development
```

## Environment Structure

Each Vim environment is managed as follows:

```
~/.vem/
├── environments/
│   ├── development/
│   │   ├── .vimrc
│   │   └── .vim/
│   └── writing/
│       ├── .vimrc
│       └── .vim/
└── current -> environments/development
```

## Project Architecture

VEM follows a modular architecture with clear separation of concerns:

### Core Modules

```
src/
├── main.rs          # Application entry point
├── cnf/             # Configuration management
├── ctl/             # Control layer (commands and CLI)
├── ent/             # Entity definitions (data models)
├── rep/             # Repository layer (data persistence)
└── usc/             # Use case layer (business logic)
```

### Architecture Overview

- **Configuration (`cnf`)**: Handles application settings and environment configurations
- **Control (`ctl`)**: Command-line interface and user interaction handling
- **Entity (`ent`)**: Core data structures and domain models
- **Repository (`rep`)**: Data storage and retrieval operations
- **Use Case (`usc`)**: Business logic and application workflows

This layered architecture ensures:
- Clear separation of concerns
- Easy testing and maintenance
- Scalable codebase structure
- Clean dependency management

## Development

### Prerequisites

- Rust 1.70 or higher (nightly toolchain recommended for development)
- Git

### Project Structure

```
vem/
├── docs/                # Documentation (mdBook)
│   ├── src/            # Documentation source
│   ├── book.toml       # mdBook configuration
│   └── book/           # Generated documentation (ignored)
├── scripts/             # Packaging and release scripts
│   ├── main.sh         # Main packaging script entry point
│   └── packages/       # Package format specific scripts
│       ├── apt/        # Debian package scripts
│       ├── rpm/        # RPM package scripts
│       ├── brew/       # Homebrew formula scripts
│       └── common/     # Shared utilities
├── src/                 # Source code
│   ├── cnf/            # Configuration layer
│   ├── ctl/            # Control layer
│   ├── ent/            # Entity layer
│   ├── rep/            # Repository layer
│   ├── usc/            # Use case layer
│   ├── util/           # Utility modules
│   └── main.rs         # Entry point
├── .github/             # GitHub Actions workflows
│   └── workflows/      # CI/CD pipelines
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── rustfmt.toml        # Rust formatter configuration
└── README.md           # Project documentation
```

### Setup

```bash
git clone https://github.com/ryo-arima/vem.git
cd vem
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Formatting

This project uses nightly rustfmt with custom configurations:

```bash
# Install nightly toolchain
rustup toolchain install nightly

# Format code
cargo +nightly fmt

# Check formatting
cargo +nightly fmt --check
```

### Building Packages

To build distribution packages locally:

```bash
# Build all packages (Linux only)
bash scripts/main.sh all

# Build specific package types
bash scripts/main.sh apt    # Debian packages
bash scripts/main.sh rpm    # RPM packages
bash scripts/main.sh dist   # tar.gz and zip archives
bash scripts/main.sh brew   # Homebrew formula
```

## License

MIT License

## Contributing

Pull requests and issue reports are welcome!
