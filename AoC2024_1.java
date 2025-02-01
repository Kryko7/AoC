package AoC;

import java.io.*;
import java.util.*;

class AoC2024_1 {
    static int[] array1 = new int[1000];
    static int[] array2 = new int[1000];

    public static int distance(int[] arr1, int[] arr2) {
        int distance = 0;
        for (int i = 0; i < arr1.length; i++) {
            distance += Math.abs(arr1[i] - arr2[i]);
        }
        return distance;
    }

    public static int similarity(int arr1[], int arr2[]) {
        Map<Integer, Integer> map = new HashMap<>();
        for (int i = 0; i < arr1.length; i++) {
            map.put(array2[i], map.getOrDefault(array2[i], 0) + 1);
        }
        int similarity = 0;
        for (int i = 0; i < arr1.length; i++) {
            similarity += array1[i] * map.getOrDefault(array1[i], 0);
        }
        return similarity;
    }
    public static void parseArray(String filePath) {
        // Arrays to store the numbers
        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            int index = 0;

            while ((line = br.readLine()) != null && index < 1000) {
                // Split the line by space and parse the integers
                String[] numbers = line.split(" ");
                // System.out.println("Numbers: " + Arrays.toString(numbers));
                // System.out.println("Length: " + numbers.length);
                if (numbers.length == 4) {
                    array1[index] = Integer.parseInt(numbers[0]);
                    array2[index] = Integer.parseInt(numbers[3]);
                    index++;
                } else {
                    System.out.println("Skipping invalid line: " + line);
                }
            }

            // System.out.println("Array 1: " + Arrays.toString(array1));
            // System.out.println("Array 2: " + Arrays.toString(array2));

        } catch (IOException e) {
            e.printStackTrace();
        } catch (NumberFormatException e) {
            System.out.println("Error parsing numbers. Ensure the file contains valid integers.");
        }

        Arrays.sort(array1);
        Arrays.sort(array2);
    }

    public static void main(String[] args) {
        String filePath = "/home/minindu/AoC/AoC2024_1.txt";
        parseArray(filePath);
        int result = distance(array1, array2);
        int similarity = similarity(array1, array2);
        System.out.println("Distance between arrays: " + result);
        System.out.println("Similarity between arrays: " + similarity);

    }
}
