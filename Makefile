# VEM Debug Makefile
# This Makefile helps debug VEM configurations with different Vim setups

# Default variables
VIM_BIN ?= /usr/bin/vim
NVIM_BIN ?= /usr/local/bin/nvim
VIM_RUNTIME ?= /usr/share/vim/vim90
NVIM_RUNTIME ?= /usr/local/share/nvim/runtime

# VEM configuration paths
VIMRC_PATH ?= $(PWD)/etc/.vem/envs/basic-vim/vimrc
NVIM_CONFIG_PATH ?= $(PWD)/etc/.vem/envs/modern-nvim
DEVELOPER_VIMRC_PATH ?= $(PWD)/etc/.vem/envs/developer-vim/vimrc
AI_NVIM_CONFIG_PATH ?= $(PWD)/etc/.vem/envs/ai-development

# Temporary directories for testing
TEMP_VIM_DIR = /tmp/vem-test-vim
TEMP_NVIM_DIR = /tmp/vem-test-nvim

.PHONY: help clean debug-basic debug-developer debug-modern debug-ai test-all setup-temp

help:
	@echo "VEM Debug Makefile"
	@echo "=================="
	@echo "Available targets:"
	@echo "  debug-basic     - Debug basic Vim configuration"
	@echo "  debug-developer - Debug developer Vim configuration" 
	@echo "  debug-modern    - Debug modern Neovim configuration"
	@echo "  debug-ai        - Debug AI-enhanced Neovim configuration"
	@echo "  test-all        - Test all configurations"
	@echo "  setup-temp      - Setup temporary directories"
	@echo "  clean          - Clean temporary files"
	@echo ""
	@echo "Environment variables:"
	@echo "  VIM_BIN        - Path to vim binary (default: $(VIM_BIN))"
	@echo "  NVIM_BIN       - Path to nvim binary (default: $(NVIM_BIN))"
	@echo "  VIM_RUNTIME    - Vim runtime directory (default: $(VIM_RUNTIME))"
	@echo "  NVIM_RUNTIME   - Neovim runtime directory (default: $(NVIM_RUNTIME))"

setup-temp:
	@echo "Setting up temporary directories..."
	@mkdir -p $(TEMP_VIM_DIR)
	@mkdir -p $(TEMP_NVIM_DIR)
	@echo "Temporary directories created:"
	@echo "  Vim:   $(TEMP_VIM_DIR)"
	@echo "  Neovim: $(TEMP_NVIM_DIR)"

debug-basic: setup-temp
	@echo "Debugging Basic Vim Configuration"
	@echo "================================="
	@echo "VIM_BIN: $(VIM_BIN)"
	@echo "VIM_RUNTIME: $(VIM_RUNTIME)"
	@echo "VIMRC: $(VIMRC_PATH)"
	@echo ""
	@if [ -f $(VIMRC_PATH) ]; then \
		echo "Starting Vim with basic configuration..."; \
		HOME=$(TEMP_VIM_DIR) \
		VIM=$(TEMP_VIM_DIR) \
		VIMRUNTIME=$(VIM_RUNTIME) \
		$(VIM_BIN) -u $(VIMRC_PATH) \
		-c "echo 'VEM Basic Configuration Loaded'" \
		-c "echo 'Press :q to quit'"; \
	else \
		echo "Error: Basic vimrc not found at $(VIMRC_PATH)"; \
		exit 1; \
	fi

debug-developer: setup-temp
	@echo "Debugging Developer Vim Configuration"
	@echo "===================================="
	@echo "VIM_BIN: $(VIM_BIN)"
	@echo "VIM_RUNTIME: $(VIM_RUNTIME)"
	@echo "VIMRC: $(DEVELOPER_VIMRC_PATH)"
	@echo ""
	@if [ -f $(DEVELOPER_VIMRC_PATH) ]; then \
		echo "Starting Vim with developer configuration..."; \
		HOME=$(TEMP_VIM_DIR) \
		VIM=$(TEMP_VIM_DIR) \
		VIMRUNTIME=$(VIM_RUNTIME) \
		$(VIM_BIN) -u $(DEVELOPER_VIMRC_PATH) \
		-c "echo 'VEM Developer Configuration Loaded'" \
		-c "echo 'Press :q to quit'"; \
	else \
		echo "Error: Developer vimrc not found at $(DEVELOPER_VIMRC_PATH)"; \
		exit 1; \
	fi

debug-modern: setup-temp
	@echo "Debugging Modern Neovim Configuration"
	@echo "===================================="
	@echo "NVIM_BIN: $(NVIM_BIN)"
	@echo "NVIM_RUNTIME: $(NVIM_RUNTIME)"
	@echo "NVIM_CONFIG: $(NVIM_CONFIG_PATH)"
	@echo ""
	@if [ -f $(NVIM_CONFIG_PATH)/init.lua ]; then \
		echo "Starting Neovim with modern configuration..."; \
		XDG_CONFIG_HOME=$(TEMP_NVIM_DIR) \
		XDG_DATA_HOME=$(TEMP_NVIM_DIR)/.local/share \
		XDG_STATE_HOME=$(TEMP_NVIM_DIR)/.local/state \
		$(NVIM_BIN) -u $(NVIM_CONFIG_PATH)/init.lua \
		-c "lua print('VEM Modern Neovim Configuration Loaded')" \
		-c "lua print('Press :q to quit')"; \
	else \
		echo "Error: Modern init.lua not found at $(NVIM_CONFIG_PATH)/init.lua"; \
		exit 1; \
	fi

