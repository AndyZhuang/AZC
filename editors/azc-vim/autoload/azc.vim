" AZC Autoload Functions
" Language: AZC
" Maintainer: AZC Team
" Latest Revision: 2024

" Compile current file
function! azc#compile(...)
    let l:file = expand('%:p')
    let l:cmd = g:azc_compiler . ' ' . l:file
    
    if a:0 > 0
        let l:cmd .= ' ' . a:1
    endif
    
    echo "Compiling " . l:file . "..."
    
    let l:output = system(l:cmd)
    
    if v:shell_error == 0
        echo "Compilation successful"
        " Open generated C file if exists
        let l:c_file = substitute(l:file, '\.azc$', '.c', '')
        if filereadable(l:c_file)
            execute 'vsplit ' . l:c_file
        endif
    else
        echohl ErrorMsg
        echo "Compilation failed:"
        echo l:output
        echohl None
    endif
endfunction

" Run current file
function! azc#run(...)
    let l:file = expand('%:p')
    let l:cmd = g:azc_compiler . ' run ' . l:file
    
    echo "Running " . l:file . "..."
    
    " Save file first
    write
    
    let l:output = system(l:cmd)
    
    " Show output in new buffer
    new
    setlocal buftype=nofile
    setlocal noswapfile
    silent put =l:output
    setlocal nomodifiable
    file AZC\ Output
endfunction

" Check current file for errors
function! azc#check(...)
    let l:file = expand('%:p')
    let l:cmd = g:azc_compiler . ' check ' . l:file
    
    let l:output = system(l:cmd)
    
    if v:shell_error != 0
        " Parse errors and add to quickfix
        let l:errors = []
        for l:line in split(l:output, '\n')
            let l:match = matchlist(l:line, '\(.\{-}\):\(\d\+\):\(\d\+\): \(error\|warning\): \(.*\)')
            if !empty(l:match)
                call add(l:errors, {
                    \ 'filename': l:match[1],
                    \ 'lnum': str2nr(l:match[2]),
                    \ 'col': str2nr(l:match[3]),
                    \ 'type': l:match[4] == 'error' ? 'E' : 'W',
                    \ 'text': l:match[5]
                \ })
            endif
        endfor
        
        call setqflist(l:errors)
        copen
    else
        echo "No errors found"
        cclose
    endif
endfunction

" Format current file
function! azc#format()
    let l:file = expand('%:p')
    let l:cmd = g:azc_compiler . ' fmt ' . l:file
    
    " Save cursor position
    let l:cursor = getcurpos()
    
    " Format file
    let l:output = system(l:cmd)
    
    if v:shell_error == 0
        " Reload file
        edit!
        " Restore cursor position
        call setpos('.', l:cursor)
    else
        echohl ErrorMsg
        echo "Format failed"
        echohl None
    endif
endfunction

" Create new project
function! azc#new_project(name)
    let l:dir = a:name
    
    " Create directories
    call mkdir(l:dir, 'p')
    call mkdir(l:dir . '/src', 'p')
    call mkdir(l:dir . '/tests', 'p')
    
    " Create azc.toml
    let l:toml = '[package]
name = "' . a:name . '"
version = "0.1.0"
edition = "2024"

[dependencies]
'
    call writefile(split(l:toml, '\n'), l:dir . '/azc.toml')
    
    " Create main.azc
    let l:main = '# ' . a:name . ' - AZC Project

def main()
    puts "Hello, ' . a:name . '!"
end
'
    call writefile(split(l:main, '\n'), l:dir . '/src/main.azc')
    
    echo "Created project: " . a:name
    execute 'edit ' . l:dir . '/src/main.azc'
endfunction

" Complete function for AZC
function! azc#complete(findstart, base)
    if a:findstart
        let l:line = getline('.')
        let l:start = col('.') - 1
        
        while l:start > 0 && l:line[l:start - 1] =~ '\w'
            let l:start -= 1
        endwhile
        
        return l:start
    else
        let l:suggestions = []
        
        " Keywords
        let l:keywords = ['def', 'let', 'if', 'else', 'elsif', 'end', 'while', 'for', 
                         \ 'in', 'return', 'class', 'struct', 'enum', 'impl', 'trait',
                         \ 'async', 'await', 'unsafe', 'macro', 'extern', 'match', 'when']
        
        for l:kw in l:keywords
            if l:kw =~ '^' . a:base
                call add(l:suggestions, {'word': l:kw, 'kind': 'k', 'menu': 'keyword'})
            endif
        endfor
        
        " Types
        let l:types = ['Int', 'Float', 'String', 'Bool', 'Array', 'Map', 'Set', 
                      \ 'Option', 'Result', 'Future']
        
        for l:t in l:types
            if l:t =~ '^' . a:base
                call add(l:suggestions, {'word': l:t, 'kind': 't', 'menu': 'type'})
            endif
        endfor
        
        return l:suggestions
    endif
endfunction

" Set up completion
autocmd FileType azc setlocal omnifunc=azc#complete