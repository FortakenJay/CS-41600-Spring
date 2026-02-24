/*
   Course: CS 41600
   Professor: Dr. Dai
   Name: Jackson Trader
   ID: 17
   Date: 2/23/2026
   File: SelectionSort.java

   Description:
      This program implements the Selection Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Selection Sort, and prints the sorted
      result to standard output (one integer per line). Input/output
      redirection is used to read from and write to files.
*/

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.IOException;
import java.util.ArrayList;

public class SelectionSort {
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
     * Sorts an ArrayList of integers in ascending order using the
     * Selection Sort algorithm.
     * 
     * Selection Sort works by dividing the list into a sorted and
     * unsorted region. It repeatedly finds the minimum element from
     * the unsorted region and swaps it with the first unsorted
     * element, growing the sorted region by one each pass.
     * 
     * @param numbers the ArrayList of integers to sort
     */
    public static void selectionSort(ArrayList<Integer> numbers) {
        int n = numbers.size();

        for (int i = 0; i < n - 1; i++) {
            // Find the index of the minimum element in the unsorted region
            int minIndex = i;

            for (int j = i + 1; j < n; j++) {
                if (numbers.get(j) < numbers.get(minIndex)) {
                    minIndex = j;
                }
            }

            // Swap the minimum element with the first unsorted element
            if (minIndex != i) {
                int temp = numbers.get(i);
                numbers.set(i, numbers.get(minIndex));
                numbers.set(minIndex, temp);
            }
        }
    }

    /**
     * Prints each integer in the ArrayList to standard output,
     * one per line.
     * 
     * Uses a BufferedWriter for efficient I/O when handling
     * large output.
     * 
     * @param numbers the sorted ArrayList of integers to print
     * @throws IOException if an I/O error occurs while writing
     */
    public static void printOutput(ArrayList<Integer> numbers) throws IOException {
        BufferedWriter writer = new BufferedWriter(new OutputStreamWriter(System.out));

        for (int num : numbers) {
            writer.write(Integer.toString(num));
            writer.newLine();
        }

        writer.flush();
    }

    /**
     * Main method that orchestrates the Selection Sort program.
     * 
     * Reads integers from standard input, sorts them using Selection
     * Sort, and prints the sorted result to standard output. This
     * program is designed to be used with input/output redirection,
     * for example:
     * java SelectionSort < input.txt > output.txt
     * 
     * @param args command-line arguments (not used)
     */
    public static void main(String[] args) {
        try {
            ArrayList<Integer> numbers = readInput();
            selectionSort(numbers);
            printOutput(numbers);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(1);
        }
    }
}
