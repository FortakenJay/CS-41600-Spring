/*
   Course: CS 41600
   Professor: Dr. Dai
   Name: Jackson Trader
   ID: 17
   Date: 2/23/2026
   File: InsertionSort.java

   Description:
      This program implements the Insertion Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Insertion Sort, and prints the sorted
      result to standard output (one integer per line). Input/output
      redirection is used to read from and write to files.
*/

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.IOException;
import java.util.ArrayList;

public class InsertionSort {
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
     * Insertion Sort algorithm.
     * 
     * Insertion Sort works by building a sorted portion of the list
     * one element at a time. For each element, it is compared against
     * the elements in the sorted portion and inserted into its correct
     * position by shifting larger elements one position to the right.
     * 
     * @param numbers the ArrayList of integers to sort
     */
    public static void insertionSort(ArrayList<Integer> numbers) {
        int n = numbers.size();

        for (int i = 1; i < n; i++) {
            int key = numbers.get(i);
            int j = i - 1;

            // Shift elements that are greater than key one position ahead
            while (j >= 0 && numbers.get(j) > key) {
                numbers.set(j + 1, numbers.get(j));
                j--;
            }

            // Insert the key into its correct position
            numbers.set(j + 1, key);
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
     * Main method that orchestrates the Insertion Sort program.
     * 
     * Reads integers from standard input, sorts them using Insertion
     * Sort, and prints the sorted result to standard output. This
     * program is designed to be used with input/output redirection,
     * for example:
     * java InsertionSort < input.txt > output.txt
     * 
     * @param args command-line arguments (not used)
     */
    public static void main(String[] args) {
        try {
            ArrayList<Integer> numbers = readInput();
            insertionSort(numbers);
            printOutput(numbers);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(1);
        }
    }
}
