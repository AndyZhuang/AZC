# AZC v0.5.0 Design Document - Ecosystem

**Version:** 0.5.0  
**Focus:** Developer Ecosystem and Tooling  
**Status:** In Development

---

## Overview

v0.5.0 focuses on building the developer ecosystem around AZC, making it easier for developers to write, share, and collaborate on AZC code. This version transforms AZC from a compiler into a complete development platform.

## Goals

1. **Package Registry** - Central repository for sharing AZC packages
2. **VSCode Extension** - Full IDE support with syntax highlighting, intellisense, and debugging
3. **Vim/Emacs Plugins** - Editor support for power users
4. **Web Playground** - Online IDE for trying AZC without installation

---

## 1. Package Registry (azc-registry)

### 1.1 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AZC Package Registry                      │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Web UI    │  │   REST API  │  │  Storage    │         │
│  │  (React)    │  │  (Axum)     │  │  (S3/FS)    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    PostgreSQL Database                   ││
│  │  - Packages  - Versions  - Users  - Dependencies        ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 1.2 Package Format (azc.toml)

```toml
[package]
name = "azc-http"
version = "0.1.0"
authors = ["AZC Team <team@azc.dev>"]
edition = "2024"
license = "MIT"
description = "HTTP client and server library for AZC"
repository = "https://github.com/azc-lang/azc-http"
keywords = ["http", "web", "client", "server"]
categories = ["web-programming", "network-programming"]

[dependencies]
azc-json = "0.2.0"
azc-async = "0.1.0"

[dev-dependencies]
azc-test = "0.1.0"

[features]
default = ["client"]
client = []
server = []
tls = ["native-tls"]

[target]
arch = ["x86_64", "arm64"]
os = ["linux", "macos", "windows"]
```

### 1.3 CLI Commands

```bash
# Publish a package
azc publish

# Search packages
azc search http

# Install a package
azc install azc-http

# Update packages
azc update

# List installed packages
azc list

# Show package info
azc info azc-http

# Login to registry
azc login

# Logout from registry
azc logout
```

### 1.4 Registry API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/packages` | GET | List packages |
| `/api/v1/packages/{name}` | GET | Get package info |
| `/api/v1/packages/{name}/versions` | GET | List versions |
| `/api/v1/packages/{name}/{version}` | GET | Get specific version |
| `/api/v1/packages/new` | PUT | Publish package |
| `/api/v1/search?q={query}` | GET | Search packages |
| `/api/v1/users/{username}` | GET | Get user info |

---

## 2. VSCode Extension (azc-vscode)

### 2.1 Features

#### Core Features
- [x] Syntax highlighting
- [x] Bracket matching
- [x] Comment toggling
- [ ] Auto-indentation
- [ ] Code folding

#### Language Server Features
- [ ] Auto-completion
- [ ] Go to definition
- [ ] Find references
- [ ] Rename symbol
- [ ] Hover information
- [ ] Diagnostics (errors/warnings)
- [ ] Code actions (quick fixes)
- [ ] Signature help

#### Advanced Features
- [ ] Debugging support
- [ ] Test explorer
- [ ] Package management UI
- [ ] Safety visualization
- [ ] Inline type hints

### 2.2 Extension Structure

```
azc-vscode/
├── package.json           # Extension manifest
├── tsconfig.json          # TypeScript config
├── src/
│   ├── extension.ts       # Extension entry point
│   ├── client.ts          # Language client
│   ├── commands.ts        # VSCode commands
│   ├── providers/
│   │   ├── completion.ts  # Completion provider
│   │   ├── hover.ts       # Hover provider
│   │   ├── definition.ts  # Definition provider
│   │   └── diagnostics.ts # Diagnostics provider
│   └── utils/
│       └── azc.ts         # AZC CLI wrapper
├── syntaxes/
│   └── azc.tmLanguage.json # TextMate grammar
├── language-configuration.json
└── README.md
```

### 2.3 TextMate Grammar (syntaxes/azc.tmLanguage.json)

