#!/bin/bash
# Benchmark runner script for comparing Rusty and Python performance

set -e  # Exit on error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"  # Change to script directory

# Set output formatting
BOLD="\033[1m"
GREEN="\033[0;32m"
RED="\033[0;31m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
NC="\033[0m" # No Color

echo -e "${BOLD}Matrix Multiplication Benchmark Comparison${NC}"
echo "========================================"
echo

# Run Rusty benchmark
echo -e "${BLUE}Running Rusty benchmark...${NC}"
cd ..
RUSTY_OUTPUT=$(cargo run --release -- "${SCRIPT_DIR}/benchmark_matrix.ry")
echo "$RUSTY_OUTPUT"
cd "$SCRIPT_DIR"
echo

# Extract Rusty results from output
RUSTY_RESULT=$(echo "$RUSTY_OUTPUT" | grep "RESULT_CSV:" | sed 's/RESULT_CSV: //')

# Run Python benchmark
echo -e "${BLUE}Running Python benchmark...${NC}"
# Capture both stdout and stderr
PYTHON_OUTPUT=$(python3 benchmark_matrix.py 2>&1)
echo "$PYTHON_OUTPUT"
echo

# Extract Python results from output
PYTHON_RESULT=$(echo "$PYTHON_OUTPUT" | grep "RESULT_CSV:" | sed 's/RESULT_CSV: //' | head -1)

# Compare results
echo -e "${BOLD}Performance Comparison:${NC}"
echo "----------------------"

# Extract values from CSV outputs
RUSTY_SIZE=$(echo "$RUSTY_RESULT" | cut -d',' -f2)
RUSTY_ITERATIONS=$(echo "$RUSTY_RESULT" | cut -d',' -f3)
RUSTY_TOTAL_TIME=$(echo "$RUSTY_RESULT" | cut -d',' -f4)
RUSTY_AVG_TIME=$(echo "$RUSTY_RESULT" | cut -d',' -f5)

PYTHON_SIZE=$(echo "$PYTHON_RESULT" | cut -d',' -f2)
PYTHON_ITERATIONS=$(echo "$PYTHON_RESULT" | cut -d',' -f3)
PYTHON_TOTAL_TIME=$(echo "$PYTHON_RESULT" | cut -d',' -f4)
PYTHON_AVG_TIME=$(echo "$PYTHON_RESULT" | cut -d',' -f5)

# Calculate ratios safely
if [[ "$RUSTY_TOTAL_TIME" != "" && "$PYTHON_TOTAL_TIME" != "" ]]; then
    TOTAL_RATIO=$(echo "scale=2; $PYTHON_TOTAL_TIME / $RUSTY_TOTAL_TIME" | bc)
else
    TOTAL_RATIO="N/A"
fi

if [[ "$RUSTY_AVG_TIME" != "" && "$PYTHON_AVG_TIME" != "" ]]; then
    AVG_RATIO=$(echo "scale=2; $PYTHON_AVG_TIME / $RUSTY_AVG_TIME" | bc)
else
    AVG_RATIO="N/A"
fi

# Display comparison table
echo -e "${BOLD}Metric           | Python      | Rusty        | Ratio (Python/Rusty)${NC}"
echo "----------------|------------|-------------|----------------------"
echo -e "Matrix Size      | $PYTHON_SIZE x $PYTHON_SIZE | $RUSTY_SIZE x $RUSTY_SIZE    | -"
echo -e "Iterations       | $PYTHON_ITERATIONS        | $RUSTY_ITERATIONS          | -"
echo -e "Total Time (ms)  | $PYTHON_TOTAL_TIME     | $RUSTY_TOTAL_TIME      | $TOTAL_RATIO"
echo -e "Avg Time (ms)    | $PYTHON_AVG_TIME     | $RUSTY_AVG_TIME      | $AVG_RATIO"

# Calculate speedup
if [[ "$AVG_RATIO" != "N/A" ]]; then
    SPEEDUP=$AVG_RATIO
    if (( $(echo "$SPEEDUP > 1" | bc -l) )); then
        PERCENT_FASTER=$(echo "scale=1; ($PYTHON_AVG_TIME - $RUSTY_AVG_TIME) * 100 / $PYTHON_AVG_TIME" | bc)
        echo -e "\n${GREEN}Rusty is ${BOLD}$SPEEDUP times faster${NC}${GREEN} than Python (${PERCENT_FASTER}% faster)${NC}"
    else
        INV_SPEEDUP=$(echo "scale=2; 1 / $SPEEDUP" | bc)
        echo -e "\n${RED}Python is ${BOLD}$INV_SPEEDUP times faster${NC}${RED} than Rusty${NC}"
    fi
else
    echo -e "\n${YELLOW}Unable to calculate performance comparison.${NC}"
fi

echo
echo "Benchmark completed."
