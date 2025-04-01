#!/usr/bin/env python3
"""
Matrix multiplication benchmark for Python
This program performs matrix multiplication with configurable dimensions and iterations
It measures execution time and outputs performance metrics
"""

import time
import random
import sys

# Configuration
MATRIX_SIZE = 100  # Larger matrix size for more pronounced comparison
ITERATIONS = 3    # Number of times to repeat the multiplication - matches Rusty benchmark
SILENT = False    # Set to true to disable intermediate output


def create_matrix(size):
    """Create a matrix with random values."""
    return [[random.random() for _ in range(size)] for _ in range(size)]


def multiply_matrices(a, b, size):
    """Perform matrix multiplication."""
    result = [[0 for _ in range(size)] for _ in range(size)]

    for i in range(size):
        for j in range(size):
            for k in range(size):
                result[i][j] += a[i][k] * b[k][j]

    return result


def matrix_trace(matrix, size):
    """Calculate matrix trace (sum of diagonal elements)."""
    return sum(matrix[i][i] for i in range(size))


def main():
    print("Python Matrix Multiplication Benchmark")
    print("====================================")
    print(f"Matrix Size: {MATRIX_SIZE}x{MATRIX_SIZE}")
    print(f"Iterations: {ITERATIONS}")
    print()

    # Initialize matrices
    print("Initializing matrices...")
    start_init = time.time()
    matrix_a = create_matrix(MATRIX_SIZE)
    matrix_b = create_matrix(MATRIX_SIZE)
    init_time = time.time() - start_init
    print(f"Initialization completed in {init_time * 1000:.2f} ms")
    print()

    # Run benchmark
    print("Running benchmark...")
    total_time = 0
    result = None

    for iter_num in range(ITERATIONS):
        if not SILENT:
            print(f"Iteration {iter_num + 1}/{ITERATIONS}...")

        start_time = time.time()
        result = multiply_matrices(matrix_a, matrix_b, MATRIX_SIZE)
        end_time = time.time()

        iter_time = end_time - start_time
        total_time += iter_time

        if not SILENT:
            print(f"  Time: {iter_time * 1000:.2f} ms")
            print(f"  Trace: {matrix_trace(result, MATRIX_SIZE)}")
            print()

    # Calculate and display results
    avg_time = total_time / ITERATIONS
    operations_per_second = (MATRIX_SIZE * MATRIX_SIZE * MATRIX_SIZE) / avg_time

    print("Benchmark Results:")
    print("------------------")
    print(f"Total time: {total_time * 1000:.2f} ms")
    print(f"Average time per iteration: {avg_time * 1000:.2f} ms")
    print(f"Operations per second: {operations_per_second:.2f}")

    # Print CSV format result for extraction
    csv_result = f"RESULT_CSV: python,{MATRIX_SIZE},{ITERATIONS},{total_time * 1000:.2f},{avg_time * 1000:.2f}"
    print(csv_result)
    # Ensure it's also printed to stderr to help with capturing
    print(csv_result, file=sys.stderr)

    print("Benchmark completed.")


if __name__ == "__main__":
    main()
