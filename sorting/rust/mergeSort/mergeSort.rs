/*
   Course: CS 41600
   Professor: Dr. Dai
   Date: 2/24/2026
   File: mergeSort.rs

   Description:
      This program implements the Merge Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Merge Sort, and prints the sorted
      result to standard output (one integer per line). Designed for use with
      input/output redirection and benchmarking.

   Usage:
      ./mergeSort < input.txt > output.txt
      cat input.txt | ./mergeSort
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

/// Sorts a slice of integers in ascending order using the Merge Sort algorithm.
///
/// Merge Sort is a divide-and-conquer algorithm that works by:
/// 1. **Divide**: Split the array into two halves
/// 2. **Conquer**: Recursively sort each half
/// 3. **Combine**: Merge the two sorted halves back together
///
/// This implementation creates a temporary buffer once and reuses it throughout
/// all recursive calls for improved memory efficiency.
///
/// # Time Complexity
/// * Best case: O(n log n)
/// * Average case: O(n log n)
/// * Worst case: O(n log n)
///
/// # Space Complexity
/// * O(n) - requires additional space for temporary buffer
///
/// # Advantages
/// * Guaranteed O(n log n) performance - no worst case degradation
/// * Stable: maintains relative order of equal elements
/// * Predictable: performance doesn't depend on input distribution
/// * Parallelizable: left and right halves can be sorted independently
///
/// # Disadvantages
/// * Requires O(n) extra space
/// * Not in-place (unlike quicksort variants)
///
/// # Arguments
/// * `numbers` - A mutable slice of i32 integers to sort
pub fn merge_sort(numbers: &mut [i32]) {
    let len = numbers.len();
    if len <= 1 {
        return;
    }
    
    // Allocate temporary buffer once for all merge operations
    let mut buffer = vec![0; len];
    merge_sort_helper(numbers, &mut buffer, 0, len - 1);
}

/// Helper function for merge sort that performs the recursive divide-and-conquer.
///
/// # Arguments
/// * `arr` - The array slice to sort
/// * `buffer` - Temporary buffer for merging (reused across recursive calls)
/// * `left` - Starting index of the portion to sort (inclusive)
/// * `right` - Ending index of the portion to sort (inclusive)
fn merge_sort_helper(arr: &mut [i32], buffer: &mut [i32], left: usize, right: usize) {
    if left < right {
        // Find the middle point to divide the array into two halves
        let mid = left + (right - left) / 2;
        
        // Recursively sort the left and right halves
        merge_sort_helper(arr, buffer, left, mid);
        merge_sort_helper(arr, buffer, mid + 1, right);
        
        // Merge the two sorted halves
        merge(arr, buffer, left, mid, right);
    }
}

/// Merges two sorted subarrays into a single sorted subarray.
///
/// The first subarray spans from index `left` to `mid`, and the second
/// subarray spans from index `mid+1` to `right`. Uses a temporary buffer
/// to hold the merged result before copying back to the original array.
///
/// # Arguments
/// * `arr` - The array containing the two subarrays to merge
/// * `buffer` - Temporary buffer for storing merged elements
/// * `left` - Starting index of the first subarray
/// * `mid` - Ending index of the first subarray
/// * `right` - Ending index of the second subarray
fn merge(arr: &mut [i32], buffer: &mut [i32], left: usize, mid: usize, right: usize) {
    // Copy the range we're working with into the buffer
    for i in left..=right {
        buffer[i] = arr[i];
    }
    
    let mut i = left;      // Index for left subarray
    let mut j = mid + 1;   // Index for right subarray
    let mut k = left;      // Index for merged array
    
    // Merge elements from both subarrays in sorted order
    while i <= mid && j <= right {
        if buffer[i] <= buffer[j] {
            arr[k] = buffer[i];
            i += 1;
        } else {
            arr[k] = buffer[j];
            j += 1;
        }
        k += 1;
    }
    
    // Copy any remaining elements from the left subarray
    while i <= mid {
        arr[k] = buffer[i];
        i += 1;
        k += 1;
    }
    
    // Copy any remaining elements from the right subarray
    // Note: if right subarray is exhausted first, elements are already in place
    while j <= right {
        arr[k] = buffer[j];
        j += 1;
        k += 1;
    }
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

/// Main entry point for the merge sort program.
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

    // Sort the numbers using merge sort
    merge_sort(&mut numbers);

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
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 2, 8, 2, 9, 1, 5, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 5, 5, 5, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, 5, -1, 0, -9, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![-9, -3, -1, 0, 2, 5]);
    }

    #[test]
    fn test_large_numbers() {
        let mut arr = vec![1000000, -1000000, 0, 999999, -999999];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![-1000000, -999999, 0, 999999, 1000000]);
    }

    #[test]
    fn test_two_elements_sorted() {
        let mut arr = vec![1, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_two_elements_unsorted() {
        let mut arr = vec![2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        merge_sort(&mut arr);
        let expected: Vec<i32> = (0..1000).collect();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_stability() {
        // While i32 doesn't show stability, this tests correct handling of equal elements
        let mut arr = vec![3, 3, 1, 2, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 3, 3]);
    }
}
