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
    if length(arr) <= 1
        return arr
    end
    mid = length(arr) ÷ 2
    left = merge_sort(arr[1:mid])
    right = merge_sort(arr[mid+1:end])
    return merge_halves(left, right)
end

function merge_halves(left, right)
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
    while i <= length(left)
        result[k] = left[i]; i += 1; k += 1
    end
    while j <= length(right)
        result[k] = right[j]; j += 1; k += 1
    end
    return result
end

function main()
    data = [parse(Int64, line) for line in eachline(stdin) if !isempty(strip(line))]
    if !isempty(data)
        # Merge sort is not in-place, so we catch the return
        sorted_data = merge_sort(data)
        for num in sorted_data
            println(num)
        end
    end
end

main()
