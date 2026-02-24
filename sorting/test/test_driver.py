#!/usr/bin/env python3
"""
Unified Test Driver for Sorting Algorithms
===========================================

Tests sorting algorithm implementations across all languages.
Each program must read integers from stdin (one per line) and
write the sorted result to stdout (one per line).

Usage:
    python3 test_driver.py                       # Interactive menu
    python3 test_driver.py --language java       # Test all Java algorithms
    python3 test_driver.py --language java --algorithm bubble  # Test one
    python3 test_driver.py --all                 # Test everything
"""

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

# ============================================================
# CONFIGURATION
# ============================================================

SCRIPT_DIR = Path(__file__).resolve().parent
SORTING_DIR = SCRIPT_DIR.parent
DATA_DIR = SCRIPT_DIR / "data" / "input_data" / "10000"

ALGORITHMS = ["bubble", "insertion", "selection", "merge", "quick"]

DISTRIBUTIONS = [
    "random",
    "sorted",
    "reverse",
    "nearly_sorted",
    "few_unique",
    "identical",
]

# Language configuration: how to compile and run each language's algorithms.
# Each entry maps an algorithm name to a dict with:
#   "compile" : list of compile commands (empty list if interpreted)
#   "run"     : the run command as a list
#   "cwd"     : working directory (relative to sorting/)
#
# Placeholders are NOT used — paths are built dynamically in get_language_config().

def get_language_config():
    """Build language configs with absolute paths."""
    configs = {}

    # --- Java ---
    java_dir = SORTING_DIR / "java"
    java_names = {
        "bubble": "BubbleSort",
        "insertion": "InsertionSort",
        "selection": "SelectionSort",
        "merge": "MergeSort",
        "quick": "QuickSort",
    }
    java_algos = {}
    for algo, cls in java_names.items():
        java_algos[algo] = {
            "compile": [["javac", str(java_dir / f"{cls}.java")]],
            "run": ["java", "-cp", str(java_dir), cls],
        }
    configs["java"] = java_algos

    # --- Go ---
    go_dir = SORTING_DIR / "go"
    go_algos = {}
    for algo in ALGORITHMS:
        src = go_dir / "cmd" / algo / "main.go"
        binary = go_dir / "bin" / f"{algo}_sort"
        go_algos[algo] = {
            "compile-cwd": go_dir,
            "compile": [["go", "build", "-o", str(binary), str(src)]],
            "run": [str(binary)],
        }
    configs["go"] = go_algos

    # --- Python ---
    py_dir = SORTING_DIR / "python"
    py_names = {
        "bubble": "bubble_sort.py",
        "insertion": "insertion_sort.py",
        "selection": "selection_sort.py",
        "merge": "merge_sort.py",
        "quick": "quick_sort.py",
    }
    py_algos = {}
    for algo, fname in py_names.items():
        py_algos[algo] = {
            "compile": [],
            "run": ["python3", str(py_dir / fname)],
        }
    configs["python"] = py_algos

    # --- Rust ---
    rust_dir = SORTING_DIR / "rust"
    rust_names = {
        "bubble": "bubble_sort",
        "insertion": "insertion_sort",
        "selection": "selection_sort",
        "merge": "merge_sort",
        "quick": "quick_sort",
    }
    rust_algos = {}
    for algo, name in rust_names.items():
        src = rust_dir / f"{name}.rs"
        binary = rust_dir / "bin" / name
        rust_algos[algo] = {
            "compile": [["rustc", str(src), "-o", str(binary)]],
            "run": [str(binary)],
        }
    configs["rust"] = rust_algos

    # --- Julia ---
    julia_dir = SORTING_DIR / "julia"
    julia_names = {
        "bubble": "bubble_sort.jl",
        "insertion": "insertion_sort.jl",
        "selection": "selection_sort.jl",
        "merge": "merge_sort.jl",
        "quick": "quick_sort.jl",
    }
    julia_algos = {}
    for algo, fname in julia_names.items():
        julia_algos[algo] = {
            "compile": [],
            "run": ["julia", str(julia_dir / fname)],
        }
    configs["julia"] = julia_algos

    # --- C++ ---
    cpp_dir = SORTING_DIR / "c++"
    cpp_names = {
        "bubble": "bubble_sort",
        "insertion": "insertion_sort",
        "selection": "selection_sort",
        "merge": "merge_sort",
        "quick": "quick_sort",
    }
    cpp_algos = {}
    for algo, name in cpp_names.items():
        src = cpp_dir / f"{name}.cpp"
        binary = cpp_dir / name
        cpp_algos[algo] = {
            "compile": [["g++", "-O2", "-o", str(binary), str(src)]],
            "run": [str(binary)],
        }
    configs["c++"] = cpp_algos

    # --- Zig ---
    zig_dir = SORTING_DIR / "zig"
    zig_names = {
        "bubble": "bubble_sort",
        "insertion": "insertion_sort",
        "selection": "selection_sort",
        "merge": "merge_sort",
        "quick": "quick_sort",
    }
    zig_algos = {}
    for algo, name in zig_names.items():
        src = zig_dir / f"{name}.zig"
        binary = zig_dir / name
        zig_algos[algo] = {
            "compile": [["zig", "build-exe", "-o", str(binary), str(src)]],
            "run": [str(binary)],
        }
    configs["zig"] = zig_algos

    # --- Odin ---
    odin_dir = SORTING_DIR / "odin"
    odin_names = {
        "bubble": "bubble_sort",
        "insertion": "insertion_sort",
        "selection": "selection_sort",
        "merge": "merge_sort",
        "quick": "quick_sort",
    }
    odin_algos = {}
    for algo, name in odin_names.items():
        src = odin_dir / f"{name}.odin"
        binary = odin_dir / name
        odin_algos[algo] = {
            "compile": [["odin", "build", str(src), f"-out:{binary}"]],
            "run": [str(binary)],
        }
    configs["odin"] = odin_algos

    return configs


