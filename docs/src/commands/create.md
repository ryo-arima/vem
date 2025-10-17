# create

The `create` command creates a new Vim environment with the specified name.

## Syntax

```bash
vem create <environment-name>
```

## Parameters

- `<environment-name>`: The name of the environment to create
  - Must be a valid directory name
  - Cannot contain spaces or special characters
  - Recommended: use lowercase with hyphens (e.g., `web-development`)

## Examples

### Basic Usage

```bash
# Create a development environment
vem create development

# Create a writing environment
vem create writing

# Create a project-specific environment
vem create my-rust-project
```

### Advanced Examples

```bash
# Create environments for different languages
vem create python-dev
vem create javascript-dev
vem create rust-dev

# Create environments for different purposes
vem create minimal
vem create full-featured
vem create presentation
```

## What Gets Created

When you run `vem create <name>`, VEM creates:

```
~/.vem/environments/<name>/
├── .vimrc          # Empty Vim configuration file
└── .vim/           # Empty Vim directory for plugins
    ├── autoload/   # For plugin managers
    ├── bundle/     # For bundled plugins
    ├── colors/     # For color schemes
    └── plugin/     # For plugins
```

## Error Conditions

The command will fail if:

- Environment name already exists
- Invalid environment name
- Insufficient permissions
- Disk space issues

## Success Output

```bash
$ vem create development
Created environment: development
```

## Error Examples

```bash
$ vem create development
Error: Environment 'development' already exists

$ vem create "invalid name"
Error: Environment name cannot contain spaces

$ vem create /invalid/path
Error: Invalid environment name
```

## Next Steps

After creating an environment:

1. [Switch to it](./switch.md): `vem switch <name>`
2. Customize the `.vimrc` file
3. Install plugins in the `.vim` directory
4. Start using your customized environment

## See Also

- [switch](./switch.md) - Switch to an environment
- [list](./list.md) - List all environments
- [remove](./remove.md) - Remove an environment