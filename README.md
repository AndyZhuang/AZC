# AZC Programming Language

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
</p>

**AZC** (Agile Zest Code) is a new programming language that combines **Rust's safety guarantees** with **Ruby's expressive syntax**. Originally designed for industrial control systems (SCADA/DCS), AZC aims to become a general-purpose language.

## ✨ Features

- **Ruby-like Syntax** - Clean, readable, expressive code
- **Rust-like Safety** - Memory safety, ownership, and borrowing at compile time
- **Industrial Ready** - Designed for SCADA/DCS systems
- **Type Inference** - Optional type annotations
- **Modern Features** - Pattern matching, closures, blocks

## 📖 Example

```ruby
# AZC - Rust Safety + Ruby Syntax

# Variables (inferred types)
name = "AZC"
count = 42
active = true

# Functions (Ruby-style)
def greet(person)
  "Hello, #{person}!"
end

# Control flow
if count > 10
  puts "Large count"
else
  puts "Small count"
end

# While loops
i = 0
while i < 5
  puts "Count: #{i}"
  i += 1
end

# Classes (coming soon)
class Counter
  def initialize
    @count = 0
  end
  
  def increment
    @count += 1
  end
end
```

## 🛠️ Installation

### Prerequisites

- Rust 1.70+ (for building the compiler)
- C compiler (gcc/clang) for compiling generated C code

### Build from Source

```bash
# Clone the repository
git clone https://github.com/andyzhuang/azc.git
cd azc

# Build the compiler
cd compiler
cargo build --release

# The binary is at target/release/azc
```

## 🚀 Quick Start

```bash
# Compile an AZC file
./target/release/azc input.azc -o output.c

# Or compile directly to executable (requires gcc)
./target/release/azc input.azc && gcc output.c -o myprogram && ./myprogram
```

### Example Programs

```bash
# Run the hello world example
cd examples
../compiler/target/release/azc hello.azc
```

## 📂 Project Structure

```
AZC/
├── compiler/           # AZC compiler (Rust)
│   ├── src/
│   │   ├── lib.rs    # Main compiler logic
│   │   └── main.rs   # CLI entry point
│   └── Cargo.toml
├── docs/
│   └── AZC_LANGUAGE_DESIGN.md  # Language specification
├── examples/          # Example AZC programs
│   └── hello.azc
├── tests/             # Test suite
│   ├── variables.azc
│   ├── functions.azc
│   ├── control_flow.azc
│   └── while_loop.azc
└── README.md
```

## 🧪 Test Suite

```bash
cd compiler
cargo test

# Or run individual tests
./target/release/azc ../tests/variables.azc
./target/release/azc ../tests/functions.azc
./target/release/azc ../tests/control_flow.azc
./target/release/azc ../tests/while_loop.azc
```

## 🔧 Language Features (MVP)

| Feature | Status | Description |
|---------|--------|-------------|
| Variables | ✅ | `let x = 10` |
| Functions | ✅ | `def foo() ... end` |
| If/Else | ✅ | Ruby-style control flow |
| While loops | ✅ | `while condition ... end` |
| Strings | ✅ | `"Hello, #{world}!"` |
| Classes | 🔄 | Basic support |
| Type annotations | 🔄 | `let x: Int = 10` |
| Ownership | 🔄 | Rust-like borrowing |
| Traits | ❌ | Planned |
| Generics | ❌ | Planned |
| Unsafe blocks | ❌ | Planned |

## 🗺️ Roadmap

### v0.1.x - MVP (Current)
- [x] Basic syntax (Ruby-like)
- [x] Variables and types
- [x] Functions
- [x] Control flow
- [x] Code generation to C

### v0.2.x - Safety
- [ ] Type system implementation
- [ ] Ownership model
- [ ] Borrow checker
- [ ] Memory safety guarantees

### v0.3.x - Features
- [ ] Trait system
- [ ] Pattern matching
- [ ] Error handling (Result, Option)
- [ ] Standard library

### v0.4.x - Industrial
- [ ] ICS data types
- [ ] Safety annotations (SIL)
- [ ] Real-time extensions
- [ ] Modbus/OPC-UA primitives

### v1.0 - Production
- [ ] Full feature parity
- [ ] Cross-platform support
- [ ] Package manager
- [ ] IDE support

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a PR

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- **Rust Team** - For the amazing safety model
- **Ruby Community** - For inspiring language design
- **Matz** - For creating Ruby

---

<p align="center">
  Made with ❤️ for safer programming
</p>
