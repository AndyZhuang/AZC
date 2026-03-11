# AZC Tooling

This directory contains all developer tools for the AZC programming language.

## Available Tools

### 1. azc-pkg - Package Manager

```bash
# Create new project
azc new myproject

# Build
azc build

# Run
azc run

# Test
azc test

# Add dependency
azc add serde

# Format code
azc fmt

# Run linter
azc lint
```

### 2. azc-fmt - Code Formatter

```bash
# Format files
azc-fmt file1.azc file2.azc

# Check formatting
azc-fmt --check src/

# Show diff
azc-fmt --diff file.azc
```

### 3. azc-lint - Linter

```bash
# Lint files
azc-lint src/

# Auto-fix
azc-lint --fix src/

# JSON output
azc-lint --format json src/
```

### 4. azc-lsp - Language Server

```bash
# Start LSP server
azc-lsp
```

Integration with editors:
- VSCode: Install AZC extension
- Vim/Neovim: Use with coc.nvim or nvim-lspconfig
- Emacs: Use with lsp-mode

### 5. azc-repl - Interactive REPL

```bash
# Start REPL
azc-repl

# Load file
azc-repl --load mymodule.azc
```

REPL Commands:
```
:help          Show help
:quit          Exit REPL
:load <file>   Load file
:type <expr>   Show type
:vars          List variables
:funcs         List functions
:history       Show history
:reset         Reset environment
```

### 6. azc-viz - Safety Visualization

```bash
# Visualize ownership
azc-viz --ownership file.azc

# Visualize borrows
azc-viz --borrows file.azc

# Generate safety report
azc-viz --report file.azc
```

### 7. azc-sim - Industrial Simulator

```bash
# Run simulation
azc-sim simulation.azc

# Test safety scenarios
azc-sim --test safety_scenarios/

# Generate test report
azc-sim --report
```

## Configuration

### azc.toml

```toml
[package]
name = "myproject"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = "1.0"

[dev-dependencies]
azc-test = "0.1"

[lint]
warn = ["S001", "S002"]
deny = ["E001", "E002"]

[format]
max_width = 100
tab_size = 4
```

## Editor Integration

### VSCode

Install the AZC extension for:
- Syntax highlighting
- Auto-completion
- Go to definition
- Find references
- Error diagnostics
- Code formatting

### Vim/Neovim

```vim
" Using coc.nvim
let g:coc_global_extensions = ['coc-azc']

" Using nvim-lspconfig
require'lspconfig'.azc.setup{}
```

### Emacs

```elisp
(require 'lsp-mode)
(add-hook 'azc-mode-hook #'lsp)
```

## CI/CD Integration

### GitHub Actions

```yaml
name: AZC CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install AZC
        run: |
          curl -sSL https://azc.dev/install.sh | sh
      - name: Build
        run: azc build --release
      - name: Test
        run: azc test
      - name: Lint
        run: azc lint
      - name: Format check
        run: azc fmt --check
```

## Performance

### Build Time Optimization

```toml
[profile.release]
opt_level = "3"
lto = true
codegen_units = 1
```

### Binary Size Reduction

```bash
# Strip symbols
strip target/release/myproject

# UPX compression (optional)
upx --best target/release/myproject
```

## Troubleshooting

### Common Issues

1. **Compiler not found**
   ```bash
   # Build compiler first
   cd compiler && cargo build --release
   ```

2. **Dependency resolution failed**
   ```bash
   # Update lock file
   azc update
   ```

3. **LSP not working**
   ```bash
   # Check LSP logs
   azc-lsp --log-level debug
   ```

## Support

- Documentation: https://azc.dev/docs
- Issues: https://github.com/azc-lang/azc/issues
- Discord: https://discord.gg/azc