/*
   Course: CS 41600
   Professor: Dr. Dai
   Date: 2/24/2026
   File: selectionSort.rs

   Description:
      This program implements the Selection Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Selection Sort, and prints the sorted
      result to standard output (one integer per line). Designed for use with
      input/output redirection and benchmarking.

   Usage:
      ./selectionSort < input.txt > output.txt
      cat input.txt | ./selectionSort
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

/// Sorts a slice of integers in ascending order using the Selection Sort algorithm.
///
/// Selection Sort works by dividing the array into two regions:
/// * **Sorted region**: Elements from index 0 to i-1 (grows with each iteration)
/// * **Unsorted region**: Elements from index i to n-1 (shrinks with each iteration)
///
/// For each iteration:
/// 1. Find the minimum element in the unsorted region
/// 2. Swap it with the first element of the unsorted region
/// 3. The sorted region grows by one element
///
/// This process continues until the entire array is sorted.
///
/// # Time Complexity
/// * Best case: O(n²) - always performs the same number of comparisons
/// * Average case: O(n²)
/// * Worst case: O(n²)
///
/// # Space Complexity
/// * O(1) - sorts in place with only a constant amount of extra space
///
/// # Advantages
/// * Simple to understand and implement
/// * Minimizes the number of swaps (at most n-1 swaps)
/// * In-place sorting (O(1) extra space)
/// * Performance is not affected by initial order of elements
///
/// # Disadvantages
/// * O(n²) time complexity even in best case
/// * Not stable (relative order of equal elements may change)
/// * Not adaptive (doesn't benefit from partially sorted data)
/// * Poor performance on large datasets
///
/// # Arguments
/// * `numbers` - A mutable slice of i32 integers to sort
fn selection_sort(numbers: &mut [i32]) {
    let n = numbers.len();
    
    // Iterate through the array, expanding the sorted region
    for i in 0..n.saturating_sub(1) {
        // Find the index of the minimum element in the unsorted region
        let mut min_index = i;
        
        for j in (i + 1)..n {
            if numbers[j] < numbers[min_index] {
                min_index = j;
            }
        }
        
        // Swap the minimum element with the first unsorted element
        // Only swap if necessary (optimization)
        if min_index != i {
            numbers.swap(i, min_index);
        }
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

/// Main entry point for the selection sort program.
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

    // Sort the numbers using selection sort
    selection_sort(&mut numbers);

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
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 2, 8, 2, 9, 1, 5, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 5, 5, 5, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, 5, -1, 0, -9, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![-9, -3, -1, 0, 2, 5]);
    }

    #[test]
    fn test_large_numbers() {
        let mut arr = vec![1000000, -1000000, 0, 999999, -999999];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![-1000000, -999999, 0, 999999, 1000000]);
    }

    #[test]
    fn test_two_elements_sorted() {
        let mut arr = vec![1, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_two_elements_unsorted() {
        let mut arr = vec![2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_all_same_elements() {
        let mut arr = vec![7, 7, 7, 7, 7];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![7, 7, 7, 7, 7]);
    }

    #[test]
    fn test_large_array() {
        let mut arr: Vec<i32> = (0..100).rev().collect();
        selection_sort(&mut arr);
        let expected: Vec<i32> = (0..100).collect();
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_minimum_swaps() {
        // Selection sort minimizes swaps - test with array needing minimal swaps
        let mut arr = vec![2, 1, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
