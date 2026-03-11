# AZC Language Development Plan
## Next Steps to a Real Safety Language

> **Mission**: Build a programming language that makes safety-critical systems impossible to get wrong. Target: Industrial Control Systems (SCADA/DCS) where bugs can cost lives.

---

## Current Status (v0.1.0 - MVP)

### What We Have
- ✅ Ruby-like syntax parser
- ✅ Basic control flow (if/else, while)
- ✅ Functions and variables
- ✅ Transpilation to C
- ✅ Basic test suite

### What We Don't Have (Yet)
- ❌ Type system
- ❌ Ownership and borrowing
- ❌ Memory safety guarantees
- ❌ Compile-time error detection
- ❌ Industrial control features

---

## Development Phases

### Phase 1: Type System (v0.2.0)
**Timeline**: 2-3 weeks  
**Priority**: Critical - Foundation for all safety features

#### 1.1 Type Architecture

```
Types in AZC:
├── Primitive Types
│   ├── Int (i8, i16, i32, i64, i128)
│   ├── UInt (u8, u16, u32, u64, u128)
│   ├── Float (f32, f64)
│   ├── Bool
│   ├── Char
│   └── String
├── Compound Types
│   ├── Array<T, N>
│   ├── Tuple(T1, T2, ...)
│   └── Struct
├── Reference Types
│   ├── &T (immutable borrow)
│   ├── &mut T (mutable borrow)
│   ├── Box<T> (heap allocation)
│   └── Rc<T> (reference counted)
└── Function Types
    └── fn(Args...) -> Return
```

#### 1.2 Type Inference Engine

Implement Hindley-Milner type inference with extensions:

```ruby
# AZC code
let x = 42          # infers Int
let y = 3.14        # infers Float
let name = "AZC"    # infers String
let flag = true     # infers Bool

# Type annotations (optional)
let count: Int = 100
let pi: Float = 3.14159

# Function types
def add(a: Int, b: Int) -> Int
  a + b
end

# Generic functions (future)
def first<T>(arr: Array<T>) -> T
  arr[0]
end
```

#### 1.3 Implementation Steps

1. **Define Type AST** (`compiler/src/types/ast.rs`)
   - Type enum for all supported types
   - TypeVariable for inference
   - TypeScheme for polymorphism

2. **Type Environment** (`compiler/src/types/env.rs`)
   - Symbol table with types
   - Scope management
   - Type bindings

3. **Type Inference** (`compiler/src/types/inference.rs`)
   - Unification algorithm
   - Constraint generation
   - Constraint solving

4. **Type Checking** (`compiler/src/types/checker.rs`)
   - Expression type checking
   - Statement type checking
   - Error reporting

#### 1.4 Deliverables

- [ ] Type system implementation
- [ ] Type inference for basic expressions
- [ ] Type annotations support
- [ ] Type error messages
- [ ] Test suite for types

---

### Phase 2: Ownership Model (v0.2.1)
**Timeline**: 2-3 weeks  
**Priority**: Critical - Core safety feature

#### 2.1 Ownership Rules

```
AZC Ownership Rules (borrowed from Rust):

1. Each value has a single owner
2. When owner goes out of scope, value is dropped
3. Values can be borrowed (immutable or mutable)
4. Multiple immutable borrows OR one mutable borrow
5. No dangling references
```

#### 2.2 Implementation in Ruby-like Syntax

```ruby
# Ownership transfer
let x = Box.new(42)    # x owns the heap value
let y = x              # ownership moves to y, x is invalid

# Explicit copy
let a = 10
let b = a.clone()      # explicit copy

# Borrowing
def process(data: &String)  # immutable borrow
  puts data
end

def modify(data: &mut String)  # mutable borrow
  data = "modified"
end

# Lifetimes (simplified)
def longest<'a>(s1: &'a String, s2: &'a String) -> &'a String
  if s1.len > s2.len
    s1
  else
    s2
  end
end
```

#### 2.3 Implementation Steps

1. **Ownership Analysis** (`compiler/src/ownership/analyzer.rs`)
   - Track value ownership
   - Identify moves and copies
   - Scope-based drop insertion

2. **Borrow Checker** (`compiler/src/ownership/borrow.rs`)
   - Track active borrows
   - Check borrow rules
   - Detect dangling references

3. **Lifetime Analysis** (`compiler/src/ownership/lifetime.rs`)
   - Infer lifetimes
   - Check lifetime constraints
   - Prevent use-after-free

#### 2.4 Deliverables

