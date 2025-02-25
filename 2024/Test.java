import java.util.ArrayList;
import java.util.List;

public class Test {
    public static List<int[]> findRectangularOutlines(int[][] grid) {
        List<int[]> rectangles = new ArrayList<>();
        int rows = grid.length;
        int cols = grid[0].length;

        // Find all rectangular outlines
        for (int r1 = 0; r1 < rows; r1++) {
            for (int c1 = 0; c1 < cols; c1++) {
                for (int r2 = r1 + 1; r2 < rows; r2++) {
                    for (int c2 = c1 + 1; c2 < cols; c2++) {
                        if (isValidRectangle(grid, r1, c1, r2, c2)) {
                            rectangles.add(new int[]{r1, c1, r2, c2});
                        }
                    }
                }
            }
        }

        return rectangles;
    }

    private static boolean isValidRectangle(int[][] grid, int r1, int c1, int r2, int c2) {
        // Check top border
        for (int c = c1; c <= c2; c++) {
            if (grid[r1][c] != 2) return false;
        }

        // Check bottom border
        for (int c = c1; c <= c2; c++) {
            if (grid[r2][c] != 2) return false;
        }

        // Check left border
        for (int r = r1; r <= r2; r++) {
            if (grid[r][c1] != 2) return false;
        }

        // Check right border
        for (int r = r1; r <= r2; r++) {
            if (grid[r][c2] != 2) return false;
        }

        // Ensure rectangle is at least 2x2
        return (r2 - r1 >= 1) && (c2 - c1 >= 1);
    }

    private static int loopCount(List<int[]> rectangles, int[][] grid) {
        int count = 0;
        for (int[] rect : rectangles) {
            int r1 = rect[0];
            int c1 = rect[1];
            int r2 = rect[2];
            int c2 = rect[3];
            try {
                int top_left_1 = grid[r1][c1-1];
                int top_left_2 = grid[r1-1][c1];

                int top_right_1 = grid[r1][c2+1];
                int top_right_2 = grid[r1-1][c2];

                int bottom_left_1 = grid[r2][c1-1];
                int bottom_left_2 = grid[r2+1][c1];

                int bottom_right_1 = grid[r2][c2+1];
                int bottom_right_2 = grid[r2+1][c2];

                int cond = 0;

                if (top_left_1 == 1 || top_left_2 == 1) cond++;
                if (top_right_1 == 1 || top_right_2 == 1) cond++;
                if (bottom_left_1 == 1 || bottom_left_2 == 1) cond++;
                if (bottom_right_1 == 1 || bottom_right_2 == 1) cond++;

                if (cond >= 3) {
                    if (top_left_2 + top_right_1 + bottom_left_1 + bottom_right_2 >= 3) {
                        count++;
                        System.out.println("Rect: " + r1 + " " + c1 + " " + r2 + " " + c2);
                    } else if (top_left_1 + top_right_2 + bottom_left_2 + bottom_right_1 >= 3) {
                        count++;
                        System.out.println("Rect: " + r1 + " " + c1 + " " + r2 + " " + c2);
                    }
                }

            } catch (Exception e) {
                continue;
            }
        }
        return count;
    }
    public static void main(String[] args) {
        int[][] grid = {
            {0, 0, 0, 0, 1, 0, 0, 0, 0, 0},
            {0, 0, 0, 0, 2, 2, 2, 2, 2, 1},
            {0, 0, 0, 0, 0, 0, 0, 0, 2, 0},
            {0, 0, 1, 0, 0, 0, 0, 0, 2, 0},
            {0, 0, 2, 2, 2, 2, 2, 1, 2, 0},
            {0, 0, 2, 0, 0, 0, 2, 0, 2, 0},
            {0, 1, 2, 2, 2, 2, 2, 2, 2, 0},
            {0, 2, 2, 2, 2, 2, 2, 2, 1, 0},
            {1, 2, 2, 2, 2, 2, 2, 2, 0, 0},
            {0, 0, 0, 0, 0, 0, 1, 2, 0, 0}
        };

        List<int[]> rectangles = findRectangularOutlines(grid);
        
        // System.out.println("Number of rectangular outlines: " + rectangles.size());
        // System.out.println("Rectangular outlines (top-left, bottom-right coordinates):");
        // for (int[] rect : rectangles) {
        //     System.out.printf("(%d, %d, %d, %d)%n", rect[0], rect[1], rect[2], rect[3]);
        // }
        int count = loopCount(rectangles, grid);
        System.out.println("Count: " + count);
    }
}