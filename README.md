# AZC Programming Language

<p align="center">
  <img src="https://img.shields.io/badge/version-0.5.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
  <img src="https://img.shields.io/badge/safety-95%2F100-brightgreen.svg" alt="Safety Score">
</p>

**AZC** (Agile Zest Code) is a modern programming language that combines **Rust's memory safety guarantees** with **Ruby's expressive syntax**. Designed for safety-critical industrial control systems (SCADA/DCS), AZC makes safe programming accessible without sacrificing productivity.

---

## 🌟 Why AZC?

| Feature | Rust | Ruby | AZC |
|---------|------|------|-----|
| Memory Safety | ✅ Compile-time | ❌ GC | ✅ Compile-time + Runtime |
| Syntax | Verbose | Elegant | ✅ Ruby-like |
| Learning Curve | Steep | Gentle | ✅ Gentle |
| Industrial Focus | General | Web | ✅ SCADA/DCS optimized |
| Safety Visualization | ❌ | ❌ | ✅ Built-in |
| Industrial Simulator | ❌ | ❌ | ✅ Built-in |

---

## ✨ Key Features

### 🛡️ Safety First
- **Ownership System** - Rust-like ownership and borrowing
- **Borrow Checker** - Compile-time borrow validation
- **Type Safety** - Hindley-Milner type inference
- **Runtime Checks** - Bounds checking, overflow detection
- **Safety Score** - Visual safety analysis (95/100)

### 🎨 Developer Experience
- **Ruby-like Syntax** - Clean, readable, expressive
- **Comprehensive Tooling** - 7 developer tools included
- **IDE Support** - VSCode, Vim, Emacs
- **Interactive REPL** - Rapid prototyping
- **Hot Reload** - Instant feedback

### 🏭 Industrial Ready
- **SIL Annotations** - Safety Integrity Level support
- **Real-time Constraints** - Deadline monitoring
- **Fault Injection** - Test safety scenarios
- **Hardware-in-Loop** - Test with real PLCs

---

## 📖 Example

```ruby
# AZC - Safety + Elegance

# Variables with type inference
let name = "AZC"
let count = 42
let active = true

# Functions with optional type annotations
def greet(person: String) -> String
    "Hello, #{person}!"
end

# Safe arithmetic with overflow checks
def safe_add(a: Int, b: Int) -> Int
    a + b  # Automatically checked for overflow
end

# Ownership and borrowing
let data = String::new("important")
let ref = &data        # Immutable borrow
let len = data.len()   # Data still valid

# Control flow
if count > 10
    puts "Large count"
else
    puts "Small count"
end

# Loops
for i in 0..10
    puts "Count: #{i}"
end

# Pattern matching (coming in v0.3)
match result
when Ok(value)
    puts value
when Err(e)
    puts "Error: #{e}"
end

# Safety annotations for industrial
@sil(3)  # Safety Integrity Level 3
@deadline(100ms)
def emergency_shutdown()
    close_all_valves()
    trigger_alarm()
end
```

---

## 🛠️ Tooling

AZC comes with a comprehensive suite of developer tools:

### 1. Package Manager (azc-pkg)

```bash
# Create new project
azc new myproject

# Build
azc build --release

# Run
azc run

# Test
azc test

# Add dependencies
azc add serde

# Format code
azc fmt

# Run linter
azc lint

# Generate docs
azc doc --open

# Publish
azc publish
```

### 2. Code Formatter (azc-fmt)

```bash
azc-fmt src/           # Format all files
azc-fmt --check src/   # Check formatting
azc-fmt --diff file.azc # Show diff
```

### 3. Linter (azc-lint)

```bash
azc-lint src/          # Lint all files
azc-lint --fix src/    # Auto-fix issues
azc-lint --format json # JSON output
```

| Code | Level | Description |
|------|-------|-------------|
| S001 | Style | Line too long |
| S002 | Style | Trailing whitespace |
| S003 | Style | Naming conventions |
| S004 | Warning | Comparison to bool |
| S005 | Warning | Unnecessary return |
| S006 | Warning | Empty block |
| S007 | Info | Complex expression |
| S008 | Info | Missing documentation |
| S009 | Error | Integer overflow risk |

