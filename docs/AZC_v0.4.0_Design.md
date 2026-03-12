# AZC v0.4.0 Advanced Features Design Document

## Overview

This document outlines the design for four major features in AZC v0.4.0:
1. **Async/await** - Asynchronous programming with Future types
2. **Macros** - Compile-time metaprogramming
3. **Unsafe blocks** - Escape hatches for low-level operations
4. **FFI** - Foreign Function Interface for C interop

---

## 1. Async/Await

### 1.1 AST Node Definitions

```rust
// New variants in Statement enum
Statement::AsyncFunction {
    name: String,
    type_params: Vec<String>,
    params: Vec<(String, Option<String>)>,
    return_type: Option<String>,
    body: Vec<Statement>,
}

// New variants in Expression enum
Expression::Await(Expression),           // Await a future
Expression::AsyncBlock(Vec<Statement>), // Async block
Expression::Yield(Option<Expression>),   // Yield (for generators)
```

### 1.2 Type System Additions

```rust
// In types/ast.rs - new Type variants
pub enum Type {
    // ... existing variants ...
    
    // Async types
    Future {
        ret: Box<Type>,  // The resolved type
    },
    AsyncFunction {
        params: Vec<Type>,
        ret: Box<Type>,
    },
}

// New built-in types
pub enum BuiltinTypes {
    Future,      // Future<T>
    Promise,     // Promise<T> (for async operations)
}
```

### 1.3 Parser Modifications

**Keywords to add:** `async`, `await`, `yield`

**Syntax examples:**
```ruby
# Async function
async def fetch_data(url: String) -> Future<String>
    let response = await http_get(url)
    response.body
end

# Async block
let result = async {
    let data = await fetch_data(url)
    process(data)
}

# Await expression
let data = await future_value

# Async lambda
let handler = async |req| -> Future<Response> {
    handle_request(req).await
}
```

### 1.4 Code Generation Strategy

**Rust target (async/await):**
- Convert async functions to Rust async functions
- Generate state machine for await points
- Use `futures::Future` or custom `Future` trait

**C target:**
- Generate state machine with explicit context
- Use callback-based async I/O
- Generate `poll()` function for each async function

**Runtime support:**
```rust
// Standard library additions
pub trait Future {
    type Output;
    fn poll(&mut self, cx: &Context) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct Context {
    waker: Waker,
}
```

### 1.5 Example AZC Code

```ruby
# Async function with error handling
async def fetch_user(id: Int) -> Future<Result<User, Error>>
    let response = await http_get("/users/#{id}")
    
    if response.status == 200
        Result::Ok(parse_user(response.body)?)
    else
        Result::Err(Error::NetworkError(response.status))
    end
end

# Concurrent async operations
async def fetch_all_data() -> Future<(User, Posts)>
    let user_future = fetch_user(1)
    let posts_future = fetch_posts(1)
    
    let user = await user_future
    let posts = await posts_future
    
    (user, posts)
end

# Parallel async operations using join!
async def parallel_fetch() -> Future<(A, B)>
    join!(fetch_a(), fetch_b())
end
```

---

## 2. Macros

### 2.1 AST Node Definitions

```rust
// New variants in Statement enum
Statement::Macro {
    name: String,
    params: Vec<MacroParam>,
    body: Vec<MacroToken>,
    is_rule: bool,  // For macro_rules! style
}

// New variants in Expression enum
Expression::MacroCall {
    name: String,
    args: Vec<MacroArg>,
    delimiter: MacroDelimiter,  // parentheses, brackets, braces
}

// Supporting types
#[derive(Debug, Clone)]
pub struct MacroParam {
    pub name: String,
    pub kind: MacroParamKind,  // Ident, Block, Expr, Type, Pat, Stmt
    pub is_repeat: bool,       // For $x...
}

#[derive(Debug, Clone)]
pub enum MacroParamKind {
    Ident,
    Expr,
    Block,
    Stmt,
    Type,
    Pat,
    Item,  // Top-level item
    Meta,  // Attribute-like
}

#[derive(Debug, Clone)]
pub enum MacroDelimiter {
    Parenthesis,
    Bracket,
    Brace,
}

#[derive(Debug, Clone)]
pub enum MacroArg {
    TokenTree(TokenTree),
    Fragment(MacroFragment),
}

#[derive(Debug, Clone)]
pub enum MacroFragment {
    Expr(Expression),
    Ident(String),
    Pat(Pattern),
    Type(TypeNode),
    Stmt(Statement),
    Item(Statement),
}

// Token tree for macro expansion
#[derive(Debug, Clone)]
pub enum TokenTree {
    Token(String),
    Ident(String),
    Punct(char),
    Delimiter(MacroDelimiter, Vec<TokenTree>),
    Concat(Vec<TokenTree>),
    Repeat(RepeatKind, Vec<TokenTree>),
}

#[derive(Debug, Clone)]
pub enum RepeatKind {
    ZeroOrOne,  // ?
    ZeroOrMore, // *
    OneOrMore,  // +
}
```

