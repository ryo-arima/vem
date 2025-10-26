# Global VEM Configuration Directory

This directory contains global configurations and resources that can be shared across all VEM environments.

## Directory Structure

```
global/
├── vim/          # Global Vim configurations and functions
├── nvim/         # Global Neovim configurations and Lua modules  
├── scripts/      # Shared VimScript functions and utilities
├── themes/       # Theme configurations and color schemes
└── ai-tools/     # AI tool configurations and integrations
```

## Purpose

- **Shared Resources**: Store configurations that are common across multiple environments
- **Reusability**: Avoid duplication of common settings and functions
- **Centralized Management**: Maintain global settings in one location
- **Modular Design**: Each directory serves a specific purpose for better organization

## Usage

These global configurations are intended to be sourced or required by individual environment configurations in the parent directories (basic-vim, developer-vim, modern-nvim, ai-development).

Example:
```vim
" In a specific environment's vimrc
source ~/.vem/global/vim/common-settings.vim
source ~/.vem/global/scripts/utility-functions.vim
```

```lua
-- In a Neovim init.lua
require('global.nvim.common-settings')
require('global.ai-tools.copilot-config')
```