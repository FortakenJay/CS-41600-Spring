function bubble_sort!(arr)
    n = length(arr)
    for i in 1:n
        # The last i elements are already sorted
        for j in 1:(n - i)
            if arr[j] > arr[j + 1]
                # Julia's elegant swap (Tuple destructuring)
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
            end
        end
    end
    return arr
end