### 4. Language Server (azc-lsp)

```bash
azc-lsp  # Start LSP server
```

**Features:**
- ✅ Auto-completion
- ✅ Hover information
- ✅ Go to definition
- ✅ Find references
- ✅ Rename symbol
- ✅ Diagnostics
- ✅ Formatting

### 5. REPL (azc-repl)

```bash
azc-repl
```

```
AZC REPL v0.1.0
Type :help for commands

azc> let x = 5
=> 5
azc> x + 10
=> 15
azc> :type x
Int
azc> :help
```

### 6. Safety Visualization (azc-viz)

```bash
azc-viz ownership file.azc
azc-viz lifetimes file.azc
azc-viz report file.azc --output report.html
```

```
┌─────────────────────────────────────────────┐
│ Safety Heat Map                              │
├─────────────────────────────────────────────┤
│ Line 1:  let x = 5          🟢 Safe         │
│ Line 2:  let y = &x         🟢 Safe         │
│ Line 3:  let z = &mut x     🔴 Error        │
│                                              │
│ Score: 95/100                                │
└─────────────────────────────────────────────┘
```

### 7. Industrial Simulator (azc-sim)

```bash
azc-sim run controller.azc
azc-sim test controller.azc --scenario emergency
azc-sim certify controller.azc --sil 3
```

```ruby
# Define PLC simulation
simulator PLC_Tank_Controller
    sensors:
        level: AnalogInput(0.0..100.0)
        temperature: AnalogInput(-20.0..150.0)
    
    actuators:
        inlet_valve: DigitalOutput
        heater: DigitalOutput
    
    scenarios:
        low_level:
            sensor.level.set(15.0)
            assert(inlet_valve.open?)
end
```

---

## 🖥️ Editor Support (v0.5.0)

AZC provides first-class editor support for popular IDEs and text editors.

### VSCode Extension

Install from the VSCode Marketplace or build from source:

```bash
cd editors/azc-vscode
npm install
npm run compile
# Press F5 in VSCode to launch extension development host
```

**Features:**
- ✅ Syntax highlighting
- ✅ Bracket matching & auto-closing
- ✅ Comment toggling
- ✅ Auto-indentation
- ✅ Code folding
- ✅ Compile & Run commands
- ✅ Diagnostics integration

**Commands:**
- `AZC: Compile to C` - Compile current file
- `AZC: Run Current File` - Compile and run
- `AZC: Create New Project` - Start new project
- `AZC: Show Safety Report` - View safety analysis

### Vim Plugin

Install using your favorite plugin manager:

```vim
" vim-plug
Plug 'azc-lang/azc-vim'

" Vundle
Plugin 'azc-lang/azc-vim'
```

**Features:**
- ✅ Syntax highlighting
- ✅ Auto-indentation
- ✅ File detection
- ✅ Commands: `:AZCCompile`, `:AZCRun`, `:AZCCheck`

---

## 🌐 Web Playground (v0.5.0)

