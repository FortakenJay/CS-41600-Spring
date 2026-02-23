import os
import json
import subprocess
import statistics
import platform
import time
from pathlib import Path
from datetime import datetime

# ===============================
# CONFIGURATION
# ===============================

DATASET_DIR = "datasets"
ALGORITHM_DIR = "algorithms"
RESULTS_DIR = "results"
REPETITIONS = 5

SIZES = [10000, 100000, 500000, 1000000]
DISTRIBUTIONS = [
    "random",
    "sorted",
    "reverse",
    "nearly_sorted",
    "few_unique",
    "identical"
]

LANGUAGES = ["python", "java", "julia", "go", "rust"]

TIME_COMMAND = ["/usr/bin/time", "-v"]

# ===============================
# UTILITIES
# ===============================

def ensure_directories():
    Path(RESULTS_DIR).mkdir(exist_ok=True)
    Path(f"{RESULTS_DIR}/logs").mkdir(exist_ok=True)

def get_machine_metadata():
    return {
        "timestamp": datetime.utcnow().isoformat(),
        "os": platform.platform(),
        "processor": platform.processor(),
        "cpu_count": os.cpu_count(),
        "python_version": platform.python_version(),
    }

def parse_time_output(stderr_text):
    metrics = {}

    for line in stderr_text.splitlines():
        if "Elapsed (wall clock) time" in line:
            metrics["wall_time"] = line.split(":")[-1].strip()
        elif "User time (seconds)" in line:
            metrics["user_time"] = float(line.split(":")[-1].strip())
        elif "System time (seconds)" in line:
            metrics["system_time"] = float(line.split(":")[-1].strip())
        elif "Maximum resident set size" in line:
            metrics["max_rss_kb"] = int(line.split(":")[-1].strip())

    return metrics

def run_command(command):
    process = subprocess.Popen(
        TIME_COMMAND + command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    stdout, stderr = process.communicate()
    metrics = parse_time_output(stderr)
    return stdout, stderr, process.returncode, metrics

# ===============================
# BENCHMARK CORE
# ===============================

def benchmark_algorithm(language, size, distribution):
    dataset_path = f"{DATASET_DIR}/{size}/{distribution}.txt"
    executable = f"{ALGORITHM_DIR}/{language}/sort"

    run_results = []

    for run in range(REPETITIONS):
        print(f"  Run {run+1}/{REPETITIONS}")

        stdout, stderr, code, metrics = run_command(
            [executable, dataset_path]
        )

        log_file = f"{RESULTS_DIR}/logs/{language}_{size}_{distribution}_run{run}.log"
        with open(log_file, "w") as f:
            f.write(stdout)
            f.write("\n--- STDERR ---\n")
            f.write(stderr)

        run_results.append({
            "exit_code": code,
            "metrics": metrics
        })

    return run_results

# ===============================
# SUMMARY STATISTICS
# ===============================

def summarize_runs(runs):
    wall_times = []
    memory = []

    for r in runs:
        if "user_time" in r["metrics"]:
            wall_times.append(r["metrics"]["user_time"])
        if "max_rss_kb" in r["metrics"]:
            memory.append(r["metrics"]["max_rss_kb"])

    return {
        "avg_time": statistics.mean(wall_times) if wall_times else None,
        "stddev_time": statistics.stdev(wall_times) if len(wall_times) > 1 else 0,
        "avg_memory_kb": statistics.mean(memory) if memory else None
    }

# ===============================
# MAIN BENCHMARK
# ===============================

def main():
    ensure_directories()

    metadata = get_machine_metadata()
    with open("metadata.json", "w") as f:
        json.dump(metadata, f, indent=4)

    all_results = {}

    for language in LANGUAGES:
        print(f"\n=== Language: {language} ===")
        all_results[language] = {}

        for size in SIZES:
            for dist in DISTRIBUTIONS:
                print(f"\nBenchmarking {language} | Size: {size} | {dist}")

                runs = benchmark_algorithm(language, size, dist)
                summary = summarize_runs(runs)

                all_results[language][f"{size}_{dist}"] = {
                    "runs": runs,
                    "summary": summary
                }

    with open(f"{RESULTS_DIR}/raw_runs.json", "w") as f:
        json.dump(all_results, f, indent=4)

    print("\nBenchmarking Complete")
    print("Results saved to:", RESULTS_DIR)

if __name__ == "__main__":
    main()