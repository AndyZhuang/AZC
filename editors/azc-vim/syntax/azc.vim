" AZC Syntax File
" Language: AZC
" Maintainer: AZC Team
" Latest Revision: 2024

if exists("b:current_syntax")
    finish
endif

" Keywords
syn keyword azcKeyword let def if else elsif end while for in return break next match when
syn keyword azcDeclaration class struct enum impl trait type macro
syn keyword azcModifier async await unsafe extern use mod pub mut ref self Self
syn keyword azcOperator and or not

" Types
syn keyword azcType Int Float String Bool Char Void Nil
syn keyword azcType I8 I16 I32 I64 I128 ISize
syn keyword azcType U8 U16 U32 U64 U128 USize
syn keyword azcType F32 F64
syn keyword azcType Array Map Set Option Result Future Pointer

" Constants
syn keyword azcConstant true false nil

" Numbers
syn match azcNumber "\<\d\+\%(_\d\+\)*\%(\.\d\+\%(_\d\+\)*\)\?\%([eE][+-]\?\d\+\%(_\d\+\)*\)\?\>"
syn match azcNumber "\<0[xX]\x\+\%(_\x\+\)*\>"
syn match azcNumber "\<0[oO]\o\+\%(_\o\+\)*\>"
syn match azcNumber "\<0[bB][01]\+\%(_[01]\+\)*\>"

" Strings
syn region azcString start=/"/ skip=/\\"/ end=/"/ contains=azcInterpolation,azcEscape
syn region azcChar start=/'/ skip=/\\'/ end=/'/ contains=azcEscape

" String interpolation
syn match azcInterpolation "#{\([^}]*\)}" contained contains=TOP
syn match azcEscape "\\[nrt\"'\\0]" contained

" Comments
syn match azcComment "#.*$" contains=azcTodo
syn region azcBlockComment start="=begin" end="=end" contains=azcTodo

" Todo items in comments
syn keyword azcTodo TODO FIXME XXX NOTE contained

" Functions
syn match azcFunction "\%(\%(def\s\+\)\@<=\h\w*\|\h\w*\s*(\@=\)"

" Type names (PascalCase)
syn match azcTypeName "\<\u\w*\>"

" Annotations
syn match azcAnnotation "@\h\w*"

" Operators
syn match azcOperator "->"
syn match azcOperator "=>"
syn match azcOperator "\.\."
syn match azcOperator "=="
syn match azcOperator "!="
syn match azcOperator "<="
syn match azcOperator ">="
syn match azcOperator "+\|-\|\*\|\/\|%"
syn match azcOperator "<\|>"
syn match azcOperator "=\|+=\|-=\|*=\|/="
syn match azcOperator "&&\|||\|!"
syn match azcOperator "&\||\|\^\|~\|<<\|>>"
syn match azcOperator "?"
syn match azcOperator "::"

" Punctuation
syn match azcPunctuation "[,;:.]"
syn match azcBracket "[\[\]{}]"

" Highlight groups
hi def link azcKeyword Keyword
hi def link azcDeclaration Structure
hi def link azcModifier StorageClass
hi def link azcOperator Operator
hi def link azcType Type
hi def link azcConstant Constant
hi def link azcNumber Number
hi def link azcString String
hi def link azcChar Character
hi def link azcInterpolation Special
hi def link azcEscape SpecialChar
hi def link azcComment Comment
hi def link azcBlockComment Comment
hi def link azcTodo Todo
hi def link azcFunction Function
hi def link azcTypeName Type
hi def link azcAnnotation PreProc
hi def link azcPunctuation Delimiter
hi def link azcBracket Delimiter

let b:current_syntax = "azc"