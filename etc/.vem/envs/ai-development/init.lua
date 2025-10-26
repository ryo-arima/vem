-- AI-Enhanced Development Environment
-- Neovim configuration with AI tools integration

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
vim.opt.background = "dark"
vim.opt.termguicolors = true

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

-- Plugin configuration
require("lazy").setup({
  -- GitHub Copilot
  {
    "github/copilot.vim",
    config = function()
      -- Enable Copilot for all file types
      vim.g.copilot_filetypes = {
        ["*"] = true,
      }
      
      -- Copilot key mappings
      vim.keymap.set('i', '<C-J>', 'copilot#Accept("\\<CR>")', {
        expr = true,
        replace_keycodes = false
      })
      vim.g.copilot_no_tab_map = true
    end,
  },

  -- ChatGPT integration
  {
    "jackMort/ChatGPT.nvim",
    event = "VeryLazy",
    dependencies = {
      "MunifTanjim/nui.nvim",
      "nvim-lua/plenary.nvim",
      "nvim-telescope/telescope.nvim"
    },
    config = function()
      require("chatgpt").setup({
        api_key_cmd = "echo $OPENAI_API_KEY",
        openai_params = {
          model = "gpt-3.5-turbo",
          frequency_penalty = 0,
          presence_penalty = 0,
          max_tokens = 300,
          temperature = 0,
          top_p = 1,
          n = 1,
        },
        openai_edit_params = {
          model = "code-davinci-edit-001",
          temperature = 0,
          top_p = 1,
          n = 1,
        },
        keymaps = {
          close = { "<C-c>" },
          submit = "<C-Enter>",
          regenerate_response = "<C-r>",
          new_session = "<C-n>",
          cycle_windows = "<Tab>",
        },
      })
    end,
  },

  -- Codeium (free AI completion)
  {
    "Exafunction/codeium.vim",
    event = "BufEnter",
    config = function()
      -- Disable default tab mapping
      vim.g.codeium_disable_bindings = 1
      
      -- Custom key mappings
      vim.keymap.set('i', '<C-g>', function () return vim.fn['codeium#Accept']() end, { expr = true })
      vim.keymap.set('i', '<c-;>', function() return vim.fn['codeium#CycleCompletions'](1) end, { expr = true })
      vim.keymap.set('i', '<c-,>', function() return vim.fn['codeium#CycleCompletions'](-1) end, { expr = true })
      vim.keymap.set('i', '<c-x>', function() return vim.fn['codeium#Clear']() end, { expr = true })
    end
  },

  -- Color scheme
  {
    "catppuccin/nvim",
    name = "catppuccin",
    priority = 1000,
    config = function()
      require("catppuccin").setup({
        flavour = "mocha",
        background = {
          light = "latte",
          dark = "mocha",
        },
        integrations = {
          cmp = true,
          gitsigns = true,
          nvimtree = true,
          telescope = true,
          treesitter = true,
        },
      })
      vim.cmd.colorscheme "catppuccin"
    end,
  },

  -- File explorer
  {
    "nvim-tree/nvim-tree.lua",
    version = "*",
    lazy = false,
    dependencies = {
      "nvim-tree/nvim-web-devicons",
    },
    config = function()
      require("nvim-tree").setup({})
    end,
  },

  -- Telescope for fuzzy finding
  {
    "nvim-telescope/telescope.nvim",
    tag = "0.1.4",
    dependencies = { "nvim-lua/plenary.nvim" },
    config = function()
      require("telescope").setup({})
    end,
  },

  -- Treesitter for better syntax highlighting
  {
    "nvim-treesitter/nvim-treesitter",
    build = ":TSUpdate",
    config = function()
      require("nvim-treesitter.configs").setup({
        ensure_installed = { "c", "lua", "vim", "vimdoc", "query", "python", "javascript", "typescript", "rust", "go" },
        sync_install = false,
        auto_install = true,
        highlight = {
          enable = true,
        },
      })
    end,
  },

  -- LSP Configuration
  {
    "neovim/nvim-lspconfig",
    dependencies = {
      "williamboman/mason.nvim",
      "williamboman/mason-lspconfig.nvim",
    },
    config = function()
      require("mason").setup()
      require("mason-lspconfig").setup({
        ensure_installed = { "lua_ls", "rust_analyzer", "pyright", "tsserver" },
      })

      local lspconfig = require("lspconfig")
      
      -- Setup LSP servers
      lspconfig.lua_ls.setup({})
      lspconfig.rust_analyzer.setup({})
      lspconfig.pyright.setup({})
      lspconfig.tsserver.setup({})
    end,
  },

  -- Auto completion
  {
    "hrsh7th/nvim-cmp",
    dependencies = {
      "hrsh7th/cmp-nvim-lsp",
      "hrsh7th/cmp-buffer",
      "hrsh7th/cmp-path",
      "hrsh7th/cmp-cmdline",
      "L3MON4D3/LuaSnip",
      "saadparwaiz1/cmp_luasnip",
    },
    config = function()
      local cmp = require("cmp")
      
      cmp.setup({
        snippet = {
          expand = function(args)
            require("luasnip").lsp_expand(args.body)
          end,
        },
        mapping = cmp.mapping.preset.insert({
          ["<C-b>"] = cmp.mapping.scroll_docs(-4),
          ["<C-f>"] = cmp.mapping.scroll_docs(4),
          ["<C-Space>"] = cmp.mapping.complete(),
          ["<C-e>"] = cmp.mapping.abort(),
          ["<CR>"] = cmp.mapping.confirm({ select = true }),
        }),
        sources = cmp.config.sources({
          { name = "nvim_lsp" },
          { name = "luasnip" },
        }, {
          { name = "buffer" },
        })
      })
    end,
  },

  -- Status line
  {
    "nvim-lualine/lualine.nvim",
    dependencies = { "nvim-tree/nvim-web-devicons" },
    config = function()
      require("lualine").setup({
        options = {
          theme = "catppuccin"
        }
      })
    end,
  },

  -- Git integration
  {
    "lewis6991/gitsigns.nvim",
    config = function()
      require("gitsigns").setup()
    end,
  },

  -- Comment plugin
  {
    "numToStr/Comment.nvim",
    config = function()
      require("Comment").setup()
    end,
  },
})

