import random

SIZE = 1_000_000   # 1 million numbers

# 1. Random data
with open("random.txt", "w") as f:
    for _ in range(SIZE):
        f.write(str(random.randint(0, SIZE)) + "\n")

# 2. Sorted data
with open("sorted.txt", "w") as f:
    for i in range(SIZE):
        f.write(str(i) + "\n")

# 3. Reverse sorted
with open("reverse.txt", "w") as f:
    for i in range(SIZE, 0, -1):
        f.write(str(i) + "\n")

# 4. Nearly sorted (1% random swaps)
arr = list(range(SIZE))
for _ in range(SIZE // 100):
    i = random.randint(0, SIZE-1)
    j = random.randint(0, SIZE-1)
    arr[i], arr[j] = arr[j], arr[i]

with open("nearly_sorted.txt", "w") as f:
    for num in arr:
        f.write(str(num) + "\n")