### 2.2 Type System Additions

```rust
// Macro type system (compile-time only)
pub struct MacroExpansion {
    pub original: Vec<TokenTree>,
    pub expanded: Vec<TokenTree>,
    pub hygiene: HygieneContext,
}

pub struct MacroRegistry {
    macros: HashMap<String, MacroDefinition>,
}

pub enum MacroDefinition {
    Text(MacroRule),
    Syntax(SyntaxRule),
}

// Macro rules for macro_rules!
pub struct MacroRule {
    pub matcher: Vec<TokenTree>,
    pub transcriber: Vec<TokenTree>,
}
```

### 2.3 Parser Modifications

**Keywords to add:** `macro`, `macro_rules`, `rule`

**Syntax examples:**
```ruby
# Basic macro
macro double(x)
    $(x) * 2
end

# Macro with multiple patterns
macro match_expr(expr, pattern)
    match $(expr)
    when $(pattern) => true
    else => false
    end
end

# Macro_rules! style
macro_rules! my_macro {
    ($x:expr) => { $x * 2 };
    ($x:expr, $y:expr) => { $x + $y };
}

# Declarative macro with repetition
macro! create_adder(n)
    $(fn add_$n(x: Int) -> Int = x + $n)
end
```

### 2.4 Code Generation Strategy

**Macro expansion phases:**
1. **Parse-time**: Parse macro definition, store in registry
2. **Expansion**: Replace macro calls with expanded tokens
3. **Hygiene**: Track identifier scopes for hygiene
4. **Type checking**: Type check expanded code
5. **Code generation**: Generate code from expanded AST

**Implementation approach:**
- Implement incremental macro expansion
- Track macro expansion history for error reporting
- Support macro hygiene (avoid variable capture)
- Implement macro-by-example (MBE) matching

### 2.5 Example AZC Code

```ruby
# Macro for logging
macro log_debug(msg)
    if DEBUG
        puts "[DEBUG] $(msg)"
    end
end

# Usage
log_debug "Value: #{x}"
# Expands to: if DEBUG; puts "[DEBUG] Value: #{x}"; end

# Macro for boilerplate
macro impl_getters(struct_name, fields)
    impl $(struct_name)
        $(fields.each |f| "
        def $(f)
            @$(f)
        end
        ")
    end
end

# Usage
impl_getters(Point, ["x", "y", "z"])

# Compile-time evaluation
const SIZE = macro! { 2 + 3 }  // Evaluates to 5 at compile time
```

---

## 3. Unsafe Blocks

### 3.1 AST Node Definitions

```rust
// New variants in Statement enum
Statement::Unsafe {
    body: Vec<Statement>,
    reasons: Vec<UnsafeReason>,  // Documentation for why it's unsafe
}

Statement::UnsafeFunction {
    name: String,
    type_params: Vec<String>,
    params: Vec<(String, Option<String>)>,
    return_type: Option<String>,
    body: Vec<Statement>,
    unsafe_blocks: Vec<UnsafeReason>,
}

// New variants in Expression enum
Expression::UnsafeBlock {
    expr: Box<Expression>,
}

// Supporting types
#[derive(Debug, Clone)]
pub enum UnsafeReason {
    DereferenceRawPointer,
    CallUnsafeFunction,
    AccessUnionField,
    MarkUnsafeFunction,
    InlineAssembly,
    ExposeAddress,
    WriteToStatic,
}

#[derive(Debug, Clone)]
pub struct SafetyAnnotation {
    pub safe: bool,
    pub reasons: Vec<UnsafeReason>,
    pub lint_level: LintLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum LintLevel {
    Allow,
    Warn,
    Deny,
}
```

### 3.2 Type System Additions

```rust
// In types/ast.rs - new Type variants
pub enum Type {
    // ... existing variants ...
    
    // Unsafe types
    RawPtr {
        inner: Box<Type>,
        mutable: bool,
    },
    UnsafeFunctionPointer {
        params: Vec<Type>,
        ret: Box<Type>,
    },
}

// Safety context
pub struct SafetyContext {
    pub is_unsafe: bool,
    pub unsafe_blocks: Vec<UnsafeBlockInfo>,
}

pub struct UnsafeBlockInfo {
    pub start_line: usize,
    pub end_line: usize,
    pub reasons: Vec<UnsafeReason>,
}
```

