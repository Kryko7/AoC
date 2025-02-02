import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.Arrays;
import java.util.List;

public class AoC2024_4 {
    static int m = 1000;
    static int n = 1000;
    static List<Character> charList = Arrays.asList('X', 'M', 'A', 'S');
    static char[][] grid;

    public static char[][] parseGrid(String filePath) {
        char[][] grid  = new char[m][n];
        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            int index = 0;
            while ((line = br.readLine()) != null && index < m) {
                char[] chars = line.toCharArray();
                int k = chars.length;
                for (int i=0; i<k; i++) {
                    grid[index][i] = chars[i];
                }
                index++;
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
        return grid;
    }

    public static int countWordsPartTwo(int x, int y) {
        int count = 0;
        if( x + 1 < m && y + 1 < n && x - 1 >= 0 && y - 1 >= 0) {
            if (grid[x+1][y+1] == 'M' && grid[x-1][y-1] == 'S') {
                if (grid[x-1][y+1] == 'M' && grid[x+1][y-1] == 'S') {
                    count++;
                } else if (grid[x-1][y+1] == 'S' && grid[x+1][y-1] == 'M') {
                    count++;
                }
            } else if (grid[x+1][y+1] == 'S' && grid[x-1][y-1] == 'M') {
                if (grid[x-1][y+1] == 'M' && grid[x+1][y-1] == 'S') {
                    count++;
                } else if (grid[x-1][y+1] == 'S' && grid[x+1][y-1] == 'M') {
                    count++;
                }
            }
        }
        return count;
    }
    public static int countWordsPartOne(int x, int y) {
        int diagonal1 = x + 3 < m && y + 3 < n ? 1 : 0;
        int diagonal2 = x + 3 < m && y - 3 >= 0 ? 1 : 0;
        int diagonal3 = x - 3 >= 0 && y + 3 < n ? 1 : 0;
        int diagonal4 = x - 3 >= 0 && y - 3 >= 0 ? 1 : 0;
        int vertical1 = x + 3 < m ? 1 : 0;
        int vertical2 = x - 3 >= 0 ? 1 : 0;
        int horizontal1 = y + 3 < n ? 1 : 0;
        int horizontal2 = y - 3 >= 0 ? 1 : 0;
        int count = 0;
        if (diagonal1 == 1) {
            if (grid[x][y] == 'X' && grid[x+1][y+1] == 'M' && grid[x+2][y+2] == 'A' && grid[x+3][y+3] == 'S') {
                count++;
            }
        } 
        if (diagonal2 == 1) {
            if (grid[x][y] == 'X' && grid[x+1][y-1] == 'M' && grid[x+2][y-2] == 'A' && grid[x+3][y-3] == 'S') {
                count++;
            }
        }
        if (diagonal3 == 1) {
            if (grid[x][y] == 'X' && grid[x-1][y+1] == 'M' && grid[x-2][y+2] == 'A' && grid[x-3][y+3] == 'S') {
                count++;
            }
        }
        if (diagonal4 == 1) {
            if (grid[x][y] == 'X' && grid[x-1][y-1] == 'M' && grid[x-2][y-2] == 'A' && grid[x-3][y-3] == 'S') {
                count++;
            }
        }
        if (vertical1 == 1) {
            if (grid[x][y] == 'X' && grid[x+1][y] == 'M' && grid[x+2][y] == 'A' && grid[x+3][y] == 'S') {
                count++;
            }
        }
        if (vertical2 == 1) {
            if (grid[x][y] == 'X' && grid[x-1][y] == 'M' && grid[x-2][y] == 'A' && grid[x-3][y] == 'S') {
                count++;
            }
        }
        if (horizontal1 == 1) {
            if (grid[x][y] == 'X' && grid[x][y+1] == 'M' && grid[x][y+2] == 'A' && grid[x][y+3] == 'S') {
                count++;
            }
        }
        if (horizontal2 == 1) {
            if (grid[x][y] == 'X' && grid[x][y-1] == 'M' && grid[x][y-2] == 'A' && grid[x][y-3] == 'S') {
                count++;
            }
        }
        return count;
    }
    public static void main(String[] args) {
        grid = parseGrid("/home/minindu/AoC/AoC2024_4.txt");
        m = grid.length;
        n = grid[0].length;
        int count1 = 0;
        int count2 = 0;
        for (int i=0; i<m; i++) {
            for (int j=0; j<n; j++) {
                if (grid[i][j] == 'X') {
                    count1 += countWordsPartOne(i, j);
                    System.out.println("CountOne: " + count1);
                }
                if (grid[i][j] == 'A') {
                    count2 += countWordsPartTwo(i, j);
                    System.out.println("CountTwo: " + count2);
                }
            }
        }
        System.out.println(count1);
        System.out.println(count2);
    }
}
