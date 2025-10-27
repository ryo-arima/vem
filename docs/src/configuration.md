# Configuration Guide

VEM uses TOML-based configuration files to manage environments, plugins, ctags, and other settings. This guide explains the configuration system and all available options.

## Configuration Structure

```
etc/.vem/
├── envs/                          # Environment-specific configurations
│   ├── basic-vim/
│   │   ├── vimrc                  # Vim configuration file
│   │   └── vem.toml              # Environment settings
│   ├── developer-vim/
│   │   ├── vimrc
│   │   └── vem.toml
│   ├── modern-nvim/
│   │   ├── init.lua              # Neovim configuration
│   │   ├── lua/plugins/
│   │   └── vem.toml
│   └── ai-development/
│       ├── init.lua
│       └── vem.toml
└── global/                       # Shared configurations
    ├── vim/                      # Global Vim settings
    ├── nvim/                     # Global Neovim settings  
    ├── scripts/                  # Shared VimScript functions
    ├── themes/                   # Color schemes
    └── ai-tools/                 # AI tool configurations
```

## vem.toml Reference

### Environment Section

```toml
[environment]
name = "environment-name"
description = "Environment description"
type = "vim"  # or "neovim"
version = "1.0.0"
author = "Your Name"
```

**Fields:**
- `name`: Unique environment identifier
- `description`: Human-readable description
- `type`: Editor type (`vim` or `neovim`)
- `version`: Environment version
- `author`: Environment creator

### Editor Configuration

```toml
[editor]
type = "vim"  # or "neovim"
config_file = "vimrc"  # or "init.lua" for Neovim
global_configs = [
    "global/vim/common-settings.vim",
    "global/scripts/utility-functions.vim"
]
```

**Fields:**
- `type`: Editor type
- `config_file`: Main configuration file name
- `global_configs`: Array of global configuration files to include

### Features

```toml
[features]
syntax_highlighting = true
line_numbers = true
relative_numbers = false
search_highlighting = true
auto_indent = true
smart_indent = true
mouse_support = true
folding = false
completion = true
treesitter = false  # Neovim only
```

### Plugin Management

```toml
[plugins]
enabled = true
manager = "vim-plug"  # Selected plugin manager
auto_install = true

[plugin_managers]
# Plugin manager selection (only one should be true)
vim-plug = true
pathogen = false
vundle = false
dein = false
lazy = false    # Neovim only
packer = false  # Neovim only
paq = false     # Neovim only

# Plugin manager configurations
[plugin_managers.configs]

[plugin_managers.configs.vim-plug]
url = "https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim"
install_path = "~/.vim/autoload/plug.vim"
config_block_start = "call plug#begin('~/.vim/plugged')"
config_block_end = "call plug#end()"
install_command = ":PlugInstall"
update_command = ":PlugUpdate"
clean_command = ":PlugClean"
packages = [
    { name = "preservim/nerdtree", description = "File explorer" },
    { name = "junegunn/fzf.vim", description = "Fuzzy finder" },
    # ... more packages
]
```

### Ctags Configuration

```toml
[ctags]
enabled = true
executable = "ctags"
global_config_file = "~/.ctags"
auto_generate = true
update_on_save = true

# Global settings
[ctags.global]
languages = ["python", "rust", "javascript", "c", "cpp"]
exclude_patterns = [".git", "node_modules", "target", "__pycache__"]
custom_options = ["--recurse=yes", "--sort=yes"]

# Repository management
[ctags.repositories]

[ctags.repositories.main_project]
name = "main_project"
description = "Main development project"
path = "."
remote_url = ""
branch = "main"
enabled = true
auto_sync = true
priority = 1

# Tag configurations
[ctags.tags]

[ctags.tags.project]
name = "project"
description = "Project-specific tags"
tag_file = "tags"
repositories = ["main_project"]
source_dirs = ["."]
languages = ["python", "rust", "javascript"]
exclude_patterns = [".git", "target", "__pycache__"]
custom_options = ["--recurse=yes", "--sort=yes"]
auto_generate = true

# Command configuration
[ctags.commands]
generate_command = "ctags"
generate_options = ["--recurse=yes", "--sort=yes"]
update_command = "ctags"
update_options = ["--recurse=yes", "--sort=yes", "--append=no"]
list_format = "table"  # table, json, yaml, simple
clean_backup = true
clean_confirm = true
```

