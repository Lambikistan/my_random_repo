""" Instructions {{{
" Based on/Courtesy Of:  https://www.freecodecamp.org/news/vimrc-configuration-guide-customize-your-vim-editor/
" Install vim plug:  curl -fLo ~/.vim/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
" Install Plugins: PlugInstall
"   Update plugin: PlugUpdate
" Tags:
"   tag generation: Universal Ctags or Exuberant Ctags
"   tag browwing:  tagbar
"   tag file mgmt: easytags
""" }}}

""" Base VIM Options CONFIG {{{
set shiftwidth=4
set tabstop=4
set expandtab
set nu
set nobackup
set nowrap
set showcmd
set showmatch
set hlsearch
set history=100
set showmode
syntax enable
" autocompletion in vim uses wildmenu
set wildmenu
set wildmode=list:longest
set wildignore=*.doc,*.docx,*.jpg,*.png,*.gif,*.pdf,*.pyc,*.exe,*.img,*.xls,*.xlsx
""" }}}

""" PLUGINS {{{
call plug#begin('~/.vim/plugged')
  Plug 'preservim/nerdtree'
  Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
  Plug 'rust-lang/rust.vim'
call plug#end()
""" }}}

""" MAPPINGS {{{ 
" help map-modes
" nnoremap - normal mode
" inoremap - insert mode
" vnoremap - visual mode
" mapleader for remapping leader key

let mapleader = ","

" Use jj as escape
inoremap jj <esc>

" Center vert when searching
nnoremap n nzz
nnoremap N Nzz

" Split window navigation remapping, c-j is ctrl-j
nnoremap <c-j> <c-w>j
nnoremap <c-k> <c-w>k
nnoremap <c-h> <c-w>h
nnoremap <c-l> <c-w>l

" NERDTree remapping
nnoremap <F3> :NERDTreeToggle<cr>
""" }}}

""" VIMSCRIPT {{{
" help autocmd
" enable folding on VIM files  w/marker method
augroup filetype_vim
    autocmd!
    autocmd FileType vim setlocal foldmethod=marker
augroup END
""" }}}

""" STATUS LINE {{{
" help: statusline
" Clear status line when vimrc is reloaded.
set statusline=

" Status line left side.
set statusline+=\ %F\ %M\ %Y\ %R

" Use a divider to separate the left side from the right side.
set statusline+=%=

" Status line right side.
set statusline+=\ ascii:\ %b\ hex:\ 0x%B\ row:\ %l\ col:\ %c\ percent:\ %p%%

" Show the status on the second to last line.
set laststatus=2
""" }}}
