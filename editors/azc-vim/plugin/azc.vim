" AZC Plugin File
" Language: AZC
" Maintainer: AZC Team
" Latest Revision: 2024

" Ensure plugin is loaded only once
if exists("g:loaded_azc")
    finish
endif
let g:loaded_azc = 1

" Save compatibility options
let s:save_cpo = &cpo
set cpo&vim

" Configuration
if !exists("g:azc_compiler")
    let g:azc_compiler = "azc"
endif

if !exists("g:azc_format_on_save")
    let g:azc_format_on_save = 1
endif

" Commands
command! -nargs=? AZCCompile call azc#compile(<args>)
command! -nargs=? AZCRun call azc#run(<args>)
command! -nargs=? AZCCheck call azc#check(<args>)
command! -nargs=1 AZCNewProject call azc#new_project(<args>)

" Mappings
nnoremap <Plug>(azc-compile) :call azc#compile()<CR>
nnoremap <Plug>(azc-run) :call azc#run()<CR>
nnoremap <Plug>(azc-check) :call azc#check()<CR>

" Autocommands
augroup azc
    autocmd!
    autocmd FileType azc setlocal commentstring=#\ %s
    autocmd FileType azc setlocal formatoptions-=t formatoptions+=croql
    autocmd FileType azc setlocal iskeyword+=?
    autocmd FileType azc setlocal suffixesadd=.azc
    autocmd BufWritePost *.azc if g:azc_format_on_save | call azc#format() | endif
augroup END

" Restore compatibility options
let &cpo = s:save_cpo
unlet s:save_cpo