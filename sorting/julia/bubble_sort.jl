#=
   Course: CS 41600
   Professor: Dr. Dai
   Name: Dimitri Josifoski
   ID: 11
   Date: 02/24/2026
   File: BubbleSort.jl

   Description:
      Implements the Bubble Sort algorithm, an O(n²) comparison sort. 
      It functions by repeatedly stepping through the list, comparing 
      adjacent elements and swapping them if they are in the wrong 
      order until the entire array is sorted.
=#

function bubble_sort!(arr)
    n = length(arr)
    for i in 1:n
        for j in 1:(n - i)
            if arr[j] > arr[j + 1]
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
            end
        end
    end
    return arr
end

function main()
    data = [parse(Int64, line) for line in eachline(stdin) if !isempty(strip(line))]
    if !isempty(data)
        bubble_sort!(data)
        for num in data
            println(num)
        end
    end
end

main()