- [ ] Ownership tracking
- [ ] Move semantics
- [ ] Borrow checker
- [ ] Lifetime inference
- [ ] Safety error messages

---

### Phase 3: Memory Safety (v0.2.2)
**Timeline**: 1-2 weeks  
**Priority**: Critical

#### 3.1 Safety Guarantees

```
Memory Safety Guarantees:

✓ No null pointer dereferences
✓ No dangling pointers
✓ No buffer overflows
✓ No use-after-free
✓ No double-free
✓ No data races (with concurrency)
✓ No uninitialized memory
```

#### 3.2 Implementation

1. **Option Type** (no nulls)
```ruby
# No null, use Option
let maybe: Option<Int> = Some(42)
let none: Option<Int> = None

# Pattern matching
match maybe
when Some(val)
  puts val
when None
  puts "No value"
end
```

2. **Bounds Checking**
```ruby
let arr = [1, 2, 3]
let x = arr[5]  # Compile error: index out of bounds
```

3. **Memory Layout**
- Stack allocation by default
- Box<T> for heap
- Deterministic destruction

#### 3.3 Deliverables

- [ ] Option type implementation
- [ ] Result type for errors
- [ ] Bounds checking
- [ ] Safe memory operations
- [ ] Documentation

---

### Phase 4: Industrial Control Features (v0.3.0)
**Timeline**: 3-4 weeks  
**Priority**: High - Domain-specific features

#### 4.1 SCADA/DCS Data Types

```ruby
# Industrial Control Types
type AnalogInput = Float      # 4-20mA, 0-10V
type AnalogOutput = Float
type DigitalInput = Bool
type DigitalOutput = Bool

# Safety-critical types
type SIL1<T> = T  # Safety Integrity Level 1
type SIL2<T> = T  # Safety Integrity Level 2
type SIL3<T> = T  # Safety Integrity Level 3
type SIL4<T> = T  # Safety Integrity Level 4

# Process variables
type Temperature = Float
type Pressure = Float
type FlowRate = Float
type Level = Float
```

#### 4.2 Real-time Features

```ruby
# Timing guarantees
@deadline(100ms)
def emergency_shutdown()
  # Must complete within 100ms
  close_valve(V1)
  close_valve(V2)
  alarm(A1, true)
end

# Periodic tasks
@periodic(50ms)
def read_sensors()
  temp = read_ai(TIC101)
  pressure = read_ai(PIC102)
end

# Watchdog
@watchdog(1s)
def heartbeat()
  send_signal(WD1)
end
```

#### 4.3 Communication Protocols

```ruby
# Modbus TCP
modbus_client = ModbusTCP.new("192.168.1.100", 502)
temp = modbus_client.read_holding_register(40001)

# OPC-UA
opc = OPCUA.new("opc.tcp://server:4840")
node = opc.get_node("ns=2;s=Temperature")
value = node.read()
```

#### 4.4 Deliverables

- [ ] Industrial data types
- [ ] Safety annotations
- [ ] Real-time task support
- [ ] Protocol bindings
- [ ] Example programs

---

### Phase 5: Advanced Features (v0.4.0)
**Timeline**: 4-6 weeks  
**Priority**: Medium

#### 5.1 Trait System

```ruby
# Traits (like Rust)
trait Display
  def to_string(self) -> String
end

trait Serialize
  def to_json(self) -> String
  def from_json(json: String) -> Self
end

# Implementation
struct Point
  x: Float
  y: Float
end

impl Display for Point
  def to_string(self) -> String
    "Point(\{self.x}, \{self.y})"
  end
end
```

#### 5.2 Pattern Matching

```ruby
# Exhaustive pattern matching
match value
when 0
  puts "zero"
when 1..10
  puts "small"
when n if n > 100
  puts "large"
else
  puts "other"
end

# Destructuring
let point = Point.new(1.0, 2.0)
match point
when Point(x, y)
  puts "x: \{x}, y: \{y}"
end
```

#### 5.3 Error Handling

```ruby
# Result type
def divide(a: Int, b: Int) -> Result<Int, String>
  if b == 0
    Err("division by zero")
  else
    Ok(a / b)
  end
end

# Error propagation
let result = divide(10, 2)?
puts result  # 5

# Try-catch alternative
let result = try
  risky_operation()
catch e
  handle_error(e)
end
```

#### 5.4 Concurrency

```ruby
# Safe concurrency with ownership
async def fetch_data() -> String
  # Async operation
end

# Message passing
channel = Channel.new(10)
spawn do
  channel.send("message")
end
msg = channel.recv()
```

---

