# AZC Vim Plugin

Syntax highlighting and support for the [AZC](https://azc.dev) programming language in Vim and Neovim.

## Installation

### Using vim-plug

```vim
Plug 'azc-lang/azc-vim'
```

### Using Vundle

```vim
Plugin 'azc-lang/azc-vim'
```

### Using Pathogen

```bash
cd ~/.vim/bundle
git clone https://github.com/azc-lang/azc-vim.git
```

### Manual Installation

```bash
cp -r syntax/* ~/.vim/syntax/
cp -r indent/* ~/.vim/indent/
cp -r ftdetect/* ~/.vim/ftdetect/
cp -r autoload/* ~/.vim/autoload/
cp -r plugin/* ~/.vim/plugin/
```

## Features

- **Syntax Highlighting** - Full syntax support for AZC
- **Auto-indentation** - Smart indentation for blocks
- **File Detection** - Automatic detection of `.azc` files
- **Commands** - Compile and run AZC files from Vim
- **Completion** - Basic keyword and type completion

## Commands

| Command | Description |
|---------|-------------|
| `:AZCCompile` | Compile current file |
| `:AZCRun` | Run current file |
| `:AZCCheck` | Check for errors |
| `:AZCNewProject name` | Create new project |

## Mappings

Create your own mappings in your `.vimrc`:

```vim
" Compile current file
nmap <leader>c <Plug>(azc-compile)

" Run current file
nmap <leader>r <Plug>(azc-run)

" Check for errors
nmap <leader>k <Plug>(azc-check)
```

## Configuration

```vim
" Path to AZC compiler (default: 'azc')
let g:azc_compiler = 'azc'

" Enable format on save (default: 1)
let g:azc_format_on_save = 1
```

## File Type Settings

The plugin sets the following options for AZC files:

```vim
setlocal commentstring=#\ %s
setlocal formatoptions-=t formatoptions+=croql
setlocal iskeyword+=?
setlocal suffixesadd=.azc
```

## Syntax Overview

```ruby
# Comments start with #
def greet(name: String) -> String
    "Hello, #{name}!"  # String interpolation
end

# Types
let x: Int = 42
let y: Float = 3.14
let active: Bool = true

# Structs
struct Point
    x: Float
    y: Float
end

# Async functions
async def fetch() -> Future<String>
    await http_get("https://example.com")
end
```

## License

MIT License