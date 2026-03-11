# AZC Test Suite

This directory contains tests for the AZC compiler.

## Running Tests

```bash
cd compiler
cargo test
```

## Test Files

### 01_variables.azc
Tests basic variable declarations and types.

```ruby
let x = 10
let name = "AZC"
let active = true
puts "Variables: OK"
```

**Expected Output:**
```c
AZC x = 10;
AZC name = "AZC";
AZC active = 1;
azc_puts(azc_str("Variables: OK"));
```

### 02_functions.azc
Tests function definitions and calls.

```ruby
def hello
    puts "Hello from function"
end

hello
```

**Expected Output:**
```c
void azc_hello(void) {
    azc_puts(azc_str("Hello from function"));
}
azc_hello();
```

### 03_control_flow.azc
Tests if/else statements.

```ruby
let x = 5

if x > 3
    puts "x > 3"
else
    puts "x <= 3"
end
```

**Expected Output:**
```c
AZC x = 5;
if (x > 3) {
    azc_puts(azc_str("x > 3"));
} else {
    azc_puts(azc_str("x <= 3"));
}
```

### 04_while_loop.azc
Tests while loops.

```ruby
let i = 0

while i < 3
    puts "Counting: #{i}"
    i = i + 1
end
```

**Expected Output:**
```c
AZC i = 0;
while (i < 3) {
    azc_puts(azc_str("Counting: #{i}"));
    i = i + 1;
}
```

### 05_nested_if.azc
Tests nested if statements.

```ruby
let x = 5
let y = 10

if x > 3
    if y > 5
        puts "Both conditions true"
    else
        puts "Only x > 3"
    end
end
```

### 06_string_interpolation.azc
Tests string interpolation.

```ruby
name = "AZC"
version = "0.1.0"

puts "Welcome to #{name} v#{version}!"
```

### 07_boolean_expressions.azc
Tests boolean operators.

```ruby
let a = true
let b = false

if a and b
    puts "Both true"
else
    puts "Not both true"
end

if a or b
    puts "At least one true"
end

if not b
    puts "b is false"
end
```

### 08_comparison_operators.azc
Tests all comparison operators.

```ruby
let x = 10
let y = 20

# Equal
if x == 10
    puts "x equals 10"
end

# Not equal
if x != 5
    puts "x not equal 5"
end

# Greater/Less
if y > x
    puts "y > x"
end

if x < y
    puts "x < y"
end

# Greater or equal
if x >= 10
    puts "x >= 10"
end

# Less or equal
if x <= 10
    puts "x <= 10"
end
```

### 09_arithmetic.azc
Tests arithmetic operations.

```ruby
let a = 10
let b = 3

# Addition
let sum = a + b

# Subtraction
let diff = a - b

# Multiplication
let prod = a * b

# Division
let quot = a / b

# Modulo
let rem = a % b

puts "Results calculated"
```

### 10_class_definition.azc
Tests class definitions (basic).

```ruby
class Point
    def initialize
        @x = 0
        @y = 0
    end
    
    def set_x(val)
        @x = val
    end
    
    def get_x
        @x
    end
end
```

## Adding New Tests

1. Create a new `.azc` file in this directory
2. Add test description as comments
3. Run the compiler to verify output
4. Update this README with test details

## Test Runner

A simple test runner is planned:

```bash
# Will run all tests
./run_tests.sh
```
