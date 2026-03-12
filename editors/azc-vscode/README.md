# AZC Language Support for Visual Studio Code

This extension provides language support for the [AZC](https://azc.dev) programming language.

## Features

- 🎨 **Syntax Highlighting** - Full syntax highlighting for AZC files
- ✅ **Bracket Matching** - Automatic bracket matching and closing
- 💬 **Comment Toggling** - Toggle comments with `Ctrl+/`
- 📝 **Auto-indentation** - Smart indentation based on code structure
- 🔧 **Commands** - Compile and run AZC files directly from VSCode
- 🛡️ **Safety Reports** - View safety analysis of your code

## Installation

### From VSCode Marketplace
1. Open VSCode
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "AZC"
4. Click Install

### From Source
```bash
cd editors/azc-vscode
npm install
npm run compile
# Press F5 in VSCode to launch extension development host
```

## Commands

| Command | Description |
|---------|-------------|
| `AZC: Compile to C` | Compile the current file to C code |
| `AZC: Run Current File` | Compile and run the current file |
| `AZC: Create New Project` | Create a new AZC project |
| `AZC: Show Safety Report` | Show safety analysis report |

## Configuration

| Setting | Description | Default |
|---------|-------------|---------|
| `azc.compilerPath` | Path to AZC compiler | `azc` |
| `azc.formatOnSave` | Format files on save | `true` |
| `azc.lintOnSave` | Lint files on save | `true` |
| `azc.safetyCheckOnSave` | Run safety checks on save | `true` |
| `azc.showSafetyAnnotations` | Show inline safety annotations | `true` |

## Requirements

- AZC compiler installed on your system
- Node.js 18+ (for extension development)

## Example

```ruby
# AZC code example
def greet(name: String) -> String
    "Hello, #{name}!"
end

# Async function
async def fetch_data() -> Future<String>
    let response = await http_get("https://api.example.com")
    response.body
end

# Safety annotation
@sil(3)
def emergency_shutdown()
    close_all_valves()
    trigger_alarm()
end

# Run
puts greet("World")
```

## Keyboard Shortcuts

| Shortcut | Command |
|----------|---------|
| `Ctrl+Shift+B` | Build/Compile |
| `F5` | Run with Debugger |

## Contributing

Contributions are welcome! Please see the [main repository](https://github.com/azc-lang/azc) for contribution guidelines.

## License

MIT License - see [LICENSE](../../LICENSE) for details.

## Links

- [AZC Website](https://azc.dev)
- [Documentation](https://azc.dev/docs)
- [GitHub Repository](https://github.com/azc-lang/azc)
- [Issue Tracker](https://github.com/azc-lang/azc/issues)