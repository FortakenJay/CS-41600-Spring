/*
   Course: CS 41600
   Professor: Dr. Dai
   Name: Jackson Trader
   ID: 17
   Date: 2/23/2026
   File: QuickSort.java

   Description:
      This program implements the Quick Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Quick Sort with a last-element pivot
      and Lomuto partition scheme, and prints the sorted result to standard
      output (one integer per line). Input/output redirection is used to
      read from and write to files.
*/

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.IOException;
import java.util.ArrayList;

public class QuickSort {
    /**
     * Reads integers from standard input, one per line.
     * 
     * Uses a BufferedReader for efficient I/O when handling
     * large input files. Blank lines are skipped.
     * 
     * @return an ArrayList of integers read from stdin
     * @throws IOException if an I/O error occurs while reading
     */
    public static ArrayList<Integer> readInput() throws IOException {
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        ArrayList<Integer> numbers = new ArrayList<>();
        String line;

        while ((line = reader.readLine()) != null) {
            line = line.trim();
            if (!line.isEmpty()) {
                numbers.add(Integer.parseInt(line));
            }
        }

        return numbers;
    }

    /**
     * Sorts a portion of an array in ascending order using the
     * Quick Sort algorithm.
     * 
     * Quick Sort is a divide-and-conquer algorithm that selects a
     * pivot element, partitions the array so that elements less than
     * the pivot come before it and elements greater come after it,
     * then recursively sorts the two partitions. It has O(n log n)
     * average time complexity and O(n^2) worst case.
     * 
     * @param arr  the array of integers to sort
     * @param low  the starting index of the portion to sort (inclusive)
     * @param high the ending index of the portion to sort (inclusive)
     */
    public static void quickSort(int[] arr, int low, int high) {
        if (low < high) {
            int pivotIndex = partition(arr, low, high);

            // Recursively sort elements before and after the pivot
            quickSort(arr, low, pivotIndex - 1);
            quickSort(arr, pivotIndex + 1, high);
        }
    }

    /**
     * Partitions a portion of the array around a pivot element
     * using the Lomuto partition scheme.
     * 
     * The last element in the range is chosen as the pivot. All
     * elements smaller than or equal to the pivot are moved to
     * the left side, and all elements greater are moved to the
     * right side. The pivot is then placed in its correct sorted
     * position.
     * 
     * @param arr  the array to partition
     * @param low  the starting index of the portion to partition
     * @param high the ending index of the portion to partition (pivot)
     * @return the final index of the pivot element
     */
    public static int partition(int[] arr, int low, int high) {
        int pivot = arr[high];
        int i = low - 1;

        for (int j = low; j < high; j++) {
            if (arr[j] <= pivot) {
                i++;

                // Swap arr[i] and arr[j]
                int temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }

        // Place the pivot in its correct position
        int temp = arr[i + 1];
        arr[i + 1] = arr[high];
        arr[high] = temp;

        return i + 1;
    }

    /**
     * Prints each integer in the array to standard output,
     * one per line.
     * 
     * Uses a BufferedWriter for efficient I/O when handling
     * large output.
     * 
     * @param arr the sorted array of integers to print
     * @throws IOException if an I/O error occurs while writing
     */
    public static void printOutput(int[] arr) throws IOException {
        BufferedWriter writer = new BufferedWriter(new OutputStreamWriter(System.out));

        for (int num : arr) {
            writer.write(Integer.toString(num));
            writer.newLine();
        }

        writer.flush();
    }

    /**
     * Main method that orchestrates the Quick Sort program.
     * 
     * Reads integers from standard input, converts them to an array,
     * sorts them using Quick Sort, and prints the sorted result to
     * standard output. This program is designed to be used with
     * input/output redirection, for example:
     * java QuickSort < input.txt > output.txt
     * 
     * @param args command-line arguments (not used)
     */
    public static void main(String[] args) {
        try {
            ArrayList<Integer> numbers = readInput();

            // Convert ArrayList to int array for efficient recursive sorting
            int[] arr = new int[numbers.size()];
            for (int i = 0; i < numbers.size(); i++) {
                arr[i] = numbers.get(i);
            }

            quickSort(arr, 0, arr.length - 1);
            printOutput(arr);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(1);
        }
    }
}
