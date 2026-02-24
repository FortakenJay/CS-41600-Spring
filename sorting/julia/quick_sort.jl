#=
   Course: CS 41600
   Professor: Dr. Dai
   Name: Dimitri Josifoski
   ID: 11
   Date: 02/24/2026
   File: QuickSort.jl

   Description:
      Implements Quick Sort using the Lomuto partition scheme. This 
      O(n log n) algorithm selects a 'pivot' element and partitions 
      the other elements into two sub-arrays according to whether 
      they are less than or greater than the pivot.
=#

function quick_sort!(arr, low=1, high=length(arr))
    if low < high
        p = partition!(arr, low, high)
        quick_sort!(arr, low, p - 1)
        quick_sort!(arr, p + 1, high)
    end
    return arr
end

function partition!(arr, low, high)
    pivot = arr[high]
    i = low - 1
    for j in low:(high - 1)
        if arr[j] <= pivot
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
        end
    end
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1
end

function main()
    data = [parse(Int64, line) for line in eachline(stdin) if !isempty(strip(line))]
    if !isempty(data)
        quick_sort!(data)
        for num in data
            println(num)
        end
    end
end

main()
