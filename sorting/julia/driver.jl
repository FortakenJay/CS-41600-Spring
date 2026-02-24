#=
   Course: CS 41600
   Professor: Dr. Dai
   Name: Dimitri Josifoski
   ID: 11
   Date: 02/24/2026
   File: driver.jl

   Description:
      The main execution script for the sorting project. It handles 
      standard input redirection to parse numerical datasets, 
      provides a user interface for algorithm selection, and 
      orchestrates the calls to the various sorting implementations.
=#

# Include the separate files
include("BubbleSort.jl")
include("SelectionSort.jl")
include("InsertionSort.jl")
include("MergeSort.jl")
include("QuickSort.jl")

function read_input()
    # Read the redirected file from stdin
    numbers = Int64[]
    for line in eachline(stdin)
        for word in split(line)
            val = tryparse(Int64, word)
            if val !== nothing
                push!(numbers, val)
            end
        end
    end
    return numbers
end

function main()
    # Step 1: Parse the data immediately
    data = read_input()
    
    if isempty(data)
        println("Error: No data found in the redirected input.")
        return
    end
    
    println("--- Data Loaded: $(length(data)) elements ---")
    
    # Step 2: Show Menu
    # Since stdin was used for the file, we look at ARGS (command line arguments)
    # OR we prompt the user. Note: Interactive menus with redirected stdin 
    # can be tricky, so it's often better to pass the choice as an argument.
    
    println("Select a sorting algorithm:")
    println("1. Bubble Sort")
    println("2. Selection Sort")
    println("3. Insertion Sort")
    println("4. Merge Sort")
    println("5. Quick Sort")
    print("Enter choice (1-5): ")
    
    # Re-opening the TTY to get user input because stdin is piped
    choice_str = readline(open("/dev/tty")) # Use "CON" on Windows
    choice = tryparse(Int, choice_str)

    sorted_data = copy(data) # Keep original safe
    
    if choice == 1
        bubble_sort!(sorted_data)
    elseif choice == 2
        selection_sort!(sorted_data)
    elseif choice == 3
        insertion_sort!(sorted_data)
    elseif choice == 4
        sorted_data = merge_sort(sorted_data) # Merge sort usually isn't in-place
    elseif choice == 5
        quick_sort!(sorted_data)
    else
        println("Invalid choice.")
        return
    end

    println("\nSorting Complete!")
    println("First 10 elements: ", sorted_data[1:min(10, end)])
end

main()