Try AZC online without installation: **[play.azc.dev](https://play.azc.dev)**

Run the playground locally:

```bash
# Start backend
cd playground/backend && cargo run

# Start frontend (in another terminal)
cd playground/frontend && npm start
```

**Features:**
- ✅ Monaco editor with AZC syntax highlighting
- ✅ Compile and run AZC code
- ✅ View generated C code
- ✅ Pre-loaded example programs
- ✅ Share code via URL

---

## 📦 Package Registry (v0.5.0)

Publish and share AZC packages: **[registry.azc.dev](https://registry.azc.dev)**

### Publishing a Package

```bash
# Login to registry
azc login

# Publish your package
azc publish
```

### Using Packages

```toml
# azc.toml
[dependencies]
azc-http = "0.1.0"
azc-json = "0.2.0"
```

```bash
azc install
```

---

## 📦 Installation

### Prerequisites
- Rust 1.70+ (for building compiler)
- C compiler (gcc/clang)
- Git

### Quick Install

```bash
# Clone repository
git clone https://github.com/andyzhuang/azc.git
cd azc

# Build compiler
cd compiler && cargo build --release

# Build tools
cd ../tools/azc-pkg && cargo build --release
cd ../azc-fmt && cargo build --release
cd ../azc-lint && cargo build --release
cd ../azc-lsp && cargo build --release
cd ../azc-repl && cargo build --release

# Add to PATH
export PATH="$PWD/target/release:$PATH"
```

### Verify Installation

```bash
azc --version
azc-fmt --version
azc-lint --version
azc-repl --version
```

---

## 🚀 Quick Start

### Create a New Project

```bash
# Create project
azc new hello_azc
cd hello_azc

# This creates:
# hello_azc/
# ├── azc.toml
# ├── src/
# │   └── main.azc
# ├── tests/
# └── README.md
```

### Write Your First Program

```ruby
# src/main.azc

# Define a function
def fibonacci(n: Int) -> Int
    if n <= 1
        n
    else
        fibonacci(n - 1) + fibonacci(n - 2)
    end
end

# Main entry point
def main()
    puts "Fibonacci sequence:"
    
    for i in 0..10
        let result = fibonacci(i)
        puts "fib(#{i}) = #{result}"
    end
end
```

### Build and Run

```bash
azc run
```

### Write Tests

```ruby
# tests/fibonacci_test.azc

test "fibonacci of 0"
    assert fibonacci(0) == 0
end

test "fibonacci of 10"
    assert fibonacci(10) == 55
end
```

```bash
azc test
```

---

## 📚 Language Features

### Variables

```ruby
# Type inference
let name = "AZC"
let count = 42
let pi = 3.14159
let active = true

# Type annotations
let x: Int = 10
let y: Float = 3.14
let flag: Bool = true

# Mutable variables
let mut counter = 0
counter = counter + 1
```

### Functions

```ruby
# Basic function
def greet(name)
    puts "Hello, #{name}!"
end

# With type annotations
def add(a: Int, b: Int) -> Int
    a + b
end

# Default parameters (coming soon)
def greet(name = "World")
    puts "Hello, #{name}!"
end

# Closures (coming soon)
let double = |x| x * 2
let result = [1, 2, 3].map(|x| x * 2)
```

### Control Flow

```ruby
# If/Else
if temperature > 100
    puts "Too hot!"
elsif temperature < 0
    puts "Too cold!"
else
    puts "Just right!"
end

# While loop
let i = 0
while i < 10
    puts i
    i = i + 1
end

# For loop (coming soon)
for i in 0..10
    puts i
end

for item in collection
    puts item
end
```

### Ownership & Borrowing

```ruby
# Ownership
let s1 = String::new("hello")
let s2 = s1  # s1 moved to s2
# puts s1  # Error: s1 no longer valid

# Borrowing
let s1 = String::new("hello")
let len = s1.len()       # s1 still valid
let ref = &s1            # Immutable borrow
let len2 = s1.len()      # s1 still valid

# Mutable borrow
let mut s = String::new("hello")
let ref = &mut s
ref.push_str(" world")
# s.len()  // Error: s borrowed
```

### Structs

```ruby
struct Point
    x: Float
    y: Float
end

impl Point
    def new(x: Float, y: Float) -> Point
        Point { x, y }
    end
    
    def distance(self) -> Float
        (self.x * self.x + self.y * self.y).sqrt()
    end
end

let p = Point::new(3.0, 4.0)
puts p.distance()  # 5.0
```

### Enums

```ruby
enum Option<T>
    Some(T)
    None
end

enum Result<T, E>
    Ok(T)
    Err(E)
end

def divide(a: Int, b: Int) -> Result<Int, String>
    if b == 0
        Err("division by zero")
    else
        Ok(a / b)
    end
end

# Error propagation
let result = divide(10, 2)?  # Unwrap or return error
```

### Safety Annotations

```ruby
# Safety Integrity Level (IEC 61508)
@sil(3)
def safety_critical_function()
    # This function is certified to SIL 3
end

# Real-time constraints
@deadline(100ms)
def must_complete_quickly()
    # Must complete within 100ms
end

# Memory constraints
@max_stack(1024)
def embedded_function()
    # Limited stack usage
end
```

### Advanced Features (v0.4.0)

#### Async/Await

```ruby
# Async function for non-blocking I/O
async def fetch_data() -> Future<String>
    let response = await http_get("https://api.example.com")
    response.body
end

# Concurrent execution
async def process_multiple()
    let task1 = async { fetch_data() }
    let task2 = async { compute_result() }
    let results = await all([task1, task2])
    results
end
```

#### Macros

```ruby
# Compile-time code generation
macro debug_log(expr)
    puts "Debug: #{expr} = #{expr}"
end

# Usage
debug_log(x + y)  # Expands to: puts "Debug: x + y = <computed value>"
```

#### Unsafe Blocks

```ruby
# Escape hatch for low-level operations
unsafe
    # Direct memory manipulation (use with caution!)
    let ptr = raw_pointer(address)
    write_memory(ptr, value)
end

# Document why unsafe is needed
unsafe "Required for hardware register access"
    write_hardware_register(0x1000, data)
end
```

#### Foreign Function Interface (FFI)

```ruby
# Import C functions
extern "C"
    def puts(s: String) -> Int
    def strlen(s: String) -> Int
    def malloc(size: Int) -> Pointer
    def free(ptr: Pointer)
end

# Call external functions
let len = strlen("Hello, World!")
puts("String length: #{len}")
```

---

## 🏭 Industrial Control Features

### Data Types

```ruby
# Industrial types
type AnalogInput = Float      # 4-20mA, 0-10V
type AnalogOutput = Float
type DigitalInput = Bool
type DigitalOutput = Bool

# Process variables
type Temperature = Float      # Celsius
type Pressure = Float         # Bar
type FlowRate = Float         # L/min
type Level = Float            # Percentage
```

### Communication Protocols

```ruby
# Modbus TCP
let client = ModbusTCP.new("192.168.1.100", 502)
let temp = client.read_holding_register(40001)
client.write_single_register(40002, setpoint)

# OPC-UA
let opc = OPCUA.new("opc.tcp://server:4840")
let node = opc.get_node("ns=2;s=Temperature")
let value = node.read()
```

### Safety Patterns

```ruby
# Watchdog pattern
@watchdog(1s)
def heartbeat()
    send_signal(WD1)
end

# Fail-safe pattern
@sil(2)
def emergency_shutdown()
    close_all_valves()
    trigger_alarm()
    log_incident()
end

# Redundant sensors
def read_temperature() -> Temperature
    let t1 = sensor.temp1.read()
    let t2 = sensor.temp2.read()
    let t3 = sensor.temp3.read()
    
    # 2-out-of-3 voting
    if t1.approx(t2)
        (t1 + t2) / 2
    elsif t1.approx(t3)
        (t1 + t3) / 2
    else
        (t2 + t3) / 2
    end
end
```

---

## 📁 Project Structure

```
AZC/
├── compiler/              # Core compiler
│   ├── src/
│   │   ├── ast/          # Abstract Syntax Tree
│   │   ├── types/        # Type system
│   │   ├── ownership/    # Ownership & borrowing
│   │   ├── safety/       # Safety analyzer
│   │   └── runtime/      # Runtime safety
│   └── Cargo.toml
├── tools/                 # Developer tools
│   ├── azc-pkg/          # Package manager
│   ├── azc-fmt/          # Code formatter
│   ├── azc-lint/         # Linter
│   ├── azc-lsp/          # Language server
│   ├── azc-repl/         # REPL
│   ├── azc-viz/          # Safety visualization
│   └── azc-sim/          # Industrial simulator
├── editors/               # Editor plugins (v0.5.0)
│   ├── azc-vscode/       # VSCode extension
│   └── azc-vim/          # Vim plugin
├── registry/              # Package registry (v0.5.0)
│   ├── server/           # Registry server
│   └── web/              # Registry web UI
├── playground/            # Web playground (v0.5.0)
│   ├── frontend/         # React app
│   └── backend/          # Compile server
├── std/                   # Standard library
│   ├── core.azc          # Core types
│   └── io.azc            # I/O operations
├── tests/                 # Test suite
│   ├── 01_variables.azc
│   ├── 02_functions.azc
│   └── ...
├── docs/                  # Documentation
│   ├── AZC_LANGUAGE_DESIGN.md
│   ├── AZC_VS_RUST_COMPARISON.md
│   ├── AZC_v0.4.0_Design.md
│   ├── AZC_v0.5.0_Design.md
│   └── IMPROVEMENTS_AND_TOOLS_SUMMARY.md
├── README.md
├── LICENSE
└── nextstep.md
```

---

## 🧪 Testing

```bash
# Run all tests
azc test

# Run specific test
azc test --test fibonacci

# Run with coverage
azc test --coverage

# Run safety tests
azc-sim test --safety
```

**Current Test Status:**
- ✅ 101 unit tests passing
- ✅ 12 integration tests passing
- ✅ Safety score: 95/100

---

## 🗺️ Roadmap

### v0.2.0 - Safety & Tooling ✅
- [x] Type system with inference
- [x] Ownership and borrowing
- [x] Borrow checker
- [x] Runtime safety checks
- [x] Package manager (azc-pkg)
- [x] Code formatter (azc-fmt)
- [x] Linter (azc-lint)
- [x] Language server (azc-lsp)
- [x] REPL (azc-repl)
- [x] Safety visualization (azc-viz)
- [x] Industrial simulator (azc-sim)

### v0.3.0 - Core Features ✅
- [x] Generics
- [x] Traits
- [x] Pattern matching
- [x] Error handling (Result/Option)
- [x] Closures

### v0.4.0 (Current) - Advanced Features
- [x] Async/await
- [x] Macros
- [x] Unsafe blocks
- [x] FFI
### v0.5.0 (Current) - Ecosystem
- [x] Package registry
- [x] VSCode extension
- [x] Vim plugin
- [x] Web playground
### v1.0 - Production
- [ ] IEC 61508 certification
- [ ] Full standard library
- [ ] Cross-platform support
- [ ] Performance optimization

---

## 📊 Comparison with Rust

| Feature | Rust | AZC | Advantage |
|---------|------|-----|-----------|
| Memory Safety | ✅ | ✅ | Same guarantees |
| Syntax | C-style | Ruby-style | ✅ AZC: More readable |
| Learning Curve | 2-3 months | 1-2 weeks | ✅ AZC: Easier |
| Compilation Speed | Medium | Fast | ✅ AZC: Simpler |
| Safety Visualization | ❌ | ✅ | ✅ AZC: Built-in |
| Industrial Simulator | ❌ | ✅ | ✅ AZC: Built-in |
| Generics | ✅ | 🔄 | Rust: More mature |
| Ecosystem | ✅ | 🔄 | Rust: Larger |

See [AZC vs Rust Comparison](docs/AZC_VS_RUST_COMPARISON.md) for detailed analysis.

---

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing`)
3. Make your changes
4. Run tests (`azc test`)
5. Format code (`azc fmt`)
6. Run linter (`azc lint`)
7. Commit changes (`git commit -m 'Add amazing feature'`)
8. Push to branch (`git push origin feature/amazing`)
9. Open a Pull Request

### Code of Conduct
- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Rust Team** - For the revolutionary safety model
- **Ruby Community** - For inspiring elegant syntax
- **Matz (Yukihiro Matsumoto)** - For creating Ruby
- **Industrial Control Community** - For domain expertise

---

## 📞 Community

- **GitHub**: [github.com/andyzhuang/azc](https://github.com/andyzhuang/azc)
- **Documentation**: [azc.dev/docs](https://azc.dev/docs)
- **Issues**: [GitHub Issues](https://github.com/andyzhuang/azc/issues)

---

<p align="center">
  <strong>Built with ❤️ for safer industrial systems</strong>
</p>

<p align="center">
  <sub>Making safety-critical programming accessible to everyone</sub>
</p>