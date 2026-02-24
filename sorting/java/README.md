# Java Sorting Algorithms

Five sorting algorithm implementations in Java.

## Algorithms

| File | Algorithm | Time Complexity |
|---|---|---|
| `BubbleSort.java` | Bubble Sort | O(n²) |
| `SelectionSort.java` | Selection Sort | O(n²) |
| `InsertionSort.java` | Insertion Sort | O(n²) |
| `MergeSort.java` | Merge Sort | O(n log n) |
| `QuickSort.java` | Quick Sort | O(n log n) avg, O(n²) worst |

## Compile & Run

```bash
# Compile
javac BubbleSort.java

# Run with input/output redirection
java BubbleSort < input.txt > output.txt
```

Replace `BubbleSort` with any of the other class names above.

## Testing with Project Data

```bash
# Sort 10,000 random integers and verify against expected output
javac MergeSort.java
java MergeSort < ../test/data/input_data/10000/random.txt > output.txt
sort -n ../test/data/input_data/10000/random.txt > expected.txt
diff output.txt expected.txt
```

No output from `diff` means the sort is correct.

## Notes

- Each program reads **one integer per line** from stdin and writes sorted output the same way to stdout.
- No external dependencies — just `java.io` and `java.util.ArrayList`.
- MergeSort and QuickSort use `int[]` arrays internally for efficient recursive sorting.
- BubbleSort includes an early-termination optimization (stops if a pass has no swaps).
