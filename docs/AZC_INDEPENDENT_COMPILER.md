# AZC 独立编译器设计方案

## 目标
建立完全不依赖Rust的AZC编译器，实现真正的语言自主可控。

## 当前架构（依赖Rust）
```
AZC源代码 → Rust编译器 → C代码 → C编译器 → 可执行文件
```

## 方案1: C语言自举编译器（推荐）

### 第一阶段：C语言最小编译器（Bootstrap）

创建一个最小化的C编译器，可以编译AZC核心功能：

```
C编写:
├── lexer.c         - 词法分析
├── parser.c        - 语法分析  
├── codegen.c       - C代码生成
├── ast.c           - AST构建
└── main.c          - 主程序
```

### 第二阶段：自举（Bootstrapping）

用C编译器编译AZC核心，然后用AZC重写编译器：

```
Phase 1: C编译器编译AZC核心
Phase 2: AZC编译器编译自身（自举）
```

### 实现计划

```c
// 核心数据结构 - 完全用C实现

// 1. Token定义
typedef enum {
    TOKEN_EOF,
    TOKEN_IDENTIFIER,
    TOKEN_NUMBER,
    TOKEN_STRING,
    TOKEN_KEYWORD,
    TOKEN_OPERATOR,
    // ... 更多token类型
} TokenType;

typedef struct {
    TokenType type;
    char* lexeme;
    int line;
    int column;
} Token;

// 2. AST节点
typedef enum {
    NODE_PROGRAM,
    NODE_LET,
    NODE_FUNCTION,
    NODE_IF,
    NODE_WHILE,
    NODE_BINARY,
    NODE_UNARY,
    NODE_CALL,
    NODE_MEMBER,
    // ... 更多节点类型
} NodeType;

typedef struct ASTNode {
    NodeType type;
    union {
        // 根据节点类型存储不同数据
    };
    struct ASTNode** children;
    int child_count;
} ASTNode;

// 3. 编译流程
typedef struct {
    const char* source;
    size_t source_len;
    size_t position;
    int line;
    int column;
    Token* tokens;
    size_t token_count;
    ASTNode* ast;
} Compiler;

// 4. 代码生成
typedef struct {
    char* buffer;
    size_t buffer_size;
    size_t buffer_capacity;
    int indent_level;
} Codegen;
```

### 目标文件结构

```
compiler-c/
├── Makefile
├── src/
│   ├── main.c          - 入口点
│   ├── lexer.c         - 词法分析器
│   ├── lexer.h
│   ├── parser.c        - 语法分析器
│   ├── parser.h
│   ├── ast.c           - AST
│   ├── ast.h
│   ├── codegen.c       - C代码生成
│   ├── codegen.h
│   ├── typecheck.c     - 类型检查
│   ├── typecheck.h
│   ├── optimizer.c     - 优化器
│   ├── optimizer.h
│   └── utils.c         - 工具函数
│   └── utils.h
├── runtime/            - 运行时库(C)
│   ├── include/
│   │   └── azc.h
│   └── src/
│       ├── memory.c
│       ├── string.c
│       ├── array.c
│       └── agent.c     - Agent运行时
└── tests/
```

## 方案2: 自引用（Self-hosting）

等AZC成熟后，用AZC重写编译器：

```ruby
# 最终目标：用AZC编写AZC编译器

# lexer.azc - 词法分析
module Lexer
    def tokenize(source)
        # ...
    end
end

# parser.azc - 语法分析
module Parser
    def parse(tokens)
        # ...
    end
end
```

## 方案3: WASM/WebAssembly目标

编译AZC到WASM，实现浏览器端运行：

```bash
# 编译到WASM
azc --target wasm32 file.azc
# 输出 file.wasm
```

## 实施步骤

### Step 1: 创建C编译器框架
```bash
mkdir -p compiler-c/src
touch compiler-c/src/main.c
```

### Step 2: 实现词法分析器
- 字符读取
- Token识别
- 关键字表

### Step 3: 实现解析器
- 递归下降解析
- AST构建
- 错误恢复

### Step 4: 实现代码生成
- C代码输出
- 类型转换
- 运行时调用

### Step 5: 添加优化
- 常量折叠
- 死代码消除

## 验收标准

1. **无Rust依赖**: `nm compiler-c/azc | grep rust` 应为空
2. **自举测试**: AZC编译器可编译自身
3. **性能**: 编译速度 < 1秒（单文件）
4. **兼容性**: 生成的C代码可被gcc/clang编译

## 时间规划

- **Phase 1** (2-3周): C核心编译器
- **Phase 2** (1周): 运行时库
- **Phase 3** (2周): 优化和测试
- **Phase 4** (持续): 自引用准备

## 关键技术决策

| 特性 | 选择 | 理由 |
|-----|------|------|
| 内存管理 | 手动+池 | 嵌入式友好 |
| 字符串 | 内部化(interning) | 减少重复分配 |
| 错误处理 | setjmp/longjmp | 兼容C |
| 并行 | 线程池 | 现代多核支持 |

## 风险管理

1. **复杂性**: C需要手动内存管理
   - 解决: 使用arena allocator

2. **调试困难**: 
   - 解决: 完善的日志和断言

3. **性能**:
   - 解决: 增量编译和缓存