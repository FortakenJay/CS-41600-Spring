/*
   Course: CS 41600
   Professor: Dr. Dai
   Date: 2/24/2026
   File: insertionSort.rs

   Description:
      This program implements the Insertion Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Insertion Sort, and prints the sorted
      result to standard output (one integer per line). Designed for use with
      input/output redirection and benchmarking.

   Usage:
      ./insertionSort < input.txt > output.txt
      cat input.txt | ./insertionSort
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

/// Sorts a slice of integers in ascending order using the Insertion Sort algorithm.
///
/// Insertion Sort works by building a sorted portion of the array one element at a
/// time. For each element, it is compared against elements in the sorted portion
/// and inserted into its correct position by shifting larger elements one position
/// to the right.
///
/// The algorithm maintains two regions:
/// * **Sorted region**: Elements from index 0 to i-1
/// * **Unsorted region**: Elements from index i to n-1
///
/// For each iteration, the algorithm takes the first element from the unsorted
/// region (the "key") and inserts it into its correct position in the sorted region.
///
/// # Time Complexity
/// * Best case: O(n) when the array is already sorted (only comparisons, no shifts)
/// * Average case: O(n²)
/// * Worst case: O(n²) when the array is reverse sorted
///
/// # Space Complexity
/// * O(1) - sorts in place
///
/// # Advantages
/// * Efficient for small datasets
/// * Adaptive: efficient for data that is already substantially sorted
/// * Stable: maintains relative order of equal elements
/// * Online: can sort a list as it receives it
///
/// # Arguments
/// * `numbers` - A mutable slice of i32 integers to sort
fn insertion_sort(numbers: &mut [i32]) {
    let n = numbers.len();
    
    // Start from the second element (index 1) since a single element is already sorted
    for i in 1..n {
        let key = numbers[i];
        let mut j = i;
        
        // Shift elements of the sorted portion that are greater than key
        // one position to the right
        while j > 0 && numbers[j - 1] > key {
            numbers[j] = numbers[j - 1];
            j -= 1;
        }
        
        // Insert the key into its correct position
        numbers[j] = key;
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

/// Main entry point for the insertion sort program.
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

    // Sort the numbers using insertion sort
    insertion_sort(&mut numbers);

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
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![42];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_order() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 2, 8, 2, 9, 1, 5, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 2, 5, 5, 5, 8, 9]);
    }

    #[test]
    fn test_negative_numbers() {
        let mut arr = vec![-3, 5, -1, 0, -9, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![-9, -3, -1, 0, 2, 5]);
    }

    #[test]
    fn test_large_numbers() {
        let mut arr = vec![1000000, -1000000, 0, 999999, -999999];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![-1000000, -999999, 0, 999999, 1000000]);
    }

    #[test]
    fn test_two_elements() {
        let mut arr = vec![2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2]);
    }

    #[test]
    fn test_stability() {
        // While i32 doesn't show stability, this tests the algorithm works correctly
        let mut arr = vec![3, 3, 1, 2, 3];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 3, 3]);
    }
}