-- Key mappings for AI tools
vim.keymap.set("n", "<leader>cc", ":ChatGPT<CR>", { silent = true, desc = "Open ChatGPT" })
vim.keymap.set("n", "<leader>ce", ":ChatGPTEditWithInstructions<CR>", { silent = true, desc = "Edit with ChatGPT" })
vim.keymap.set("n", "<leader>cg", ":ChatGPTRun grammar_correction<CR>", { silent = true, desc = "Grammar correction" })
vim.keymap.set("n", "<leader>ct", ":ChatGPTRun translate<CR>", { silent = true, desc = "Translate" })
vim.keymap.set("n", "<leader>ck", ":ChatGPTRun keywords<CR>", { silent = true, desc = "Extract keywords" })
vim.keymap.set("n", "<leader>cd", ":ChatGPTRun docstring<CR>", { silent = true, desc = "Generate docstring" })
vim.keymap.set("n", "<leader>ca", ":ChatGPTRun add_tests<CR>", { silent = true, desc = "Add tests" })
vim.keymap.set("n", "<leader>co", ":ChatGPTRun optimize_code<CR>", { silent = true, desc = "Optimize code" })
vim.keymap.set("n", "<leader>cs", ":ChatGPTRun summarize<CR>", { silent = true, desc = "Summarize" })
vim.keymap.set("n", "<leader>cf", ":ChatGPTRun fix_bugs<CR>", { silent = true, desc = "Fix bugs" })
vim.keymap.set("n", "<leader>cx", ":ChatGPTRun explain_code<CR>", { silent = true, desc = "Explain code" })

-- File management key mappings
vim.keymap.set("n", "<C-n>", ":NvimTreeToggle<CR>", { silent = true })
vim.keymap.set("n", "<C-p>", ":Telescope find_files<CR>", { silent = true })
vim.keymap.set("n", "<C-f>", ":Telescope live_grep<CR>", { silent = true })

-- LSP key mappings
vim.keymap.set("n", "gD", vim.lsp.buf.declaration, { silent = true })
vim.keymap.set("n", "gd", vim.lsp.buf.definition, { silent = true })
vim.keymap.set("n", "K", vim.lsp.buf.hover, { silent = true })
vim.keymap.set("n", "gi", vim.lsp.buf.implementation, { silent = true })
vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, { silent = true })
vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, { silent = true })
vim.keymap.set("n", "gr", vim.lsp.buf.references, { silent = true })