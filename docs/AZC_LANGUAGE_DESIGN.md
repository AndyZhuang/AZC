# AZC Language Design Document

## Version: 0.1.0 (MVP)

---

## 1. Language Philosophy

### 1.1 Core Vision

AZC = **A**bstra**Z**ed **C**ontrol Language

A programming language that combines:
- **Rust's Safety Guarantees**: Ownership, borrowing, lifetimes, memory safety
- **Ruby's Expressive Syntax**: Natural, readable, programmer-friendly
- **Industrial Control Focus**: Built for SCADA/DCS systems
- **General-Purpose Foundation**: Eventually viable as a general language

### 1.2 Design Principles

| Principle | Description |
|-----------|-------------|
| **Safe by Default** | Memory safety guaranteed unless explicitly marked unsafe |
| **Ruby-like Readability** | Code reads like natural English sentences |
| **Expressive > Concise** | Clear intent over clever one-liners |
| **Convention over Configuration** | Sensible defaults reduce boilerplate |
| **Domain-First** | Industrial control features built-in from day 1 |

### 1.3 Target Domains (Phased)

| Phase | Domain | Description |
|-------|--------|-------------|
| v0.1 | Industrial Control | SCADA, DCS, PLC programming |
| v0.2 | Systems Programming | Embedded, OS components |
| v0.5 | General Purpose | Web, CLI tools, applications |
| v1.0 | Full-Featured | Comparable to Rust feature set |

---

## 2. Syntax Design (Ruby-Inspired)

### 2.1 Core Philosophy: "Ruby Syntax, Rust Safety"

AZC adopts Ruby's natural syntax while enforcing Rust's safety guarantees at compile time.

### 2.2 Basic Types and Variables

```ruby
# Ruby-like variable declaration (type inference)
name = "AZC"
age = 5
pi = 3.14159
is_safe = true

# Explicit type annotation (optional, like Rust)
count: Int = 42
message: String = "hello"
```

### 2.2.1 Type Annotations

AZC uses colon-based type annotations (Ruby 3.x style):

```ruby
# Variable with type
name: String = "AZC"

# Type is optional (inference default)
name = "AZC"  # inferred as String
```

### 2.3 Functions

```ruby
# Function definition (Ruby-like, no fn keyword)
def greet(name)
  "Hello, #{name}!"
end

# With return type (Rust-like)
def add(a: Int, b: Int) -> Int
  a + b
end

# With type annotations on parameters
def process(data: Array<Int>) -> Result<Int, String>
  if data.empty?
    Err("No data")
  else
    Ok(data.sum)
  end
end

# Block-based iteration (Ruby-like)
def process_items(items, &block)
  items.each { |item| block.call(item) }
end
```

### 2.4 Control Flow

```ruby
# If/unless modifiers (Ruby-style)
return "valid" if age >= 18
return "invalid" unless age >= 0

# Traditional if (Ruby supports both)
if temperature > 30
  "hot"
elsif temperature > 20
  "warm"
else
  "cold"
end

# Case statement (like switch)
case status
when :idle   then "Waiting..."
when :running then "Processing..."
when :error  then "Failed!"
else              "Unknown"
end

# Unless (inverse if)
unless connected?
  reconnect()
end

# Loops (Ruby-style)
while running?
  process_tick()
end

# Iterator-based (preferred)
10.times { |i| puts i }
(1..5).each { |n| puts n }
```

### 2.5 Classes and Objects

```ruby
# Class definition (Ruby-like)
class Point
  attr_accessor :x, :y
  
  def initialize(x = 0, y = 0)
    @x = x
    @y = y
  end
  
  def distance_from_origin
    Math.sqrt(@x * @x + @y * @y)
  end
  
  # Method overloading via pattern matching (AZC extension)
  def scale(factor: Int)
    Point.new(@x * factor, @y * factor)
  end
  
  def scale(factor: Float)
    Point.new(@x * factor, @y * factor)
  end
end

# Inheritance
class Point3D < Point
  attr_accessor :z
  
  def initialize(x = 0, y = 0, z = 0)
    super(x, y)
    @z = z
  end
end
```

### 2.6 Blocks and Closures (Ruby-Core Feature)