### Theme Configuration

```toml
[theme]
name = "gruvbox"
variant = "dark"  # or "light"
background = "dark"
```

**Popular themes:**
- `gruvbox`, `nord`, `dracula`, `tokyonight`
- `catppuccin`, `onedark`, `solarized`, `monokai`

### Keymaps

```toml
[keymaps]
leader = " "
custom_maps = [
    { key = "<C-n>", action = ":NERDTreeToggle<CR>", mode = "n" },
    { key = "<C-p>", action = ":FZF<CR>", mode = "n" },
    { key = "gd", action = "<Plug>(coc-definition)", mode = "n" },
]
```

**Keymap fields:**
- `key`: Key combination
- `action`: Command or function to execute
- `mode`: Vim mode (`n`, `i`, `v`, `c`)
- `type`: Optional, `lua` for Lua functions in Neovim

### LSP Configuration

```toml
[lsp]
enabled = true
provider = "native"  # or "coc"
auto_install = true
languages = [
    "lua_ls",
    "rust_analyzer", 
    "pyright",
    "tsserver"
]
```

### AI Tools (AI Development Environment)

```toml
[ai_tools]
enabled = true

[ai_tools.copilot]
enabled = true
accept_key = "<C-J>"
disable_tab = true

[ai_tools.chatgpt]
enabled = true
api_key_cmd = "echo $OPENAI_API_KEY"
model = "gpt-3.5-turbo"

[ai_tools.codeium]
enabled = true
accept_key = "<C-g>"
```

### System Packages

```toml
[packages]
system = [
    { name = "fzf", package_managers = { brew = "fzf", apt = "fzf", yum = "fzf" } },
    { name = "ripgrep", package_managers = { brew = "ripgrep", apt = "ripgrep", yum = "ripgrep" } },
    { name = "ctags", package_managers = { brew = "universal-ctags", apt = "universal-ctags", yum = "ctags" } }
]
```

### Performance Settings

```toml
[performance]
swap_files = false
backup_files = true
backup_dir = "~/.vim/backup"
undo_levels = 10000
update_time = 300
```

### Compatibility

```toml
[compatibility]
vim_version = "8.0+"
neovim_version = "0.8.0+"  # Neovim environments only
```

## Environment Templates

### Basic Vim Template
```toml
[environment]
name = "my-basic"
type = "vim"

[features]
syntax_highlighting = true
line_numbers = true

[plugins]
enabled = false

[ctags]
enabled = false
```

### Developer Template
```toml
[environment]
name = "my-dev"
type = "vim"

[plugins]
enabled = true
manager = "vim-plug"

[plugin_managers]
vim-plug = true

[ctags]
enabled = true
auto_generate = true

[theme]
name = "gruvbox"
```

### Modern Neovim Template
```toml
[environment]
name = "my-nvim"
type = "neovim"

[plugins]
enabled = true
manager = "lazy"

[plugin_managers]
lazy = true

[lsp]
enabled = true
provider = "native"

[treesitter]
enabled = true

[theme]
name = "tokyonight"
```

## Configuration Validation

VEM validates configurations on load:

```bash
# Validate current environment configuration
vem config validate

# Validate specific configuration file
vem config validate --file path/to/vem.toml

# Show configuration schema
vem config schema
```

## Best Practices

1. **Keep environments focused**: Each environment should serve a specific purpose
2. **Use global configs**: Share common settings via global configuration files
3. **Document changes**: Add comments to explain custom configurations
4. **Test configurations**: Use `vem config validate` before switching environments
5. **Backup configurations**: Keep configuration files in version control

## Troubleshooting

### Common Issues

1. **Invalid TOML syntax**
   ```bash
   vem config validate --file vem.toml
   ```

2. **Plugin conflicts**
   - Check plugin compatibility
   - Verify plugin manager selection

3. **Ctags errors**
   - Ensure ctags executable is available
   - Check repository paths and permissions

### Debug Mode

```bash
# Run VEM with debug logging
RUST_LOG=debug vem switch environment-name

# Show current configuration
vem config show

# List all configuration files
vem config list
```
