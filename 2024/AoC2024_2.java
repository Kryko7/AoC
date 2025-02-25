import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;

public class AoC2024_2 {
    public static int countSafe(int[][] grid) {
        int safeCount = 0;
        int newOrientation = 0;
        for (int[] row : grid) {
            int n = row.length;
            int orientation = row[0] < row[1] ? 1 : -1;
            int firstDiff = Math.abs(row[0] - row[1]);
            if (firstDiff < 1 || firstDiff > 3) {
                continue;
            }
            boolean valid = true;
            for(int i=2; i<n; i++) {
                if (orientation == 1) {
                    if (row[i] - row[i-1] < 1 || row[i] - row[i-1] > 3) {
                        valid = false;
                        break;
                    }
                } else {
                    if (row[i-1] - row[i] < 1 || row[i-1] - row[i] > 3) {
                        valid = false;
                        break;
                    }
                }
                newOrientation = row[i-1] < row[i] ? 1 : -1;
                if (newOrientation != orientation) {
                    valid = false;
                    break;
                } 
            }
            if (valid) {
                safeCount++;
            }
        }
        return safeCount;
    }


    public static boolean isRowSafe(int[] row) {
        int newOrientation = 0;
        boolean valid = true;
        int n = row.length;
        int orientation = row[0] < row[1] ? 1 : -1;
        int firstDiff = Math.abs(row[0] - row[1]);
        if (firstDiff < 1 || firstDiff > 3) {
            return false;
        }
        for(int i=2; i<n; i++) {
            if (orientation == 1) {
                if (row[i] - row[i-1] < 1 || row[i] - row[i-1] > 3) {
                    valid = false;
                    break;
                }
            } else {
                if (row[i-1] - row[i] < 1 || row[i-1] - row[i] > 3) {
                    valid = false;
                    break;
                }
            }
            newOrientation = row[i-1] < row[i] ? 1 : -1;
            if (newOrientation != orientation) {
                valid = false;
                break;
            } 
        }
        return valid;
    }

    private static int[] removeElement(int[] array, int index) {
        int[] newArray = new int[array.length - 1];
        System.arraycopy(array, 0, newArray, 0, index);
        System.arraycopy(array, index + 1, newArray, index, array.length - index - 1);
        return newArray;
    }

    public static int countSafePartTwo(int[][] grid) {
        int safeCount = 0;
        for (int[] row : grid) {
            if (isRowSafe(row)) {
                safeCount++;
                continue;
            }
            boolean canFix = false;
            for (int i = 0; i < row.length; i++) {
                int[] modified = removeElement(row, i);
                if (isRowSafe(modified)) {
                    canFix = true;
                    break;
                }
            }
            if (canFix) {
                safeCount++;
            }
        }
        return safeCount;
    }

    public static int[][] parse(String filePath) {
        ArrayList<int[]> listOfArrays = new ArrayList<>();

        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = br.readLine()) != null) {
                line = line.trim();
                if (!line.isEmpty()) { // Skip empty lines
                    String[] tokens = line.split(" ");
                    int[] numbers = Arrays.stream(tokens).mapToInt(Integer::parseInt).toArray();
                    listOfArrays.add(numbers);
                }
            }
        } catch (IOException e) {
            e.printStackTrace();
        }

        // Convert the list to an array of arrays
        int[][] result = listOfArrays.toArray(new int[0][]);

        // Print the result for verification
        System.out.println("Parsed Array of Arrays:");
        for (int[] row : result) {
            System.out.println(Arrays.toString(row));
        }
        return result;
    }    

    public static void main(String[] args) {
        int[][] grid = parse("/home/minindu/AoC/AoC2024_2.txt");
        int[][] array = {
            {7, 6, 4, 2, 1},
            {1, 2, 7, 8, 9},
            {9, 7, 6, 2, 1},
            {1, 3, 2, 4, 5},
            {8, 6, 4, 4, 1},
            {1, 3, 6, 7, 9}
        };
        int safeCount = countSafe(grid);
        int safeCount2 = countSafePartTwo(grid);
        System.out.println("Safe Count: " + safeCount);
        System.out.println("Safe Count 2: " + safeCount2);
    }
}