### Phase 6: Standard Library (v0.5.0)
**Timeline**: 4-6 weeks  
**Priority**: Medium

#### 6.1 Core Library

```
std/
├── collections/
│   ├── Vec<T>
│   ├── HashMap<K, V>
│   ├── HashSet<T>
│   └── LinkedList<T>
├── io/
│   ├── File
│   ├── Stdin
│   ├── Stdout
│   └── Stderr
├── net/
│   ├── TcpStream
│   ├── TcpListener
│   └── UdpSocket
├── thread/
│   ├── Thread
│   ├── Mutex<T>
│   └── Arc<T>
├── time/
│   ├── Duration
│   └── Instant
└── industrial/
    ├── Modbus
    ├── OPCUA
    └── DNP3
```

---

### Phase 7: Tooling (v0.6.0)
**Timeline**: 4-6 weeks  
**Priority**: Medium

#### 7.1 Package Manager (azc-pkg)

```bash
# Initialize project
azc init my_project

# Add dependency
azc add modbus

# Build
azc build

# Run tests
azc test

# Documentation
azc doc
```

#### 7.2 IDE Support

- VSCode extension
- Language server (LSP)
- Syntax highlighting
- Autocompletion
- Type hints
- Error highlighting

#### 7.3 Documentation

- Language reference
- Standard library docs
- Tutorial series
- Safety guidelines
- Industrial examples

---

## Implementation Priority Order

### Immediate (Next 4 weeks)

1. **Type System** - Foundation for all safety
   - Type inference
   - Type checking
   - Error messages

2. **Ownership Model** - Core safety
   - Move semantics
   - Borrow checker
   - Lifetime analysis

3. **Memory Safety** - Guarantees
   - Option type
   - Bounds checking
   - Safe operations

### Short-term (1-3 months)

4. **Industrial Features** - Domain-specific
   - SCADA types
   - Safety annotations
   - Protocols

5. **Trait System** - Abstraction
   - Traits
   - Implementations
   - Generics

### Medium-term (3-6 months)

6. **Standard Library** - Ecosystem
   - Collections
   - I/O
   - Networking

7. **Tooling** - Developer experience
   - Package manager
   - IDE support
   - Documentation

### Long-term (6-12 months)

8. **Advanced Features**
   - Concurrency
   - Async/await
   - Macros

9. **Production Ready**
   - Optimization
   - Cross-platform
   - Certification

---

## Success Metrics

### Safety Metrics
- [ ] Zero memory safety bugs in test suite
- [ ] All ownership violations caught at compile time
- [ ] No undefined behavior possible in safe code

### Functionality Metrics
- [ ] Pass Rust's borrow checker tests (adapted)
- [ ] Type inference for all common patterns
- [ ] Industrial protocol support

### Quality Metrics
- [ ] 100% test coverage for core features
- [ ] Comprehensive documentation
- [ ] Example programs for all features

---

## Risk Mitigation

### Technical Risks

1. **Complexity of Borrow Checker**
   - Risk: Borrow checking is complex, may have bugs
   - Mitigation: Start simple, extensive testing, reference Rust implementation

2. **Performance**
   - Risk: Compile times may be slow
   - Mitigation: Incremental compilation, caching

3. **Adoption**
   - Risk: Learning curve for ownership
   - Mitigation: Great error messages, gradual typing, tutorials

### Safety Risks

1. **Undetected Unsafe Code**
   - Risk: Unsafe blocks may introduce bugs
   - Mitigation: Minimize unsafe, audit all unsafe code

2. **Specification Errors**
   - Risk: Language spec may have holes
   - Mitigation: Formal verification where possible, extensive testing

---

## Resources Needed

### Development Team
- Language designers (2-3)
- Compiler engineers (3-4)
- Safety experts (1-2)
- Industrial domain experts (1-2)

### Infrastructure
- CI/CD pipeline
- Test infrastructure
- Documentation hosting
- Package registry

### Community
- Early adopters
- Industrial partners
- Academic collaborators

---

## Conclusion

AZC has the potential to revolutionize safety-critical programming by making safe code the default. By combining Rust's proven safety model with Ruby's elegant syntax, we can create a language that is both powerful and accessible.

The path is clear:
1. **Type System** → Foundation
2. **Ownership** → Core Safety
3. **Industrial Features** → Domain Value
4. **Ecosystem** → Adoption

Every step brings us closer to a world where safety-critical systems are built on a foundation of provable safety, not just testing and good intentions.

**Let's build the future of safe programming.**

---

*Last Updated: 2025-03-11*  
*Version: 1.0*