```ruby
# Block with braces
[1, 2, 3].map { |n| n * 2 }  # => [2, 4, 6]

# Block with do...end
[1, 2, 3].each do |n|
  puts "Number: #{n}"
end

# Lambda expressions
add = ->(a, b) { a + b }
double = ->(n) { n * 2 }

# Method with block parameter
def transform(values, &block)
  values.map { |v| block.call(v) }
end

# Symbol to proc (Ruby feature)
["1", "2", "3"].map(&:to_i)  # => [1, 2, 3]
```

### 2.7 Collections

```ruby
# Arrays
numbers = [1, 2, 3, 4, 5]
first = numbers[0]
last = numbers[-1]
slice = numbers[1..3]

# Hashes (dictionaries)
config = { host: "localhost", port: 8080 }
value = config[:host]

# Ranges
range = 1..10
half_open = 1...10
```

### 2.8 Pattern Matching (Modern Ruby + Rust)

```ruby
# Match expression (Ruby 3.1+ style)
def process(value)
  match value
  in Integer if value > 0
    "Positive number: #{value}"
  in Integer
    "Non-positive: #{value}"
  in String
    "String of length #{value.length}"
  in [first, *rest]
    "Array starting with #{first}"
  in nil
    "Nothing provided"
  end
end
```

### 2.9 Safety Annotations (Rust-inspired)

```ruby
# Reference parameters (Rust borrow)
def modify(ptr: &mut Point)
  ptr.x += 1
end

# Ownership transfer
def consume(point: Point)
  puts "Consumed: #{point.x}, #{point.y}"
end

# Lifetimes (simplified)
def extract<'a>(data: &'a String) -> &'a str
  data.as_str()
end
```

### 2.10 Unsafe Blocks (Limited)

```ruby
# Unsafe code requires explicit marker
unsafe
  # Direct memory access
  pointer = raw_load(0x1000, Int)
  
  # Call unsafe FFI
  call_ffi_function()
end

# Unsafe function requires unsafe call
unsafe
  process_unsafe()
end
```

---

## 3. Type System

### 3.1 Type Categories

```
┌─────────────────────────────────────────────────────────┐
│                    AZC Type System                       │
├─────────────────────────────────────────────────────────┤
│  Primitive Types     │  i8, i16, i32, i64, i128, isize │
│                       │  u8, u16, u32, u64, u128, usize  │
│                       │  f32, f64, bool, char           │
├─────────────────────────────────────────────────────────┤
│  Composite Types     │  Array<T, N>                    │
│                       │  Tuple(T1, T2, ...)            │
│                       │  Struct { field: T }           │
│                       │  Enum { A, B(T), C { f: T } }  │
├─────────────────────────────────────────────────────────┤
│  Reference Types     │  &T (shared reference)         │
│                       │  &mut T (mutable reference)     │
│                       │  &str (string slice)           │
├─────────────────────────────────────────────────────────┤
│  Pointer Types       │  *const T (unsafe const)        │
│                       │  *mut T (unsafe mutable)       │
├─────────────────────────────────────────────────────────┤
│  Special Types       │  Option<T> (nullable)           │
│                       │  Result<T, E> (error handling) │
│                       │  Never (unreachable)           │
├─────────────────────────────────────────────────────────┤
│  Custom Types        │  Trait (interface)              │
│                       │  Class (Ruby-style)             │
│                       │  Protocol (structural typing)   │
└─────────────────────────────────────────────────────────┘
```

### 3.2 Type Inference

