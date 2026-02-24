# Testing Guide for Sorting Algorithms

A unified test driver lives in `sorting/test/test_driver.py`. It tests all languages in one place — no need for separate test scripts.

## Requirements

Each sorting program must:
- Read integers from **stdin** (one per line)
- Sort them in ascending order
- Write the sorted result to **stdout** (one per line)

## Usage

```bash
cd sorting/test

# Interactive menu — pick your language and algorithm
python3 test_driver.py

# Test all algorithms for a specific language
python3 test_driver.py --language java

# Test specific algorithm(s) for a language
python3 test_driver.py --language java --algorithm bubble merge

# Test multiple languages
python3 test_driver.py --language java go

# Test everything that has source files
python3 test_driver.py --all
```

## Expected File Names

The driver expects these source file names in each language directory:

| Language | Directory | File pattern |
|---|---|---|
| **Java** | `sorting/java/` | `BubbleSort.java`, `InsertionSort.java`, etc. |
| **Go** | `sorting/go/cmd/<algo>/` | `main.go` (bubble, insertion, etc.) |
| **Python** | `sorting/python/` | `bubble_sort.py`, `insertion_sort.py`, etc. |
| **Rust** | `sorting/rust/` | `bubble_sort.rs`, `insertion_sort.rs`, etc. |
| **Julia** | `sorting/julia/` | `bubble_sort.jl`, `insertion_sort.jl`, etc. |
| **C++** | `sorting/c++/` | `bubble_sort.cpp`, `insertion_sort.cpp`, etc. |
| **Zig** | `sorting/zig/` | `bubble_sort.zig`, `insertion_sort.zig`, etc. |
| **Odin** | `sorting/odin/` | `bubble_sort.odin`, `insertion_sort.odin`, etc. |

## How It Verifies

For each test case the driver:
1. Runs your sorting program with test data piped to stdin
2. Generates expected output using `sort -n`
3. Compares with `diff` — a match means PASS

Test data is in `sorting/test/data/input_data/10000/` (6 distributions × 10,000 integers each).
