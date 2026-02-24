"""
Course: CS 41600
Professor: Dr. Dai
Name: Fabian Rojas
ID: 13
Date: 02/23/2026
File: bubblesort.py
"""

"""
Description:
        Bubble Sort implementation in Python.
        Module function `bubble_sort(arr)` that performs an in-place bubble sort.

Data Type Support:
        - Sorting is performed using Python comparison operators on the elements
            of the supplied list.
        - Numeric types (integers, `decimal.Decimal`, floats, etc.) are sorted
            numerically.
        - Strings are sorted lexicographically (standard string comparison).
"""


def bubble_sort(arr):
    # Calculate the number of elements once so we don't recompute it.
    n = len(arr)

    # Outer loop: we will make at most 'n' full passes through the list.
    # After each pass the largest remaining unsorted element "bubbles" to
    # its correct position at the end of the array, so we can reduce the
    # inner loop range by 'i' on subsequent passes.
    for i in range(n):
        # Track whether any swaps happened during this pass. If no swaps
        # happen, the list is already sorted and we can exit early.
        swapped = False

        # Inner loop: compare adjacent items and swap if out of order.
        # We stop at 'n - i - 1' because the last 'i' elements are already
        # guaranteed to be in correct position after 'i' passes.
        for j in range(0, n - i - 1):
            # If the left element is greater than the right element,
            # swap them to move the larger element to the right.
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True

        # If no swaps occurred in the entire pass, the list is sorted.
        if not swapped:
            break

    # Return the (now sorted) list. Note that bubble sort operates in-place
    # on the provided list and also returns it for convenience.
    return arr


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

    # Sort using bubble_sort
    sorted_data = bubble_sort(list(data))

    # Output one value per line
    for item in sorted_data:
        print(item)

