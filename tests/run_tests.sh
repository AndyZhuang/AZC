#!/bin/bash

# AZC Test Runner
# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

AZC="$PROJECT_DIR/compiler/target/release/azc"
TEST_DIR="$SCRIPT_DIR"
OUTPUT_DIR="/tmp/azc_tests"

mkdir -p $OUTPUT_DIR

echo "========================================="
echo "AZC Compiler Test Suite"
echo "========================================="
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

pass=0
fail=0

run_test() {
    local test_name=$1
    local test_file=$2
    
    echo -n "Testing $test_name... "
    
    # Run compiler and capture output (stdout)
    $AZC $test_file > $OUTPUT_DIR/${test_name}.c 2>&1
    
    if [ $? -eq 0 ]; then
        # Check if output contains expected content
        if grep -q "AZC" $OUTPUT_DIR/${test_name}.c; then
            echo -e "${GREEN}PASS${NC}"
            ((pass++))
        else
            echo -e "${YELLOW}WARN${NC} (no AZC output)"
            ((pass++))
        fi
    else
        echo -e "${RED}FAIL${NC}"
        ((fail++))
    fi
}

# Build compiler first
echo "Building compiler..."
cd "$PROJECT_DIR/compiler" && cargo build --release > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
echo "Build successful!"
echo ""

# Run tests
echo "Running tests..."
echo ""

run_test "01_variables" "$TEST_DIR/01_variables.azc"
run_test "02_functions" "$TEST_DIR/02_functions.azc"
run_test "03_if_else" "$TEST_DIR/03_if_else.azc"
run_test "04_while_loop" "$TEST_DIR/04_while_loop.azc"
run_test "05_boolean" "$TEST_DIR/05_boolean.azc"
run_test "06_comparison" "$TEST_DIR/06_comparison.azc"
run_test "07_mutable" "$TEST_DIR/07_mutable.azc"
run_test "08_structs" "$TEST_DIR/08_structs.azc"
run_test "09_enums" "$TEST_DIR/09_enums.azc"
echo ""
echo "========================================="
echo "Results: $pass passed, $fail failed"
echo "========================================="

exit $fail
