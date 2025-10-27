# Troubleshooting Guide

This guide helps you diagnose and resolve common issues with VEM.

## Diagnostic Commands

Before troubleshooting, gather system information:

```bash
# Check VEM version and build info
vem --version

# Verify installation
which vem

# Check current environment
vem current

# List all environments
vem list

# Validate configuration
vem config validate

# Enable debug logging
export RUST_LOG=debug
vem <command>

# Check system dependencies
ctags --version
vim --version
nvim --version
```

## Installation Issues

### VEM Command Not Found

**Problem**: `bash: vem: command not found`

**Solutions**:

1. **Verify installation location**:
   ```bash
   # Find VEM binary
   find / -name "vem" -type f 2>/dev/null
   
   # Check if installed via package manager
   dpkg -l | grep vem        # Debian/Ubuntu
   rpm -qa | grep vem        # Red Hat/CentOS
   brew list | grep vem      # macOS Homebrew
   ```

2. **Fix PATH**:
   ```bash
   # Add to PATH (temporary)
   export PATH="/usr/local/bin:$PATH"
   
   # Add to shell profile (permanent)
   echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **Manual installation**:
   ```bash
   # Download and install manually
   wget https://github.com/ryo-arima/vem/releases/latest/download/vem-linux-x64
   chmod +x vem-linux-x64
   sudo mv vem-linux-x64 /usr/local/bin/vem
   ```

### Permission Denied

**Problem**: `Permission denied` when running VEM

**Solutions**:

1. **Fix binary permissions**:
   ```bash
   sudo chmod +x /usr/local/bin/vem
   ```

2. **Fix config directory permissions**:
   ```bash
   sudo chown -R $USER:$USER ~/.vem/
   chmod -R 755 ~/.vem/
   ```

3. **SELinux issues (Red Hat systems)**:
   ```bash
   # Check SELinux status
   sestatus
   
   # Allow VEM execution
   sudo setsebool -P allow_execheap on
   
   # Or disable SELinux temporarily
   sudo setenforce 0
   ```

### Missing Dependencies

**Problem**: VEM runs but features don't work

**Solutions**:

1. **Install Universal Ctags**:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install universal-ctags
   
   # Red Hat/CentOS
   sudo yum install ctags
   
   # macOS
   brew install universal-ctags
   
   # Verify installation
   ctags --version | grep "Universal Ctags"
   ```

2. **Install Vim/Neovim**:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install vim neovim
   
   # macOS
   brew install vim neovim
   
   # Verify versions
   vim --version | head -1
   nvim --version | head -1
   ```

3. **Install Git**:
   ```bash
   # Required for multi-repository features
   sudo apt-get install git
   git --version
   ```

## Environment Management Issues

### Environment Creation Fails

**Problem**: `vem create` fails with errors

**Diagnosis**:
```bash
# Try with debug output
RUST_LOG=debug vem create test-env

# Check available templates
vem list --templates

# Verify config directory exists
ls -la ~/.vem/
```

**Common causes and solutions**:

1. **Config directory doesn't exist**:
   ```bash
   mkdir -p ~/.vem/environments
   ```

2. **Invalid template**:
   ```bash
   # List available templates
   ls etc/.vem/envs/
   
   # Use valid template
   vem create my-env --template=basic-vim
   ```

3. **Name conflicts**:
   ```bash
   # Check existing environments
   vem list
   
   # Use different name
   vem create my-env-2
   ```

4. **Disk space issues**:
   ```bash
   # Check available space
   df -h ~/.vem/
   
   # Clean up if needed
   vem clean --unused
   ```

### Environment Switching Fails

**Problem**: `vem switch` doesn't work properly

**Diagnosis**:
```bash
# Check current environment
vem current

# Try switching with debug output
RUST_LOG=debug vem switch environment-name

# Verify environment exists
vem list | grep environment-name

