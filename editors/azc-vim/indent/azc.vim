" AZC Indent File
" Language: AZC
" Maintainer: AZC Team
" Latest Revision: 2024

if exists("b:did_indent")
    finish
endif
let b:did_indent = 1

setlocal autoindent
setlocal indentexpr=GetAZCIndent()
setlocal indentkeys+=0=end,0=else,0=elsif,0=when,0=},0=\)

" Only define the function once
if exists("*GetAZCIndent")
    finish
endif

function! GetAZCIndent()
    let line = getline(v:lnum)
    let prevline = getline(v:lnum - 1)
    
    " At start of file
    if v:lnum == 1
        return 0
    endif
    
    " Get current indent
    let indent = indent(v:lnum - 1)
    
    " Increase indent after block-starting keywords
    if prevline =~ '^\s*\(def\|if\|else\|elsif\|while\|for\|class\|struct\|enum\|impl\|trait\|async\|unsafe\|macro\|extern\|match\|when\)\>'
        let indent += &shiftwidth
    endif
    
    " Increase indent after opening brackets
    if prevline =~ '[{(\[]\s*$'
        let indent += &shiftwidth
    endif
    
    " Decrease indent for closing keywords
    if line =~ '^\s*\(end\|else\|elsif\|when\)\>'
        let indent -= &shiftwidth
    endif
    
    " Decrease indent for closing brackets
    if line =~ '^\s*[})\]]'
        let indent -= &shiftwidth
    endif
    
    return indent < 0 ? 0 : indent
endfunction