debug-ai: setup-temp
	@echo "Debugging AI-Enhanced Neovim Configuration"
	@echo "=========================================="
	@echo "NVIM_BIN: $(NVIM_BIN)"
	@echo "NVIM_RUNTIME: $(NVIM_RUNTIME)"
	@echo "AI_CONFIG: $(AI_NVIM_CONFIG_PATH)"
	@echo ""
	@if [ -f $(AI_NVIM_CONFIG_PATH)/init.lua ]; then \
		echo "Starting Neovim with AI-enhanced configuration..."; \
		XDG_CONFIG_HOME=$(TEMP_NVIM_DIR) \
		XDG_DATA_HOME=$(TEMP_NVIM_DIR)/.local/share \
		XDG_STATE_HOME=$(TEMP_NVIM_DIR)/.local/state \
		$(NVIM_BIN) -u $(AI_NVIM_CONFIG_PATH)/init.lua \
		-c "lua print('VEM AI-Enhanced Configuration Loaded')" \
		-c "lua print('Press :q to quit')"; \
	else \
		echo "Error: AI init.lua not found at $(AI_NVIM_CONFIG_PATH)/init.lua"; \
		exit 1; \
	fi

# Syntax check for Vim configurations
check-vim-syntax:
	@echo "Checking Vim configuration syntax..."
	@for config in $(VIMRC_PATH) $(DEVELOPER_VIMRC_PATH); do \
		if [ -f $$config ]; then \
			echo "Checking: $$config"; \
			$(VIM_BIN) -u NONE -c "source $$config" -c "echo 'Syntax OK'" -c "q" 2>/dev/null || echo "Syntax error in $$config"; \
		fi \
	done

# Syntax check for Neovim Lua configurations
check-nvim-syntax:
	@echo "Checking Neovim Lua configuration syntax..."
	@for config in $(NVIM_CONFIG_PATH)/init.lua $(AI_NVIM_CONFIG_PATH)/init.lua; do \
		if [ -f $$config ]; then \
			echo "Checking: $$config"; \
			$(NVIM_BIN) -u NONE -c "luafile $$config" -c "lua print('Syntax OK')" -c "q" 2>/dev/null || echo "Syntax error in $$config"; \
		fi \
	done

test-all: setup-temp check-vim-syntax check-nvim-syntax
	@echo "Testing all VEM configurations..."
	@echo "================================="
	@make debug-basic && echo "✓ Basic Vim configuration OK" || echo "✗ Basic Vim configuration failed"
	@make debug-developer && echo "✓ Developer Vim configuration OK" || echo "✗ Developer Vim configuration failed" 
	@make debug-modern && echo "✓ Modern Neovim configuration OK" || echo "✗ Modern Neovim configuration failed"
	@make debug-ai && echo "✓ AI-Enhanced Neovim configuration OK" || echo "✗ AI-Enhanced Neovim configuration failed"

# Debug specific VEM features
debug-plugins:
	@echo "Debugging plugin installations..."
	@echo "Vim-plug status:"
	@curl -fLo $(TEMP_VIM_DIR)/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
	@echo "Lazy.nvim status:"
	@git clone --filter=blob:none https://github.com/folke/lazy.nvim.git --branch=stable $(TEMP_NVIM_DIR)/lazy/lazy.nvim 2>/dev/null || echo "Lazy.nvim already exists"

# Environment information
info:
	@echo "VEM Debug Environment Information"
	@echo "================================"
	@echo "Operating System: $$(uname -s)"
	@echo "Architecture: $$(uname -m)"
	@echo "Vim Binary: $(VIM_BIN)"
	@echo "Vim Version: $$($(VIM_BIN) --version 2>/dev/null | head -1 || echo 'Not found')"
	@echo "Neovim Binary: $(NVIM_BIN)"
	@echo "Neovim Version: $$($(NVIM_BIN) --version 2>/dev/null | head -1 || echo 'Not found')"
	@echo "Vim Runtime: $(VIM_RUNTIME)"
	@echo "Neovim Runtime: $(NVIM_RUNTIME)"
	@echo "Sample Environments:"
	@ls -la ./etc/.vem/envs/ 2>/dev/null || echo "Sample environments not found"

clean:
	@echo "Cleaning temporary files..."
	@rm -rf $(TEMP_VIM_DIR)
	@rm -rf $(TEMP_NVIM_DIR)
	@echo "Cleanup completed."

# Install dependencies (macOS specific)
install-deps-macos:
	@echo "Installing VEM dependencies on macOS..."
	@command -v brew >/dev/null 2>&1 || (echo "Error: Homebrew not installed" && exit 1)
	@brew install vim neovim git curl
	@echo "Dependencies installed."

# Install dependencies (Ubuntu/Debian)
install-deps-ubuntu:
	@echo "Installing VEM dependencies on Ubuntu/Debian..."
	@sudo apt update
	@sudo apt install -y vim neovim git curl build-essential
	@echo "Dependencies installed."