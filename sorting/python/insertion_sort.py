"""
Course: CS 41600
Professor: Dr. Dai
Name: Fabian Rojas
ID: 13
Date: 02/23/2026
File: insertionsort.py
"""

"""
Description:
        Insertion Sort implementation in Python.
        Module function `insertion_sort(arr)` that performs an in-place insertion sort.

Data Type Support:
        - Sorting is performed using Python comparison operators on the elements
            of the supplied list.
        - Numeric types (integers, `decimal.Decimal`, floats, etc.) are sorted
            numerically.
        - Strings are sorted lexicographically (standard string comparison).
"""


def insertion_sort(arr):
    # Start from the second element (index 1) and insert each element
    # into the sorted portion to its left.
    for i in range(1, len(arr)):
        # The value to insert into the sorted subarray
        key = arr[i]
        # Start comparing with the element immediately to the left
        j = i - 1

        # Shift elements of the sorted subarray [0..i-1] that are greater
        # than 'key' one position to the right to make space.
        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1

        # Place 'key' into the correct location after shifting.
        # After the loop, j points to the element just before the insertion
        # point, so insert at j+1.
        arr[j + 1] = key

    # Return the list (sorted in-place) for convenience.
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

    # Sort using insertion_sort
    sorted_data = insertion_sort(list(data))

    # Output one value per line
    for item in sorted_data:
        print(item)

