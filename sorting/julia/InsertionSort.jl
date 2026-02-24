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
