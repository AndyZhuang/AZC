# AZC Standard Library

This directory contains the standard library for the AZC programming language.

## Modules

- `core` - Core types and operations
- `collections` - Collection types (Array, Map, Set)
- `io` - Input/Output operations
- `math` - Mathematical functions
- `text` - String manipulation
- `time` - Time and date operations
- `industrial` - Industrial control system primitives

## Usage

```ruby
# Import a module
use std::io
use std::collections::Array

# Use functions
io.puts("Hello, AZC!")
let arr = Array.new(1, 2, 3)
```