```json
{
  "name": "AZC",
  "scopeName": "source.azc",
  "patterns": [
    { "include": "#keywords" },
    { "include": "#strings" },
    { "include": "#numbers" },
    { "include": "#comments" },
    { "include": "#types" },
    { "include": "#functions" }
  ],
  "repository": {
    "keywords": {
      "patterns": [{
        "match": "\\b(let|def|if|else|end|while|for|in|return|class|struct|enum|impl|trait|async|await|unsafe|macro|extern|match|when)\\b",
        "name": "keyword.control.azc"
      }]
    },
    "strings": {
      "patterns": [{
        "begin": "\"",
        "end": "\"",
        "name": "string.quoted.double.azc",
        "patterns": [
          { "match": "#\\{[^}]*\\}", "name": "entity.string.interpolation.azc" }
        ]
      }]
    },
    "numbers": {
      "patterns": [{
        "match": "\\b[0-9]+(\\.[0-9]+)?\\b",
        "name": "constant.numeric.azc"
      }]
    },
    "comments": {
      "patterns": [{
        "match": "#.*$",
        "name": "comment.line.number-sign.azc"
      }]
    },
    "types": {
      "patterns": [{
        "match": "\\b(Int|Float|String|Bool|Array|Map|Set|Option|Result|Future)\\b",
        "name": "entity.name.type.azc"
      }]
    },
    "functions": {
      "patterns": [{
        "match": "\\b([a-z_][a-z0-9_]*)\\s*(?=\\()",
        "name": "entity.name.function.azc"
      }]
    }
  }
}
```

### 2.4 Language Configuration

```json
{
  "comments": {
    "lineComment": "#",
    "blockComment": ["=begin", "=end"]
  },
  "brackets": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"]
  ],
  "autoClosingPairs": [
    { "open": "{", "close": "}" },
    { "open": "[", "close": "]" },
    { "open": "(", "close": ")" },
    { "open": "\"", "close": "\"" },
    { "open": "'", "close": "'" }
  ],
  "surroundingPairs": [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
    ["\"", "\""],
    ["'", "'"]
  ],
  "indentationRules": {
    "increaseIndentPattern": "^\\s*(def|if|else|while|for|class|struct|enum|impl|trait|async|unsafe|macro|extern|match|when)\\b",
    "decreaseIndentPattern": "^\\s*(end|else|when)\\b"
  }
}
```

---

## 3. Vim/Emacs Plugins

### 3.1 Vim Plugin (azc-vim)

```vim
" File type detection
au BufRead,BufNewFile *.azc set filetype=azc

" Syntax highlighting
syn keyword azcKeyword let def if else end while for in return class struct enum impl trait async await unsafe macro extern match when
syn keyword azcType Int Float String Bool Array Map Set Option Result Future
syn keyword azcConstant true false nil
syn match azcNumber /\d\+\(\.\d\+\)\?/
syn region azcString start=/"/ skip=/\\"/ end=/"/ contains=azcInterpolation
syn match azcInterpolation /#{[^}]*}/ contained
syn match azcComment /#.*$/

hi def link azcKeyword Keyword
hi def link azcType Type
hi def link azcConstant Constant
hi def link azcNumber Number
hi def link azcString String
hi def link azcInterpolation Special
hi def link azcComment Comment
```

### 3.2 Emacs Plugin (azc-mode)

```elisp
(define-derived-mode azc-mode prog-mode "AZC"
  "Major mode for editing AZC code."
  
  ;; Syntax table
  (modify-syntax-entry ?# "<" azc-mode-syntax-table)
  (modify-syntax-entry ?\n ">" azc-mode-syntax-table)
  
  ;; Font locking
  (setq font-lock-defaults
        '((azc-font-lock-keywords)))
  
  ;; Indentation
  (setq indent-line-function 'azc-indent-line))

(defvar azc-font-lock-keywords
  `(("\\b\\(let\\|def\\|if\\|else\\|end\\|while\\|for\\|in\\|return\\|class\\|struct\\|enum\\|impl\\|trait\\|async\\|await\\|unsafe\\|macro\\|extern\\|match\\|when\\)\\b" . font-lock-keyword-face)
    ("\\b\\(Int\\|Float\\|String\\|Bool\\|Array\\|Map\\|Set\\|Option\\|Result\\|Future\\)\\b" . font-lock-type-face)
    ("\\b\\(true\\|false\\|nil\\)\\b" . font-lock-constant-face)
    ("\\b[0-9]+\\(\\.[0-9]+\\)?\\b" . font-lock-number-face)
    ("\"[^\"]*\"" . font-lock-string-face)
    ("#.*$" . font-lock-comment-face)))

