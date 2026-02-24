function quick_sort!(arr, low=1, high=length(arr))
    if low < high
        # Partition the array and get the pivot index
        p = partition!(arr, low, high)
        
        # Recursively sort elements before and after partition
        quick_sort!(arr, low, p - 1)
        quick_sort!(arr, p + 1, high)
    end
    return arr
end

function partition!(arr, low, high)
    pivot = arr[high]
    i = low - 1 # Index of smaller element
    
    for j in low:(high - 1)
        if arr[j] <= pivot
            i += 1
            arr[i], arr[j] = arr[j], arr[i]
        end
    end
    
    # Swap the pivot element to its final sorted position
    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return i + 1
end
