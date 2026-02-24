"""
Course: CS 41600
Professor: Dr. Dai
Name: Fabian Rojas
ID: 13
Date: 02/23/2026
File: selectionsort.py
"""

"""
Description:
        Selection Sort implementation in Python.
        Module function `selection_sort(arr)` that performs an in-place selection sort.

Data Type Support:
        - Sorting is performed using Python comparison operators on the elements
            of the supplied list.
        - Numeric types (integers, `decimal.Decimal`, floats, etc.) are sorted
            numerically.
        - Strings are sorted lexicographically (standard string comparison).
"""


def selection_sort(arr):
    # Number of elements in the list
    n = len(arr)

    # We iterate over each position in the array and place the correct
    # minimum element for that position.
    for i in range(n):
        # Assume the minimum is at the current position.
        min_idx = i

        # Search the unsorted portion (i+1..n-1) for a smaller element.
        for j in range(i + 1, n):
            # If we find a smaller item, remember its index.
            if arr[j] < arr[min_idx]:
                min_idx = j

        # After scanning the unsorted portion, swap the smallest found
        # element into the current position 'i'. This grows the sorted
        # portion of the array by one element.
        arr[i], arr[min_idx] = arr[min_idx], arr[i]

    # The list has been sorted in-place; return it for convenience.
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

    # Sort using selection_sort
    sorted_data = selection_sort(list(data))

    # Output one value per line
    for item in sorted_data:
        print(item)

