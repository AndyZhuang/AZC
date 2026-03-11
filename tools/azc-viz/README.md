# AZC Safety Visualization Tool

## Overview

The `azc-viz` tool provides visual analysis of your AZC code's safety properties.

## Features

### 1. Ownership Visualization

```
┌─────────────────────────────────────────────┐
│ Ownership Graph                              │
├─────────────────────────────────────────────┤
│                                              │
│  [x: String] ──owner──> "hello"              │
│       │                                      │
│       ├── &x ──borrow──> y (immutable)       │
│       │                                      │
│       └── &mut x ──blocked (already borrowed)│
│                                              │
│  Status: ⚠️ Multiple borrows                 │
└─────────────────────────────────────────────┘
```

### 2. Lifetime Visualization

```
┌─────────────────────────────────────────────┐
│ Lifetime Analysis                            │
├─────────────────────────────────────────────┤
│                                              │
│  'a ──────────────────────────────┐         │
│  let x = String::new();           │         │
│  let y = &x;              'b ─────┼───┐     │
│  let z = &x;              'c ─────┼───┼───┐ │
│  use(y);                          │   │   │ │
│  use(z);                          │   │   │ │
│  drop(x);  // Error: x borrowed   │   │   │ │
│                                   ▼   ▼   ▼ │
└─────────────────────────────────────────────┘
```

### 3. Memory Layout

```
┌─────────────────────────────────────────────┐
│ Memory Layout                                │
├─────────────────────────────────────────────┤
│                                              │
│  Stack:                                      │
│  ┌──────────────────────────┐               │
│  │ x: Box<Int>              │               │
│  │   ptr ────────────┐      │               │
│  └───────────────────┼──────┘               │
│                      │                       │
│  Heap:               ▼                       │
│  ┌──────────────────────────┐               │
│  │ Int { value: 42 }        │               │
│  └──────────────────────────┘               │
│                                              │
│  Total: 8 bytes (stack) + 8 bytes (heap)    │
└─────────────────────────────────────────────┘
```

### 4. Safety Heat Map

```
┌─────────────────────────────────────────────┐
│ Safety Heat Map                              │
├─────────────────────────────────────────────┤
│                                              │
│  Line 1:  let x = 5          🟢 Safe         │
│  Line 2:  let y = &x         🟢 Safe         │
│  Line 3:  let z = &mut x     🔴 Error        │
│  Line 4:  puts x             🟢 Safe         │
│  Line 5:  unsafe { ... }     🟡 Needs review │
│                                              │
│  Score: 80/100                               │
└─────────────────────────────────────────────┘
```

## Usage

```bash
# Visualize ownership
azc-viz ownership file.azc

# Visualize lifetimes
azc-viz lifetimes file.azc

# Show memory layout
azc-viz memory file.azc

# Generate safety report
azc-viz report file.azc --output report.html

# Interactive mode
azc-viz interactive file.azc
```

## Safety Report

Generates an HTML report with:

- Ownership graph
- Borrow checker results
- Lifetime analysis
- Memory safety score
- Recommendations

## Integration

```bash
# Pre-commit hook
azc-viz report src/ --fail-under 80

# CI pipeline
azc-viz report src/ --format json > safety_report.json
```

## Example Output

```
AZC Safety Analysis Report
==========================

File: src/main.azc
Lines: 150
Safety Score: 95/100

Issues Found:
  [Warning] Line 45: Potential integer overflow
  [Info] Line 78: Consider using checked arithmetic

Recommendations:
  1. Add overflow checks for arithmetic operations
  2. Consider using saturating arithmetic for counters
  3. Add bounds checking for array access

Memory Analysis:
  Stack: 1.2 KB
  Heap: 4.5 KB
  Peak: 8.2 KB

Ownership Status: ✅ No issues
Borrow Status: ✅ No issues
Lifetime Status: ✅ All lifetimes valid
```