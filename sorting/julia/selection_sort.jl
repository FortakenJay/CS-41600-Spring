#=
   Course: CS 41600
   Professor: Dr. Dai
   Name: Dimitri Josifoski
   ID: 11
   Date: 02/24/2026
   File: SelectionSort.jl

   Description:
      Implements Selection Sort, an O(n²) in-place comparison sort. 
      The algorithm divides the input list into a sorted and an 
      unsorted region, repeatedly finding the minimum element from 
      the unsorted part and moving it to the end of the sorted region.
=#


function selection_sort!(arr)
    n = length(arr)
    for i in 1:(n - 1)
        min_idx = i
        for j in (i + 1):n
            if arr[j] < arr[min_idx]
                min_idx = j
            end
        end
        arr[i], arr[min_idx] = arr[min_idx], arr[i]
    end
    return arr
end

function main()
    data = [parse(Int64, line) for line in eachline(stdin) if !isempty(strip(line))]
    if !isempty(data)
        selection_sort!(data)
        for num in data
            println(num)
        end
    end
end

main()
