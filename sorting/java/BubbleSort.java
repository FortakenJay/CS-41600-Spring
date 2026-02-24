/*
   Course: CS 41600
   Professor: Dr. Dai
   Name: Jackson Trader
   ID: 17
   Date: 2/23/2026
   File: BubbleSort.java

   Description:
      This program implements the Bubble Sort algorithm to sort a list of
      integers. It reads integers from standard input (one per line), sorts
      them in ascending order using Bubble Sort with early termination
      optimization, and prints the sorted result to standard output (one
      integer per line). Input/output redirection is used to read from and
      write to files.
*/

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.IOException;
import java.util.ArrayList;

public class BubbleSort {
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
     * Bubble Sort algorithm.
     * 
     * Bubble Sort works by repeatedly stepping through the list,
     * comparing adjacent elements, and swapping them if they are
     * in the wrong order. This implementation includes an early
     * termination optimization: if no swaps are made during a
     * complete pass, the list is already sorted and the algorithm
     * stops early.
     * 
     * @param numbers the ArrayList of integers to sort
     */
    public static void bubbleSort(ArrayList<Integer> numbers) {
        int n = numbers.size();

        for (int i = 0; i < n - 1; i++) {
            boolean swapped = false;

            for (int j = 0; j < n - i - 1; j++) {
                if (numbers.get(j) > numbers.get(j + 1)) {
                    // Swap adjacent elements
                    int temp = numbers.get(j);
                    numbers.set(j, numbers.get(j + 1));
                    numbers.set(j + 1, temp);
                    swapped = true;
                }
            }

            // If no swaps occurred, the array is already sorted
            if (!swapped) {
                break;
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
     * Main method that orchestrates the Bubble Sort program.
     * 
     * Reads integers from standard input, sorts them using Bubble
     * Sort, and prints the sorted result to standard output. This
     * program is designed to be used with input/output redirection,
     * for example:
     * java BubbleSort < input.txt > output.txt
     * 
     * @param args command-line arguments (not used)
     */
    public static void main(String[] args) {
        try {
            ArrayList<Integer> numbers = readInput();
            bubbleSort(numbers);
            printOutput(numbers);
        } catch (IOException e) {
            System.err.println("Error: " + e.getMessage());
            System.exit(1);
        }
    }
}
