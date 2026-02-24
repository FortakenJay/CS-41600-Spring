"""
Course: CS 41600
Professor: Dr. Dai
Name: Fabian Rojas
ID: 13
Date: 02/23/2026
File: mergesort.py
"""

"""
Description:
        Merge Sort implementation in Python.
        Module function `merge_sort(arr)` that performs a merge sort and returns a
        new sorted list (does not modify the original list in-place).

Data Type Support:
        - Sorting is performed using Python comparison operators on the elements
            of the supplied list.
        - Numeric types (integers, `decimal.Decimal`, floats, etc.) are sorted
            numerically.
        - Strings are sorted lexicographically (standard string comparison).
"""


def merge_sort(arr):

    # Base case: a list of zero or one elements is already sorted.
    if len(arr) <= 1:
        return arr

    # Divide: split the list approximately in half.
    mid = len(arr) // 2
    left = merge_sort(arr[:mid])   # Recursively sort the left half
    right = merge_sort(arr[mid:])  # Recursively sort the right half

    # Conquer: merge the two sorted halves and return the merged result.
    return merge(left, right)


def merge(left, right):
    # Merge two sorted lists into a single sorted list.
    result = []
    i = j = 0

    # Walk through both lists, always taking the smaller of the two
    # current elements and appending it to the result. This preserves
    # ordering and runs in linear time relative to the total number
    # of elements.
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1

    # If one list still has remaining elements, they are already sorted
    # and greater than all previously appended elements, so we can extend
    # the result list with the remaining slice.
    result.extend(left[i:])
    result.extend(right[j:])
    return result


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

    # Sort using merge_sort
    sorted_data = merge_sort(list(data))

    # Output one value per line
    for item in sorted_data:
        print(item)