(add-to-list 'auto-mode-alist '("\\.azc\\'" . azc-mode))
```

---

## 4. Web Playground (azc-playground)

### 4.1 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    AZC Web Playground                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    Frontend (React)                      ││
│  │  ┌───────────────┐  ┌───────────────┐  ┌─────────────┐  ││
│  │  │  Monaco Editor │  │   Output      │  │  Examples   │  ││
│  │  │  (AZC code)    │  │   Panel       │  │  Dropdown   │  ││
│  │  └───────────────┘  └───────────────┘  └─────────────┘  ││
│  └─────────────────────────────────────────────────────────┘│
│                           │                                  │
│                           ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                    Backend (Axum)                        ││
│  │  - Compile AZC to C                                      ││
│  │  - Execute with emscripten/WASM                          ││
│  │  - Return results                                        ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 4.2 Features

- **Code Editor** - Monaco editor with AZC syntax highlighting
- **Run Button** - Compile and run AZC code
- **Output Panel** - Show program output
- **Share** - Generate shareable URLs with encoded code
- **Examples** - Pre-loaded example programs
- **Version Selector** - Choose AZC version

### 4.3 API

```typescript
// POST /api/compile
interface CompileRequest {
  code: string;
  version?: string;  // AZC version
}

interface CompileResponse {
  success: boolean;
  output?: string;   // Program output
  errors?: string[]; // Compilation errors
  c_code?: string;   // Generated C code (for debugging)
}

// GET /api/examples
interface Example {
  name: string;
  description: string;
  code: string;
}
```

---

## 5. Implementation Plan

### Phase 1: VSCode Extension (Week 1-2)
1. Create extension scaffold
2. Implement TextMate grammar
3. Add language configuration
4. Basic syntax highlighting
5. Publish to VSCode Marketplace

### Phase 2: Language Server Integration (Week 3-4)
1. Implement LSP server in Rust
2. Add completion support
3. Add diagnostics
4. Add go-to-definition
5. Add hover information

### Phase 3: Package Registry (Week 5-6)
1. Design database schema
2. Implement REST API
3. Create web UI
4. Integrate with azc CLI
5. Deploy to production

### Phase 4: Web Playground (Week 7-8)
1. Create React frontend
2. Implement backend API
3. Add example programs
4. Deploy to azc.dev/play

### Phase 5: Vim/Emacs Plugins (Week 9)
1. Create vim plugin
2. Create emacs mode
3. Publish to plugin repositories

---

## 6. Success Metrics

| Metric | Target |
|--------|--------|
| VSCode extension downloads | 1,000+ in first month |
| Package registry packages | 50+ packages |
| Web playground users | 500+ unique visitors/week |
| GitHub stars | 2,000+ |

---

## 7. Dependencies

### VSCode Extension
- Node.js 18+
- TypeScript 5+
- VSCode API 1.80+
- vscode-languageclient 9.0+

### Package Registry
- Rust 1.70+
- Axum 0.7+
- PostgreSQL 15+
- S3-compatible storage

### Web Playground
- React 18+
- Monaco Editor 0.40+
- Emscripten / WASM

---

## Appendix: File Structure

```
AZC/
├── compiler/              # Core compiler (v0.1.0 - v0.4.0)
├── tools/
│   ├── azc-pkg/          # Package manager
│   ├── azc-fmt/          # Formatter
│   ├── azc-lint/         # Linter
│   ├── azc-lsp/          # Language server (NEW)
│   ├── azc-repl/         # REPL
│   └── azc-viz/          # Visualization
├── editors/               # Editor plugins (NEW)
│   ├── azc-vscode/       # VSCode extension
│   ├── azc-vim/          # Vim plugin
│   └── azc-emacs/        # Emacs mode
├── registry/              # Package registry (NEW)
│   ├── server/           # Registry server
│   └── web/              # Registry web UI
├── playground/            # Web playground (NEW)
│   ├── frontend/         # React app
│   └── backend/          # Compile server
└── std/                   # Standard library
```