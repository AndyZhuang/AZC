# AZC Safety Features Test Suite

This directory contains tests for AZC's safety features including:
- Type safety
- Ownership
- Borrow checking
- Memory safety

## Test Categories

### 1. Type Safety Tests (`types/`)
- Type inference
- Type checking
- Type annotations
- Generic types

### 2. Ownership Tests (`ownership/`)
- Move semantics
- Copy semantics
- Scope-based destruction
- Resource management

### 3. Borrow Checker Tests (`borrow/`)
- Immutable borrows
- Mutable borrows
- Borrow conflicts
- Lifetime validation

### 4. Memory Safety Tests (`memory/`)
- No null pointers
- No dangling references
- No buffer overflows
- No use-after-free

## Running Tests

```bash
# Run all tests
./run_safety_tests.sh

# Run specific category
./run_safety_tests.sh types
./run_safety_tests.sh ownership
./run_safety_tests.sh borrow
./run_safety_tests.sh memory
```

## Test Results

All tests should pass with AZC's safety guarantees in place.
Any failures indicate a gap in the safety system that must be fixed.