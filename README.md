# VEM (Vim Environment Manager)

VEM is a command-line tool written in Rust for efficiently managing multiple Vim environments. Switch between different `.vim` configurations easily based on your needs and preferences.

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

### Using Cargo

```bash
cargo install vem
```

### Build from Source

```bash
git clone https://github.com/ryo-arima/vem.git
cd vem
cargo build --release
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

- Rust 1.70 or higher
- Git

### Project Structure

```
vem/
├── docs/                # Documentation (mdBook)
│   ├── src/            # Documentation source
│   ├── book.toml       # mdBook configuration
│   └── book/           # Generated documentation (ignored)
├── src/                 # Source code
│   ├── cnf/            # Configuration layer
│   ├── ctl/            # Control layer
│   ├── ent/            # Entity layer
│   ├── rep/            # Repository layer
│   ├── usc/            # Use case layer
│   └── main.rs         # Entry point
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
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

## License

MIT License

## Contributing

Pull requests and issue reports are welcome!
