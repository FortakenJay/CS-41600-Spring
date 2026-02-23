import os
import random
from pathlib import Path

# ==============================
# CONFIGURATION
# ==============================

OUTPUT_DIR = "sorting_test_suite"
SIZES = [10_000, 100_000, 500_000, 1_000_000]
SEED = 42
NEARLY_SORTED_SWAP_PERCENT = 0.01
FEW_UNIQUE_VALUES = 10

random.seed(SEED)


# ==============================
# DATA GENERATORS
# ==============================

def generate_random(size):
    return [random.randint(0, size) for _ in range(size)]

def generate_sorted(size):
    return list(range(size))

def generate_reverse(size):
    return list(range(size, 0, -1))

def generate_nearly_sorted(size):
    arr = list(range(size))
    swaps = int(size * NEARLY_SORTED_SWAP_PERCENT)
    for _ in range(swaps):
        i = random.randint(0, size - 1)
        j = random.randint(0, size - 1)
        arr[i], arr[j] = arr[j], arr[i]
    return arr

def generate_few_unique(size):
    return [random.randint(0, FEW_UNIQUE_VALUES - 1) for _ in range(size)]

def generate_identical(size):
    value = random.randint(0, size)
    return [value] * size


# ==============================
# FILE WRITING
# ==============================

def write_to_file(path, data):
    with open(path, "w") as f:
        f.write("\n".join(map(str, data)))


# ==============================
# MAIN TEST SUITE CREATION
# ==============================

def create_test_suite():
    base_path = Path(OUTPUT_DIR)
    base_path.mkdir(exist_ok=True)

    generators = {
        "random": generate_random,
        "sorted": generate_sorted,
        "reverse": generate_reverse,
        "nearly_sorted": generate_nearly_sorted,
        "few_unique": generate_few_unique,
        "identical": generate_identical
    }

    for size in SIZES:
        print(f"\nGenerating datasets for size: {size}")
        size_dir = base_path / str(size)
        size_dir.mkdir(exist_ok=True)

        for name, generator in generators.items():
            print(f"  -> {name}")
            data = generator(size)
            file_path = size_dir / f"{name}.txt"
            write_to_file(file_path, data)

    print("\n✅ Test suite generation complete.")
    print(f"Output directory: {OUTPUT_DIR}")


if __name__ == "__main__":
    create_test_suite()