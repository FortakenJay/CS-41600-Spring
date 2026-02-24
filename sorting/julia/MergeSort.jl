#=
   Course: CS 41600
   Professor: Dr. Dai
   Name: Dimitri Josifoski
   ID: 11
   Date: 02/24/2026
   File: MergeSort.jl

   Description:
      Implements Merge Sort, an O(n log n) divide-and-conquer algorithm. 
      It recursively splits the array into halves, sorts them 
      independently, and then merges the sorted halves back together 
      using a temporary buffer to maintain stability and efficiency.
=#

function merge_sort(arr)
    # Base case: arrays with 0 or 1 element are already sorted
    if length(arr) <= 1
        return arr
    end

    # Divide: Find the midpoint (using integer division ÷)
    mid = length(arr) ÷ 2
    left_half = merge_sort(arr[1:mid])
    right_half = merge_sort(arr[mid+1:end])

    # Conquer: Merge the two halves back together
    return merge_halves(left_half, right_half)
end

function merge_halves(left, right)
    # Pre-allocate the result array for efficiency
    result = Vector{eltype(left)}(undef, length(left) + length(right))
    i, j, k = 1, 1, 1

    while i <= length(left) && j <= length(right)
        if left[i] <= right[j]
            result[k] = left[i]
            i += 1
        else
            result[k] = right[j]
            j += 1
        end
        k += 1
    end

    # Copy any remaining elements
    while i <= length(left)
        result[k] = left[i]
        i += 1; k += 1
    end
    while j <= length(right)
        result[k] = right[j]
        j += 1; k += 1
    end

    return result
end
