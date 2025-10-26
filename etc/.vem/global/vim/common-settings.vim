" Global Vim Settings
" Common configurations shared across all Vim environments

" Basic editor settings
set nocompatible
set encoding=utf-8
set fileencoding=utf-8
set fileencodings=utf-8,cp932,euc-jp,sjis
set backspace=indent,eol,start
set autoindent
set smartindent
set tabstop=4
set shiftwidth=4
set expandtab
set smarttab

" Search settings
set hlsearch
set incsearch
set ignorecase
set smartcase
set wrapscan

" Display settings
set number
set ruler
set showmatch
set matchtime=1
set laststatus=2
set cmdheight=2
set showmode
set showcmd

" Performance settings
set hidden
set history=1000
set updatetime=300
set timeoutlen=500

" File handling
set autoread
set noswapfile
set nobackup
set writebackup

" Syntax and colors
syntax enable
if &t_Co > 2 || has("gui_running")
    syntax on
endif

" Common key mappings
let mapleader = " "

" Quick save and quit
nnoremap <Leader>w :write<CR>
nnoremap <Leader>q :quit<CR>
nnoremap <Leader>x :wq<CR>

" Buffer navigation
nnoremap <Leader>bn :bnext<CR>
nnoremap <Leader>bp :bprevious<CR>
nnoremap <Leader>bd :bdelete<CR>

" Window navigation
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l

" Search improvements
nnoremap <Leader>/ :nohlsearch<CR>

" Toggle settings
nnoremap <Leader>tn :set number!<CR>
nnoremap <Leader>tr :set relativenumber!<CR>
nnoremap <Leader>tw :set wrap!<CR>

" Quick editing
nnoremap <Leader>ev :edit $MYVIMRC<CR>
nnoremap <Leader>sv :source $MYVIMRC<CR>