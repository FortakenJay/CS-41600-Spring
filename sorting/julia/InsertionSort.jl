#=
   Course: CS 41600
   Professor: Dr. Dai
   Name: Dimitri Josifoski
   ID: 11
   Date: 02/24/2026
   File: InsertionSort.jl

   Description:
      Implements Insertion Sort, an O(n²) algorithm that builds the 
      final sorted array one item at a time. It is highly efficient 
      for small datasets or partially sorted arrays, functioning 
      similarly to the way people manually sort a hand of cards.
=#

function insertion_sort!(arr)
    for i in 2:length(arr)
        key = arr[i]
        j = i - 1
            
        # Shift elements that are greater than key to the right
        while j >= 1 && arr[j] > key
            arr[j + 1] = arr[j]
            j -= 1
        end
        arr[j + 1] = key
    end
    return arr
end
