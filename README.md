# VEM (Vim Environment Manager)

VEM is a command-line tool written in Rust for efficiently managing multiple Vim environments. Switch between different `.vim` configurations easily based on your needs and preferences.

## Features

- 🚀 **Fast**: Lightweight and fast environment switching powered by Rust
- 🔧 **Flexible**: Manage multiple Vim configuration profiles
- 📁 **Organized**: Keep each environment isolated to prevent configuration conflicts
- 🎯 **Simple**: Intuitive command-line interface

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

## Development

### Prerequisites

- Rust 1.70 or higher
- Git

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
