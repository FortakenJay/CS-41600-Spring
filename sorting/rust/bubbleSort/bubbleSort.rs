/*
   Course: CS 41600
   Professor: Dr. Dai
   Date: 2/24/2026
   File: bubbleSort.rs

   Description:
      This program implements the Bubble Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Bubble Sort with early termination
      optimization, and prints the sorted result to standard output (one
      integer per line). Designed for use with input/output redirection and
      benchmarking.

   Usage:
      ./bubbleSort < input.txt > output.txt
      cat input.txt | ./bubbleSort
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

/// Sorts a slice of integers in ascending order using the Bubble Sort algorithm.
///
/// Bubble Sort works by repeatedly stepping through the list, comparing adjacent
/// elements, and swapping them if they are in the wrong order. This continues
/// until the list is sorted.
///
/// # Optimizations
/// * **Early termination**: If no swaps are made during a complete pass,
///   the list is already sorted and the algorithm stops early.
/// * **Reduced passes**: After each pass, the largest unsorted element is
///   guaranteed to be in its final position, so we reduce the comparison
///   range by one each iteration.
///
/// # Time Complexity
/// * Best case: O(n) when the array is already sorted
/// * Average case: O(n²)
/// * Worst case: O(n²)
///
/// # Space Complexity
/// * O(1) - sorts in place
///
/// # Arguments
/// * `numbers` - A mutable slice of i32 integers to sort
fn bubble_sort(numbers: &mut [i32]) {
    let n = numbers.len();
    
    // Outer loop: controls the number of passes
    for i in 0..n {
        let mut swapped = false;
        
        // Inner loop: compares adjacent elements
        // We subtract i because the last i elements are already in place
        for j in 0..n - i - 1 {
            if numbers[j] > numbers[j + 1] {
                // Swap adjacent elements if they're out of order
                numbers.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // Early termination: if no swaps occurred, the array is sorted
        if !swapped {
            break;
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

/// Main entry point for the bubble sort program.
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

    // Sort the numbers using bubble sort
    bubble_sort(&mut numbers);

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
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 2, 8, 2, 9, 1, 5, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 5, 5, 5, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, 5, -1, 0, -9, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![-9, -3, -1, 0, 2, 5]);
    }

    #[test]
    fn test_large_numbers() {
        let mut arr = vec![1000000, -1000000, 0, 999999, -999999];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![-1000000, -999999, 0, 999999, 1000000]);
    }
}
