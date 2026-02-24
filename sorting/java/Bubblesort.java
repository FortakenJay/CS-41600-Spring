/*
   Course: CS 41600
   Professor: Dr. Dai
   Name: Jackson Trader
   ID: 17
   Date: 02/23/2026
   File: Bubblesort.java

   Description:
      Bubble Sort implementation in Java.
      Reads whitespace-separated values from standard input and prints the sorted values
      (ascending) to standard output, one value per line.

      Data Type Support:
      - If ALL input tokens can be parsed as numbers, sorting is numeric using BigDecimal
        (supports integers, decimals, scientific notation, and very large values).
      - Otherwise, sorting is lexicographic (String compare).

      Input Note:
      - If the first token is an integer that equals (totalTokens - 1), it is treated as
        a count header and ignored (common dataset format). Otherwise, all tokens are sorted.
*/

import java.io.BufferedInputStream;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.OutputStreamWriter;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.List;

public class Bubblesort {
   /**
    * A small container to keep the original token and its numeric value (when
    * applicable).
    */
   private static class TokenValue {
      public final String raw;
      public final BigDecimal num;

      /**
       * Method Javadoc.
       * 
       * Creates a TokenValue that stores the original input token and its parsed
       * numeric value.
       */
      public TokenValue(String raw, BigDecimal num) {
         this.raw = raw;
         this.num = num;
      }
   }

   /**
    * Method Javadoc.
    * 
    * Program entry point.
    * Reads tokens from standard input, optionally removes a count header, detects
    * whether
    * the data is numeric or not, bubble-sorts accordingly, then prints sorted
    * output to stdout.
    */
   public static void main(String[] args) throws Exception {
      List<String> tokens = readAllTokens();

      // If input is empty, print nothing.
      if (tokens.isEmpty()) {
         return;
      }

      // Optional: ignore a leading count header if it matches the remaining token
      // count.
      if (hasCountHeader(tokens)) {
         tokens.remove(0);
      }

      // If nothing left after removing header, print nothing.
      if (tokens.isEmpty()) {
         return;
      }

      // Try numeric mode first: if every token parses as BigDecimal, sort
      // numerically.
      TokenValue[] numericArr = new TokenValue[tokens.size()];
      boolean allNumeric = true;

      for (int i = 0; i < tokens.size(); i++) {
         String t = tokens.get(i);
         try {
            BigDecimal bd = new BigDecimal(t);
            numericArr[i] = new TokenValue(t, bd);
         } catch (NumberFormatException ex) {
            allNumeric = false;
            break;
         }
      }

      BufferedWriter out = new BufferedWriter(new OutputStreamWriter(System.out));

      if (allNumeric) {
         bubbleSortNumeric(numericArr);
         writeNumeric(out, numericArr);
      } else {
         String[] arr = tokens.toArray(new String[0]);
         bubbleSortStrings(arr);
         writeStrings(out, arr);
      }

      out.flush();
   }

   /**
    * Method Javadoc.
    * 
    * Reads all whitespace-separated tokens from standard input.
    * This supports inputs with one item per line or multiple items per line.
    */
   private static List<String> readAllTokens() throws IOException {
      FastScanner fs = new FastScanner();
      List<String> tokens = new ArrayList<>();

      String tok;
      while ((tok = fs.next()) != null) {
         tokens.add(tok);
      }

      return tokens;
   }

   /**
    * Method Javadoc.
    * 
    * Detects a common dataset format where the first token is a count header.
    * If the first token is an integer and equals (totalTokens - 1), treat it as a
    * header.
    */
   private static boolean hasCountHeader(List<String> tokens) {
      if (tokens.size() < 2) {
         return false;
      }

      String first = tokens.get(0);

      try {
         long n = Long.parseLong(first);
         return (n == (tokens.size() - 1));
      } catch (NumberFormatException ex) {
         return false;
      }
   }

   /**
    * Method Javadoc.
    * 
    * Bubble-sorts an array of strings in ascending lexicographic order.
    * Optimizations:
    * - Early exit if no swaps occur in a pass.
    * - Shrinks the "unsorted" end boundary each pass.
    */
   private static void bubbleSortStrings(String[] arr) {
      int n = arr.length;

      for (int end = n - 1; end > 0; end--) {
         boolean swapped = false;

         for (int i = 0; i < end; i++) {
            if (arr[i].compareTo(arr[i + 1]) > 0) {
               String tmp = arr[i];
               arr[i] = arr[i + 1];
               arr[i + 1] = tmp;
               swapped = true;
            }
         }

         if (!swapped) {
            break;
         }
      }
   }

   /**
    * Method Javadoc.
    * 
    * Bubble-sorts an array of TokenValue in ascending numeric order using
    * BigDecimal.
    * Stable behavior: if equal, it does not swap, preserving original order for
    * ties.
    * Optimizations:
    * - Early exit if no swaps occur in a pass.
    * - Shrinks the "unsorted" end boundary each pass.
    */
   private static void bubbleSortNumeric(TokenValue[] arr) {
      int n = arr.length;

      for (int end = n - 1; end > 0; end--) {
         boolean swapped = false;

         for (int i = 0; i < end; i++) {
            if (arr[i].num.compareTo(arr[i + 1].num) > 0) {
               TokenValue tmp = arr[i];
               arr[i] = arr[i + 1];
               arr[i + 1] = tmp;
               swapped = true;
            }
         }

         if (!swapped) {
            break;
         }
      }
   }

   /**
    * Method Javadoc.
    * 
    * Writes sorted string tokens to standard output, one per line.
    */
   private static void writeStrings(BufferedWriter out, String[] arr) throws IOException {
      for (String s : arr) {
         out.write(s);
         out.newLine();
      }
   }

   /**
    * Method Javadoc.
    * 
    * Writes sorted numeric tokens to standard output, one per line, preserving
    * original token text.
    */
   private static void writeNumeric(BufferedWriter out, TokenValue[] arr) throws IOException {
      for (TokenValue tv : arr) {
         out.write(tv.raw);
         out.newLine();
      }
   }

   /**
    * Method Javadoc.
    * 
    * A fast scanner for reading whitespace-separated tokens from stdin using a
    * byte buffer.
    * This is much faster than java.util.Scanner for large input files.
    */
   private static class FastScanner {
      private final BufferedInputStream in = new BufferedInputStream(System.in);
      private final byte[] buffer = new byte[1 << 16];
      private int ptr = 0;
      private int len = 0;

      /**
       * Method Javadoc.
       * 
       * Reads the next token from stdin, or returns null if end-of-file is reached.
       */
      public String next() throws IOException {
         StringBuilder sb = new StringBuilder();

         int c;

         // Skip leading whitespace.
         while (true) {
            c = readByte();
            if (c == -1) {
               return null;
            }
            if (!Character.isWhitespace((char) c)) {
               break;
            }
         }

         // Read until next whitespace or EOF.
         while (c != -1 && !Character.isWhitespace((char) c)) {
            sb.append((char) c);
            c = readByte();
         }

         return sb.toString();
      }

      /**
       * Method Javadoc.
       * 
       * Reads a single byte from the internal buffer, refilling the buffer as needed.
       * Returns -1 on EOF.
       */
      private int readByte() throws IOException {
         if (ptr >= len) {
            len = in.read(buffer);
            ptr = 0;
            if (len <= 0) {
               return -1;
            }
         }
         return buffer[ptr++] & 0xFF;
      }
   }
}
