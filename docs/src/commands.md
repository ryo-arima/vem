# Commands

VEM provides a simple and intuitive command-line interface. Here's a comprehensive overview of all available commands.

## Command Overview

| Command | Description |
|---------|-------------|
| `create` | Create a new Vim environment |
| `list` | List all available environments |
| `switch` | Switch to a specific environment |
| `current` | Show the currently active environment |
| `remove` | Remove an environment |

## Global Options

All commands support these global options:

- `--help`, `-h`: Show help information
- `--version`, `-V`: Show version information
- `--verbose`, `-v`: Enable verbose output
- `--quiet`, `-q`: Suppress non-essential output

## Command Details

### `vem create <name>`

Creates a new Vim environment with the specified name.

**Usage:**
```bash
vem create <environment-name>
```

**Examples:**
```bash
vem create development
vem create writing
vem create experimental
```

**Options:**
- `--from <template>`: Create from existing environment (future feature)
- `--description <desc>`: Add description to environment (future feature)

**Behavior:**
- Creates directory structure under `~/.vem/environments/<name>/`
- Initializes empty `.vimrc` and `.vim/` directory
- Environment becomes available for switching

[Read more →](./commands/create.md)

### `vem list`

Lists all available Vim environments.

**Usage:**
```bash
vem list
```

**Options:**
- `--detailed`, `-d`: Show detailed information (future feature)
- `--current-first`: Show current environment first (future feature)

**Output:**
```
development
writing
experimental
```

[Read more →](./commands/list.md)

### `vem switch <name>`

Switches to the specified Vim environment.

**Usage:**
```bash
vem switch <environment-name>
```

**Examples:**
```bash
vem switch development
vem switch writing
```

**Behavior:**
- Updates symbolic links to point to the specified environment
- Makes the environment active for new Vim sessions
- Preserves existing Vim sessions

[Read more →](./commands/switch.md)

### `vem current`

Shows the currently active Vim environment.

**Usage:**
```bash
vem current
```

**Output:**
```
development
```

**Exit Codes:**
- `0`: Success, environment shown
- `1`: No environment currently active

[Read more →](./commands/current.md)

### `vem remove <name>`

Removes a Vim environment permanently.

**Usage:**
```bash
vem remove <environment-name>
```

**Examples:**
```bash
vem remove experimental
```

**Options:**
- `--force`, `-f`: Skip confirmation prompt (future feature)
- `--backup`: Create backup before removal (future feature)

**Safety:**
- Prompts for confirmation before removal
- Cannot remove currently active environment
- Permanently deletes all environment data

[Read more →](./commands/remove.md)

## Exit Codes

VEM uses standard exit codes:

- `0`: Success
- `1`: General error
- `2`: Invalid command or arguments
- `3`: Environment not found
- `4`: Environment already exists
- `5`: Permission denied

## Shell Completion

VEM supports shell completion for bash, zsh, and fish (future feature):

```bash
# bash
vem completions bash > /etc/bash_completion.d/vem

# zsh  
vem completions zsh > ~/.zsh/completions/_vem

# fish
vem completions fish > ~/.config/fish/completions/vem.fish
```