# Frequently Asked Questions

## General Questions

### Q: What is VEM?
**A:** VEM (Vim Environment Manager) is a command-line tool for managing multiple Vim/Neovim configurations. It allows you to switch between different editor environments tailored for specific tasks (development, writing, AI-assisted coding, etc.).

### Q: Why use VEM instead of manual configuration management?
**A:** VEM provides:
- **Isolation**: Each environment is completely isolated
- **Easy switching**: One command to switch entire configurations
- **Advanced features**: Built-in ctags management, AI integration, multi-repository support
- **Templates**: Pre-configured environments for common use cases
- **Consistency**: TOML-based configuration for all environments

### Q: Is VEM compatible with my existing Vim configuration?
**A:** Yes! VEM can import your existing `.vimrc` and `.vim` directory. You can also create a new environment based on your current setup.

## Installation & Setup

### Q: What are the system requirements?
**A:** 
- Rust 1.70+ for building from source
- Universal Ctags (for tag management)
- Vim 8.0+ or Neovim 0.8.0+
- Git (for multi-repository features)

### Q: How do I install VEM?
**A:** Several options:
```bash
# Pre-built packages (recommended)
wget https://github.com/ryo-arima/vem/releases/latest/vem_amd64.deb
sudo dpkg -i vem_amd64.deb

# Build from source
git clone https://github.com/ryo-arima/vem.git
cd vem && cargo build --release
sudo cp target/release/vem /usr/local/bin/
```

### Q: How do I get started quickly?
**A:** 
```bash
# List available sample environments
vem list

# Try the developer environment
vem switch developer-vim

# Or create your own
vem create my-environment --template=basic-vim
```

## Environment Management

### Q: How many environments can I have?
**A:** There's no hard limit. You can create as many environments as needed. Each environment is stored separately and doesn't affect others.

### Q: Can I share environments between machines?
**A:** Yes! Environment configurations are stored in TOML files that can be version controlled and shared. The `etc/.vem/` directory contains all environment definitions.

### Q: How do I backup my environments?
**A:** 
```bash
# Backup configuration directory
cp -r ~/.vem ~/vem-backup

# Or just the configurations
tar -czf vem-configs.tar.gz ~/.vem/environments/
```

### Q: Can I use different plugin managers in different environments?
**A:** Absolutely! Each environment can use a different plugin manager (vim-plug, lazy.nvim, packer, etc.). This is configured in the environment's `vem.toml` file.

## Ctags Management

### Q: What are ctags and why should I use them?
**A:** Ctags create an index of code symbols (functions, classes, variables) for quick navigation. VEM's ctags management provides:
- Multi-repository support
- Automatic updates
- AI-enhanced relevance scoring
- Cross-project navigation

### Q: How do I generate ctags for my project?
**A:** 
```bash
# Generate tags for current project
vem generate ctags main_project

# Include specific languages
vem generate ctags . --languages=python,rust,javascript

# List all available tags
vem list ctags
```

### Q: Can I use ctags with multiple repositories?
**A:** Yes! VEM supports multi-repository ctags:
```bash
# Configure repositories in vem.toml
[ctags.repositories.shared_libs]
path = "../shared-libraries" 
remote_url = "git@github.com:company/shared-libs.git"

# Generate cross-repository tags
vem generate ctags cross_repo
```

### Q: My ctags aren't updating automatically. What's wrong?
**A:** Check:
1. `auto_generate = true` in your vem.toml
2. Universal Ctags is installed (`ctags --version`)
3. Repository paths are correct
4. File permissions allow writing

## AI Integration

### Q: How do I set up GitHub Copilot?
**A:** 
1. Use the `ai-development` environment:
   ```bash
   vem switch ai-development
   ```
2. Install GitHub Copilot extension in your editor
3. The environment includes pre-configured Copilot settings

### Q: Can I use multiple AI tools simultaneously?
**A:** Yes! The AI development environment supports:
- GitHub Copilot
- ChatGPT integration
- Codeium
- Custom AI tools

