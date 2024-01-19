" Disable `vi` compatibility
set nocompatible

" Enable file type detection
filetype on
filetype plugin on

" Auto read when a file is changed externally
set autoread
au FocusGained,BufEnter * silent! checktime

" Enable syntax highlighting
syntax on

" Always show current position
set ruler

" Command bar height
set cmdheight=1

" Configure backspace
set backspace=eol,start,indent
set whichwrap+=<,>,h,l

" Turn on magic regular expressions
set magic

" Disable sounds
set noerrorbells
set novisualbell
set t_vb=
set tm=500

" Set utf8 as standard encoding
set encoding=utf8

" Disable backups
set nobackup
set nowb
set noswapfile

" Use Unix as the standard file type
set ffs=unix,dos,mac

" Enable line numbers
set number
set relativenumber

" Tab settings
set expandtab
set shiftwidth=4
set tabstop=4
set autoindent
set smartindent

" Disable wrap
set nowrap

" Search settings
set incsearch
set ignorecase
set smartcase
set hlsearch
set history=1000

" Show mode
set showmode

" Enable autocompletion
set wildmenu
set wildignore=*.docx,*.xlsx,*.pptx,*.jpg,*.png,*.gif,*.qoi,*.pdf,*.pyc,*.jar,*.exe,*.elf,*.a,*.o,*.flv,*.mp4,*.wmv,*.img,*.iso,*.tar,*.tar.gz,*.zip