/*
   Course: CS 41600
   Professor: Dr. Dai
   Date: 2/24/2026
   File: quickSort.rs

   Description:
      This program implements the Quick Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Quick Sort with the Lomuto partition
      scheme, and prints the sorted result to standard output (one integer
      per line). Designed for use with input/output redirection and
      benchmarking.

   Usage:
      ./quickSort < input.txt > output.txt
      cat input.txt | ./quickSort
*/

use std::io::{self, BufRead, BufWriter, Write};
use std::process;

/// Reads integers from standard input, one per line.
///
/// Uses a buffered reader for efficient I/O when handling large input files.
/// Empty lines and whitespace-only lines are skipped. The function will
/// return an error if a line cannot be parsed as an i32.
///
/// # Returns
/// * `io::Result<Vec<i32>>` - A vector of integers read from stdin
///
/// # Errors
/// * Returns an error if reading from stdin fails or if a line cannot be
///   parsed as an integer
fn read_input() -> io::Result<Vec<i32>> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        
        if !trimmed.is_empty() {
            match trimmed.parse::<i32>() {
                Ok(num) => numbers.push(num),
                Err(e) => {
                    eprintln!("Error parsing '{}': {}", trimmed, e);
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to parse integer: {}", trimmed)
                    ));
                }
            }
        }
    }

    Ok(numbers)
}

/// Sorts a slice of integers in ascending order using the Quick Sort algorithm.
///
/// Quick Sort is a divide-and-conquer algorithm that works by:
/// 1. **Select**: Choose a pivot element from the array
/// 2. **Partition**: Rearrange array so elements < pivot come before it,
///    and elements > pivot come after it
/// 3. **Conquer**: Recursively sort the subarrays on either side of the pivot
///
/// This implementation uses the Lomuto partition scheme with the last element
/// as the pivot. The algorithm sorts in-place for O(log n) space complexity
/// (due to recursion stack in the average case).
///
/// # Time Complexity
/// * Best case: O(n log n) when partitions are balanced
/// * Average case: O(n log n)
/// * Worst case: O(n²) when array is already sorted or reverse sorted
///   (with last-element pivot)
///
/// # Space Complexity
/// * O(log n) average case - recursion stack depth
/// * O(n) worst case - recursion stack depth for unbalanced partitions
///
/// # Advantages
/// * In-place sorting (no extra array needed like merge sort)
/// * Cache-friendly due to sequential access patterns
/// * Very fast in practice for random data
/// * Good average-case performance
///
/// # Disadvantages
/// * Not stable (relative order of equal elements may change)
/// * Worst-case O(n²) performance on sorted/reverse-sorted data
/// * Recursive implementation can cause stack overflow on very large arrays
///
/// # Arguments
/// * `numbers` - A mutable slice of i32 integers to sort
pub fn quick_sort(numbers: &mut [i32]) {
    let len = numbers.len();
    if len <= 1 {
        return;
    }
    quick_sort_helper(numbers, 0, (len - 1) as isize);
}

/// Helper function for quick sort that performs the recursive sorting.
///
/// # Arguments
/// * `arr` - The array slice to sort
/// * `low` - Starting index of the portion to sort (inclusive)
/// * `high` - Ending index of the portion to sort (inclusive)
fn quick_sort_helper(arr: &mut [i32], low: isize, high: isize) {
    if low < high {
        // Partition the array and get the pivot index
        let pivot_index = partition(arr, low, high);
        
        // Recursively sort elements before and after the pivot
        quick_sort_helper(arr, low, pivot_index - 1);
        quick_sort_helper(arr, pivot_index + 1, high);
    }
}

/// Partitions a portion of the array around a pivot element using the
/// Lomuto partition scheme.
///
/// The last element in the range is chosen as the pivot. All elements
/// less than or equal to the pivot are moved to the left side, and all
/// elements greater are moved to the right side. The pivot is then
/// placed in its correct sorted position.
///
/// After partitioning:
/// * Elements at indices [low..pivot_index-1] are ≤ pivot
/// * Element at index pivot_index is the pivot (in final position)
/// * Elements at indices [pivot_index+1..high] are > pivot
///
/// # Arguments
/// * `arr` - The array to partition
/// * `low` - Starting index of the portion to partition
/// * `high` - Ending index of the portion to partition (contains pivot)
///
/// # Returns
/// * `isize` - The final index of the pivot element
fn partition(arr: &mut [i32], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = low - 1;
    
    // Iterate through the range, moving smaller elements to the left
    for j in low..high {
        if arr[j as usize] <= pivot {
            i += 1;
            // Swap arr[i] and arr[j]
            arr.swap(i as usize, j as usize);
        }
    }
    
    // Place the pivot in its correct position
    arr.swap((i + 1) as usize, high as usize);
    
    i + 1
}

/// Prints each integer in the vector to standard output, one per line.
///
/// Uses a buffered writer for efficient I/O when handling large output.
///
/// # Arguments
/// * `numbers` - A slice of integers to print
///
/// # Returns
/// * `io::Result<()>` - Ok if successful, Err if writing fails
///
/// # Errors
/// * Returns an error if writing to stdout fails
fn print_output(numbers: &[i32]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for &num in numbers {
        writeln!(writer, "{}", num)?;
    }

    writer.flush()?;
    Ok(())
}

/// Main entry point for the quick sort program.
///
/// Orchestrates reading from stdin, sorting, and writing to stdout.
/// Exits with status code 1 if any errors occur during execution.
fn main() {
    // Read integers from standard input
    let mut numbers = match read_input() {
        Ok(nums) => nums,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            process::exit(1);
        }
    };

    // Sort the numbers using quick sort
    quick_sort(&mut numbers);

    // Print the sorted numbers to standard output
    if let Err(e) = print_output(&numbers) {
        eprintln!("Error writing output: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 2, 8, 2, 9, 1, 5, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 5, 5, 5, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, 5, -1, 0, -9, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![-9, -3, -1, 0, 2, 5]);
    }

    #[test]
    fn test_large_numbers() {
        let mut arr = vec![1000000, -1000000, 0, 999999, -999999];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![-1000000, -999999, 0, 999999, 1000000]);
    }

    #[test]
    fn test_two_elements_sorted() {
        let mut arr = vec![1, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_two_elements_unsorted() {
        let mut arr = vec![2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_all_same_elements() {
        let mut arr = vec![7, 7, 7, 7, 7];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        quick_sort(&mut arr);
        let expected: Vec<i32> = (0..1000).collect();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_partition_simple() {
        let mut arr = vec![3, 1, 4, 1, 5];
        let pivot_idx = partition(&mut arr, 0, 4);
        // After partition with pivot=5 (last element):
        // All elements before pivot_idx should be <= 5
        // Element at pivot_idx should be 5
        // All elements after pivot_idx should be > 5
        assert_eq!(arr[pivot_idx as usize], 5);
        for i in 0..pivot_idx as usize {
            assert!(arr[i] <= 5);
        }
    }
}
