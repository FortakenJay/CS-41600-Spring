"""
Course: CS 41600
Professor: Dr. Dai
Name: Fabian Rojas
ID: 13
Date: 02/23/2026
File: quicksort.py
"""

"""
Description:
        Quick Sort implementation in Python.
        Module function `quick_sort(arr)` that performs a quick sort and returns a
        new sorted list (this implementation builds partitions and is not in-place).

Data Type Support:
        - Sorting is performed using Python comparison operators on the elements
            of the supplied list.
        - Numeric types (integers, `decimal.Decimal`, floats, etc.) are sorted
            numerically.
        - Strings are sorted lexicographically (standard string comparison).
"""


def quick_sort(arr):
    # Base case: lists with 0 or 1 elements are already sorted.
    if len(arr) <= 1:
        return arr

    # Choose a pivot element. Here we choose the middle element to help
    # avoid worst-case behavior on already sorted inputs (choosing first
    # or last element can degrade to O(n^2) for sorted data).
    pivot = arr[len(arr) // 2]

    # Partition: build three lists using list comprehensions.
    # - left: elements less than pivot
    # - middle: elements equal to pivot (keeps duplicates together)
    # - right: elements greater than pivot
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]

    # Recursively sort the left and right partitions and concatenate
    # the results. This implementation uses additional memory for the
    # partitions; an in-place partitioning scheme can reduce memory.
    return quick_sort(left) + middle + quick_sort(right)


if __name__ == "__main__":
    import sys
    from decimal import Decimal, InvalidOperation, getcontext

    # Read all input tokens from stdin
    tokens = []
    for line in sys.stdin:
        parts = line.strip().split()
        tokens.extend(parts)

    if not tokens:
        sys.exit(0)

    # Try to parse all tokens as Decimal numbers; if any fail, fall back to strings
    data = []
    max_digits = 0
    for t in tokens:
        digits_only = "".join(ch for ch in t if ch.isdigit())
        if len(digits_only) > max_digits:
            max_digits = len(digits_only)

    try:
        getcontext().prec = max(28, max_digits + 5)
    except Exception:
        pass

    try:
        for t in tokens:
            data.append(Decimal(t))
    except (InvalidOperation, ValueError):
        # Fallback to strings
        data = tokens

    # Sort using quick_sort
    sorted_data = quick_sort(list(data))

    # Output one value per line
    for item in sorted_data:
        print(item)

