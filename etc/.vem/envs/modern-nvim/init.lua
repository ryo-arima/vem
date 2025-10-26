-- Modern Neovim Configuration with lazy.nvim
-- init.lua for Neovim

-- Basic settings
vim.opt.number = true
vim.opt.relativenumber = true
vim.opt.autoindent = true
vim.opt.smartindent = true
vim.opt.tabstop = 4
vim.opt.shiftwidth = 4
vim.opt.expandtab = true
vim.opt.hlsearch = true
vim.opt.incsearch = true
vim.opt.ignorecase = true
vim.opt.smartcase = true
vim.opt.showmatch = true
vim.opt.mouse = "a"
vim.opt.ruler = true
vim.opt.encoding = "utf-8"
vim.opt.backspace = {"indent", "eol", "start"}

-- Color scheme
vim.opt.background = "dark"
vim.opt.termguicolors = true

-- File backup settings
vim.opt.backup = true
vim.opt.backupdir = vim.fn.expand("~/.local/share/nvim/backup//")
vim.opt.swapfile = true
vim.opt.directory = vim.fn.expand("~/.local/share/nvim/swap//")
vim.opt.undofile = true
vim.opt.undodir = vim.fn.expand("~/.local/share/nvim/undo//")

-- Create backup directories
local backup_dir = vim.fn.expand("~/.local/share/nvim/backup")
local swap_dir = vim.fn.expand("~/.local/share/nvim/swap")
local undo_dir = vim.fn.expand("~/.local/share/nvim/undo")

if vim.fn.isdirectory(backup_dir) == 0 then
    vim.fn.mkdir(backup_dir, "p")
end
if vim.fn.isdirectory(swap_dir) == 0 then
    vim.fn.mkdir(swap_dir, "p")
end
if vim.fn.isdirectory(undo_dir) == 0 then
    vim.fn.mkdir(undo_dir, "p")
end

-- Leader key
vim.g.mapleader = " "

-- Bootstrap lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)

-- Load plugins
require("lazy").setup("plugins")

-- Key mappings
vim.keymap.set("n", "<C-n>", ":Neotree toggle<CR>", { silent = true })
vim.keymap.set("n", "<C-p>", ":Telescope find_files<CR>", { silent = true })
vim.keymap.set("n", "<C-f>", ":Telescope live_grep<CR>", { silent = true })
vim.keymap.set("n", "<leader>ff", ":Telescope find_files<CR>", { silent = true })
vim.keymap.set("n", "<leader>fg", ":Telescope live_grep<CR>", { silent = true })
vim.keymap.set("n", "<leader>fb", ":Telescope buffers<CR>", { silent = true })
vim.keymap.set("n", "<leader>fh", ":Telescope help_tags<CR>", { silent = true })

-- LSP keymaps (will be set up by LSP plugin)
vim.keymap.set("n", "gD", vim.lsp.buf.declaration, { silent = true })
vim.keymap.set("n", "gd", vim.lsp.buf.definition, { silent = true })
vim.keymap.set("n", "K", vim.lsp.buf.hover, { silent = true })
vim.keymap.set("n", "gi", vim.lsp.buf.implementation, { silent = true })
vim.keymap.set("n", "<C-k>", vim.lsp.buf.signature_help, { silent = true })
vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, { silent = true })
vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, { silent = true })
vim.keymap.set("n", "gr", vim.lsp.buf.references, { silent = true })