Configure them in your `vem.toml`:
```toml
[ai_tools.copilot]
enabled = true

[ai_tools.codeium] 
enabled = true
```

### Q: How does AI context enhancement work?
**A:** VEM provides AI tools with rich context from:
- Ctags for symbol information
- Cross-repository code awareness
- Project structure understanding
- Smart filtering based on relevance

## Plugin Management

### Q: How do I add plugins to an environment?
**A:** Edit your environment's `vem.toml`:
```toml
[plugin_managers.configs.vim-plug]
packages = [
    { name = "tpope/vim-fugitive", description = "Git integration" },
    { name = "preservim/nerdtree", description = "File explorer" }
]
```

### Q: Can I switch plugin managers for an existing environment?
**A:** Yes, but be careful:
1. Update the plugin manager selection in `vem.toml`
2. Migrate plugin configurations
3. Reinstall plugins with the new manager

### Q: Why isn't my plugin working?
**A:** Check:
1. Plugin is listed in the correct package list
2. Plugin manager is enabled (`vim-plug = true`)
3. Plugin manager is installed
4. Run plugin manager's install command (`:PlugInstall`, etc.)

## Troubleshooting

### Q: VEM command not found after installation
**A:** 
```bash
# Check if VEM is in PATH
which vem

# Add to PATH if needed (add to ~/.bashrc or ~/.zshrc)
export PATH="/usr/local/bin:$PATH"

# Verify installation
vem --version
```

### Q: Environment switching isn't working
**A:** 
```bash
# Check current environment
vem current

# List available environments
vem list

# Try switching with debug output
RUST_LOG=debug vem switch environment-name
```

### Q: Vim/Neovim shows errors after switching environments
**A:** 
1. Check for syntax errors in configuration files
2. Verify all plugins are installed
3. Check for conflicting settings
4. Use `:checkhealth` in Neovim for diagnostics

### Q: Ctags generation fails
**A:** 
```bash
# Check ctags installation
ctags --version

# Verify repository paths
vem config show | grep repositories

# Check permissions
ls -la ~/.vem/
```

### Q: How do I get more detailed error information?
**A:** 
```bash
# Enable debug logging
RUST_LOG=debug vem <command>

# Check log files
tail -f ~/.vem/logs/vem.log

# Validate configuration
vem config validate
```

## Performance

### Q: VEM seems slow when switching environments
**A:** 
- Large plugin installations can slow switching
- Use `lazy = false` for plugins you don't need immediately
- Consider using lighter alternatives for development vs. production environments

### Q: Ctags generation is taking too long
**A:** 
- Exclude unnecessary directories (`node_modules`, `target`, `.git`)
- Use language-specific filtering
- Consider incremental updates instead of full regeneration

## Contributing

### Q: How can I contribute to VEM?
**A:** 
1. Check the [contributing guide](contributing.md)
2. Look for "good first issue" labels on GitHub
3. Submit bug reports with detailed information
4. Suggest new features through GitHub Discussions

### Q: Can I create custom environment templates?
**A:** Yes! Create a new environment configuration in `etc/.vem/envs/` and share it:
1. Copy an existing template
2. Modify settings for your use case
3. Document the template's purpose
4. Submit a pull request

### Q: How do I report bugs?
**A:** 
1. Check existing issues on GitHub
2. Include VEM version (`vem --version`)
3. Provide steps to reproduce
4. Include relevant configuration files
5. Add debug output if possible (`RUST_LOG=debug`)

## Advanced Usage

### Q: Can I use VEM in scripts or automation?
**A:** Yes! VEM supports:
```bash
# JSON output for parsing
vem list --format=json

# Non-interactive mode
vem switch environment --no-confirm

# Exit codes for scripting
if vem current | grep -q "ai-development"; then
    echo "AI environment active"
fi
```

### Q: How do I integrate VEM with my existing workflow?
**A:** 
- Add environment switching to your shell prompt
- Create aliases for common environments
- Use Git hooks to automatically switch environments
- Integrate with tmux or terminal multiplexers