# Check environment configuration
cat ~/.vem/environments/environment-name/vem.toml
```

**Solutions**:

1. **Invalid environment name**:
   ```bash
   # List exact names
   vem list
   
   # Use correct name (case-sensitive)
   vem switch "exact-name"
   ```

2. **Corrupted environment**:
   ```bash
   # Validate environment
   vem config validate environment-name
   
   # Recreate if needed
   vem remove environment-name
   vem create environment-name --template=basic-vim
   ```

3. **Permission issues**:
   ```bash
   # Fix permissions
   chmod -R 755 ~/.vem/environments/environment-name/
   ```

### Configuration File Errors

**Problem**: TOML parsing errors

**Example error**: `invalid TOML syntax at line 15`

**Diagnosis**:
```bash
# Validate specific environment
vem config validate environment-name

# Check TOML syntax
toml-lint ~/.vem/environments/environment-name/vem.toml
```

**Common TOML syntax issues**:

1. **Missing quotes in strings**:
   ```toml
   # Wrong
   name = My Environment
   
   # Correct
   name = "My Environment"
   ```

2. **Invalid boolean values**:
   ```toml
   # Wrong
   enabled = yes
   
   # Correct
   enabled = true
   ```

3. **Incorrect array syntax**:
   ```toml
   # Wrong
   languages = rust, python
   
   # Correct
   languages = ["rust", "python"]
   ```

4. **Invalid table structure**:
   ```toml
   # Wrong
   [ctags.repositories.main
   path = "."
   
   # Correct
   [ctags.repositories.main]
   path = "."
   ```

## Ctags Issues

### Ctags Generation Fails

**Problem**: `vem generate ctags` produces errors

**Diagnosis**:
```bash
# Test ctags directly
ctags --version
ctags -R . --languages=rust

# Check VEM ctags configuration
vem config show | grep -A 10 ctags

# Try manual generation
RUST_LOG=debug vem generate ctags test-repo
```

**Solutions**:

1. **Universal Ctags not installed**:
   ```bash
   # Install proper version
   sudo apt-get install universal-ctags
   
   # Verify it's Universal Ctags, not Exuberant
   ctags --version | head -1
   ```

2. **Invalid repository path**:
   ```bash
   # Check path in configuration
   grep -A 5 "repositories" ~/.vem/environments/*/vem.toml
   
   # Update with correct paths
   [ctags.repositories.main]
   path = "/absolute/path/to/repo"
   ```

3. **Language not supported**:
   ```bash
   # List supported languages
   ctags --list-languages
   
   # Use supported language names
   ctags --list-languages | grep -i rust
   ```

4. **Permission issues**:
   ```bash
   # Check repository permissions
   ls -la /path/to/repository
   
   # Fix if needed
   chmod -R 755 /path/to/repository
   ```

### Ctags Files Not Found

**Problem**: Generated tags don't appear in editor

**Diagnosis**:
```bash
# Check if tags file exists
ls -la ~/.vem/environments/current/tags

# Verify tags content
head -10 ~/.vem/environments/current/tags

# Check editor configuration
vim -c "set tags?" -c "q"
```

**Solutions**:

1. **Tags file not in expected location**:
   ```bash
   # Find tags files
   find ~/.vem/ -name "tags" -type f
   
   # Configure editor to use correct path
   # Add to .vimrc:
   set tags=~/.vem/environments/current/tags
   ```

2. **Editor not configured for ctags**:
   ```bash
   # For Vim, add to .vimrc:
   set tags+=./tags;
   set tags+=~/.vem/tags
   
   # For Neovim, add to init.lua:
   vim.opt.tags:append("./tags;")
   vim.opt.tags:append(vim.fn.expand("~/.vem/tags"))
   ```

### Cross-Repository Ctags Issues

**Problem**: Multi-repository ctags not working

**Diagnosis**:
```bash
# Check repository configuration
vem config show | grep -A 20 repositories

# Verify all paths exist
for repo in $(vem config show | grep "path =" | awk '{print $3}' | tr -d '"'); do
    echo "Checking: $repo"
    ls -la "$repo" || echo "Path not found: $repo"
done

# Test individual repository tagging
ctags -R /path/to/repo1 -f /tmp/test-tags
```

**Solutions**:

1. **Missing repository paths**:
   ```toml
   # Add all repositories to vem.toml
   [ctags.repositories.main]
   path = "/home/user/project"
   
   [ctags.repositories.shared]
   path = "/home/user/shared-libs"
   remote_url = "git@github.com:company/shared-libs.git"
   ```

2. **Git repository synchronization**:
   ```bash
   # Update repositories before generating tags
   vem ctags update --all-repos
   
   # Or update individual repo
   cd /path/to/shared-repo && git pull
   ```

## Plugin Manager Issues

### Plugin Installation Fails

**Problem**: Plugins not installing in environment

**Diagnosis**:
```bash
# Check plugin manager configuration
grep -A 10 "plugin_managers" ~/.vem/environments/current/vem.toml

# Check if plugin manager is installed
which vim-plug     # or packer, lazy, etc.

# Test plugin manager manually
vim +PlugInstall +qall    # for vim-plug
```

**Solutions**:

1. **Plugin manager not enabled**:
   ```toml
   [plugin_managers]
   vim-plug = true
   packer = false
   lazy = false
   ```

2. **Plugin manager not installed**:
   ```bash
   # Install vim-plug
   curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
       https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
   
   # For Neovim
   sh -c 'curl -fLo "${XDG_DATA_HOME:-$HOME/.local/share}"/nvim/site/autoload/plug.vim --create-dirs \
          https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim'
   ```

3. **Network issues**:
   ```bash
   # Test network connectivity
   curl -I https://github.com
   
   # Configure proxy if needed
   git config --global http.proxy http://proxy:port
   ```

4. **Invalid plugin syntax**:
   ```toml
   # Check plugin configuration
   [plugin_managers.configs.vim-plug]
   packages = [
       { name = "tpope/vim-fugitive", description = "Git integration" },
       # Ensure proper TOML array syntax
   ]
   ```

### Plugin Configuration Conflicts

**Problem**: Plugins interfere with each other

**Solutions**:

1. **Check for conflicting keybindings**:
   ```vim
   " Check current mappings
   :map
   :imap
   :vmap
   ```

2. **Plugin loading order**:
   ```toml
   # Control loading order in vem.toml
   [plugin_managers.configs.vim-plug.options]
   load_order = ["essential", "ui", "language", "completion"]
   ```

3. **Conditional loading**:
   ```toml
   # Load plugins conditionally
   { name = "plugin/name", condition = "has('nvim')" }
   ```

## AI Integration Issues

### GitHub Copilot Not Working

**Problem**: Copilot suggestions not appearing

**Diagnosis**:
```bash
# Check environment configuration
grep -A 10 "ai_tools" ~/.vem/environments/current/vem.toml

# Verify Copilot installation in editor
vim -c "echo exists('g:loaded_copilot')" -c "q"
```

**Solutions**:

1. **Copilot not enabled in configuration**:
   ```toml
   [ai_tools.copilot]
   enabled = true
   auto_suggest = true
   ```

2. **Copilot plugin not installed**:
   ```bash
   # Add to plugin configuration
   { name = "github/copilot.vim", description = "GitHub Copilot" }
   ```

3. **Authentication issues**:
   ```bash
   # In Vim/Neovim
   :Copilot auth
   :Copilot status
   ```

### AI Context Enhancement Not Working

**Problem**: AI tools not getting enhanced context

**Solutions**:

1. **Ensure ctags are generated**:
   ```bash
   vem generate ctags main_project
   vem list ctags
   ```

2. **Check AI configuration**:
   ```toml
   [ai_tools.context]
   use_ctags = true
   include_related_files = true
   max_context_lines = 1000
   ```

## Performance Issues

### Slow Environment Switching

**Problem**: `vem switch` takes too long

**Diagnosis**:
```bash
# Profile switching
time vem switch environment-name

# Check environment size
du -sh ~/.vem/environments/environment-name/

# Monitor system resources
htop    # Check CPU/memory usage during switch
```

**Solutions**:

1. **Large plugin installations**:
   ```bash
   # Use lazy loading
   [plugin_managers.configs.vim-plug.options]
   lazy_loading = true
   ```

2. **Reduce unnecessary plugins**:
   ```toml
   # Disable unused plugins
   { name = "heavy/plugin", enabled = false }
   ```

3. **Optimize ctags generation**:
   ```toml
   [ctags]
   auto_generate = false    # Disable auto-generation
   exclude_patterns = ["node_modules", "target", ".git"]
   ```

### High Memory Usage

**Problem**: VEM uses too much memory

**Diagnosis**:
```bash
# Monitor memory usage
ps aux | grep vem
top -p $(pgrep vem)

# Check for memory leaks
valgrind --tool=memcheck vem switch env-name
```

**Solutions**:

1. **Reduce ctags scope**:
   ```toml
   [ctags]
   max_file_size = "1MB"
   exclude_patterns = ["*.min.js", "vendor/"]
   ```

2. **Limit concurrent operations**:
   ```bash
   # Avoid parallel ctags generation
   vem config set ctags.parallel_generation false
   ```

## Logging and Debugging

### Enable Debug Logging

```bash
# Environment variable
export RUST_LOG=debug

# Or inline
RUST_LOG=debug vem command

# Specific modules
RUST_LOG=vem::ctags=debug vem generate ctags repo

# Save to file
RUST_LOG=debug vem command 2>&1 | tee debug.log
```

### Log File Locations

```bash
# Default log location
~/.vem/logs/vem.log

# View recent logs
tail -f ~/.vem/logs/vem.log

# Search for errors
grep -i error ~/.vem/logs/vem.log

# Rotate large log files
logrotate ~/.vem/logs/
```

### System Information Collection

For bug reports, collect:

```bash
#!/bin/bash
# vem-diagnostics.sh

echo "=== VEM Diagnostics ==="
echo "Date: $(date)"
echo "System: $(uname -a)"
echo "VEM Version: $(vem --version)"
echo "Shell: $SHELL"
echo

echo "=== Dependencies ==="
echo "Ctags: $(ctags --version 2>/dev/null | head -1 || echo 'Not installed')"
echo "Vim: $(vim --version 2>/dev/null | head -1 || echo 'Not installed')"
echo "Neovim: $(nvim --version 2>/dev/null | head -1 || echo 'Not installed')"
echo "Git: $(git --version 2>/dev/null || echo 'Not installed')"
echo

echo "=== VEM Configuration ==="
echo "Config directory: ~/.vem/"
ls -la ~/.vem/ 2>/dev/null || echo "Config directory not found"
echo

echo "=== Current Environment ==="
vem current 2>/dev/null || echo "No current environment"
echo

echo "=== Available Environments ==="
vem list 2>/dev/null || echo "Failed to list environments"
echo

echo "=== Recent Logs ==="
tail -20 ~/.vem/logs/vem.log 2>/dev/null || echo "No log file found"
```

Run with:
```bash
chmod +x vem-diagnostics.sh
./vem-diagnostics.sh > vem-debug-info.txt
```

## Getting Help

### Community Support

1. **GitHub Issues**: Report bugs and request features
   - Include diagnostic information
   - Provide minimal reproduction steps
   - Attach configuration files

2. **Documentation**: Check docs for detailed guides
   - [Configuration Guide](configuration.md)
   - [Commands Reference](commands.md)
   - [FAQ](faq.md)

3. **Debug Mode**: Always use debug output for issue reports
   ```bash
   RUST_LOG=debug vem problematic-command 2>&1 | tee issue-debug.log
   ```

### Creating Bug Reports

Include in bug reports:

1. **System Information**:
   - OS and version
   - VEM version
   - Dependencies versions

2. **Steps to Reproduce**:
   - Exact commands run
   - Expected vs actual behavior
   - Error messages

3. **Configuration**:
   - Relevant vem.toml sections
   - Environment setup
   - Custom modifications

4. **Debug Output**:
   - Full debug logs
   - Stack traces if available
   - System resource usage

This comprehensive troubleshooting guide should help resolve most common VEM issues. When in doubt, enable debug logging and check the specific error messages for more targeted solutions.