### 3.3 Parser Modifications

**Keywords to add:** `unsafe`

**Syntax examples:**
```ruby
# Unsafe block
unsafe
    let ptr = &mut data as *mut u8
    ptr.write(42)
end

# Unsafe function
unsafe def raw_read(addr: *const u8) -> u8
    *addr
end

# Unsafe trait method
unsafe trait MyTrait
    unsafe fn dangerous_operation()
end
```

### 3.4 Code Generation Strategy

**Safety tracking:**
- Track all unsafe blocks in the AST
- Generate safety comments in output
- Emit safety attributes (#[unsafe] in Rust)

**Rust target:**
- Wrap unsafe code with `unsafe { }` blocks
- Use `unsafe` function modifiers
- Generate `#[allow(unused_unsafe)]` as needed

**C target:**
- Mark functions as unsafe in comments
- Generate `/* SAFETY: ... */` annotations
- Remove runtime safety checks in unsafe blocks

**Borrow checker modifications:**
- Allow raw pointer dereference in unsafe blocks
- Skip lifetime checks in unsafe contexts
- Track unsafe operations for safety analysis

### 3.5 Example AZC Code

```ruby
# Manual memory management
unsafe def exchange(addr: *mut Int, new_val: Int) -> Int
    let old = *addr
    *addr = new_val
    old
end

# FFI call (combined with FFI feature)
unsafe def call_c_function(ptr: *const c_char) -> Int
    extern "C" {
        strlen(ptr)
    }
end

# Union access
struct MyUnion
    union
        int_val: Int
        float_val: Float
    end
end

unsafe def use_union(u: MyUnion)
    unsafe
        u.int_val = 42
        puts u.float_val
    end
end
```

---

## 4. FFI (Foreign Function Interface)

### 4.1 AST Node Definitions

```rust
// New variants in Statement enum
Statement::Extern {
    abi: String,              // "C", "System", "Rust", etc.
    foreign_name: Option<String>,  // Name in foreign language
    name: String,
    params: Vec<(String, Option<String>)>,
    return_type: Option<String>,
    varargs: bool,
}

Statement::ExternBlock {
    abi: String,
    declarations: Vec<Statement>,
}

Statement::Import {
    path: String,
    items: Vec<ImportItem>,
    import_type: ImportType,
}

// New variants in Expression enum
Expression::CallForeign {
    func: Box<Expression>,
    abi: String,
    args: Vec<Expression>,
}

// Supporting types
#[derive(Debug, Clone)]
pub struct ForeignFunction {
    pub name: String,
    pub abi: String,
    pub params: Vec<ForeignParam>,
    pub return_type: Option<Type>,
    pub is_variadic: bool,
    pub link_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ForeignParam {
    pub name: String,
    pub ty: Type,
    pub by_ref: bool,
}

#[derive(Debug, Clone)]
pub enum ImportType {
    Function,
    Static,
    Type,
    Module,
}

#[derive(Debug, Clone)]
pub struct CTypeMapping {
    pub c_type: String,
    pub azc_type: Type,
    pub converter: Option<ConverterFunc>,
}

// Built-in type mappings
pub struct TypeConverters;
impl TypeConverters {
_to_    pub fn cazc(c_type: &str, azc_type: &Type) -> Option<String>;
    pub fn azc_to_c(azc_type: &Type, c_type: &str) -> Option<String>;
}
```

### 4.2 Type System Additions

```rust
// C-compatible types
pub enum CType {
    CChar,
    CShort,
    CInt,
    CLong,
    CLongLong,
    CFloat,
    CDouble,
    CVoid,
    CPointer(Box<CType>),
    CArray(Box<CType>, usize),
    CStruct(Vec<(String, CType)>),
    CUnion(Vec<(String, CType)>),
}

// FFI type context
pub struct FfiContext {
    pub loaded_libraries: HashMap<String, Library>,
    pub type_mappings: HashMap<String, CTypeMapping>,
}

pub struct Library {
    pub path: String,
    pub handle: *mut std::ffi::c_void,
    pub functions: HashMap<String, ForeignFunction>,
}
```

### 4.3 Parser Modifications

**Keywords to add:** `extern`, `import`, `c_type`, `link`

**Syntax examples:**
```ruby
# Basic extern function
extern "C" {
    def printf(format: *const c_char, ...) -> c_int
}

# Import from library
import "libc" {
    fn malloc(size: usize) -> *mut c_void
    fn free(ptr: *mut c_void)
}

# External block
extern "C" {
    def sin(x: c_double) -> c_double
    def cos(x: c_double) -> c_double
}

# Using foreign function
def call_c()
    let result = extern call_my_c_function(42)
end
```

### 4.4 Code Generation Strategy

**C FFI generation:**
- Generate compatible C function declarations
- Handle name mangling for different ABIs
- Generate wrapper functions for type conversions
- Handle variadic functions specially

**Rust target:**
- Use `extern "C" { }` blocks
- Generate `#[link]` attributes
- Handle c_char, c_int, etc. type mappings

**Type conversions:**
```rust
// Example conversions
c_int    -> i32
c_long   -> isize
c_float  -> f32
c_double -> f64
*c_char  -> *const i8
*c_void  -> *mut c_void
```

**Runtime support:**
- Dynamic library loading (dlopen)
- Symbol resolution (dlsym)
- Foreign function call wrapper

### 4.5 Example AZC Code

```ruby
# Using libc
import "libc" {
    fn exit(status: c_int) -> !
    fn getpid() -> c_pid_t
}

# Wrapper for C library
def exit_program(code: Int)
    extern exit(code as c_int)
end

# Unsafe wrapper for C functions
unsafe def read_file(path: *const c_char) -> *mut c_char
    let fd = extern open(path, O_RDONLY)
    if fd < 0
        return nil
    end
    
    let buffer = extern malloc(1024)
    let bytes_read = extern read(fd, buffer, 1023)
    
    if bytes_read < 0
        extern free(buffer)
        return nil
    end
    
    buffer
end

# Creating C-compatible structures
struct CString
    data: *mut c_char
end

impl CString
    def from_azc(s: String) -> CString
        let ptr = extern malloc(s.len() + 1)
        # Copy string data to pointer
        CString { data: ptr }
    end
end
```

---

## 5. Implementation Roadmap

### Phase 1: Async/Await (4 weeks)
1. Add Future type to type system
2. Implement async/await parser
3. Add async type inference
4. Generate async state machines
5. Test with async stdlib

### Phase 2: Unsafe Blocks (3 weeks)
1. Add unsafe keyword and AST nodes
2. Modify borrow checker for unsafe contexts
3. Track unsafe blocks for safety analysis
4. Generate safety annotations in output

### Phase 3: FFI (4 weeks)
1. Add extern/import statements
2. Implement C type mappings
3. Add library loading mechanism
4. Generate FFI glue code

### Phase 4: Macros (5 weeks)
1. Design macro syntax
2. Implement macro parser
3. Add macro expansion engine
4. Support macro_rules! style
5. Add hygiene tracking

---

## 6. Safety Considerations

### 6.1 Unsafe Block Safety Analysis
- Track all unsafe operations
- Report unsafe operations in safety score
- Provide unsafe audit tooling
- Document unsafe reasons

### 6.2 FFI Safety
- Validate foreign function signatures
- Check memory safety of FFI calls
- Track unsafe operations from FFI
- Generate safety documentation

### 6.3 Macro Safety
- Prevent infinite macro expansion
- Validate macro hygiene
- Report macro expansion errors clearly

---

## 7. Testing Strategy

### Unit Tests
- Test each new AST node parsing
- Test type inference for new types
- Test code generation for each target

### Integration Tests
- Test async functions end-to-end
- Test unsafe block with FFI
- Test macro expansion

### Safety Tests
- Verify safety score calculations
- Test unsafe block tracking
- Validate FFI safety checks

---

## 8. Backwards Compatibility

All v0.4.0 features are additive:
- Existing code continues to work unchanged
- New syntax is opt-in
- Unsafe code requires explicit `unsafe` blocks
- FFI requires `extern` blocks
- Macros require explicit definition and use

---

## Summary

v0.4.0 adds four powerful features that bring AZC closer to systems programming capabilities while maintaining its safety-first philosophy:

| Feature | Safety Impact | Complexity | Use Case |
|---------|---------------|------------|----------|
| Async/Await | None | Medium | Concurrent I/O, parallelism |
| Macros | None | High | Code generation, DSLs |
| Unsafe Blocks | Lowers safety score | Medium | Low-level operations, FFI |
| FFI | Requires unsafe | Medium | C library interop |

The implementation maintains AZC's safety guarantees by:
- Requiring explicit `unsafe` blocks
- Tracking all unsafe operations
- Generating safety documentation
- Including unsafe operations in safety score
