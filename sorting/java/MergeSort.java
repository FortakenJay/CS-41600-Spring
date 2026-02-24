/*
   Course: CS 41600
   Professor: Dr. Dai
   Name: Jackson Trader
   ID: 17
   Date: 2/23/2026
   File: MergeSort.java

   Description:
      This program implements the Merge Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Merge Sort, and prints the sorted
      result to standard output (one integer per line). Input/output
      redirection is used to read from and write to files.
*/

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.IOException;
import java.util.ArrayList;

public class MergeSort {
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
     * Merge Sort algorithm.
     * 
     * Merge Sort is a divide-and-conquer algorithm that recursively
     * splits the array into two halves, sorts each half, and then
     * merges the two sorted halves back together. It has O(n log n)
     * time complexity in all cases (best, average, and worst).
     * 
     * @param arr   the array of integers to sort
     * @param left  the starting index of the portion to sort (inclusive)
     * @param right the ending index of the portion to sort (inclusive)
     */
    public static void mergeSort(int[] arr, int left, int right) {
        if (left < right) {
            int mid = left + (right - left) / 2;

            // Recursively sort the left and right halves
            mergeSort(arr, left, mid);
            mergeSort(arr, mid + 1, right);

            // Merge the two sorted halves
            merge(arr, left, mid, right);
        }
    }

    /**
     * Merges two sorted subarrays into a single sorted subarray.
     * 
     * The first subarray spans from index left to mid, and the
     * second subarray spans from index mid+1 to right. A temporary
     * array is used to hold the merged result before copying it
     * back into the original array.
     * 
     * @param arr   the array containing the two subarrays to merge
     * @param left  the starting index of the first subarray
     * @param mid   the ending index of the first subarray
     * @param right the ending index of the second subarray
     */
    public static void merge(int[] arr, int left, int mid, int right) {
        int n1 = mid - left + 1;
        int n2 = right - mid;

        // Create temporary arrays for the left and right halves
        int[] leftArr = new int[n1];
        int[] rightArr = new int[n2];

        for (int i = 0; i < n1; i++) {
            leftArr[i] = arr[left + i];
        }
        for (int j = 0; j < n2; j++) {
            rightArr[j] = arr[mid + 1 + j];
        }

        // Merge the temporary arrays back into the original array
        int i = 0;
        int j = 0;
        int k = left;

        while (i < n1 && j < n2) {
            if (leftArr[i] <= rightArr[j]) {
                arr[k] = leftArr[i];
                i++;
            } else {
                arr[k] = rightArr[j];
                j++;
            }
            k++;
        }

        // Copy any remaining elements from the left half
        while (i < n1) {
            arr[k] = leftArr[i];
            i++;
            k++;
        }

        // Copy any remaining elements from the right half
        while (j < n2) {
            arr[k] = rightArr[j];
            j++;
            k++;
        }
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
     * Main method that orchestrates the Merge Sort program.
     * 
     * Reads integers from standard input, converts them to an array,
     * sorts them using Merge Sort, and prints the sorted result to
     * standard output. This program is designed to be used with
     * input/output redirection, for example:
     * java MergeSort < input.txt > output.txt
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

            mergeSort(arr, 0, arr.length - 1);
            printOutput(arr);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(1);
        }
    }
}