AZC uses local type inference (similar to Rust's `let` inference):

```ruby
# Type inferred from right-hand side
x = 10           # => i32
y = "hello"      # => String
z = [1, 2, 3]    # => Array<i32>

# Context-based inference
def process(x)
  # x inferred from usage
  y = x + 1      # x must support +
  z = x.to_s     # x must support to_s
end
```

### 3.3 Trait System (Rust-Inspired)

```ruby
# Trait definition
trait Printable
  def print(&stream)
  
  def println(&stream)
    print(stream)
    stream.write("\n")
  end
end

# Implement trait
class Point
  implements Printable
  
  def initialize(@x, @y)
  end
  
  def print(&stream)
    stream.write("Point(#{@x}, #{@y})")
  end
end

# Trait bounds
def print_all<T: Printable>(items: Array<T>)
  items.each { |item| item.println() }
end

# Default implementations
trait Default
  def self.default
    # Default implementation
  end
end
```

### 3.4 Protocol (Structural Typing - Ruby Duck Typing)

```ruby
# Protocol for duck typing
protocol Quack
  def quack: () -> String
end

# Structural typing - any type with quack method
class Duck
  def quack
    "Quack!"
  end
end

class Person
  def quack
    "I'm pretending to quack!"
  end
end

# Works with any Quack
def make_it_quack(thing: Quack)
  thing.quack
end
```

### 3.5 Null Safety (Option/Result)

```ruby
# Option<T> - nullable values
def find_user(id: Int) -> Option<User>
  # Returns Some(user) or None
end

# Safe access with pattern matching
user = find_user(123)
match user
in Some(u)
  puts "Found: #{u.name}"
in None
  puts "User not found"
end

# Safe navigation operator (Ruby-style)
name = user&.name  # nil if user is nil

# Result<T, E> - error handling
def read_file(path: String) -> Result<String, IOError>
  # Returns Ok(content) or Err(error)
end

# ? operator for propagation
def process
  content = read_file("data.txt")?
  # Early return on error
  parse(content)?
end
```

---

## 4. Ownership and Borrowing

### 4.1 Core Model

AZC adopts Rust's ownership model with simplified syntax:

```
┌────────────────────────────────────────────────────────────┐
│                   Ownership Rules                           │
├────────────────────────────────────────────────────────────┤
│ 1. Each value has a single owner                           │
│ 2. When owner goes out of scope, value is dropped          │
│ 3. References (&T) provide borrow without ownership       │
│ 4. Mutable references (&mut T) allow mutation            │
│ 5. Only ONE mutable reference OR many shared refs          │
└────────────────────────────────────────────────────────────┘
```

### 4.2 Ownership Syntax

```ruby
# Ownership transfer (move)
def consume(point: Point)
  # point is moved into this function
end

p = Point.new(1, 2)
consume(p)  # p is moved, no longer valid

# Borrowing
def print_point(point: &Point)
  # &Point - borrowed reference
  puts "(#{point.x}, #{point.y})"
end

p = Point.new(1, 2)
print_point(&p)  # borrow p
# p still valid after call

# Mutable borrow
def modify(point: &mut Point)
  point.x += 1
end

modify(&mut p)
```

### 4.3 Lifetime Elision (Simplified)

AZC simplifies lifetimes with smart defaults:

```ruby
# These are equivalent
def get_first(s: &String) -> &str
  s.get(0..1)
end

# Compiler infers lifetimes automatically
def process(data: &str) -> &str
  data.trim()
end
```

### 4.4 Move Semantics

```ruby
# By default, assignments move
a = Point.new(1, 2)
b = a           # a is moved to b
# a is now invalid!

# Clone for copying
b = a.clone()  # creates a copy

# Reference for borrowing
b = &a         # b is a reference to a
```

### 4.5 Borrowing Rules (Compile-Time Enforced)

```ruby
# This won't compile:
p = Point.new(1, 2)
q = &mut p     # mutable borrow
r = &p         # shared borrow - ERROR!

# Must use in sequence:
p = Point.new(1, 2)
q = &mut p
q.x = 10
# q goes out of scope here
r = &p         # OK now
```

---

## 5. Industrial Control Features

### 5.1 Built-in Types for ICS

```ruby
# PLC-style data types
# Bit-level operations
bit: Bit = 1
word: Word = 0xABCD  # 16-bit unsigned

# Industrial ranges
temperature: Int<0, 150>    # 0-150 degrees
pressure: Float<0.0, 1000.0> # bounded float

# Timestamp (IEC 61512)
timestamp: DateTime

# Duration (IEC 61512)
duration: Duration
```

### 5.2 Safety Annotations (IEC 61508)

```ruby
# Safety integrity level
@sil_level(2)
def emergency_stop()
  # SIL2 compliant code
end

# Deterministic timing
@deterministic
def control_loop(cycle_time: Milliseconds(10))
  # Must complete within 10ms
end
```

### 5.3 Modbus/OPC-UA Primitives

```ruby
# Register mapping
register HoldingRegister(40001) as Int
register InputRegister(30001) as Float
register Coil(00001) as Bit

# Read/write
value = HoldingRegister.read()
HoldingRegister.write(100)

# OPC-UA node
node "ns=2;i=2051" as temperature_sensor: Float
```

### 5.4 Real-time Extensions

```ruby
# Task scheduling (IEC 61131-3)
task control_task(interval: 100.ms, priority: 10)
  def execute
    # Cyclic execution
  end
end

# Interrupt handler
interrupt handler for timer_1
  # Handle interrupt
end

# Shared variables (deterministic access)
@shared
var critical_value: Int

@atomic
def update_critical(value: Int)
  critical_value = value
end
```

---

## 6. Compiler Architecture

### 6.1 Compilation Pipeline

```
┌──────────────────────────────────────────────────────────────────┐
│                    AZC Compilation Pipeline                       │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Source ──► Lexer ──► Parser ──► AST ──► HIR ──► MIR ──► LLVM   │
│                         │         │       │      │          │     │
│                         ▼         ▼       ▼      ▼          ▼     │
│                    Errors     Type    Type   Borrow      Object  │
│                              Infer   Check  Check       Code    │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                    Error Reporting                          │ │
│  │  - Syntax errors (parser)                                    │ │
│  │  - Type errors (type checker)                                │ │
│  │  - Borrow errors (borrow checker)                            │ │
│  │  - Safety violations (safety checker)                        │ │
│  └─────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
```

### 6.2 Compiler Stages

| Stage | Input | Output | Key Functions |
|-------|-------|--------|---------------|
| Lexer | Source text | Tokens | Tokenize identifiers, keywords, literals |
| Parser | Tokens | AST | Build syntax tree, handle precedence |
| HIR Builder | AST | HIR | Desugar, resolve names |
| Type Inference | HIR | Typed HIR | Propagate types, resolve traits |
| Borrow Checker | Typed HIR | Validated MIR | Enforce ownership rules |
| MIR Builder | Validated HIR | MIR | SSA form, optimizations |
| Code Gen | MIR | LLVM IR | Emit LLVM bitcode |
| LLVM | LLVM IR | Object | Compile to machine code |

### 6.3 Core Compiler Modules

```
compiler/azc/
├── azc_driver/          # Main compiler driver
├── azc_lexer/           # Tokenization
│   └── Lexer, Token, TokenKind
├── azc_parser/          # Syntax analysis
│   ├── Parser, AstNode
│   └── Grammar rules
├── azc_ast/             # AST definitions
│   ├── Expr, Stmt, Decl
│   └── Visitor pattern
├── azc_hir/             # High-level IR
│   ├── HirNode, HirDef
│   └── Name resolution
├── azc_typeck/          # Type checking
│   ├── TypeEnv, TypeScheme
│   ├── Trait solving
│   └── Type inference
├── azc_borrowck/        # Borrow checking
│   ├── BorrowSet, BorrowData
│   ├── Region inference
│   └── Conflict detection
├── azc_mir/             # Mid-level IR
│   ├── BasicBlock, Instruction
│   └── Control flow
├── azc_codegen/         # Code generation
│   ├── LLVM backend
│   └── Target support
└── azc_errors/          # Error reporting
    ├── Diagnostic
    └── Span tracking
```

### 6.4 Key Data Structures (Rust-Inspired)

```ruby
# Based on rustc's design

# Source location
class Span
  file: FileId
  start: BytePos
  end: BytePos
  ctxt: SyntaxContext
end

# Definition identifier
class DefId
  crate: CrateNum
  index: DefIndex
end

# Type representation
class Ty
  kind: TyKind
  flags: TypeFlags
end

enum TyKind
  # Primitives
  Bool
  Int(IntTy)
  Uint(UintTy)
  Float(FloatTy)
  Char
  Str
  
  # Compounds
  Array(Ty, Option<Const>)
  Tuple(Array<Ty>)
  Struct(DefId, Array<GenericArg>)
  Enum(DefId, Array<GenericArg>)
  
  # References
  Ref(Ty, Mutability)
  RawPtr(Ty, Mutability)
  
  # Functions
  FnPtr(Array<Ty>, Abi)
  FnDef(DefId)
  
  # Special
  Closure(Ty, Ty)
  Never
  Error
end

# Borrow data
class BorrowData
  place: Place
  kind: BorrowKind  # Shared, Mut, TwoPhase
  region: Region
  location: Location
end

# Place in memory
class Place
  local: Local
  projection: Array<ProjectionElem>
end
```

---

## 7. Minimal Viable Product (MVP) Scope

### 7.1 v0.1.0 Features

| Feature | Status | Description |
|---------|--------|-------------|
| Basic types | ✅ | i32, f64, bool, String |
| Variables | ✅ | let, const, mutability |
| Functions | ✅ | def, parameters, return |
| Control flow | ✅ | if, unless, while, for |
| Classes | ✅ | class, inheritance, methods |
| Arrays | ✅ | Array<T>, indexing |
| Ownership | ⚠️ | Basic move semantics |
| Borrowing | ⚠️ | & references |
| Type inference | ⚠️ | Local inference |
| Error handling | ⚠️ | Result, Option |
| FFI | ❌ | Not in MVP |
| Unsafe | ❌ | Not in MVP |
| Traits | ❌ | Not in MVP |
| Generics | ❌ | Not in MVP |
| ICS features | ❌ | Phase 2 |

### 7.2 MVP Syntax Subset

```ruby
# Working MVP examples

# Hello world
puts "Hello, AZC!"

# Variables and types
x = 10
name = "AZC"
is_ready = true

# Functions
def add(a, b)
  a + b
end

# Control flow
if x > 5
  puts "Large"
else
  puts "Small"
end

# Loops
i = 0
while i < 10
  puts i
  i += 1
end

# Arrays
numbers = [1, 2, 3, 4, 5]
sum = numbers.reduce(0) { |acc, n| acc + n }

# Classes
class Counter
  def initialize
    @count = 0
  end
  
  def increment
    @count += 1
  end
  
  def value
    @count
  end
end

c = Counter.new
c.increment()
puts c.value()
```

---

## 8. Implementation Roadmap

### Phase 1: Foundation (v0.1.x)
- [ ] Lexer implementation
- [ ] Parser for MVP syntax
- [ ] AST representation
- [ ] Basic type inference
- [ ] Code generation to LLVM

### Phase 2: Safety (v0.2.x)
- [ ] Full ownership model
- [ ] Borrow checker
- [ ] Lifetime inference
- [ ] Move semantics

### Phase 3: Features (v0.3.x)
- [ ] Trait system
- [ ] Generics
- [ ] Pattern matching
- [ ] Error handling

### Phase 4: Industrial (v0.4.x)
- [ ] ICS data types
- [ ] Safety annotations
- [ ] Deterministic scheduling
- [ ] Modbus/OPC-UA support

### Phase 5: Production (v0.5.x)
- [ ] Optimizations
- [ ] Cross-platform
- [ ] Standard library
- [ ] Package manager

---

## 9. Design Decisions and Trade-offs

### 9.1 Ruby vs Rust Balance

| Decision | Rationale |
|----------|-----------|
| Use `def` instead of `fn` | Ruby compatibility |
| Optional type annotations | Developer choice |
| `&` for references | Familiar syntax |
| Blocks as first-class | Core Ruby feature |
| Enforce safety at compile | Rust guarantee |

### 9.2 Simplifications from Rust

| Rust Feature | AZC Approach |
|--------------|---------------|
| Complex lifetimes | Simplified elision |
| Generic bounds | Trait constraints only |
| Unsafe by default | Limited unsafe |
| Macro system | Phases later |
| Procedural macros | Phase 3+ |

### 9.3 Industrial Focus

| Feature | Priority |
|---------|----------|
| Deterministic timing | High |
| Safety integrity levels | High |
| PLC data types | High |
| Real-time constraints | High |
| FFI to industrial protocols | Medium |

---

## 10. Open Questions for Iteration

1. **Ownership syntax**: Should AZC use `&mut` (Rust) or `.mut` (Ruby-idiomatic)?
2. **Null handling**: Full Option<T> or allow nil with `&.` operator?
3. **Class vs Trait**: How to unify Ruby classes with Rust traits?
4. **Garbage collection**: Consider GC for cyclic data structures?
5. **Concurrency**: Actor model (Erlang) or Ownership (Rust)?

---

## Appendix A: Syntax Comparison

### Ruby vs AZC

```ruby
# Ruby
def greet(name)
  "Hello, #{name}!"
end

# AZC (similar)
def greet(name)
  "Hello, #{name}!"
end

# AZC with types (optional)
def greet(name: String) -> String
  "Hello, #{name}!"
end
```

### Rust vs AZC

```rust
// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// AZC
def add(a: Int, b: Int) -> Int
  a + b
end

// Or Ruby-style
def add(a, b)
  a + b
end
```

---

*Document Status: Initial Design - Subject to Iteration*
*Created: 2026-03-11*
*Version: 0.1.0-draft*