# ============================================================
# TEST RUNNER
# ============================================================

def generate_expected(input_file, output_file):
    """Generate expected sorted output using sort -n."""
    with open(input_file) as fin, open(output_file, "w") as fout:
        subprocess.run(["sort", "-n"], stdin=fin, stdout=fout, check=True)


def run_test(run_cmd, input_file, actual_file):
    """Run a sorting algorithm and capture its output. Returns (success, error_msg)."""
    try:
        with open(input_file) as fin, open(actual_file, "w") as fout:
            result = subprocess.run(
                run_cmd, stdin=fin, stdout=fout, stderr=subprocess.PIPE,
                timeout=60, text=True,
            )
        if result.returncode != 0:
            return False, f"exit code {result.returncode}: {result.stderr.strip()}"
        return True, ""
    except subprocess.TimeoutExpired:
        return False, "timed out (60s)"
    except FileNotFoundError:
        return False, f"executable not found: {run_cmd[0]}"


def compile_algorithm(compile_cmds, compile_cwd):
    """Run compile commands. Returns (success, error_msg)."""
    for cmd in compile_cmds:
        try:
            result = subprocess.run(
                cmd, capture_output=True, text=True, timeout=30, cwd=compile_cwd
            )
            if result.returncode != 0:
                return False, f"compile failed: {result.stderr.strip()}"
        except FileNotFoundError:
            return False, f"compiler not found: {cmd[0]}"
        except subprocess.TimeoutExpired:
            return False, "compile timed out"
    return True, ""


def test_algorithm(language, algo, config):
    """Test one algorithm against all distributions. Returns (passed, failed) counts."""
    passed = 0
    failed = 0

    # Compile if needed
    if config["compile"]:
        cwd = config["compile-cwd"] if "compile-cwd" in config else None
        ok, err = compile_algorithm(config["compile"], cwd)
        if not ok:
            print(f"  SKIP  {language}/{algo} — {err}")
            return 0, len(DISTRIBUTIONS)

    with tempfile.TemporaryDirectory() as tmpdir:
        for dist in DISTRIBUTIONS:
            input_file = DATA_DIR / f"{dist}.txt"
            expected_file = os.path.join(tmpdir, "expected.txt")
            actual_file = os.path.join(tmpdir, "actual.txt")

            if not input_file.exists():
                print(f"  SKIP  {algo} + {dist}  (input file missing)")
                failed += 1
                continue

            generate_expected(str(input_file), expected_file)

            ok, err = run_test(config["run"], str(input_file), actual_file)
            if not ok:
                print(f"  FAIL  {algo} + {dist}  ({err})")
                failed += 1
                continue

            # Compare output
            diff = subprocess.run(
                ["diff", "-q", expected_file, actual_file],
                capture_output=True,
            )
            if diff.returncode == 0:
                print(f"  PASS  {algo} + {dist}")
                passed += 1
            else:
                print(f"  FAIL  {algo} + {dist}  (output mismatch)")
                failed += 1

    return passed, failed


