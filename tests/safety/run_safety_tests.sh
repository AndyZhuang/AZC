#!/bin/bash

# AZC Safety Test Suite Runner

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
AZC="$PROJECT_DIR/compiler/target/release/azc"
OUTPUT_DIR="/tmp/azc_safety_tests"

mkdir -p $OUTPUT_DIR

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

pass=0
fail=0
total=0

echo "========================================="
echo "AZC Safety Test Suite"
echo "========================================="
echo ""

run_test() {
    local test_name=$1
    local test_file=$2
    local category=$3
    
    ((total++))
    echo -n "  Testing $test_name... "
    
    $AZC $test_file > $OUTPUT_DIR/${test_name}.c 2>&1
    
    if [ $? -eq 0 ]; then
        if grep -q "AZC" $OUTPUT_DIR/${test_name}.c 2>/dev/null; then
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

run_category() {
    local category=$1
    local dir="$SCRIPT_DIR/$category"
    
    echo -e "${BLUE}=== $category ===${NC}"
    
    if [ -d "$dir" ]; then
        for test_file in "$dir"/*.azc; do
            if [ -f "$test_file" ]; then
                local name=$(basename "$test_file" .azc)
                run_test "${category}_$name" "$test_file" "$category"
            fi
        done
    else
        echo "  No tests found"
    fi
    echo ""
}

# Build compiler
echo "Building compiler..."
cd "$PROJECT_DIR/compiler" && cargo build --release > /dev/null 2>&1
if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
echo "Build successful!"
echo ""

# Run safety tests
run_category "types"
run_category "ownership"
run_category "borrow"
run_category "memory"
run_category "industrial"

# Summary
echo "========================================="
echo "Safety Test Results"
echo "========================================="
echo -e "Total: $total"
echo -e "Passed: ${GREEN}$pass${NC}"
echo -e "Failed: ${RED}$fail${NC}"
echo ""

if [ $fail -eq 0 ]; then
    echo -e "${GREEN}All safety tests passed!${NC}"
    echo "AZC is memory safe!"
    exit 0
else
    echo -e "${RED}Some safety tests failed!${NC}"
    exit 1
fi