# ============================================================
# INTERACTIVE MENU
# ============================================================

def pick_from_list(prompt, options):
    """Display a numbered menu and return the chosen option(s)."""
    print(f"\n{prompt}")
    print(f"  0) All")
    for i, opt in enumerate(options, 1):
        print(f"  {i}) {opt}")

    while True:
        try:
            choice = input("\nEnter choice: ").strip()
            if choice == "0":
                return options
            idx = int(choice) - 1
            if 0 <= idx < len(options):
                return [options[idx]]
        except (ValueError, EOFError):
            pass
        print("Invalid choice, try again.")


def interactive_mode(configs):
    """Run the interactive menu."""
    available = [lang for lang in configs if has_source_files(lang, configs[lang])]

    if not available:
        print("No languages with source files found!")
        return

    languages = pick_from_list("Choose a language to test:", available)
    algorithms = pick_from_list("Choose an algorithm to test:", ALGORITHMS)

    run_tests(configs, languages, algorithms)


def has_source_files(language, algo_configs):
    """Check if at least one algorithm has source files for this language."""
    for algo, config in algo_configs.items():
        run_cmd = config["run"]
        # For compiled languages, check if source exists
        if config["compile"]:
            for cmd in config["compile"]:
                # The source file is usually the last argument
                src = cmd[-1]
                if os.path.exists(src):
                    return True
        else:
            # For interpreted, check if the script file exists
            if len(run_cmd) > 1 and os.path.exists(run_cmd[-1]):
                return True
    return False


# ============================================================
# MAIN
# ============================================================

def run_tests(configs, languages, algorithms):
    """Run tests for the given languages and algorithms."""
    total_passed = 0
    total_failed = 0

    for lang in languages:
        if lang not in configs:
            print(f"\n⚠  Language '{lang}' is not configured.")
            continue

        lang_config = configs[lang]
        print(f"\n{'='*48}")
        print(f"  Testing: {lang}")
        print(f"{'='*48}")

        for algo in algorithms:
            if algo not in lang_config:
                print(f"\n  ⚠  Algorithm '{algo}' not configured for {lang}")
                continue

            p, f = test_algorithm(lang, algo, lang_config[algo])
            total_passed += p
            total_failed += f

    total = total_passed + total_failed
    print(f"\n{'='*48}")
    print(f"  Results: {total_passed}/{total} tests passed")
    if total_failed > 0:
        print(f"  ⚠  {total_failed} test(s) FAILED")
    else:
        print(f"  ✅ All tests passed!")
    print(f"{'='*48}")

    return total_failed


def main():
    parser = argparse.ArgumentParser(
        description="Unified test driver for sorting algorithms.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 test_driver.py                              # Interactive menu
  python3 test_driver.py --language java              # Test all Java algorithms
  python3 test_driver.py --language java --algorithm bubble   # Test one
  python3 test_driver.py --language java go            # Test Java and Go
  python3 test_driver.py --all                         # Test all languages
        """,
    )
    parser.add_argument(
        "--language", "-l", nargs="+",
        help="Language(s) to test (e.g. java python go)",
    )
    parser.add_argument(
        "--algorithm", "-a", nargs="+",
        choices=ALGORITHMS,
        help="Algorithm(s) to test (e.g. bubble merge)",
    )
    parser.add_argument(
        "--all", action="store_true",
        help="Test all available languages and algorithms",
    )

    args = parser.parse_args()
    configs = get_language_config()

    if args.all:
        languages = [l for l in configs if has_source_files(l, configs[l])]
        algorithms = ALGORITHMS
        if not languages:
            print("No languages with source files found!")
            sys.exit(1)
        failures = run_tests(configs, languages, algorithms)
        sys.exit(min(failures, 1))

    elif args.language:
        algorithms = args.algorithm or ALGORITHMS
        failures = run_tests(configs, args.language, algorithms)
        sys.exit(min(failures, 1))

    else:
        interactive_mode(configs)


if __name__ == "__main__":
    main()
